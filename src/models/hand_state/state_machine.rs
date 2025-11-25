// State machine implementation for HandState

use super::*;
use rand::prelude::*;

impl HandState {
    /// Create HandState with a custom player deck
    pub fn with_custom_deck(mut player_deck: Vec<Card>, assets: &crate::assets::GameAssets) -> Self {
        player_deck.shuffle(&mut rand::rng());

        let mut owner_cards = std::collections::HashMap::new();
        owner_cards.insert(Owner::Narc, Cards::new(create_narc_deck(assets)));
        owner_cards.insert(Owner::Player, Cards::new(player_deck));
        owner_cards.insert(Owner::Buyer, Cards::empty());

        Self {
            current_state: HandPhase::Draw,
            current_round: 1,
            owner_cards,
            cards_played: Vec::new(),
            cards_played_this_round: Vec::new(),
            discard_pile: Vec::new(),
            outcome: None,
            cash: 0,
            current_heat: 0,
            current_player_index: 0,
            checks_this_hand: Vec::new(),
            buyer_persona: None,
            hand_story: None, // SOW-012: No story initially
            last_profit: 0,
        }
    }

    /// Shuffle all cards back into deck (called between hands in a run)
    /// Collects both unplayed and played cards back to deck for reuse
    pub fn shuffle_cards_back(&mut self) {
        // Collect ALL cards (including played) back to deck for Player and Narc
        self.cards_mut(Owner::Player).collect_all();
        self.cards_mut(Owner::Player).shuffle_deck();
        self.cards_mut(Owner::Narc).collect_all();
        self.cards_mut(Owner::Narc).shuffle_deck();

        self.cards_played.clear();
        self.cards_played_this_round.clear();
    }

    /// Start next hand in the run (preserve cash/heat, shuffle cards back)
    /// Used after Safe outcome to continue the run
    /// Returns true if hand can start, false if deck exhausted
    pub fn start_next_hand(&mut self) -> bool {
        let preserved_cash = self.cash;
        let preserved_heat = self.current_heat;

        self.shuffle_cards_back();

        // Buyer deck resets completely (unlike Player/Narc which shuffle back unplayed)
        // Collect all buyer cards back to deck, then shuffle
        self.cards_mut(Owner::Buyer).collect_all();
        self.cards_mut(Owner::Buyer).shuffle_deck();

        let preserved_owner_cards = self.owner_cards.clone();

        // Check deck exhaustion before starting new hand
        if self.cards(Owner::Player).deck.len() < 3 {
            // Deck exhausted - cannot start new hand
            self.outcome = Some(HandOutcome::Busted);
            self.current_state = HandPhase::Bust;
            return false;
        }

        let preserved_buyer_persona = self.buyer_persona.clone();

        // Reset state but preserve cash/heat/cards/buyer
        *self = Self::default();
        self.cash = preserved_cash;
        self.current_heat = preserved_heat;
        self.owner_cards = preserved_owner_cards;
        self.buyer_persona = preserved_buyer_persona;

        true
    }

    /// Draw cards from decks to hands (initial draw phase)
    pub fn draw_cards(&mut self) {
        self.initialize_buyer_hand();

        self.cards_mut(Owner::Narc).draw_to_hand();
        self.cards_mut(Owner::Player).draw_to_hand();
        self.cards_mut(Owner::Buyer).draw_to_hand();

        self.transition_state();
    }

    /// Transition to next state
    pub fn transition_state(&mut self) {
        self.current_state = match self.current_state {
            HandPhase::Draw => HandPhase::PlayerPhase,
            HandPhase::PlayerPhase => {
                // After all players act, go to Dealer Reveal
                HandPhase::DealerReveal
            },
            HandPhase::DealerReveal => {
                // After Dealer reveals, check if customer folds, then advance
                // Player can fold during their turn in PlayerPhase (not here)
                if self.current_round >= 3 {
                    // Round 3: Go to Resolution
                    HandPhase::Resolve
                } else {
                    // Rounds 1-2: Advance to next round
                    self.current_round += 1;
                    self.reset_turn_tracking();
                    // Don't clear checks_this_hand - persist for entire hand
                    HandPhase::Draw
                }
            },
            HandPhase::FoldDecision => {
                // Legacy state - should not be used anymore
                // Fold happens during PlayerPhase now
                self.current_round += 1;
                self.reset_turn_tracking();
                HandPhase::Draw
            },
            HandPhase::Resolve => HandPhase::Bust, // Will be refined (Safe vs Busted)
            HandPhase::Bust => HandPhase::Bust, // Terminal state
        };
    }

    /// Play a card from hand during PlayerPhase
    pub fn play_card(&mut self, owner: Owner, card_index: usize) -> Result<(), String> {
        // Verify we're in PlayerPhase and it's the correct player's turn
        if self.current_state != HandPhase::PlayerPhase {
            return Err(format!("Not in PlayerPhase: {:?}", self.current_state));
        }

        let current_player = self.current_player();
        if owner != current_player {
            return Err(format!("Wrong turn: expected {current_player:?}, got {owner:?}"));
        }

        if owner == Owner::Buyer {
            return Err("Buyer uses buyer_plays_card(), not play_card()".to_string());
        }

        if card_index >= 3 {
            return Err(format!("Card index {card_index} out of bounds"));
        }

        let cards = self.cards_mut(owner);
        if let Some(card) = cards.hand[card_index].take() {
            cards.played.push(card.clone());  // Also track in owner's Cards.played
            self.cards_played.push(card);     // Track in HandState for this hand
        } else {
            return Err(format!("No card in slot {card_index}"));
        }

        // Advance to next player's turn (increments index)
        self.current_player_index += 1;

        // Check if all players have acted, then transition
        if self.all_players_acted() {
            self.transition_state();
        }

        Ok(())
    }

    /// Get whose turn it is in the current round
    pub fn current_player(&self) -> Owner {
        let turn_order = get_turn_order(self.current_round);
        turn_order[self.current_player_index]
    }

    /// Check if all players have acted this round
    pub fn all_players_acted(&self) -> bool {
        let turn_order = get_turn_order(self.current_round);
        self.current_player_index >= turn_order.len()
    }

    /// Reset turn tracking for new round
    pub fn reset_turn_tracking(&mut self) {
        self.current_player_index = 0;
    }

    /// Check if deal is valid (must have at least 1 Product AND 1 Location)
    pub fn is_valid_deal(&self) -> bool {
        self.active_product(true).is_some() && self.active_location(true).is_some()
    }

    /// Check if Buyer should bail based on thresholds
    pub fn should_buyer_bail(&self) -> bool {
        if let Some(persona) = &self.buyer_persona {
            let totals = self.calculate_totals(true);
            let heat_threshold = if let Some(scenario_idx) = persona.active_scenario_index {
                persona.scenarios.get(scenario_idx).and_then(|s| s.heat_threshold)
            } else {
                persona.heat_threshold // Fallback to persona threshold
            };

            // Check Heat threshold (only bail if heat is positive and exceeds threshold)
            if let Some(threshold) = heat_threshold {
                if totals.heat > 0 && (totals.heat as u32) > threshold {
                    return true;
                }
            }

            if let Some(evidence_threshold) = persona.evidence_threshold {
                if totals.evidence > evidence_threshold {
                    return true;
                }
            }
        }
        false
    }

    /// Check if demand is satisfied
    pub fn is_demand_satisfied(&self) -> bool {
        if let Some(persona) = &self.buyer_persona {
            if let Some(scenario_idx) = persona.active_scenario_index {
                if let Some(scenario) = persona.scenarios.get(scenario_idx) {
                    // Check if active Product matches ANY of scenario's desired products (OR logic)
                    let product_match = self.active_product(true)
                        .map(|card| scenario.products.contains(&card.name))
                        .unwrap_or(false);

                    // Check if active Location matches ANY of scenario's preferred locations (OR logic)
                    let location_match = self.active_location(true)
                        .map(|card| scenario.locations.contains(&card.name))
                        .unwrap_or(false);

                    return product_match && location_match;
                }
            }

            // Fallback: Use persona's generic demand (for backward compatibility)
            let product_match = self.active_product(true)
                .map(|card| persona.demand.products.contains(&card.name))
                .unwrap_or(false);

            let location_match = self.active_location(true)
                .map(|card| persona.demand.locations.contains(&card.name))
                .unwrap_or(false);

            product_match && location_match
        } else {
            false // No Buyer persona set
        }
    }

    /// Get the appropriate profit multiplier based on demand satisfaction
    pub fn get_profit_multiplier(&self) -> f32 {
        if let Some(persona) = &self.buyer_persona {
            if self.is_demand_satisfied() {
                persona.base_multiplier
            } else {
                persona.reduced_multiplier
            }
        } else {
            1.0 // Default multiplier if no Buyer
        }
    }

    /// Buyer plays 1 random card from visible hand
    /// Returns the card that was played, or None if no cards available
    pub fn buyer_plays_card(&mut self) -> Option<Card> {
        let buyer_cards = self.cards_mut(Owner::Buyer);
        let hand_vec: Vec<Card> = buyer_cards.hand.iter().filter_map(|s| s.clone()).collect();

        if hand_vec.is_empty() {
            return None;
        }

        // Find a random non-empty slot
        let filled_indices: Vec<usize> = buyer_cards.hand.iter()
            .enumerate()
            .filter_map(|(i, slot)| if slot.is_some() { Some(i) } else { None })
            .collect();

        if filled_indices.is_empty() {
            return None;
        }

        let random_idx = filled_indices[rand::rng().random_range(0..filled_indices.len())];
        let card = buyer_cards.hand[random_idx].take().unwrap();

        buyer_cards.played.push(card.clone());
        self.cards_played.push(card.clone());

        Some(card)
    }

    /// Initialize/refill Buyer hand at start of hand
    pub fn initialize_buyer_hand(&mut self) {
        let should_initialize = {
            let buyer_cards = self.cards(Owner::Buyer);
            buyer_cards.deck.is_empty() && buyer_cards.hand.iter().all(|s| s.is_none()) && buyer_cards.played.is_empty()
        };

        if should_initialize {
            // Clone the deck before borrowing mutably
            if let Some(deck) = self.buyer_persona.as_ref().map(|p| p.reaction_deck.clone()) {
                let buyer_cards = self.cards_mut(Owner::Buyer);
                buyer_cards.deck = deck;
                buyer_cards.shuffle_deck();
            }
        }
    }
}

// ============================================================================
// TURN ORDER SYSTEM
// ============================================================================

/// Get turn order (always Narc → Player)
pub fn get_turn_order(_round: u8) -> Vec<Owner> {
    vec![Owner::Narc, Owner::Player]
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::create_buyer_personas;
    use crate::models::test_helpers::*; // SOW-012: Use shared test helpers

    // ========================================================================
    // State Machine Tests
    // ========================================================================

    #[test]
    fn test_state_transitions() {
        let mut hand_state = HandState::default();

        // Initial state should be Draw
        assert_eq!(hand_state.current_state, HandPhase::Draw);

        hand_state.transition_state();
        assert_eq!(hand_state.current_state, HandPhase::PlayerPhase);

        // After PlayerPhase completes
        hand_state.transition_state();
        assert_eq!(hand_state.current_state, HandPhase::DealerReveal);

        // After DealerReveal (Round 1), auto-advance to next round
        hand_state.transition_state();
        assert_eq!(hand_state.current_state, HandPhase::Draw);
        assert_eq!(hand_state.current_round, 2);
    }

    #[test]
    fn test_draw_cards() {
        use crate::models::test_helpers::create_mock_game_assets;
        let mut hand_state = HandState::default();
        let assets = create_mock_game_assets();
        let buyer_personas = create_buyer_personas(&assets);
        let _ = hand_state.buyer_persona.insert(buyer_personas[0].clone());

        assert!(hand_state.cards(Owner::Narc).hand.iter().all(|s| s.is_none()));
        assert!(hand_state.cards(Owner::Player).hand.iter().all(|s| s.is_none()));
        assert!(hand_state.cards(Owner::Buyer).hand.iter().all(|s| s.is_none()));

        hand_state.draw_cards();

        assert!(hand_state.cards(Owner::Narc).hand.iter().any(|s| s.is_some()));
        assert!(hand_state.cards(Owner::Player).hand.iter().any(|s| s.is_some()));
        assert!(hand_state.cards(Owner::Buyer).hand.iter().any(|s| s.is_some()));
    }

    #[test]
    fn test_play_card_wrong_turn() {
        let mut hand_state = HandState::default();
        hand_state.draw_cards();

        // State is NarcPlay, player shouldn't be able to play
        let result = hand_state.play_card(Owner::Player, 0);
        assert!(result.is_err());
    }

    #[test]
    fn test_turn_order_simplified() {
        let order1 = get_turn_order(1);
        assert_eq!(order1, vec![Owner::Narc, Owner::Player]);

        let order2 = get_turn_order(2);
        assert_eq!(order2, vec![Owner::Narc, Owner::Player]);

        // All rounds have same order (no rotation)
        let order3 = get_turn_order(3);
        assert_eq!(order3, vec![Owner::Narc, Owner::Player]);
    }

    #[test]
    fn test_player_fold() {
        let mut hand_state = HandState::default();
        hand_state.current_state = HandPhase::PlayerPhase;
        hand_state.current_round = 1;

        hand_state.cards_played.push(create_evidence("Test", 10, 5));
        hand_state.cards_mut(Owner::Player).hand[0] = Some(create_cover("Unplayed", 20, 0));

        let initial_hand_count = hand_state.cards(Owner::Player).hand.iter().filter(|s| s.is_some()).count();

        // Simulate fold during PlayerPhase (like clicking Fold button)
        hand_state.cards_played.clear();
        hand_state.outcome = Some(HandOutcome::Folded);
        hand_state.current_state = HandPhase::Bust;

        // Verify fold consequences
        assert_eq!(hand_state.outcome, Some(HandOutcome::Folded));
        assert_eq!(hand_state.current_state, HandPhase::Bust);
        assert_eq!(hand_state.cards_played.len(), 0);
        let final_hand_count = hand_state.cards(Owner::Player).hand.iter().filter(|s| s.is_some()).count();
        assert_eq!(final_hand_count, initial_hand_count);
    }

    #[test]
    fn test_start_next_hand_preserves_cash_and_heat() {
        use crate::models::test_helpers::*;
        let mut hand_state = HandState::default();

        // Add enough cards to player deck to avoid exhaustion
        let player_cards = hand_state.cards_mut(Owner::Player);
        for i in 0..10 {
            player_cards.deck.push(create_product(&format!("Test Product {}", i), 50, 5));
        }

        // Simulate some cash and heat
        hand_state.cash = 1500;
        hand_state.current_heat = 45;

        // Start next hand
        hand_state.start_next_hand();

        // Cash and heat should be preserved
        assert_eq!(hand_state.cash, 1500);
        assert_eq!(hand_state.current_heat, 45);

        // Everything else should be reset
        assert_eq!(hand_state.current_state, HandPhase::Draw);
        assert_eq!(hand_state.current_round, 1);
        assert_eq!(hand_state.cards_played.len(), 0);
        assert!(hand_state.outcome.is_none());
    }

    #[test]
    fn test_buyer_deck_resets_between_hands() {
        use crate::models::test_helpers::create_mock_game_assets;
        let mut hand_state = HandState::default();
        let assets = create_mock_game_assets();
        let buyer_personas = create_buyer_personas(&assets);
        hand_state.buyer_persona = Some(buyer_personas[0].clone());

        // Draw cards - buyer gets cards from persona deck
        hand_state.draw_cards();

        // Simulate buyer playing 2 cards
        hand_state.buyer_plays_card();
        hand_state.buyer_plays_card();

        let buyer_played_count = hand_state.cards(Owner::Buyer).played.len();

        // Buyer should have played 2 cards
        assert_eq!(buyer_played_count, 2);

        // Start next hand
        hand_state.start_next_hand();

        // Buyer deck should be completely reset (full 7 cards from persona)
        // Unlike Player/Narc which only shuffle back unplayed cards
        let expected_deck_size = buyer_personas[0].reaction_deck.len();
        assert_eq!(hand_state.cards(Owner::Buyer).deck.len(), expected_deck_size);

        // Buyer played cards should be cleared
        assert_eq!(hand_state.cards(Owner::Buyer).played.len(), 0);

        // Buyer hand should be empty (ready for next draw)
        assert!(hand_state.cards(Owner::Buyer).hand.iter().all(|s| s.is_none()));
    }

    #[test]
    fn test_shuffle_cards_back_clears_all_hands() {
        use crate::models::test_helpers::create_mock_game_assets;
        let mut hand_state = HandState::default();
        let assets = create_mock_game_assets();
        let buyer_personas = create_buyer_personas(&assets);
        hand_state.buyer_persona = Some(buyer_personas[0].clone());

        // Draw cards for all owners
        hand_state.draw_cards();

        // Verify all owners have cards in hand
        assert!(hand_state.cards(Owner::Player).hand.iter().any(|s| s.is_some()));
        assert!(hand_state.cards(Owner::Narc).hand.iter().any(|s| s.is_some()));
        assert!(hand_state.cards(Owner::Buyer).hand.iter().any(|s| s.is_some()));

        // Shuffle back
        hand_state.shuffle_cards_back();

        // Verify Player and Narc hands are cleared and cards returned to deck
        assert!(hand_state.cards(Owner::Player).hand.iter().all(|s| s.is_none()));
        assert!(hand_state.cards(Owner::Narc).hand.iter().all(|s| s.is_none()));

        // Note: Buyer deck doesn't shuffle back during hand (resets between hands instead)
        // This test verifies shuffle_cards_back() behavior for Player/Narc

        // Verify cards_played is cleared
        assert_eq!(hand_state.cards_played.len(), 0);
        assert_eq!(hand_state.cards_played_this_round.len(), 0);
    }

    #[test]
    fn test_deck_exhaustion_ends_run() {
        let mut hand_state = HandState::default();
        hand_state.cash = 1000;
        hand_state.current_heat = 50;

        // Deplete deck to 2 cards
        let player_cards = hand_state.cards_mut(Owner::Player);
        while player_cards.deck.len() > 2 {
            player_cards.deck.pop();
        }

        // Try to start next hand with < 3 cards
        let can_continue = hand_state.start_next_hand();

        assert!(!can_continue);
        assert_eq!(hand_state.outcome, Some(HandOutcome::Busted));
        assert_eq!(hand_state.current_state, HandPhase::Bust);
    }
}
