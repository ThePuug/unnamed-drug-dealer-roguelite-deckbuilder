// SOW-AAA: UI update systems
// Extracted from main.rs
// Updated for Bevy 0.17

use bevy::prelude::*;
use crate::{CardType, HandState, HandPhase, DeckBuilder, Owner};
use crate::game_state::GameState;
use crate::ui::components::*;
use crate::ui::theme;
use crate::ui;
use crate::data::validate_deck;

pub fn ui_update_system(
    hand_state_query: Query<&HandState>,
    mut totals_query: Query<&mut Text, (With<TotalsDisplay>, Without<StatusDisplay>, Without<BuyerScenarioCardText>)>,
    mut status_query: Query<(&mut Text, &mut TextColor), (With<StatusDisplay>, Without<TotalsDisplay>, Without<BuyerScenarioCardText>)>,
    mut scenario_query: Query<&mut Text, (With<BuyerScenarioCardText>, Without<StatusDisplay>, Without<TotalsDisplay>)>,
) {
    let Ok(hand_state) = hand_state_query.single() else {
        return;
    };

    // Update totals display (only exists during InRun state)
    if let Ok(mut text) = totals_query.single_mut() {
        let include_current_round = true;
        let totals = hand_state.calculate_totals(include_current_round);
        **text = format!(
            "Evidence: {} | Cover: {} | Heat: {} | Profit: ${}\nCash: ${} | Total Heat: {} | Deck: {} cards",
            totals.evidence, totals.cover, totals.heat, totals.profit,
            hand_state.cash, hand_state.current_heat, hand_state.cards(Owner::Player).deck.len()
        );
    }

    // Simplified status display - just Round and Cash (only exists during InRun state)
    if let Ok((mut text, mut text_color)) = status_query.single_mut() {
        let turn_info = if hand_state.current_state == HandPhase::PlayerPhase {
            format!(" - Turn: {:?}", hand_state.current_player())
        } else {
            String::new()
        };

        **text = format!(
            "Round {}/3{}\nCash: ${}",
            hand_state.current_round,
            turn_info,
            hand_state.cash
        );
        text_color.0 = theme::TEXT_HEADER;
    }

    // Update scenario card (only exists during InRun state)
    if let Ok(mut text) = scenario_query.single_mut() {
        let scenario_info = if let Some(persona) = &hand_state.buyer_persona {
            let scenario_idx = persona.active_scenario_index
                .expect("Buyer persona should have an active scenario");
            let scenario = persona.scenarios.get(scenario_idx)
                .expect("Active scenario index should be valid");

            let heat_info = if let Some(threshold) = scenario.heat_threshold {
                let heat_warning = if hand_state.current_heat >= threshold.saturating_sub(5) {
                    " ⚠️ CLOSE!"
                } else {
                    ""
                };
                format!("Heat Limit: {} (Current: {}){}", threshold, hand_state.current_heat, heat_warning)
            } else {
                "Heat Limit: None (fearless)".to_string()
            };

            format!(
                "👤 {}\n\nScenario: {}\n{}\n\nWants: {}\n\nPrefers:\n{}\n\n{}",
                persona.display_name,
                scenario.display_name,
                scenario.description,
                scenario.products.join(" OR "),
                scenario.locations.join(", "),
                heat_info
            )
        } else {
            "No Buyer Selected".to_string()
        };

        **text = scenario_info;
    }
}

pub fn recreate_hand_display_system(
    hand_state_changed: Query<&HandState, Changed<HandState>>,
    hand_state_all: Query<&HandState>,
    hand_display_query: Query<Entity, With<PlayerHandDisplay>>,
    mut commands: Commands,
    children_query: Query<&Children>,
    _card_button_query: Query<Entity, With<CardButton>>,
    game_assets: Res<crate::assets::GameAssets>,
    emoji_font: Res<crate::EmojiFont>,
) {
    // Check if hand state changed
    if hand_state_changed.single().is_err() {
        return; // Nothing changed
    }

    // Get current state
    let Ok(hand_state) = hand_state_all.single() else {
        return;
    };

    let Ok(hand_entity) = hand_display_query.single() else {
        return;
    };

    // Clear ALL existing children (card buttons and played card displays)
    if let Ok(children) = children_query.get(hand_entity) {
        for child in children.iter() {
            commands.entity(child).despawn();
        }
    }

    // Add card buttons for current hand
    // SOW-008: Show cards during PlayerPhase and other states
    // SOW-009: Also show during DealerReveal (Buyer card reveal)
    let show_cards = hand_state.current_state == HandPhase::PlayerPhase ||
                     hand_state.current_state == HandPhase::DealerReveal ||
                     hand_state.current_state == HandPhase::FoldDecision ||
                     hand_state.current_state == HandPhase::Resolve ||
                     hand_state.current_state == HandPhase::Bust;

    if show_cards {
        // SOW-011-B: Use slot-based hand to preserve card positions
        commands.entity(hand_entity).with_children(|parent| {
            for (slot_index, slot) in hand_state.cards(Owner::Player).hand.iter().enumerate() {
                if let Some(card) = slot {
                    // Show actual card (Medium size, no margin)
                    ui::spawn_card_button(
                        parent,
                        &card.name,
                        &card.card_type,
                        ui::CardSize::Medium,
                        ui::CardDisplayState::Active,
                        CardButton { card_index: slot_index },
                        game_assets.card_template.clone(),
                        &emoji_font,
                    );
                } else {
                    // Show placeholder for empty slot (Medium size, no margin)
                    ui::spawn_placeholder(
                        parent,
                        "Drawing...",
                        ui::CardSize::Medium,
                        theme::CARD_BORDER_NORMAL,
                        game_assets.card_back.clone(),
                    );
                }
            }
        });
    }
}

pub fn update_deck_builder_ui_system(
    deck_builder: Res<DeckBuilder>,
    mut stats_query: Query<(&mut Text, &mut TextColor), With<DeckStatsDisplay>>,
) {
    if !deck_builder.is_changed() {
        return;
    }

    for (mut text, mut text_color) in stats_query.iter_mut() {
        let count = deck_builder.selected_cards.len();
        let validation = validate_deck(&deck_builder.selected_cards);

        let is_valid = validation.is_ok();
        **text = match validation {
            Ok(_) => format!("Deck: {count}/20 cards ✓ VALID"),
            Err(msg) => format!("Deck: {count}/20 cards ✗ {msg}"),
        };

        text_color.0 = if is_valid {
            theme::SELECTED_DECK_BG_VALID
        } else {
            theme::SELECTED_DECK_BG_INVALID
        };
    }
}

pub fn populate_deck_builder_cards_system(
    mut commands: Commands,
    deck_builder: Res<DeckBuilder>,
    pool_container_query: Query<Entity, With<CardPoolContainer>>,
    card_button_query: Query<Entity, With<DeckBuilderCardButton>>,
    game_assets: Res<crate::assets::GameAssets>,
    emoji_font: Res<crate::EmojiFont>,
) {
    if !deck_builder.is_changed() {
        return;
    }

    // Clear existing card buttons
    for entity in card_button_query.iter() {
        commands.entity(entity).despawn();
    }

    // SOW-010: Populate single grid with all cards (styled like played cards)
    if let Ok(pool_entity) = pool_container_query.single() {
        commands.entity(pool_entity).with_children(|parent| {
            // Sort cards by type, then alphabetically by name
            let mut sorted_cards = deck_builder.available_cards.clone();
            sorted_cards.sort_by(|a, b| {
                let type_order_a = match a.card_type {
                    CardType::Product { .. } => 0,
                    CardType::Location { .. } => 1,
                    CardType::Cover { .. } => 2,
                    CardType::DealModifier { .. } => 3,
                    CardType::Insurance { .. } => 4,
                    CardType::Evidence { .. } => 5,
                    CardType::Conviction { .. } => 6,
                };
                let type_order_b = match b.card_type {
                    CardType::Product { .. } => 0,
                    CardType::Location { .. } => 1,
                    CardType::Cover { .. } => 2,
                    CardType::DealModifier { .. } => 3,
                    CardType::Insurance { .. } => 4,
                    CardType::Evidence { .. } => 5,
                    CardType::Conviction { .. } => 6,
                };

                // First sort by type, then alphabetically by name
                type_order_a.cmp(&type_order_b).then_with(|| a.name.cmp(&b.name))
            });

            for card in &sorted_cards {
                let is_selected = deck_builder.selected_cards.iter().any(|c| c.id == card.id);

                let display_state = if is_selected {
                    ui::CardDisplayState::Selected
                } else {
                    ui::CardDisplayState::Inactive
                };

                // Use template-based rendering for deck builder cards
                ui::spawn_card_button(
                    parent,
                    &card.name,
                    &card.card_type,
                    ui::CardSize::Small,
                    display_state,
                    DeckBuilderCardButton {
                        card_id: card.id.clone(),
                    },
                    game_assets.card_template.clone(),
                    &emoji_font,
                );
            }
        });
    }
}

pub fn toggle_game_state_ui_system(
    current_state: Res<bevy::state::state::State<GameState>>,
    mut deck_builder_query: Query<&mut Node, (With<DeckBuilderRoot>, Without<UiRoot>)>,
    mut gameplay_ui_query: Query<&mut Node, (With<UiRoot>, Without<DeckBuilderRoot>)>,
) {
    // Show deck builder in DeckBuilding state, hide in InRun
    if let Ok(mut node) = deck_builder_query.single_mut() {
        node.display = if current_state.get() == &GameState::DeckBuilding {
            Display::Flex
        } else {
            Display::None
        };
    }

    // Show gameplay UI in InRun state, hide in DeckBuilding
    if let Ok(mut node) = gameplay_ui_query.single_mut() {
        node.display = if current_state.get() == &GameState::InRun {
            Display::Flex
        } else {
            Display::None
        };
    }
}

pub fn update_played_cards_display_system(
    hand_state_query: Query<&HandState, Changed<HandState>>,
    played_pool_query: Query<Entity, With<PlayAreaDealer>>,
    mut evidence_text_query: Query<&mut Text, (With<EvidencePool>, Without<CoverPool>, Without<DealModPool>)>,
    mut cover_text_query: Query<&mut Text, (With<CoverPool>, Without<EvidencePool>, Without<DealModPool>)>,
    mut deal_mod_text_query: Query<&mut Text, (With<DealModPool>, Without<EvidencePool>, Without<CoverPool>)>,
    mut commands: Commands,
    children_query: Query<&Children>,
    card_display_query: Query<Entity, With<PlayedCardDisplay>>,
    game_assets: Res<crate::assets::GameAssets>,
    emoji_font: Res<crate::EmojiFont>,
) {
    let Ok(hand_state) = hand_state_query.single() else {
        return;
    };

    // SOW-011-A: Clear old cards from single played pool
    if let Ok(pool_entity) = played_pool_query.single() {
        if let Ok(children) = children_query.get(pool_entity) {
            for child in children.iter() {
                if card_display_query.get(child).is_ok() {
                    commands.entity(child).despawn();
                }
            }
        }
    }

    // Calculate totals for display
    let totals = hand_state.calculate_totals(true);

    // Update totals text
    if let Ok(mut text) = evidence_text_query.single_mut() {
        **text = format!("● Evidence: {}", totals.evidence);
    }
    if let Ok(mut text) = cover_text_query.single_mut() {
        **text = format!("● Cover: {}", totals.cover);
    }
    if let Ok(mut text) = deal_mod_text_query.single_mut() {
        let multiplier = hand_state.get_profit_multiplier();
        **text = format!("● Multiplier: ×{multiplier:.1}");
    }

    // SOW-011-A: ALL Evidence/Cover/DealModifier cards go to single shared pool
    if let Ok(pool) = played_pool_query.single() {
        commands.entity(pool).with_children(|parent| {
            for card in hand_state.cards_played.iter() {
                // Only show Evidence, Cover, DealModifier in played pool
                match card.card_type {
                    CardType::Evidence { .. } | CardType::Cover { .. } | CardType::DealModifier { .. } => {
                        ui::spawn_card_display_with_marker(
                            parent,
                            &card.name,
                            &card.card_type,
                            ui::CardSize::Small,
                            ui::CardDisplayState::Active,
                            true, // compact text
                            PlayedCardDisplay,
                            game_assets.card_template.clone(),
                            &emoji_font,
                        );
                    }
                    // Product, Location, Conviction, Insurance go to active slots (Phase 4)
                    _ => {}
                }
            }
        });
    }
}

pub fn render_buyer_visible_hand_system(
    hand_state_query: Query<&HandState, Changed<HandState>>,
    buyer_area_query: Query<Entity, With<BuyerVisibleHand>>,
    mut commands: Commands,
    children_query: Query<&Children>,
    _card_display_query: Query<Entity, With<PlayedCardDisplay>>,
    game_assets: Res<crate::assets::GameAssets>,
    emoji_font: Res<crate::EmojiFont>,
) {
    let Ok(hand_state) = hand_state_query.single() else {
        return;
    };

    let Ok(buyer_area) = buyer_area_query.single() else {
        return;
    };

    // Clear ALL children (cards and placeholders)
    if let Ok(children) = children_query.get(buyer_area) {
        for child in children.iter() {
            commands.entity(child).despawn();
        }
    }

    // Display buyer hand with placeholders for empty slots (like player hand)
    commands.entity(buyer_area).with_children(|parent| {
        for slot in hand_state.cards(Owner::Buyer).hand.iter() {
            if let Some(card) = slot {
                // Show actual card (Medium size)
                ui::spawn_card_with_template(
                    parent,
                    &card.name,
                    &card.card_type,
                    ui::CardSize::Medium,
                    ui::CardDisplayState::Active,
                    PlayedCardDisplay,
                    game_assets.card_template.clone(),
                    &emoji_font,
                );
            } else {
                // Show placeholder for empty slot (Medium size)
                ui::spawn_placeholder(
                    parent,
                    "Drawing...",
                    ui::CardSize::Medium,
                    theme::CARD_BORDER_NORMAL,
                    game_assets.card_back.clone(),
                );
            }
        }
    });
}

pub fn update_actor_portraits_system(
    hand_state_query: Query<&HandState>,
    mut buyer_portrait_query: Query<&mut ImageNode, (With<BuyerPortrait>, Without<NarcPortrait>)>,
    mut narc_portrait_query: Query<&mut ImageNode, (With<NarcPortrait>, Without<BuyerPortrait>)>,
    game_assets: Res<crate::assets::GameAssets>,
) {
    let Ok(hand_state) = hand_state_query.single() else {
        return;
    };

    // Update buyer portrait based on current buyer persona
    if let Ok(mut buyer_image) = buyer_portrait_query.single_mut() {
        if let Some(persona) = &hand_state.buyer_persona {
            if let Some(portrait_handle) = game_assets.actor_portraits.get(&persona.display_name) {
                if buyer_image.image != *portrait_handle {
                    buyer_image.image = portrait_handle.clone();
                }
            }
        }
    }

    // Update narc portrait (always "Narc")
    if let Ok(mut narc_image) = narc_portrait_query.single_mut() {
        if let Some(portrait_handle) = game_assets.actor_portraits.get("Narc") {
            if narc_image.image != *portrait_handle {
                narc_image.image = portrait_handle.clone();
            }
        }
    }
}

pub fn render_narc_visible_hand_system(
    hand_state_query: Query<&HandState, Changed<HandState>>,
    narc_area_query: Query<Entity, With<NarcVisibleHand>>,
    mut commands: Commands,
    children_query: Query<&Children>,
    card_display_query: Query<Entity, With<PlayedCardDisplay>>,
    game_assets: Res<crate::assets::GameAssets>,
) {
    let Ok(hand_state) = hand_state_query.single() else {
        return;
    };

    let Ok(narc_area) = narc_area_query.single() else {
        return;
    };

    // Clear old card displays
    if let Ok(children) = children_query.get(narc_area) {
        for child in children.iter() {
            if card_display_query.get(child).is_ok() {
                commands.entity(child).despawn();
            }
        }
    }

    // Display face-down cards using card back image
    commands.entity(narc_area).with_children(|parent| {
        let narc_hand: Vec<_> = hand_state.cards(Owner::Narc).into();
        for _ in 0..narc_hand.len() {
            let (width, _) = ui::CardSize::Medium.dimensions();

            // Use template aspect ratio (601x870) to calculate height
            let template_aspect = 601.0 / 870.0;
            let height = width / template_aspect;

            parent.spawn((
                Node {
                    width: Val::Px(width),
                    height: Val::Px(height),
                    position_type: PositionType::Relative,
                    ..default()
                },
                PlayedCardDisplay,
            ))
            .with_children(|parent| {
                // Card back image (fills entire card)
                parent.spawn((
                    ImageNode::new(game_assets.card_back.clone()),
                    Node {
                        position_type: PositionType::Absolute,
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
                        ..default()
                    },
                ));
            });
        }
    });
}
