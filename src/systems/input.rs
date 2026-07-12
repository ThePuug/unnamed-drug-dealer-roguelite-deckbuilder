// SOW-AAA: Input and button systems
// Extracted from main.rs
// Updated for Bevy 0.18

use bevy::prelude::*;
use rand::prelude::*;
use crate::{Owner, HandState, HandPhase, HandOutcome, DeckBuilder};
use crate::game_state::GameState;
use crate::ui::components::*;
use crate::ui::theme;
use crate::data::create_buyer_personas;

// ============================================================================
// SOW-008: BETTING BUTTON SYSTEM
// ============================================================================
// Check and Fold buttons during PlayerPhase
pub fn betting_button_system(
    check_query: Query<&Interaction, (Changed<Interaction>, With<CheckButton>)>,
    fold_query: Query<&Interaction, (Changed<Interaction>, With<FoldButton>)>,
    mut hand_state_query: Query<&mut HandState>,
    story_composer: Res<crate::models::narrative::StoryComposer>,
) {
    let Ok(mut hand_state) = hand_state_query.single_mut() else {
        return;
    };

    // Only during PlayerPhase and when it's Player's turn
    if hand_state.current_state != HandPhase::PlayerPhase || hand_state.current_player() != Owner::Player {
        return;
    }

    // Check button - skip playing a card this turn
    for interaction in check_query.iter() {
        if *interaction == Interaction::Pressed {
            let current_round = hand_state.current_round;
            println!("Player checks (skips card) in Round {current_round}");

            // Record that player checked this round
            hand_state.checks_this_hand.push((Owner::Player, current_round));

            // Advance to next player without playing a card
            hand_state.current_player_index += 1;

            // If all players have acted, transition to DealerReveal
            if hand_state.all_players_acted() {
                hand_state.transition_state();
            }
        }
    }

    // Fold button - player folds immediately (available during player's turn)
    for interaction in fold_query.iter() {
        if *interaction == Interaction::Pressed {
            println!("Player folds during turn!");

            // Set outcome and transition state
            hand_state.outcome = Some(HandOutcome::Folded);
            hand_state.current_state = HandPhase::Bust;

            // Generate story
            let story = story_composer.compose_story_from_hand(&hand_state);
            hand_state.hand_story = Some(story.clone());
            hand_state.session_stories.push(story.clone()); // Add to session history
            println!("\n📖 Story: {}\n", story);

            // Discard played cards, keep unplayed
            hand_state.cards_played.clear();
        }
    }
}

// ============================================================================
// UPDATE BETTING BUTTON STATES
// ============================================================================
pub fn update_betting_button_states(
    hand_state_query: Query<&HandState>,
    mut check_button_query: Query<(&mut BackgroundGradient, &mut BoxShadow, &Children), (With<CheckButton>, Without<FoldButton>)>,
    mut fold_button_query: Query<(&mut BorderColor, &Children), (With<FoldButton>, Without<CheckButton>)>,
    mut text_colors: Query<&mut TextColor>,
    mut last_enabled: Local<Option<bool>>,
) {
    let Ok(hand_state) = hand_state_query.single() else {
        return;
    };

    // SOW-011-B: Buttons enabled only during PlayerPhase when it's player's turn, disabled otherwise
    let is_player_turn = hand_state.current_state == HandPhase::PlayerPhase &&
                         hand_state.current_player() == Owner::Player;

    // SOW-022: restyle only on change (gradient/glow/border/text swap)
    if *last_enabled == Some(is_player_turn) {
        return;
    }
    *last_enabled = Some(is_player_turn);

    // PASS button: green gradient face + glow when enabled, inert gray when not
    if let Ok((mut gradient, mut shadow, children)) = check_button_query.single_mut() {
        *gradient = crate::ui::setup::pass_button_gradient(is_player_turn);
        let glow = if is_player_turn { theme::PASS_BUTTON_GLOW } else { Color::NONE };
        *shadow = BoxShadow::new(glow, Val::Px(0.0), Val::Px(0.0), Val::Px(0.0), Val::Px(20.0));
        for child in children.iter() {
            if let Ok(mut color) = text_colors.get_mut(child) {
                color.0 = if is_player_turn {
                    theme::PASS_BUTTON_TEXT
                } else {
                    theme::PASS_BUTTON_TEXT_DISABLED
                };
            }
        }
    }

    // BAIL OUT button: red border dims when disabled
    if let Ok((mut border, children)) = fold_button_query.single_mut() {
        *border = BorderColor::all(if is_player_turn {
            theme::BAIL_BUTTON_BORDER
        } else {
            theme::BAIL_BUTTON_BORDER_DISABLED
        });
        for child in children.iter() {
            if let Ok(mut color) = text_colors.get_mut(child) {
                color.0 = if is_player_turn {
                    theme::BAIL_BUTTON_TEXT
                } else {
                    theme::BAIL_BUTTON_TEXT_DISABLED
                };
            }
        }
    }
}

// ============================================================================
// RESTART BUTTON SYSTEM
// ============================================================================
pub fn restart_button_system(
    restart_query: Query<&Interaction, (Changed<Interaction>, With<RestartButton>)>,
    mut hand_state_query: Query<&mut HandState>,
) {
    let Ok(mut hand_state) = hand_state_query.single_mut() else {
        return;
    };

    // Only at Bust state
    if hand_state.current_state != HandPhase::Bust {
        return;
    }

    // NEW DEAL button - only for Safe/Folded outcomes (not Busted)
    for interaction in restart_query.iter() {
        if *interaction == Interaction::Pressed {
            // SOW-005: Can't new deal if busted (game over)
            if matches!(hand_state.outcome, Some(HandOutcome::Busted)) {
                return; // Button should be hidden, but ignore click if somehow pressed
            }

            // Check if deck is exhausted
            // SOW-021: count deck + unplayed hand (matches start_next_hand's
            // own post-shuffle-back exhaustion check)
            if hand_state.playable_cards_remaining() < 3 {
                // Button disabled, ignore click
                return;
            }

            // Start next hand (preserve cash/heat) and draw cards
            let can_continue = hand_state.start_next_hand();
            if can_continue {
                hand_state.draw_cards();
            }
        }
    }
}

// ============================================================================
// UPDATE RESTART BUTTON STATES
// ============================================================================
pub fn update_restart_button_states(
    hand_state_query: Query<&HandState>,
    save_data: Option<Res<crate::save::SaveData>>, // SOW-023: kingpin game-over label
    mut restart_button_query: Query<(&mut BackgroundColor, &mut Visibility, &Children), With<RestartButton>>,
    go_home_button_query: Query<(Entity, &Children), With<GoHomeButton>>,
    mut text_query: Query<&mut Text>,
) {
    let Ok(hand_state) = hand_state_query.single() else {
        return;
    };

    // Only at Bust state
    if hand_state.current_state != HandPhase::Bust {
        return;
    }

    let is_busted = matches!(hand_state.outcome, Some(HandOutcome::Busted));

    // NEW DEAL button: Hide if busted, disable if deck exhausted
    let (mut bg_color, mut visibility, restart_children) = restart_button_query
        .single_mut()
        .expect("Expected exactly one RestartButton in resolution overlay");

    if is_busted {
        // Busted: Hide NEW DEAL button entirely
        *visibility = Visibility::Hidden;
    } else {
        // Safe/Folded: Show NEW DEAL, disable if deck exhausted
        // SOW-021: exhaustion counts deck + unplayed hand, matching the engine
        *visibility = Visibility::Visible;
        let can_deal = hand_state.playable_cards_remaining() >= 3;
        *bg_color = if can_deal {
            theme::BUTTON_ENABLED_BG.into()
        } else {
            theme::BUTTON_DISABLED_BG.into()
        };

        // SOW-021: Explain WHY the button is disabled on deck exhaustion
        // (write only on change - this system runs every frame)
        let label = if can_deal { "NEW DEAL" } else { "OUT OF CARDS" };
        for child in restart_children.iter() {
            if let Ok(mut text) = text_query.get_mut(child) {
                if **text != label {
                    **text = label.to_string();
                }
            }
        }
    }

    // GO HOME button text: "GO HOME" if safe, "END RUN" if busted,
    // "NEW EMPIRE" when the KINGPIN busted (SOW-023: the empire already fell;
    // this button walks into the fresh one)
    let (_button_entity, children) = go_home_button_query
        .single()
        .expect("Expected exactly one GoHomeButton in resolution overlay");

    let kingpin_fell = is_busted
        && save_data
            .as_ref()
            .map(|save| save.active_dealer_state().is_kingpin)
            .unwrap_or(false);
    let go_home_label = if kingpin_fell {
        "NEW EMPIRE"
    } else if is_busted {
        "END RUN"
    } else {
        "GO HOME"
    };
    for child in children.iter() {
        if let Ok(mut text) = text_query.get_mut(child) {
            if **text != go_home_label {
                **text = go_home_label.to_string();
            }
        }
    }
}

// ============================================================================
// GO HOME BUTTON SYSTEM
// ============================================================================
pub fn go_home_button_system(
    mut commands: Commands,
    go_home_query: Query<&Interaction, (Changed<Interaction>, With<GoHomeButton>)>,
    hand_state_query: Query<(Entity, &HandState)>,
    mut next_state: ResMut<NextState<GameState>>,
    game_assets: Res<crate::assets::GameAssets>, // SOW-013-B: Need for DeckBuilder::from_assets
    save_data: Option<ResMut<crate::save::SaveData>>,
    save_manager: Option<Res<crate::save::SaveManager>>,
) {
    let Ok((entity, hand_state)) = hand_state_query.single() else {
        return;
    };

    // Only at Bust state
    if hand_state.current_state != HandPhase::Bust {
        return;
    }

    // Go Home button - return to deck builder
    let mut should_go_home = false;
    for interaction in go_home_query.iter() {
        if *interaction == Interaction::Pressed {
            should_go_home = true;
        }
    }

    if should_go_home {
        // SOW-020: Capture unlocked cards before save_data is consumed
        // (re-snapshotted below after tick_fronts - a repossession there
        // changes ownership mid-flight)
        let mut unlocked_cards = save_data
            .as_ref()
            .map(|data| data.account.unlocked_cards.clone())
            .unwrap_or_else(|| crate::save::AccountState::starting_collection());

        // Transfer deck heat and stories to the active dealer before despawning HandState
        if let (Some(mut save_data), Some(save_manager)) = (save_data, save_manager) {
            // RFC-023: busted runs were already priced at resolution (jail
            // for a dealer, empire reset for the kingpin) - transferring
            // here too would double-charge the heat
            if !matches!(hand_state.outcome, Some(HandOutcome::Busted)) {
                let character = save_data.active_character_mut();
                let deck_heat = hand_state.current_heat;
                // Signed transfer: a cooling session reduces career heat (floor 0)
                character.apply_session_heat(deck_heat);
                character.last_played = crate::save::current_timestamp();

                // Add session stories to the dealer's history
                character.story_history.extend(hand_state.session_stories.iter().cloned());

                // Count as completed deck if outcome was Safe
                if matches!(hand_state.outcome, Some(HandOutcome::Safe)) {
                    character.mark_deck_completed();
                }

                bevy::log::info!("Go Home - transferred {} deck heat to dealer (total: {}), {} stories",
                      deck_heat, character.heat, hand_state.session_stories.len());
            }

            // RFC-023: a completed run anywhere in the empire serves a unit
            // of every OTHER jailed dealer's sentence (turn-based jail)
            let runner = save_data.active_dealer;
            // SOW-025: the tick now serves jail sentences AND relocations
            let now_available = save_data.complete_run_tick(runner);
            if !now_available.is_empty() {
                bevy::log::info!("Back in action: {}", now_available.join(", "));
            }

            // SOW-031: fronts tick at the same choke - the runner's own
            // run INCLUDED (an unproductive run still spends a tick;
            // that's the run-quality pressure the mechanic exists for)
            for event in save_data.tick_fronts() {
                match event {
                    crate::save::FrontEvent::CutOff { area_id } => {
                        bevy::log::info!("Front overdue in {area_id}: supplier cut you off - one more window");
                    }
                    crate::save::FrontEvent::MuscleSeized { area_id, amount } => {
                        bevy::log::info!("Muscle visited over the {area_id} front: seized ${amount}");
                    }
                    crate::save::FrontEvent::MuscleBenched { area_id, dealer } => {
                        bevy::log::info!("Muscle visited over the {area_id} front: {dealer} took a beating (benched 1 run)");
                    }
                    crate::save::FrontEvent::Soured { area_id, card_id } => {
                        bevy::log::info!("Supplier in {area_id} soured: {card_id} repossessed, no more fronts there");
                    }
                }
            }

            // SOW-031 review fix: tick_fronts may have REPOSSESSED a card -
            // re-snapshot ownership after the tick so the DeckBuilder
            // rebuild below can't resurrect it for the rest of the session
            unlocked_cards = save_data.account.unlocked_cards.clone();

            if let Err(e) = save_manager.save(&save_data) {
                bevy::log::warn!("Failed to save on go home: {:?}", e);
            }
        }

        // SOW-013-B: Collect all cards from HandState before despawning
        let mut player_cards = hand_state.owner_cards.get(&Owner::Player)
            .expect("Player cards not found")
            .clone();

        // Collect all cards (hand + deck + played) back into deck
        player_cards.collect_all();

        // SOW-031 review fix: the just-played deck goes through the same
        // ownership filter - a repossessed card must not ride selected_cards
        // back into the next run
        player_cards.deck.retain(|c| unlocked_cards.contains(&c.id));

        // SOW-020: Update DeckBuilder with unlocked cards filter
        let mut deck_builder = DeckBuilder::from_assets_filtered(&game_assets, &unlocked_cards);
        deck_builder.selected_cards = player_cards.deck; // Cards you just played with
        commands.insert_resource(deck_builder);

        // Despawn HandState
        commands.entity(entity).despawn();

        // Transition back to DeckBuilding state
        next_state.set(GameState::DeckBuilding);
    }
}

// ============================================================================
// DECK BUILDER SYSTEMS
// ============================================================================
pub fn deck_builder_card_click_system(
    interaction_query: Query<(&Interaction, &DeckBuilderCardButton), Changed<Interaction>>,
    deck_builder: Option<ResMut<DeckBuilder>>,
) {
    let Some(mut deck_builder) = deck_builder else {
        return;
    };
    for (interaction, button) in interaction_query.iter() {
        if *interaction == Interaction::Pressed {
            // Find the card in available cards
            let Some(card) = deck_builder.available_cards.iter()
                .find(|c| c.id == button.card_id)
                .cloned() else {
                continue;
            };

            // Check if card is already in selected deck
            let is_selected = deck_builder.selected_cards.iter()
                .any(|c| c.id == button.card_id);

            if is_selected {
                // Remove from selected deck
                deck_builder.selected_cards.retain(|c| c.id != button.card_id);
            } else {
                // Add to selected deck (if under max)
                if deck_builder.selected_cards.len() < 20 {
                    deck_builder.selected_cards.push(card);
                }
            }
        }
    }
}


pub fn start_run_button_system(
    mut commands: Commands,
    interaction_query: Query<&Interaction, (Changed<Interaction>, With<StartRunButton>)>,
    deck_builder: Option<Res<DeckBuilder>>,
    mut next_state: ResMut<NextState<GameState>>,
    hand_state_query: Query<Entity, With<HandState>>,
    game_assets: Res<crate::assets::GameAssets>, // SOW-013-B: Need loaded assets for buyer/narc deck
    save_data: Option<Res<crate::save::SaveData>>, // RFC-017: Need play counts for upgrades
) {
    let Some(deck_builder) = deck_builder else {
        return;
    };
    for interaction in interaction_query.iter() {
        if *interaction == Interaction::Pressed && deck_builder.is_valid() {
            // SOW-023: a jailed dealer can't be sent out (closes the Phase 1-2
            // gap where a just-jailed active dealer could still START RUN)
            let active_available = save_data
                .as_ref()
                .map(|save| save.active_dealer_state().is_available())
                .unwrap_or(true);
            if !active_available {
                continue;
            }

            // Despawn any existing HandState
            for entity in hand_state_query.iter() {
                commands.entity(entity).despawn();
            }

            // SOW-025 (stationing, per Reed): the run happens WHERE the active
            // dealer is stationed - dealers are placed assets. The persona is
            // then drawn from that area's clientele only (SOW-024 two-stage).
            // Defensive: if the station somehow isn't an unlocked area (stale
            // save content), fall back to the first unlocked area.
            let unlocked = save_data
                .as_ref()
                .map(|save| save.account.unlocked_locations.clone())
                .unwrap_or_else(|| std::collections::HashSet::from([crate::save::DEFAULT_STATION.to_string()]));
            let station = save_data
                .as_ref()
                .map(|save| save.active_dealer_state().station.clone())
                .unwrap_or_else(|| crate::save::DEFAULT_STATION.to_string());
            let run_area = if unlocked.contains(&station) {
                station
            } else {
                bevy::log::warn!("station '{station}' is not an unlocked area - falling back");
                crate::models::shop_location::unlocked_area_ids(&game_assets.shop_locations, &unlocked)
                    .first()
                    .copied()
                    .unwrap_or(crate::save::DEFAULT_STATION)
                    .to_string()
            };
            let run_area = run_area.as_str();

            let buyer_personas = create_buyer_personas(&game_assets);
            let area_personas = crate::data::personas_in_area(&buyer_personas, run_area);
            // Load-time validation guarantees clientele per area; fall back to
            // the full pool defensively rather than crash a run
            let mut random_buyer = if let Some(buyer) = area_personas.choose(&mut rand::rng()) {
                (**buyer).clone()
            } else {
                bevy::log::warn!("area '{run_area}' has no clientele at runtime - drawing from all personas");
                buyer_personas.choose(&mut rand::rng()).unwrap().clone()
            };
            bevy::log::info!("Run area: {} - buyer: {}", run_area, random_buyer.display_name);

            // SOW-010: Randomly select one of the Buyer's 2 scenarios
            if !random_buyer.scenarios.is_empty() {
                let scenario_index = rand::rng().random_range(0..random_buyer.scenarios.len());
                random_buyer.active_scenario_index = Some(scenario_index);
            }

            // SOW-027: narc difficulty = deck composition for (run area x the
            // ACTIVE dealer's heat tier) - WHO you send and WHERE both matter
            let heat_tier = save_data
                .as_ref()
                .map(|save| save.active_character().heat_tier())
                .unwrap_or(crate::save::HeatTier::Cold);

            // Create new HandState; the constructor records the run area and
            // builds the narc deck from it (Safe hands here earn the runner
            // street cred in this area at resolution - SOW-025)
            let mut hand_state = HandState::with_custom_deck(
                deck_builder.selected_cards.clone(),
                &game_assets,
                heat_tier,
                run_area,
            );
            hand_state.buyer_persona = Some(random_buyer);

            // RFC-017/019/023: Copy the active dealer's play counts and upgrade
            // choices into the hand engine (copied back out at run end)
            if let Some(ref save) = save_data {
                let character = save.active_character();
                hand_state.card_play_counts = character.card_play_counts.clone();
                hand_state.card_upgrades = character.card_upgrades.clone();
            }

            hand_state.draw_cards(); // This will also initialize buyer hand
            commands.spawn(hand_state);

            // Transition to InRun state
            next_state.set(GameState::InRun);
        }
    }
}

// ============================================================================
// CARD CLICK SYSTEM
// ============================================================================
pub fn card_click_system(
    mut interaction_query: Query<(&Interaction, &CardButton), Changed<Interaction>>,
    mut hand_state_query: Query<&mut HandState>,
) {
    let Ok(mut hand_state) = hand_state_query.single_mut() else {
        return;
    };

    for (interaction, card_button) in interaction_query.iter_mut() {
        if *interaction == Interaction::Pressed {
            println!("Card clicked! Index: {}, State: {:?}", card_button.card_index, hand_state.current_state);

            // SOW-008: Clicking card during PlayerPhase plays it immediately
            if hand_state.current_state == HandPhase::PlayerPhase {
                // Only if it's Player's turn
                if hand_state.current_player() == Owner::Player {
                    // SOW-022: validate against the SLOT array, not the
                    // None-filtered hand - with an empty deck, a card in slot 2
                    // behind empty slots was silently unclickable
                    let slot_has_card = hand_state
                        .cards(Owner::Player)
                        .hand
                        .get(card_button.card_index)
                        .is_some_and(|slot| slot.is_some());
                    if slot_has_card {
                        println!("Player playing card {}", card_button.card_index);

                        // Play the card face-up immediately
                        // RFC-017: Play count is incremented on successful deal resolution, not here
                        let _ = hand_state.play_card(Owner::Player, card_button.card_index);
                    }
                }
            }
        }
    }
}

// ============================================================================
// SOW-023: OPERATIONS ROSTER BUTTONS (deck-builder screen)
// ============================================================================
pub fn roster_button_system(
    dealer_query: Query<(&Interaction, &RosterDealerButton), Changed<Interaction>>,
    bail_query: Query<(&Interaction, &RosterBailButton), Changed<Interaction>>,
    hire_query: Query<&Interaction, (Changed<Interaction>, With<RosterHireButton>)>,
    move_query: Query<(&Interaction, &RosterMoveButton), Changed<Interaction>>,
    lay_low_query: Query<(&Interaction, &RosterLayLowButton), Changed<Interaction>>,
    lawyer_query: Query<(&Interaction, &RosterLawyerButton), Changed<Interaction>>,
    save_data: Option<ResMut<crate::save::SaveData>>,
    save_manager: Option<Res<crate::save::SaveManager>>,
) {
    let (Some(mut save_data), Some(save_manager)) = (save_data, save_manager) else {
        return;
    };

    let mut dirty = false;

    // Select who runs next (jailed dealers can be selected to inspect;
    // START RUN stays disabled for them)
    for (interaction, button) in dealer_query.iter() {
        if *interaction == Interaction::Pressed
            && button.dealer_index < save_data.dealers.len()
            && save_data.active_dealer != button.dealer_index
        {
            save_data.active_dealer = button.dealer_index;
            dirty = true;
        }
    }

    // Pay bail (bail_out itself no-ops when unaffordable or not jailed)
    for (interaction, button) in bail_query.iter() {
        if *interaction == Interaction::Pressed {
            dirty |= save_data.bail_out(button.dealer_index);
        }
    }

    // Hire the next recruit (hire_dealer no-ops when unaffordable)
    for interaction in hire_query.iter() {
        if *interaction == Interaction::Pressed {
            dirty |= save_data.hire_dealer();
        }
    }

    // SOW-025: relocate a dealer (move_dealer no-ops when unavailable,
    // already there, or the fee is unaffordable)
    for (interaction, button) in move_query.iter() {
        if *interaction == Interaction::Pressed {
            let moved = save_data.move_dealer(button.dealer_index, &button.to_area);
            if moved {
                bevy::log::info!(
                    "{} is relocating to {}",
                    save_data.dealers[button.dealer_index].name,
                    button.to_area
                );
            }
            dirty |= moved;
        }
    }

    // SOW-027: lay low (lay_low no-ops when ineligible or unaffordable)
    for (interaction, button) in lay_low_query.iter() {
        if *interaction == Interaction::Pressed {
            let laying = save_data.lay_low(button.dealer_index);
            if laying {
                bevy::log::info!(
                    "{} is laying low",
                    save_data.dealers[button.dealer_index].name
                );
            }
            dirty |= laying;
        }
    }

    // SOW-027: crooked lawyer (hire_lawyer no-ops when ineligible or
    // unaffordable)
    for (interaction, button) in lawyer_query.iter() {
        if *interaction == Interaction::Pressed {
            let cooled = save_data.hire_lawyer(button.dealer_index);
            if cooled {
                bevy::log::info!(
                    "{} lawyered up - heat now {}",
                    save_data.dealers[button.dealer_index].name,
                    save_data.dealers[button.dealer_index].character.heat
                );
            }
            dirty |= cooled;
        }
    }

    if dirty {
        if let Err(e) = save_manager.save(&save_data) {
            bevy::log::warn!("Failed to save roster change: {:?}", e);
        }
    }
}

/// SOW-023: START RUN reflects the active dealer's availability (the click
/// handler independently guards, this is the visual)
pub fn update_start_run_button_system(
    save_data: Option<Res<crate::save::SaveData>>,
    mut button_query: Query<(&mut BackgroundColor, &Children), With<StartRunButton>>,
    mut text_query: Query<&mut Text>,
) {
    let Some(save_data) = save_data else {
        return;
    };
    let Ok((mut bg, children)) = button_query.single_mut() else {
        return;
    };

    // SOW-025: the label says WHY the active dealer can't be sent out
    let dealer = save_data.active_dealer_state();
    let (color, label) = if dealer.is_available() {
        (theme::CONTINUE_BUTTON_BG, "START RUN")
    } else if dealer.relocating_remaining().is_some() {
        (theme::BUTTON_DISABLED_BG, "MOVING")
    } else if dealer.laying_low_remaining().is_some() {
        // SOW-027: committed to the package - can't run until it ticks out
        (theme::BUTTON_DISABLED_BG, "LAYING LOW")
    } else {
        (theme::BUTTON_DISABLED_BG, "JAILED")
    };

    if bg.0 != color {
        *bg = color.into();
    }
    for child in children.iter() {
        if let Ok(mut text) = text_query.get_mut(child) {
            if **text != label {
                **text = label.to_string();
            }
        }
    }
}
