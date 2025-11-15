// SOW-001: Minimal Playable Hand
// SOW-002: Betting System and AI Opponents
// SOW-006: Deck Building
// SOW-011-A: Modular UI organization
// SOW-AAA: Code organization and modularization

mod ui;
mod data;
mod models;
mod systems;
mod game_state;

use bevy::prelude::*;
use bevy::asset::load_internal_binary_asset;
use ui::setup::*;
use models::card::*;
use models::deck_builder::*;
use models::hand_state::*; // SOW-AAA Phase 5
use systems::*;
use game_state::{GameState, AiActionTimer}; // SOW-AAA Phase 8

// ============================================================================
// SOW-AAA Phase 8: GameState and AiActionTimer extracted to game_state.rs
// ============================================================================

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins)
        .init_state::<GameState>()  // SOW-006: Add state management
        .insert_resource(DeckBuilder::new())  // SOW-006: Initialize deck builder
        .insert_resource(AiActionTimer::default())  // SOW-008: AI pacing timer
        .add_systems(Startup, setup)
        .add_systems(Startup, setup_deck_builder);  // SOW-006: Setup deck builder UI

    // Embed custom font as the default font (supports emojis)
    load_internal_binary_asset!(
        app,
        TextStyle::default().font,
        "../assets/fonts/FiraCode.ttf",
        |bytes: &[u8], _path: String| {
            Font::try_from_bytes(bytes.to_vec()).unwrap()
        }
    );

    app
        .add_systems(Update, toggle_game_state_ui_system)
        .add_systems(Update, (
            ai_betting_system,
            auto_flip_system,
            betting_button_system,
            restart_button_system,
            go_home_button_system,
            update_betting_button_states,
            update_restart_button_states,
            toggle_ui_visibility_system,
        ).chain())
        .add_systems(Update, (
            update_played_cards_display_system,
            render_buyer_visible_hand_system,
            render_narc_visible_hand_system,
            recreate_hand_display_system,
            ui_update_system,
            ui::update_active_slots_system,  // SOW-011-A Phase 4
            ui::update_heat_bar_system,      // SOW-011-A Phase 4
            ui::update_resolution_overlay_system, // SOW-011-B Phase 1
        ).chain())
        .add_systems(Update, (
            card_click_system,
            deck_builder_card_click_system,
            preset_button_system,
            start_run_button_system,
            update_deck_builder_ui_system,
            populate_deck_builder_cards_system,
        ).chain())
        .run();
}

// ============================================================================
// SOW-AAA: All systems extracted to systems/ module
// - setup() -> systems/game_loop.rs
// - update_restart_button_states() -> systems/input.rs
// - go_home_button_system() -> systems/input.rs
// - card_click_system() -> systems/input.rs
// - All deck builder systems -> systems/input.rs & systems/ui_update.rs
// ============================================================================

// ============================================================================
// SOW-AAA Phase 5: HandState extracted to models/hand_state.rs (~823 lines)
// - State enum, HandOutcome enum, HandState struct
// - All impl blocks, get_turn_order function
// ============================================================================
// 8-CARD COLLECTION (MVP)
// ============================================================================
// SOW-AAA Phase 1: Data creators moved to src/data/ module
// - create_narc_deck() -> data/narc_deck.rs
// - create_player_deck() -> data/player_deck.rs
// - create_buyer_personas() -> data/buyer_personas.rs
// - validate_deck(), create_*_deck() presets -> data/presets.rs

// SOW-009: create_dealer_deck() removed - replaced by Buyer reaction decks

// ============================================================================
// SOW-008 PHASE 2: DEALER DECK CREATION (REMOVED IN SOW-009)
// ============================================================================

// Dealer deck function removed - ~140 lines of obsolete code deleted

// ============================================================================
// TESTS - Phase 1
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use rand::Rng;
    use crate::data::*;

    #[test]
    fn test_card_instantiation() {
        // SOW-005: Verify rebalanced deck sizes
        let narc_deck = create_narc_deck();
        assert_eq!(narc_deck.len(), 25); // 17 Evidence + 8 Conviction

        // Verify deck composition (shuffled, so can't check specific positions)
        let evidence_count = narc_deck.iter().filter(|c| matches!(c.card_type, CardType::Evidence { .. })).count();
        let conviction_count = narc_deck.iter().filter(|c| matches!(c.card_type, CardType::Conviction { .. })).count();
        assert_eq!(evidence_count, 17); // Variety of threat levels
        assert_eq!(conviction_count, 8); // Moved from player + new

        // SOW-009: Test Buyer personas instead of customer deck
        let buyer_personas = create_buyer_personas();
        assert_eq!(buyer_personas.len(), 3); // MVP: 3 personas

        // Verify each persona has 7 reaction cards
        for persona in buyer_personas.iter() {
            assert_eq!(persona.reaction_deck.len(), 7);
        }

        // Verify personas have distinct identities
        assert_eq!(buyer_personas[0].id, "frat_bro");
        assert_eq!(buyer_personas[1].id, "desperate_housewife");
        assert_eq!(buyer_personas[2].id, "wall_street_wolf");

        let player_deck = create_player_deck();
        assert_eq!(player_deck.len(), 24); // SOW-010: Expanded player deck
    }

    #[test]
    fn test_state_transitions() {
        let mut hand_state = HandState::default();

        // Initial state should be Draw
        assert_eq!(hand_state.current_state, HandPhase::Draw);

        // SOW-008: Draw → PlayerPhase → DealerReveal → Draw (next round)
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
        let mut hand_state = HandState::default();
        assert_eq!(hand_state.narc_hand.len(), 0);
        assert_eq!(hand_state.player_hand.len(), 0);

        hand_state.draw_cards();

        // SOW-002/009: Draw to hand size 3 (multi-round play, 2 players now)
        assert_eq!(hand_state.narc_hand.len(), 3);
        assert_eq!(hand_state.player_hand.len(), 3);
        // SOW-009: Customer removed

        // State should advance to PlayerPhase (SOW-008)
        assert_eq!(hand_state.current_state, HandPhase::PlayerPhase);
    }

    #[test]
    fn test_play_card_wrong_turn() {
        let mut hand_state = HandState::default();
        hand_state.draw_cards();

        // State is NarcPlay, player shouldn't be able to play
        let result = hand_state.play_card(Owner::Player, 0);
        assert!(result.is_err());
    }

    // ========================================================================
    // TESTS - Phase 2 (Card Interaction Engine)
    // ========================================================================

    #[test]
    fn test_override_product() {
        let mut hand_state = HandState::default();

        // Play Weed, then Meth
        let weed = Card {
            id: 1,
            name: "Weed".to_string(),

            card_type: CardType::Product { price: 30, heat: 5 },
        };
        let meth = Card {
            id: 2,
            name: "Meth".to_string(),

            card_type: CardType::Product { price: 100, heat: 30 },
        };

        hand_state.cards_played.push(weed);
        hand_state.cards_played.push(meth.clone());

        // Active product should be Meth (last played)
        let active = hand_state.active_product(true);
        assert!(active.is_some());
        assert_eq!(active.unwrap().name, "Meth");

        // Totals should reflect Meth price (100), not Weed price (30)
        let totals = hand_state.calculate_totals(true);
        assert_eq!(totals.profit, 100);
    }

    #[test]
    fn test_override_location() {
        let mut hand_state = HandState::default();

        // Play School Zone, then Safe House
        let school_zone = Card {
            id: 1,
            name: "School Zone".to_string(),

            card_type: CardType::Location { evidence: 40, cover: 5, heat: 20 },
        };
        let safe_house = Card {
            id: 2,
            name: "Safe House".to_string(),

            card_type: CardType::Location { evidence: 10, cover: 30, heat: -5 },
        };

        hand_state.cards_played.push(school_zone);
        hand_state.cards_played.push(safe_house.clone());

        // Active location should be Safe House (last played)
        let active = hand_state.active_location(true);
        assert!(active.is_some());
        assert_eq!(active.unwrap().name, "Safe House");

        // Totals should reflect Safe House base (Evidence 10, Cover 30)
        let totals = hand_state.calculate_totals(true);
        assert_eq!(totals.evidence, 10);
        assert_eq!(totals.cover, 30);
    }

    #[test]
    fn test_additive_evidence() {
        let mut hand_state = HandState::default();

        // Play Location (base 10 Evidence)
        let location = Card {
            id: 1,
            name: "Safe House".to_string(),

            card_type: CardType::Location { evidence: 10, cover: 30, heat: -5 },
        };
        hand_state.cards_played.push(location);

        // Play Evidence cards
        let patrol = Card {
            id: 2,
            name: "Patrol".to_string(),

            card_type: CardType::Evidence { evidence: 5, heat: 2 },
        };
        let surveillance = Card {
            id: 3,
            name: "Surveillance".to_string(),

            card_type: CardType::Evidence { evidence: 20, heat: 5 },
        };
        hand_state.cards_played.push(patrol);
        hand_state.cards_played.push(surveillance);

        // Evidence should stack: 10 (location) + 5 (patrol) + 20 (surveillance) = 35
        let totals = hand_state.calculate_totals(true);
        assert_eq!(totals.evidence, 35);
    }

    #[test]
    fn test_additive_cover() {
        let mut hand_state = HandState::default();

        // Play Location (base 30 Cover)
        let location = Card {
            id: 1,
            name: "Safe House".to_string(),

            card_type: CardType::Location { evidence: 10, cover: 30, heat: -5 },
        };
        hand_state.cards_played.push(location);

        // Play Cover card
        let alibi = Card {
            id: 2,
            name: "Alibi".to_string(),

            card_type: CardType::Cover { cover: 30, heat: -5 },
        };
        hand_state.cards_played.push(alibi);

        // Cover should stack: 30 (location) + 30 (alibi) = 60
        let totals = hand_state.calculate_totals(true);
        assert_eq!(totals.cover, 60);
    }

    #[test]
    fn test_heat_accumulation() {
        let mut hand_state = HandState::default();

        // Play cards with various heat modifiers
        let meth = Card {
            id: 1,
            name: "Meth".to_string(),

            card_type: CardType::Product { price: 100, heat: 30 },
        };
        let school_zone = Card {
            id: 2,
            name: "School Zone".to_string(),

            card_type: CardType::Location { evidence: 40, cover: 5, heat: 20 },
        };
        let surveillance = Card {
            id: 3,
            name: "Surveillance".to_string(),

            card_type: CardType::Evidence { evidence: 20, heat: 5 },
        };

        hand_state.cards_played.push(meth);
        hand_state.cards_played.push(school_zone);
        hand_state.cards_played.push(surveillance);

        // Heat should accumulate: 30 + 20 + 5 = 55
        let totals = hand_state.calculate_totals(true);
        assert_eq!(totals.heat, 55);
    }

    #[test]
    fn test_no_product_played() {
        let mut hand_state = HandState::default();

        // Play Location only
        let location = Card {
            id: 1,
            name: "Safe House".to_string(),

            card_type: CardType::Location { evidence: 10, cover: 30, heat: -5 },
        };
        hand_state.cards_played.push(location);

        // Profit should be 0 (no Product played)
        let totals = hand_state.calculate_totals(true);
        assert_eq!(totals.profit, 0);
    }

    #[test]
    fn test_complete_hand_scenario() {
        let mut hand_state = HandState::default();

        // Scenario: Player plays complete round
        // 1. Location: Safe House (Evidence 10, Cover 30, Heat -5)
        let location = Card {
            id: 1,
            name: "Safe House".to_string(),

            card_type: CardType::Location { evidence: 10, cover: 30, heat: -5 },
        };
        hand_state.cards_played.push(location);

        // 2. Product: Meth (Price 100, Heat 30)
        let product = Card {
            id: 2,
            name: "Meth".to_string(),

            card_type: CardType::Product { price: 100, heat: 30 },
        };
        hand_state.cards_played.push(product);

        // 3. Cover: Alibi (Cover 30, Heat -5)
        let cover = Card {
            id: 3,
            name: "Alibi".to_string(),

            card_type: CardType::Cover { cover: 30, heat: -5 },
        };
        hand_state.cards_played.push(cover);

        // 4. Evidence: Surveillance (Evidence 20, Heat 5)
        let evidence = Card {
            id: 4,
            name: "Surveillance".to_string(),

            card_type: CardType::Evidence { evidence: 20, heat: 5 },
        };
        hand_state.cards_played.push(evidence);

        let totals = hand_state.calculate_totals(true);

        // Verify totals:
        // Evidence: 10 (location) + 20 (surveillance) = 30
        // Cover: 30 (location) + 30 (alibi) = 60
        // Heat: -5 (location) + 30 (meth) - 5 (alibi) + 5 (surveillance) = 25
        // Profit: 100 (meth)
        assert_eq!(totals.evidence, 30);
        assert_eq!(totals.cover, 60);
        assert_eq!(totals.heat, 25);
        assert_eq!(totals.profit, 100);
    }

    // ========================================================================
    // TESTS - Phase 3 (Bust Check & Resolution)
    // ========================================================================

    #[test]
    fn test_bust_evidence_greater_than_cover() {
        let mut hand_state = HandState::default();

        // SOW-009: Need Product for valid deal
        let product = Card {
            id: 1,
            name: "Weed".to_string(),

            card_type: CardType::Product { price: 30, heat: 5 },
        };
        hand_state.cards_played.push(product);

        // Location with high Evidence, low Cover
        let location = Card {
            id: 2,
            name: "School Zone".to_string(),

            card_type: CardType::Location { evidence: 40, cover: 5, heat: 20 },
        };
        hand_state.cards_played.push(location);

        // Add more Evidence
        let evidence = Card {
            id: 3,
            name: "Surveillance".to_string(),

            card_type: CardType::Evidence { evidence: 20, heat: 5 },
        };
        hand_state.cards_played.push(evidence);

        // Totals: Evidence 60, Cover 5 → Busted
        let outcome = hand_state.resolve_hand();
        assert_eq!(outcome, HandOutcome::Busted);
        assert_eq!(hand_state.outcome, Some(HandOutcome::Busted));
        assert_eq!(hand_state.current_state, HandPhase::Bust);
    }

    #[test]
    fn test_safe_evidence_less_than_cover() {
        let mut hand_state = HandState::default();

        // SOW-009: Need Product for valid deal
        let product = Card {
            id: 1,
            name: "Weed".to_string(),

            card_type: CardType::Product { price: 30, heat: 5 },
        };
        hand_state.cards_played.push(product);

        // Location with low Evidence, high Cover
        let location = Card {
            id: 2,
            name: "Safe House".to_string(),

            card_type: CardType::Location { evidence: 10, cover: 30, heat: -5 },
        };
        hand_state.cards_played.push(location);

        // Add Cover
        let cover = Card {
            id: 3,
            name: "Alibi".to_string(),

            card_type: CardType::Cover { cover: 30, heat: -5 },
        };
        hand_state.cards_played.push(cover);

        // Totals: Evidence 10, Cover 60 → Safe
        let outcome = hand_state.resolve_hand();
        assert_eq!(outcome, HandOutcome::Safe);
        assert_eq!(hand_state.outcome, Some(HandOutcome::Safe));
        assert_eq!(hand_state.current_state, HandPhase::Bust);
    }

    #[test]
    fn test_tie_goes_to_player() {
        let mut hand_state = HandState::default();

        // SOW-009: Need Product + Location for valid deal
        let product = Card {
            id: 1,
            name: "Weed".to_string(),

            card_type: CardType::Product { price: 30, heat: 5 },
        };
        hand_state.cards_played.push(product);

        // Location with equal Evidence and Cover
        let location = Card {
            id: 2,
            name: "Location".to_string(),

            card_type: CardType::Location { evidence: 30, cover: 30, heat: 0 },
        };
        hand_state.cards_played.push(location);

        // Totals: Evidence 30, Cover 30 → Safe (tie goes to player)
        let outcome = hand_state.resolve_hand();
        assert_eq!(outcome, HandOutcome::Safe);
        assert_eq!(hand_state.outcome, Some(HandOutcome::Safe));
    }

    // ========================================================================
    // TESTS - SOW-002 Phase 1 (Multi-Round State Machine)
    // ========================================================================

    // SOW-008: test_continue_to_next_round and test_fold_at_decision_point removed
    // These methods don't exist in sequential play - rounds advance automatically

    // SOW-008 Phase 3: Fold mechanic tests

    #[test]
    fn test_player_fold() {
        let mut hand_state = HandState::default();
        hand_state.current_state = HandPhase::PlayerPhase;
        hand_state.current_round = 1;

        // Play some cards
        hand_state.cards_played.push(Card {
            id: 1, name: "Test".to_string(),            card_type: CardType::Evidence { evidence: 10, heat: 5 },
        });
        hand_state.player_hand.push(Card {
            id: 2, name: "Unplayed".to_string(),            card_type: CardType::Cover { cover: 20, heat: 0 },
        });

        let initial_played_count = hand_state.cards_played.len();
        let initial_hand_count = hand_state.player_hand.len();

        // Simulate fold during PlayerPhase (like clicking Fold button)
        hand_state.cards_played.clear();
        hand_state.outcome = Some(HandOutcome::Folded);
        hand_state.current_state = HandPhase::Bust;

        // Verify fold consequences
        assert_eq!(hand_state.outcome, Some(HandOutcome::Folded));
        assert_eq!(hand_state.current_state, HandPhase::Bust); // Terminal
        assert_eq!(hand_state.cards_played.len(), 0); // Played cards discarded
        assert_eq!(hand_state.player_hand.len(), initial_hand_count); // Unplayed cards kept
    }

    // SOW-009: test_customer_fold_removes_cards OBSOLETE (Customer removed)
    // TODO: Add test for Buyer bail mechanics
    /*
    #[test]
    fn test_customer_fold_removes_cards() {
        // OBSOLETE: Customer fold no longer exists
    */

    // SOW-009: test_dealer_deck_creation OBSOLETE (dealer deck removed)
    // TODO: Add test for Buyer persona creation

    // SOW-009: test_dealer_hand_initialization OBSOLETE (dealer deck removed)
    // TODO: Add test for buyer_hand initialization

    #[test]
    fn test_turn_order_simplified() {
        // SOW-009: Turn order simplified to always Narc → Player
        let order1 = get_turn_order(1);
        assert_eq!(order1, vec![Owner::Narc, Owner::Player]);

        let order2 = get_turn_order(2);
        assert_eq!(order2, vec![Owner::Narc, Owner::Player]);

        // All rounds have same order (no rotation)
        let order3 = get_turn_order(3);
        assert_eq!(order3, vec![Owner::Narc, Owner::Player]);
    }

    // ========================================================================
    // TESTS - SOW-002 Phase 2 (Betting Phase & Initiative)
    // ========================================================================

    // ========================================================================
    // SOW-AAA: OBSOLETE BETTING TESTS REMOVED (~120 lines)
    // ========================================================================
    // Tests for BettingState removed - system obsolete per ADR-006
    // - test_first_raise_gains_initiative
    // - test_response_raise_does_not_steal_initiative
    // - test_initiative_player_checks_loses_initiative
    // - test_max_raises_limit
    // - test_all_check_betting_complete
    // - test_can_raise_limit_check
    // - test_can_raise_empty_hand

    // ========================================================================
    // TESTS - SOW-003 (Insurance and Conviction Mechanics)
    // ========================================================================

    #[test]
    fn test_insurance_acts_as_cover() {
        let mut hand_state = HandState::default();

        // Location: E:20 C:20
        hand_state.cards_played.push(Card {
            id: 1,
            name: "Location".to_string(),

            card_type: CardType::Location { evidence: 20, cover: 20, heat: 0 },
        });

        // Insurance: +15 Cover
        hand_state.cards_played.push(Card {
            id: 2,
            name: "Fake ID".to_string(),

            card_type: CardType::Insurance { cover: 15, cost: 0, heat_penalty: 40 },
        });

        // Totals: E:20 C:35 (20 + 15 from insurance)
        let totals = hand_state.calculate_totals(true);
        assert_eq!(totals.evidence, 20);
        assert_eq!(totals.cover, 35); // Insurance adds to cover
        assert_eq!(totals.heat, 0); // Insurance heat penalty only applies on activation, not in totals
    }

    #[test]
    fn test_conviction_no_effect_on_totals() {
        let mut hand_state = HandState::default();

        // Location: E:20 C:20
        hand_state.cards_played.push(Card {
            id: 1,
            name: "Location".to_string(),

            card_type: CardType::Location { evidence: 20, cover: 20, heat: 0 },
        });

        // Conviction: No effect on totals
        hand_state.cards_played.push(Card {
            id: 2,
            name: "Warrant".to_string(),

            card_type: CardType::Conviction { heat_threshold: 40 },
        });

        // Totals: E:20 C:20 (conviction doesn't change anything)
        let totals = hand_state.calculate_totals(true);
        assert_eq!(totals.evidence, 20);
        assert_eq!(totals.cover, 20);
        assert_eq!(totals.heat, 0);
    }

    #[test]
    fn test_insurance_activation_affordable() {
        let mut hand_state = HandState::default();
        hand_state.cash = 1500; // Have enough cash

        // SOW-009: Need Product for valid deal
        hand_state.cards_played.push(Card {
            id: 1,
            name: "Weed".to_string(),

            card_type: CardType::Product { price: 30, heat: 5 },
        });

        // Bust scenario: E:30 C:20
        hand_state.cards_played.push(Card {
            id: 2,
            name: "Location".to_string(),

            card_type: CardType::Location { evidence: 30, cover: 20, heat: 0 },
        });

        // Insurance: Cost $1000, Heat +20
        hand_state.cards_played.push(Card {
            id: 3,
            name: "Plea Bargain".to_string(),

            card_type: CardType::Insurance { cover: 5, cost: 1000, heat_penalty: 20 },
        });

        // Evidence > Cover, but insurance should save us
        let outcome = hand_state.resolve_hand();
        assert_eq!(outcome, HandOutcome::Safe);
        // SOW-009: Cash = initial (1500) - insurance cost (1000) + profit (30) = 530
        assert_eq!(hand_state.cash, 530);
        // SOW-009: Heat = Product heat (5) + penalty (20) = 25 (not 20)
        assert_eq!(hand_state.current_heat, 25);
    }

    #[test]
    fn test_conviction_overrides_insurance() {
        let mut hand_state = HandState::default();
        hand_state.cash = 2000; // Can afford insurance
        hand_state.current_heat = 50; // Heat above threshold

        // SOW-009: Need Product for valid deal
        hand_state.cards_played.push(Card {
            id: 1,
            name: "Weed".to_string(),

            card_type: CardType::Product { price: 30, heat: 5 },
        });

        // Bust scenario: E:30 C:20
        hand_state.cards_played.push(Card {
            id: 2,
            name: "Location".to_string(),

            card_type: CardType::Location { evidence: 30, cover: 20, heat: 0 },
        });

        // Insurance: Available and affordable
        hand_state.cards_played.push(Card {
            id: 3,
            name: "Plea Bargain".to_string(),

            card_type: CardType::Insurance { cover: 5, cost: 1000, heat_penalty: 20 },
        });

        // Conviction: Threshold 40 (we're at 50, so it activates)
        hand_state.cards_played.push(Card {
            id: 4,
            name: "Warrant".to_string(),

            card_type: CardType::Conviction { heat_threshold: 40 },
        });

        // Evidence > Cover, conviction overrides insurance → Busted
        let outcome = hand_state.resolve_hand();
        assert_eq!(outcome, HandOutcome::Busted);
        assert_eq!(hand_state.cash, 2000); // Cash unchanged (conviction blocked insurance)
    }

    #[test]
    fn test_cash_accumulation_safe_hands() {
        let mut hand_state = HandState::default();
        hand_state.cash = 100; // Starting cash

        // Safe scenario with profit
        hand_state.cards_played.push(Card {
            id: 1,
            name: "Location".to_string(),

            card_type: CardType::Location { evidence: 20, cover: 30, heat: 0 },
        });

        hand_state.cards_played.push(Card {
            id: 2,
            name: "Weed".to_string(),

            card_type: CardType::Product { price: 50, heat: 5 },
        });

        // Safe outcome, profit should be banked
        let outcome = hand_state.resolve_hand();
        assert_eq!(outcome, HandOutcome::Safe);
        assert_eq!(hand_state.cash, 150); // 100 + 50 profit
    }

    #[test]
    fn test_heat_accumulation_across_hands() {
        let mut hand_state = HandState::default();
        hand_state.current_heat = 10; // Starting heat

        // Safe hand with heat
        hand_state.cards_played.push(Card {
            id: 1,
            name: "Location".to_string(),

            card_type: CardType::Location { evidence: 20, cover: 30, heat: 15 },
        });

        hand_state.cards_played.push(Card {
            id: 2,
            name: "Weed".to_string(),

            card_type: CardType::Product { price: 50, heat: 5 },
        });

        hand_state.resolve_hand();

        // Heat should accumulate: 10 + 15 + 5 = 30
        assert_eq!(hand_state.current_heat, 30);
    }

    #[test]
    fn test_conviction_uses_projected_heat() {
        let mut hand_state = HandState::default();
        hand_state.cash = 2000;
        hand_state.current_heat = 40; // Heat BEFORE this hand

        // SOW-009: Need Product for valid deal
        hand_state.cards_played.push(Card {
            id: 1,
            name: "Weed".to_string(),

            card_type: CardType::Product { price: 30, heat: 5 },
        });

        // Bust scenario: E:30 C:20, this hand adds +30 heat
        hand_state.cards_played.push(Card {
            id: 2,
            name: "Location".to_string(),

            card_type: CardType::Location { evidence: 30, cover: 20, heat: 30 }, // +30 heat
        });

        hand_state.cards_played.push(Card {
            id: 3,
            name: "Plea Bargain".to_string(),

            card_type: CardType::Insurance { cover: 5, cost: 1000, heat_penalty: 20 },
        });

        hand_state.cards_played.push(Card {
            id: 4,
            name: "DA Approval".to_string(),

            card_type: CardType::Conviction { heat_threshold: 60 },
        });

        // Projected heat: 40 + 30 (location) + 5 (product) = 75, which is >= 60, so conviction should activate
        let outcome = hand_state.resolve_hand();
        assert_eq!(outcome, HandOutcome::Busted); // Conviction blocks insurance
        assert_eq!(hand_state.cash, 2000); // Cash unchanged (conviction blocked insurance)
        assert_eq!(hand_state.current_heat, 75); // Heat accumulated correctly (40 + 30 + 5)
    }

    #[test]
    fn test_start_next_hand_preserves_cash_and_heat() {
        let mut hand_state = HandState::default();

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

    // ========================================================================
    // TESTS - SOW-004 (Card Retention Between Hands)
    // ========================================================================

    #[test]
    fn test_shuffle_cards_back_returns_unplayed_only() {
        let mut hand_state = HandState::default();
        let initial_deck_size = hand_state.player_deck.len();

        // Draw 3 cards
        hand_state.draw_cards();

        // Play 2 cards (1 remains in hand)
        hand_state.cards_played.push(hand_state.player_hand.remove(0));
        hand_state.cards_played.push(hand_state.player_hand.remove(0));
        let unplayed_cards = hand_state.player_hand.len(); // Should be 1

        // Shuffle back
        hand_state.shuffle_cards_back();

        // Only unplayed cards (1) returned to deck
        // Deck: initial - 3 (drawn) + 1 (unplayed) = initial - 2
        assert_eq!(hand_state.player_deck.len(), initial_deck_size - 2);
        assert_eq!(hand_state.player_hand.len(), 0);
        assert_eq!(hand_state.cards_played.len(), 0); // Cleared
    }

    #[test]
    fn test_fold_penalty_removes_card() {
        let mut hand_state = HandState::default();
        let initial_deck_size = hand_state.player_deck.len();

        // Draw cards
        hand_state.draw_cards();

        // Simulate fold (remove 1 random card)
        if !hand_state.player_deck.is_empty() {
            let idx = rand::thread_rng().gen_range(0..hand_state.player_deck.len());
            hand_state.player_deck.remove(idx);
        }

        // Deck should be 1 smaller
        assert_eq!(hand_state.player_deck.len(), initial_deck_size - 3 - 1); // Drew 3, removed 1
    }

    #[test]
    fn test_deck_exhaustion_ends_run() {
        let mut hand_state = HandState::default();
        hand_state.cash = 1000;
        hand_state.current_heat = 50;

        // Deplete deck to 2 cards
        while hand_state.player_deck.len() > 2 {
            hand_state.player_deck.pop();
        }

        // Try to start next hand with < 3 cards
        let can_continue = hand_state.start_next_hand();

        assert!(!can_continue); // Cannot continue
        assert_eq!(hand_state.outcome, Some(HandOutcome::Busted)); // Run ends
        assert_eq!(hand_state.current_state, HandPhase::Bust); // Terminal state
    }

    // ========================================================================
    // SOW-006: DECK BUILDING TESTS
    // ========================================================================

    #[test]
    fn test_validate_deck_valid() {
        let deck = create_player_deck(); // Default 20-card deck
        assert!(validate_deck(&deck).is_ok());
    }

    // SOW-010: Removed deck size limit tests (balance decisions, not logic validation)

    #[test]
    fn test_validate_deck_missing_product() {
        let deck = vec![
            Card { id: 15, name: "Safe House".to_string(),                card_type: CardType::Location { evidence: 10, cover: 30, heat: -5 } },
            Card { id: 19, name: "Alibi".to_string(),                card_type: CardType::Cover { cover: 30, heat: -5 } },
            Card { id: 20, name: "Bribe".to_string(),                card_type: CardType::Cover { cover: 25, heat: 10 } },
            Card { id: 21, name: "Fake Receipts".to_string(),                card_type: CardType::Cover { cover: 20, heat: 5 } },
            Card { id: 22, name: "Bribed Witness".to_string(),                card_type: CardType::Cover { cover: 15, heat: -10 } },
            Card { id: 23, name: "Plea Bargain".to_string(),                card_type: CardType::Insurance { cover: 20, cost: 1000, heat_penalty: 20 } },
            Card { id: 24, name: "Fake ID".to_string(),                card_type: CardType::Insurance { cover: 15, cost: 0, heat_penalty: 40 } },
            Card { id: 25, name: "Disguise".to_string(),                card_type: CardType::DealModifier { price_multiplier: 1.0, evidence: 0, cover: 20, heat: -5 } },
            Card { id: 26, name: "Burner Phone".to_string(),                card_type: CardType::DealModifier { price_multiplier: 1.0, evidence: 0, cover: 15, heat: -10 } },
            Card { id: 27, name: "Lookout".to_string(),                card_type: CardType::DealModifier { price_multiplier: 1.0, evidence: 0, cover: 20, heat: 0 } },
        ];
        let result = validate_deck(&deck);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Need at least 1 Product card");
    }

    #[test]
    fn test_validate_deck_missing_location() {
        let deck = vec![
            Card { id: 10, name: "Weed".to_string(),                card_type: CardType::Product { price: 30, heat: 5 } },
            Card { id: 11, name: "Meth".to_string(),                card_type: CardType::Product { price: 100, heat: 30 } },
            Card { id: 12, name: "Heroin".to_string(),                card_type: CardType::Product { price: 150, heat: 45 } },
            Card { id: 13, name: "Cocaine".to_string(),                card_type: CardType::Product { price: 120, heat: 35 } },
            Card { id: 14, name: "Fentanyl".to_string(),                card_type: CardType::Product { price: 200, heat: 50 } },
            Card { id: 19, name: "Alibi".to_string(),                card_type: CardType::Cover { cover: 30, heat: -5 } },
            Card { id: 20, name: "Bribe".to_string(),                card_type: CardType::Cover { cover: 25, heat: 10 } },
            Card { id: 21, name: "Fake Receipts".to_string(),                card_type: CardType::Cover { cover: 20, heat: 5 } },
            Card { id: 22, name: "Bribed Witness".to_string(),                card_type: CardType::Cover { cover: 15, heat: -10 } },
            Card { id: 23, name: "Plea Bargain".to_string(),                card_type: CardType::Insurance { cover: 20, cost: 1000, heat_penalty: 20 } },
        ];
        let result = validate_deck(&deck);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Need at least 1 Location card");
    }

    #[test]
    fn test_preset_aggro_valid() {
        let deck = create_aggro_deck();
        assert!(validate_deck(&deck).is_ok());
        assert_eq!(deck.len(), 10); // Aggro is minimal deck (meets 10 card minimum)
        // Verify has products and locations
        let has_product = deck.iter().any(|c| matches!(c.card_type, CardType::Product { .. }));
        let has_location = deck.iter().any(|c| matches!(c.card_type, CardType::Location { .. }));
        assert!(has_product);
        assert!(has_location);
    }

    #[test]
    fn test_preset_control_valid() {
        let deck = create_control_deck();
        assert!(validate_deck(&deck).is_ok());
        assert_eq!(deck.len(), 16); // Control is larger deck
        // Verify has products and locations
        let has_product = deck.iter().any(|c| matches!(c.card_type, CardType::Product { .. }));
        let has_location = deck.iter().any(|c| matches!(c.card_type, CardType::Location { .. }));
        assert!(has_product);
        assert!(has_location);
    }

    #[test]
    fn test_deck_builder_default() {
        let builder = DeckBuilder::new();
        assert_eq!(builder.available_cards.len(), 24); // SOW-010: Expanded to 24 cards
        assert_eq!(builder.selected_cards.len(), 20); // SOW-011-B: Default preset has 20 cards (not all 24)
        assert!(builder.is_valid());
    }

    #[test]
    fn test_deck_builder_load_presets() {
        let mut builder = DeckBuilder::new();

        // All presets should be valid (actual counts may vary with product expansion)
        builder.load_preset(DeckPreset::Aggro);
        assert!(builder.is_valid());

        builder.load_preset(DeckPreset::Control);
        assert!(builder.is_valid());

        builder.load_preset(DeckPreset::Default);
        assert!(builder.is_valid());
    }
}
