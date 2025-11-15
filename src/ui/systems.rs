// UI Systems - Update systems for active slots, heat bar, etc.
// SOW-011-A Phase 4: Active slot population and heat bar updates
// SOW-011-B Phase 1: Resolution overlay system

use bevy::prelude::*;
use crate::{HandState, CardType, Card, HandPhase, HandOutcome};
use super::components::*;
use super::helpers;
use super::theme;

/// Update active slots with current Product/Location/Conviction/Insurance cards
pub fn update_active_slots_system(
    mut hand_state_query: Query<&mut HandState, Changed<HandState>>,
    slots_query: Query<(Entity, &ActiveSlot)>,
    discard_pile_query: Query<Entity, With<super::components::DiscardPile>>,
    mut commands: Commands,
    children_query: Query<&Children>,
    _card_display_query: Query<Entity, With<PlayedCardDisplay>>,
) {
    let Ok(mut hand_state) = hand_state_query.get_single_mut() else {
        return;
    };

    // Track cards that were replaced this frame (for discard pile)
    let mut replaced_cards = Vec::new();

    // Check for override replacements
    // When a new Product/Location/Conviction/Insurance is played, it replaces the old one
    for slot_type in [SlotType::Product, SlotType::Location, SlotType::Conviction, SlotType::Insurance] {
        // Get all cards of this type (reversed to get last-played first)
        let matching_cards: Vec<&Card> = hand_state.cards_played.iter()
            .filter(|c| match (slot_type, &c.card_type) {
                (SlotType::Product, CardType::Product { .. }) => true,
                (SlotType::Location, CardType::Location { .. }) => true,
                (SlotType::Conviction, CardType::Conviction { .. }) => true,
                (SlotType::Insurance, CardType::Insurance { .. }) => true,
                _ => false,
            })
            .collect();

        // If there are 2+ cards of this type, the older ones were replaced
        if matching_cards.len() >= 2 {
            // All but the last one (active) should be in discard
            for &card in matching_cards.iter().take(matching_cards.len() - 1) {
                // Check if already in discard
                if !hand_state.discard_pile.iter().any(|d| d.id == card.id) {
                    replaced_cards.push(card.clone());
                }
            }
        }
    }

    // Move replaced cards to discard pile
    for card in replaced_cards {
        hand_state.discard_pile.push(card);
    }

    // Update discard pile display (vertical list of card names)
    if let Ok(discard_entity) = discard_pile_query.get_single() {
        // Clear old discard items (except header)
        if let Ok(children) = children_query.get(discard_entity) {
            // Skip first child (header "Discard Pile")
            for &child in children.iter().skip(1) {
                commands.entity(child).despawn_recursive();
            }
        }

        // Add discarded cards (most recent first)
        commands.entity(discard_entity).with_children(|parent| {
            for card in hand_state.discard_pile.iter().rev() {
                parent.spawn(TextBundle::from_section(
                    &card.name,
                    TextStyle {
                        font_size: 11.0,
                        color: theme::TEXT_SECONDARY,
                        ..default()
                    },
                ));
            }
        });
    }

    // For each slot type, determine which card (if any) is active
    for (slot_entity, slot) in slots_query.iter() {
        // Clear ALL children from this slot (cards and placeholders)
        if let Ok(children) = children_query.get(slot_entity) {
            for &child in children.iter() {
                commands.entity(child).despawn_recursive();
            }
        }

        // Get the active card for this slot type
        let active_card: Option<&Card> = match slot.slot_type {
            SlotType::Product => hand_state.active_product(true),
            SlotType::Location => hand_state.active_location(true),
            SlotType::Conviction => hand_state.active_conviction(true),
            SlotType::Insurance => hand_state.active_insurance(true),
        };

        // Spawn card or placeholder (Medium size, no margin for override slots)
        commands.entity(slot_entity).with_children(|parent| {
            if let Some(card) = active_card {
                // Spawn actual card with Medium size (no margin)
                let (width, height) = helpers::CardSize::Medium.dimensions();
                let font_size = helpers::CardSize::Medium.font_size();
                let card_color = helpers::get_card_color(&card.card_type, helpers::CardDisplayState::Active);

                parent.spawn((
                    NodeBundle {
                        style: Style {
                            width: Val::Px(width),
                            height: Val::Px(height),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            padding: UiRect::all(Val::Px(8.0)),
                            border: UiRect::all(Val::Px(theme::CARD_BORDER_WIDTH)),
                            // No margin for Medium cards (override slots)
                            ..default()
                        },
                        background_color: card_color.into(),
                        border_color: theme::CARD_BORDER_BRIGHT.into(),
                        ..default()
                    },
                    PlayedCardDisplay,
                ))
                .with_children(|parent| {
                    let card_text = helpers::format_card_text_compact(&card.name, &card.card_type);
                    parent.spawn(TextBundle::from_section(
                        card_text,
                        TextStyle {
                            font_size,
                            color: theme::TEXT_PRIMARY,
                            ..default()
                        },
                    ));
                });
            } else {
                // Spawn ghosted placeholder (Medium size, no margin)
                let (color, label) = match slot.slot_type {
                    SlotType::Location => (theme::LOCATION_CARD_COLOR, "Location"),
                    SlotType::Product => (theme::PRODUCT_CARD_COLOR, "Product"),
                    SlotType::Conviction => (theme::CONVICTION_CARD_COLOR, "Conviction"),
                    SlotType::Insurance => (theme::INSURANCE_CARD_COLOR, "Insurance"),
                };

                helpers::spawn_placeholder(
                    parent,
                    label,
                    helpers::CardSize::Medium,
                    color,
                );
            }
        });
    }
}

/// Update heat bar fill and color based on current heat
pub fn update_heat_bar_system(
    hand_state_query: Query<&HandState, Changed<HandState>>,
    mut bar_fill_query: Query<&mut Style, (With<HeatBarFill>, Without<HeatBar>)>,
    mut bar_color_query: Query<&mut BackgroundColor, With<HeatBarFill>>,
    mut bar_text_query: Query<&mut Text, With<HeatBarText>>,
) {
    let Ok(hand_state) = hand_state_query.get_single() else {
        return;
    };

    // Calculate current heat and threshold
    let totals = hand_state.calculate_totals(true);
    let current_heat = totals.heat.max(0) as u32;

    // Get threshold from buyer scenario or default to 100
    let heat_threshold = if let Some(persona) = &hand_state.buyer_persona {
        if let Some(scenario_index) = persona.active_scenario_index {
            if let Some(scenario) = persona.scenarios.get(scenario_index) {
                scenario.heat_threshold.or(persona.heat_threshold).unwrap_or(100)
            } else {
                persona.heat_threshold.unwrap_or(100)
            }
        } else {
            persona.heat_threshold.unwrap_or(100)
        }
    } else {
        100
    };

    // Update heat bar fill percentage
    let fill_percentage = if heat_threshold > 0 {
        ((current_heat as f32 / heat_threshold as f32) * 100.0).min(100.0)
    } else {
        0.0
    };

    if let Ok(mut style) = bar_fill_query.get_single_mut() {
        style.height = Val::Percent(fill_percentage);
    }

    // Update heat bar color based on percentage
    if let Ok(mut color) = bar_color_query.get_single_mut() {
        let bar_color = if fill_percentage >= 80.0 {
            theme::HEAT_BAR_RED
        } else if fill_percentage >= 50.0 {
            theme::HEAT_BAR_YELLOW
        } else {
            theme::HEAT_BAR_GREEN
        };
        *color = bar_color.into();
    }

    // Update heat bar text
    if let Ok(mut text) = bar_text_query.get_single_mut() {
        text.sections[0].value = format!("{current_heat}/{heat_threshold}");
    }
}

/// Show/hide resolution overlay and update results text
pub fn update_resolution_overlay_system(
    hand_state_query: Query<&HandState, Changed<HandState>>,
    mut overlay_query: Query<&mut Style, With<ResolutionOverlay>>,
    mut title_query: Query<&mut Text, (With<ResolutionTitle>, Without<ResolutionResults>)>,
    mut results_query: Query<&mut Text, (With<ResolutionResults>, Without<ResolutionTitle>)>,
) {
    let Ok(hand_state) = hand_state_query.get_single() else {
        return;
    };

    let Ok(mut overlay_style) = overlay_query.get_single_mut() else {
        return;
    };

    // Show overlay when hand reaches Bust state
    if hand_state.current_state == HandPhase::Bust {
        overlay_style.display = Display::Flex;

        // Update title based on outcome
        if let Ok(mut title_text) = title_query.get_single_mut() {
            title_text.sections[0].value = match hand_state.outcome {
                Some(HandOutcome::Safe) => "DEAL COMPLETE!".to_string(),
                Some(HandOutcome::Busted) => "BUSTED!".to_string(),
                Some(HandOutcome::Folded) => "HAND FOLDED".to_string(),
                Some(HandOutcome::InvalidDeal) => "INVALID DEAL".to_string(),
                Some(HandOutcome::BuyerBailed) => "BUYER BAILED".to_string(),
                None => "HAND COMPLETE".to_string(),
            };

            // Color code title
            title_text.sections[0].style.color = match hand_state.outcome {
                Some(HandOutcome::Safe) => theme::STATUS_SAFE,
                Some(HandOutcome::Busted) => theme::STATUS_BUSTED,
                Some(HandOutcome::Folded) => theme::STATUS_FOLDED,
                Some(HandOutcome::InvalidDeal) => theme::STATUS_INVALID,
                Some(HandOutcome::BuyerBailed) => theme::STATUS_BAILED,
                None => theme::TEXT_HEADER,
            };
        }

        // Update results breakdown
        if let Ok(mut results_text) = results_query.get_single_mut() {
            let totals = hand_state.calculate_totals(true);
            let mut results = String::new();

            match hand_state.outcome {
                Some(HandOutcome::Safe) => {
                    results.push_str(&format!("Evidence: {} ≤ Cover: {} ✓\n\n", totals.evidence, totals.cover));
                    results.push_str(&format!("Profit: ${}\n", totals.profit));
                    results.push_str(&format!("Heat: {}\n", totals.heat));

                    if hand_state.is_demand_satisfied() {
                        let multiplier = hand_state.get_profit_multiplier();
                        results.push_str(&format!("\nDemand Met! ×{multiplier:.1} multiplier"));
                    } else {
                        results.push_str("\nDemand Not Met (reduced multiplier)");
                    }
                }
                Some(HandOutcome::Busted) => {
                    if hand_state.player_deck.len() < 3 {
                        results.push_str(&format!("Deck Exhausted: {} cards\n\nRun Ends", hand_state.player_deck.len()));
                    } else {
                        results.push_str(&format!("Evidence: {} > Cover: {} ✗\n\n", totals.evidence, totals.cover));
                        results.push_str(&format!("You got caught!\nHeat: {}", totals.heat));
                    }
                }
                Some(HandOutcome::Folded) => {
                    results.push_str("You bailed out\n\nNo profit, no risk");
                }
                Some(HandOutcome::InvalidDeal) => {
                    let has_product = hand_state.active_product(true).is_some();
                    let has_location = hand_state.active_location(true).is_some();

                    if !has_product && !has_location {
                        results.push_str("Missing Product AND Location!");
                    } else if !has_product {
                        results.push_str("Missing Product card!");
                    } else {
                        results.push_str("Missing Location card!");
                    }
                    results.push_str("\n\nNo profit");
                }
                Some(HandOutcome::BuyerBailed) => {
                    if let Some(persona) = &hand_state.buyer_persona {
                        results.push_str(&format!("{} got nervous!\n\n", persona.display_name));
                    }
                    results.push_str("Deal fell through\nNo profit");
                }
                None => {
                    results.push_str("Hand ended");
                }
            }

            results_text.sections[0].value = results;
        }
    } else {
        overlay_style.display = Display::None;
    }
}

