// SOW-AAA Phase 5: HandState model extracted from main.rs
// Core game state machine for hand progression

use bevy::prelude::*;
use rand::Rng;
use rand::seq::SliceRandom;

use crate::models::card::*;
use crate::models::buyer::*;
use crate::data::*;

// ============================================================================
// HAND STATE MACHINE
// ============================================================================

/// States the hand can be in (SOW-008: Sequential play with dealer reveals)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HandPhase {
    Draw,           // Draw cards to hand
    PlayerPhase,    // Sequential turn-based card play (SOW-008 Phase 1)
    DealerReveal,   // Dealer community card reveals (SOW-008 Phase 2)
    FoldDecision,   // Player/AI can fold after seeing dealer card (SOW-008 Phase 3, Rounds 1-2 only)
    Resolve,        // Calculate totals, check bust (after Round 3)
    Bust,           // Terminal state

    // REMOVED by SOW-008: Betting, Flip (obsolete betting system from ADR-002/005)
    // REMOVED by SOW-008: DecisionPoint (replaced by FoldDecision after dealer reveal)
    // REMOVED: Legacy NarcPlay/CustomerPlay/PlayerPlay states
}

/// Outcome of hand resolution
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HandOutcome { // SOW-011-B: Made public for UI systems
    Safe,
    Busted,
    Folded,        // SOW-004: Player folded during hand (not bust, not completed)
    InvalidDeal,   // SOW-009: Missing Product or Location
    BuyerBailed,   // SOW-009: Buyer threshold exceeded
}

/// Hand state tracking (Extended for SOW-002/003, SOW-008)
#[derive(Component)]
pub struct HandState {
    pub current_state: HandPhase,
    pub current_round: u8,           // 1, 2, or 3 (ADR-004)
    pub cards_played: Vec<Card>,
    pub cards_played_this_round: Vec<Card>, // SOW-008: Repurposed for sequential play tracking
    pub narc_deck: Vec<Card>,
    pub player_deck: Vec<Card>,
    pub narc_hand: Vec<Card>,
    pub player_hand: Vec<Card>,
    pub outcome: Option<HandOutcome>,
    // SOW-003: Cash and Heat tracking
    pub cash: u32,           // Cumulative profit for insurance affordability
    pub current_heat: u32,   // Cumulative Heat for conviction thresholds
    // SOW-008 Phase 1: Turn tracking for sequential play
    pub current_player_index: usize, // Index into turn_order (0 or 1 for 2 players)
    // SOW-009: dealer_deck, dealer_hand, customer_deck, customer_hand, customer_folded removed
    // SOW-008 Phase 1: Check tracking (for display)
    pub checks_this_hand: Vec<(Owner, u8)>, // Who checked and in which round (persists entire hand)
    // SOW-009: Buyer system (replaces dealer_deck + customer)
    pub buyer_persona: Option<BuyerPersona>,   // Selected Buyer for this hand (None during transition)
    pub buyer_deck: Vec<Card>,                 // 7 cards from persona (shuffled)
    pub buyer_hand: Vec<Card>,             // 3 visible cards drawn at hand start
    pub buyer_played: Vec<Card>,           // Cards played so far (for UI tracking)
    // SOW-011-A: Discard pile for replaced cards
    pub discard_pile: Vec<Card>,           // Cards replaced via override mechanic
    // SOW-011-B: Track player hand as Option array to preserve slot positions
    pub player_hand_slots: [Option<Card>; 3], // Fixed 3-slot hand (None = empty/drawing)
}

impl Default for HandState {
    fn default() -> Self {
        // SOW-009: Dealer deck removed, replaced by Buyer system
        Self {
            current_state: HandPhase::Draw,
            current_round: 1,
            cards_played: Vec::new(),
            cards_played_this_round: Vec::new(),
            narc_deck: create_narc_deck(),
            player_deck: create_player_deck(),
            narc_hand: Vec::new(),
            player_hand: Vec::new(),
            outcome: None,
            cash: 0,          // SOW-003: Start with no cash
            current_heat: 0,  // SOW-003: Start with no Heat
            current_player_index: 0, // SOW-008: Start at first player in turn order
            checks_this_hand: Vec::new(), // SOW-008 Phase 1
            // SOW-009: Buyer system (initialized as None, will be set when Buyer selected)
            buyer_persona: None,
            buyer_deck: Vec::new(),
            buyer_hand: Vec::new(),
            buyer_played: Vec::new(),
            discard_pile: Vec::new(), // SOW-011-A
            player_hand_slots: [None, None, None], // SOW-011-B
        }
    }
}

impl HandState {
    /// Create HandState with a custom player deck (SOW-006)
    pub fn with_custom_deck(mut player_deck: Vec<Card>) -> Self {
        // SOW-010: Shuffle deck at start (deck builder maintains sorted order)
        player_deck.shuffle(&mut rand::thread_rng());

        // SOW-009: Dealer deck removed, replaced by Buyer system
        Self {
            current_state: HandPhase::Draw,
            current_round: 1,
            cards_played: Vec::new(),
            cards_played_this_round: Vec::new(),
            narc_deck: create_narc_deck(),
            player_deck,  // Use custom deck (now shuffled)
            narc_hand: Vec::new(),
            player_hand: Vec::new(),
            outcome: None,
            cash: 0,
            current_heat: 0,
            current_player_index: 0, // SOW-008
            checks_this_hand: Vec::new(), // SOW-008 Phase 1
            // SOW-009: Buyer system
            buyer_persona: None,
            buyer_deck: Vec::new(),
            buyer_hand: Vec::new(),
            buyer_played: Vec::new(),
            discard_pile: Vec::new(), // SOW-011-A
            player_hand_slots: [None, None, None], // SOW-011-B
        }
    }

    // SOW-AAA: reset() removed (unused)

    /// Shuffle unplayed hand cards back into deck (SOW-004: Card retention)
    /// Called at end of hand to return ONLY unplayed cards to deck
    /// Played cards are "spent" and discarded (not returned)
    pub fn shuffle_cards_back(&mut self) {
        // Player: Return only unplayed hand cards to deck
        self.player_deck.append(&mut self.player_hand);
        self.player_deck.shuffle(&mut rand::thread_rng());

        // Narc: Return only unplayed hand cards to deck
        self.narc_deck.append(&mut self.narc_hand);
        self.narc_deck.shuffle(&mut rand::thread_rng());

        // SOW-009: Customer deck removed

        // Played cards are discarded (not shuffled back)
        self.cards_played.clear();
        self.cards_played_this_round.clear();
    }

    /// Start next hand in the run (preserve cash/heat, shuffle cards back)
    /// Used after Safe outcome to continue the run
    /// Returns true if hand can start, false if deck exhausted
    pub fn start_next_hand(&mut self) -> bool {
        let preserved_cash = self.cash;
        let preserved_heat = self.current_heat;

        // SOW-004: Shuffle cards back into deck before resetting
        self.shuffle_cards_back();

        // SOW-004: Preserve decks (they've been modified by shuffle-back and fold penalties)
        // SOW-009: Customer deck removed
        let preserved_player_deck = self.player_deck.clone();
        let preserved_narc_deck = self.narc_deck.clone();

        // SOW-004 Phase 3: Check deck exhaustion before starting new hand
        if preserved_player_deck.len() < 3 {
            // Deck exhausted - cannot start new hand
            self.outcome = Some(HandOutcome::Busted);
            self.current_state = HandPhase::Bust;
            // Don't reset - keep preserved decks to show deck size in UI
            self.player_deck = preserved_player_deck;
            self.narc_deck = preserved_narc_deck;
            return false;
        }

        // SOW-009: Preserve Buyer persona across hands
        let preserved_buyer_persona = self.buyer_persona.clone();

        // Reset state but preserve cash/heat/decks/buyer
        *self = Self::default();
        self.cash = preserved_cash;
        self.current_heat = preserved_heat;
        self.player_deck = preserved_player_deck;
        self.narc_deck = preserved_narc_deck;
        self.buyer_persona = preserved_buyer_persona;  // Keep same Buyer for entire session
        true
    }

    /// Draw cards from decks to hands (initial draw phase)
    pub fn draw_cards(&mut self) {
        // SOW-002: Draw to hand size 3 (multi-round play)
        const HAND_SIZE: usize = 3;

        // Draw for each player up to hand size
        while self.narc_hand.len() < HAND_SIZE && !self.narc_deck.is_empty() {
            self.narc_hand.push(self.narc_deck.remove(0));
        }

        // SOW-011-B: Draw player cards into empty slots (preserves positions)
        for slot in &mut self.player_hand_slots {
            if slot.is_none() && !self.player_deck.is_empty() {
                *slot = Some(self.player_deck.remove(0));
            }
        }

        // Sync player_hand vec from slots (for compatibility with existing code)
        self.player_hand = self.player_hand_slots.iter()
            .filter_map(|s| s.clone())
            .collect();

        // SOW-009 Phase 3: Initialize Buyer hand (draw 3 visible cards)
        self.initialize_buyer_hand();

        // Transition to next state after draw
        self.transition_state();
    }

    /// Transition to next state (SOW-008: Sequential play state machine)
    pub fn transition_state(&mut self) {
        self.current_state = match self.current_state {
            // SOW-008: Sequential play flow
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

    // SOW-008 Note: continue_to_next_round and fold_at_decision_point removed
    // These methods are obsolete - sequential play handles round advancement differently
    // Fold logic will be added in Phase 3

    /// Play a card from hand during PlayerPhase (SOW-008: Sequential play)
    pub fn play_card(&mut self, owner: Owner, card_index: usize) -> Result<(), String> {
        // Verify we're in PlayerPhase and it's the correct player's turn
        if self.current_state != HandPhase::PlayerPhase {
            return Err(format!("Not in PlayerPhase: {:?}", self.current_state));
        }

        let current_player = self.current_player();
        if owner != current_player {
            return Err(format!("Wrong turn: expected {current_player:?}, got {owner:?}"));
        }

        // SOW-011-B: Handle player specially (slot-based hand)
        if owner == Owner::Player {
            if card_index >= 3 {
                return Err(format!("Card index {card_index} out of bounds"));
            }

            if let Some(card) = self.player_hand_slots[card_index].take() {
                self.cards_played.push(card);
                // Sync player_hand vec for compatibility
                self.player_hand = self.player_hand_slots.iter()
                    .filter_map(|s| s.clone())
                    .collect();
            } else {
                return Err(format!("No card in slot {card_index}"));
            }
        } else {
            // Narc/Buyer use normal Vec-based hands
            let hand = match owner {
                Owner::Narc => &mut self.narc_hand,
                Owner::Buyer => unreachable!("Buyer uses buyer_plays_card(), not play_card()"),
                Owner::Player => unreachable!(),
            };

            if card_index >= hand.len() {
                return Err(format!("Card index {card_index} out of bounds"));
            }

            let card = hand.remove(card_index);
            self.cards_played.push(card);
        }

        // Advance to next player's turn (increments index)
        self.current_player_index += 1;

        // Check if all players have acted, then transition
        if self.all_players_acted() {
            self.transition_state();
        }

        Ok(())
    }

    // SOW-008 Phase 1: Sequential play helpers

    /// Get whose turn it is in the current round (SOW-008 rotating turn order)
    pub fn current_player(&self) -> Owner {
        let turn_order = get_turn_order(self.current_round);
        turn_order[self.current_player_index]
    }

    // SOW-AAA: advance_turn() removed (unused)

    /// Check if all players have acted this round
    pub fn all_players_acted(&self) -> bool {
        let turn_order = get_turn_order(self.current_round);
        self.current_player_index >= turn_order.len()
    }

    /// Reset turn tracking for new round
    pub fn reset_turn_tracking(&mut self) {
        self.current_player_index = 0;
    }

    // SOW-009: Dealer card system removed, replaced by Buyer reaction cards
    // SOW-AAA: current_dealer_card() removed (obsolete)

    // SOW-009 Phase 4: Resolution checks

    /// Check if deal is valid (must have at least 1 Product AND 1 Location)
    pub fn is_valid_deal(&self) -> bool {
        self.active_product(true).is_some() && self.active_location(true).is_some()
    }

    /// Check if Buyer should bail based on thresholds (SOW-010: Uses scenario threshold)
    pub fn should_buyer_bail(&self) -> bool {
        if let Some(persona) = &self.buyer_persona {
            let totals = self.calculate_totals(true);

            // SOW-010: Use scenario's heat threshold (if scenario active)
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

            // Check Evidence threshold (still uses persona threshold - scenarios don't vary this)
            if let Some(evidence_threshold) = persona.evidence_threshold {
                if totals.evidence > evidence_threshold {
                    return true;
                }
            }
        }
        false
    }

    /// Check if demand is satisfied (SOW-010: Product + Location match scenario preferences)
    pub fn is_demand_satisfied(&self) -> bool {
        if let Some(persona) = &self.buyer_persona {
            // SOW-010: Use active scenario's demands (if scenario selected)
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

                    // BOTH must be satisfied for base multiplier
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

    // SOW-008 Phase 3: Fold mechanics
    // SOW-AAA: player_fold() removed (not implemented yet, FoldDecision state not used)

    // SOW-009 Phase 3: Buyer card play system

    /// Buyer plays 1 random card from visible hand
    /// Returns the card that was played, or None if no cards available
    pub fn buyer_plays_card(&mut self) -> Option<Card> {
        if self.buyer_hand.is_empty() {
            return None;  // No cards left
        }

        // Select random card from visible hand
        let random_index = rand::thread_rng().gen_range(0..self.buyer_hand.len());
        let card = self.buyer_hand.remove(random_index);

        // Add to buyer_played for tracking
        self.buyer_played.push(card.clone());

        // Add to cards_played for totals calculation
        self.cards_played.push(card.clone());

        Some(card)
    }

    /// Initialize/refill Buyer hand at start of NEW hand (draw to 3 visible cards)
    /// Called at hand start - refills hand from deck (like player)
    /// Buyer deck reshuffles BETWEEN hands (not during hand - 7 cards, max 3 played)
    pub fn initialize_buyer_hand(&mut self) {
        const BUYER_HAND_SIZE: usize = 3;

        // First time initialization: Set up deck from persona
        if self.buyer_deck.is_empty() && self.buyer_hand.is_empty() && self.buyer_played.is_empty() {
            if let Some(persona) = &self.buyer_persona {
                self.buyer_deck = persona.reaction_deck.clone();
                self.buyer_deck.shuffle(&mut rand::thread_rng());
            }
        }

        // Draw to fill hand to 3 cards (draws from existing deck, depleting it)
        while self.buyer_hand.len() < BUYER_HAND_SIZE && !self.buyer_deck.is_empty() {
            self.buyer_hand.push(self.buyer_deck.remove(0));
        }

        // Note: Buyer can never deplete deck mid-hand (7 cards, max 3 played per hand)
    }

    // SOW-009: should_customer_fold() and customer_fold() removed - Customer no longer exists
}

// ============================================================================
// SOW-009: TURN ORDER SYSTEM (Simplified from SOW-008)
// ============================================================================

/// Get turn order (SOW-009: No rotation, always Narc → Player)
/// Narc always plays first, Player always plays second
pub fn get_turn_order(_round: u8) -> Vec<Owner> {
    vec![Owner::Narc, Owner::Player]
}

// ============================================================================
// BUST CHECK & RESOLUTION - Phase 3
// ============================================================================

impl HandState {
    /// Resolve hand outcome (bust check with insurance/conviction - SOW-003 Phase 3)
    ///
    /// Resolution Order (per ADR-003):
    /// 1. Evidence ≤ Cover → Safe (no insurance/conviction checks needed)
    /// 2. Evidence > Cover → Check Conviction:
    ///    - Conviction active AND current_heat >= threshold → Busted (override insurance)
    /// 3. Check Insurance:
    ///    - No insurance → Busted
    ///    - Has insurance, can't afford → Busted
    ///    - Has insurance, can afford → Pay cost, gain heat_penalty, burn insurance → Safe
    ///
    /// Post-resolution:
    /// - Safe outcome: Bank profit to cash (for future insurance affordability)
    /// - All outcomes: Accumulate totals.heat to current_heat (for conviction thresholds)
    pub fn resolve_hand(&mut self) -> HandOutcome {
        // SOW-009 Phase 4: Add validity and bail checks before bust check

        // Check 1: Validity (must have Product AND Location)
        if !self.is_valid_deal() {
            println!("Invalid deal: Must play at least 1 Product AND 1 Location");
            self.outcome = Some(HandOutcome::InvalidDeal);  // Can retry, not game over
            self.current_state = HandPhase::Bust;
            return HandOutcome::InvalidDeal;
        }

        // Check 2: Buyer bail (threshold exceeded)
        if self.should_buyer_bail() {
            if let Some(persona) = &self.buyer_persona {
                println!("Buyer ({}) bailed! Threshold exceeded", persona.display_name);
            }
            self.outcome = Some(HandOutcome::BuyerBailed);  // Can retry with same Buyer
            self.current_state = HandPhase::Bust;
            return HandOutcome::BuyerBailed;
        }

        let totals = self.calculate_totals(true); // Always include all cards at resolution

        // Calculate projected heat (current heat + this hand's heat)
        // This is what heat will be AFTER this hand, used for conviction checks
        let projected_heat = self.current_heat.saturating_add(totals.heat as u32);

        // Step 3: Evidence ≤ Cover → Safe (tie goes to player)
        let outcome = if totals.evidence <= totals.cover {
            HandOutcome::Safe
        } else {
            // Evidence > Cover → Potential bust, check insurance/conviction

            // Step 2: Check Conviction override (using PROJECTED heat after this hand)
            if let Some(conviction) = self.active_conviction(true) {
                if let CardType::Conviction { heat_threshold } = conviction.card_type {
                    if projected_heat >= heat_threshold {
                        // Conviction overrides insurance - run ends
                        HandOutcome::Busted
                    } else {
                        // Heat below threshold, conviction doesn't activate
                        self.try_insurance_activation()
                    }
                } else {
                    self.try_insurance_activation()
                }
            } else {
                // No conviction active
                self.try_insurance_activation()
            }
        };

        // Post-resolution: Accumulate cash and heat
        match outcome {
            HandOutcome::Safe => {
                // Bank profit to cash (for future insurance purchases)
                self.cash += totals.profit;
            }
            HandOutcome::Busted => {
                // No cash gained on bust
            }
            HandOutcome::Folded => {
                // Unreachable - folding handled in handle_action(), not resolve_hand()
                // But added for exhaustive matching
            }
            HandOutcome::InvalidDeal | HandOutcome::BuyerBailed => {
                // No cash gained on invalid deal or buyer bail (deal didn't complete)
            }
        }

        // Always accumulate heat (regardless of outcome)
        // SOW-005: Handle negative heat (can't go below 0)
        if totals.heat >= 0 {
            self.current_heat = self.current_heat.saturating_add(totals.heat as u32);
        } else {
            self.current_heat = self.current_heat.saturating_sub((-totals.heat) as u32);
        }

        self.outcome = Some(outcome);
        self.current_state = HandPhase::Bust; // Transition to terminal state
        outcome
    }

    /// Try to activate insurance (Step 3 of resolution order)
    ///
    /// Returns:
    /// - Safe: Insurance activated (cost paid, heat gained, card burned)
    /// - Busted: No insurance OR can't afford
    fn try_insurance_activation(&mut self) -> HandOutcome {
        // Extract insurance values first to avoid borrow issues
        let insurance_info = self.active_insurance(true).and_then(|insurance| {
            if let CardType::Insurance { cost, heat_penalty, .. } = insurance.card_type {
                Some((insurance.name.clone(), cost, heat_penalty))
            } else {
                None
            }
        });

        if let Some((insurance_name, cost, heat_penalty)) = insurance_info {
            // Check affordability
            if self.cash >= cost {
                // Activate insurance: pay cost, gain heat penalty
                self.cash -= cost;
                self.current_heat = self.current_heat.saturating_add(heat_penalty as u32);

                // Burn insurance card (remove from deck permanently)
                self.player_deck.retain(|card| card.name != insurance_name);

                HandOutcome::Safe
            } else {
                // Can't afford insurance
                HandOutcome::Busted
            }
        } else {
            // No insurance active
            HandOutcome::Busted
        }
    }
}

// ============================================================================
// CARD INTERACTION ENGINE - Phase 2
// ============================================================================

impl HandState {
    /// Get active Product card (last Product played, if any)
    /// Override rule: Only last Product matters
    /// include_current_round: Whether to include face-down cards from this round
    pub fn active_product(&self, include_current_round: bool) -> Option<&Card> {
        let cards: Vec<&Card> = if include_current_round {
            self.cards_played.iter().chain(self.cards_played_this_round.iter()).collect()
        } else {
            self.cards_played.iter().collect()
        };

        cards.into_iter().rev().find(|card| matches!(card.card_type, CardType::Product { .. }))
    }

    /// Get active Location card (last Location played, required)
    /// Override rule: Only last Location matters
    /// include_current_round: Whether to include face-down cards from this round
    pub fn active_location(&self, include_current_round: bool) -> Option<&Card> {
        let cards: Vec<&Card> = if include_current_round {
            self.cards_played.iter().chain(self.cards_played_this_round.iter()).collect()
        } else {
            self.cards_played.iter().collect()
        };

        // Find last Location (player or Buyer)
        // SOW-009: Buyer Location cards can override player Location cards (both use CardType::Location)
        cards.into_iter().rev().find(|card| {
            matches!(card.card_type, CardType::Location { .. })
        })
    }

    /// Get active Insurance card (last Insurance played, if any)
    /// Override rule: Only last Insurance matters (SOW-003 Phase 1)
    /// include_current_round: Whether to include face-down cards from this round
    pub fn active_insurance(&self, include_current_round: bool) -> Option<&Card> {
        let cards: Vec<&Card> = if include_current_round {
            self.cards_played.iter().chain(self.cards_played_this_round.iter()).collect()
        } else {
            self.cards_played.iter().collect()
        };

        cards.into_iter().rev().find(|card| matches!(card.card_type, CardType::Insurance { .. }))
    }

    /// Get active Conviction card (last Conviction played, if any)
    /// Override rule: Only last Conviction matters (SOW-003 Phase 2)
    /// include_current_round: Whether to include face-down cards from this round
    pub fn active_conviction(&self, include_current_round: bool) -> Option<&Card> {
        let cards: Vec<&Card> = if include_current_round {
            self.cards_played.iter().chain(self.cards_played_this_round.iter()).collect()
        } else {
            self.cards_played.iter().collect()
        };

        cards.into_iter().rev().find(|card| matches!(card.card_type, CardType::Conviction { .. }))
    }

    /// Calculate current totals from all played cards
    ///
    /// Override rules:
    /// - Last Product played becomes active (previous discarded)
    /// - Last Location played becomes active (Evidence/Cover base changes)
    /// - Last Insurance played becomes active (SOW-003)
    /// - Last Conviction played becomes active (SOW-003)
    ///
    /// Additive rules:
    /// - Evidence = Location base + sum(all Evidence cards + DealModifier evidence)
    /// - Cover = Location base + sum(all Cover cards + Insurance cover + DealModifier cover)
    /// - Heat = sum(all heat modifiers from all cards)
    /// - Profit = Active Product price × product(all DealModifier price_multiplier)
    ///
    /// Special rules (SOW-003):
    /// - Insurance acts as Cover card during totals calculation
    /// - Conviction has no effect on totals (only affects bust resolution)
    ///
    /// Multi-round (SOW-002):
    /// - include_current_round: Whether to include cards_played_this_round (after Flip) or not (during Betting)
    pub fn calculate_totals(&self, include_current_round: bool) -> Totals {
        let mut totals = Totals::default();
        let mut price_multiplier: f32 = 1.0; // SOW-002 Phase 4: multiplicative modifiers

        // Get base Evidence/Cover from active Location (player or dealer)
        if let Some(location) = self.active_location(include_current_round) {
            match location.card_type {
                CardType::Location { evidence, cover, heat } => {
                    totals.evidence = evidence;
                    totals.cover = cover;
                    totals.heat += heat;
                }
                _ => {} // Shouldn't happen
            }
        }

        // Process finalized cards from previous rounds (always included)
        let cards_to_process: Box<dyn Iterator<Item = &Card>> = if include_current_round {
            // After Flip: Include both finalized and current round cards
            Box::new(self.cards_played.iter().chain(self.cards_played_this_round.iter()))
        } else {
            // During Betting: Only finalized cards (current round is face-down)
            Box::new(self.cards_played.iter())
        };

        for card in cards_to_process {
            match card.card_type {
                CardType::Evidence { evidence, heat } => {
                    totals.evidence += evidence;
                    totals.heat += heat;
                }
                CardType::Cover { cover, heat } => {
                    totals.cover += cover;
                    totals.heat += heat;
                }
                // SOW-002 Phase 4: Deal Modifiers
                CardType::DealModifier { price_multiplier: multiplier, evidence, cover, heat } => {
                    price_multiplier *= multiplier; // Multiplicative for price
                    totals.evidence = totals.evidence.saturating_add_signed(evidence); // Additive
                    totals.cover = totals.cover.saturating_add_signed(cover); // Additive
                    totals.heat += heat; // Additive
                }
                // SOW-003 Phase 1: Insurance acts as Cover (dual function)
                CardType::Insurance { cover, .. } => {
                    totals.cover += cover; // Adds to Cover total (like Cover cards)
                    // Note: heat_penalty only applies when insurance activates on bust
                }
                // SOW-003 Phase 2: Conviction has no effect on totals
                CardType::Conviction { .. } => {
                    // No effect on totals - only affects bust resolution
                }
                _ => {}
            }
        }

        // SOW-008 Phase 2: Process revealed dealer modifier cards
        // Dealer cards are revealed AFTER PlayerPhase, so count previous rounds only
        // Round 1: No dealer cards revealed yet (0 cards)
        // After Round 1 DealerReveal: 1 dealer card revealed
        // After Round 2 DealerReveal: 2 dealer cards revealed
        // After Round 3 DealerReveal: 3 dealer cards revealed
        let revealed_dealer_cards = if self.current_state == HandPhase::DealerReveal ||
                                        self.current_state == HandPhase::Resolve ||
                                        self.current_state == HandPhase::Bust {
            // After DealerReveal or later: Include dealer card for current round
            self.current_round as usize
        } else {
            // Before DealerReveal: Only include previous rounds
            self.current_round.saturating_sub(1) as usize
        };

        // SOW-009: Dealer card processing removed, will be replaced by Buyer cards in Phase 3
        // TODO: Process buyer_played cards here instead
        for _i in 0..revealed_dealer_cards {
            // Placeholder: Buyer card effects will be added in Phase 3
        }

        // Get profit from active Product (apply multipliers)
        if let Some(product) = self.active_product(include_current_round) {
            if let CardType::Product { price, heat } = product.card_type {
                // SOW-009 Phase 4: Apply Buyer multiplier in addition to DealModifier multipliers
                let buyer_multiplier = self.get_profit_multiplier();
                totals.profit = (price as f32 * price_multiplier * buyer_multiplier) as u32;
                totals.heat += heat;
            }
        }

        totals
    }
}
