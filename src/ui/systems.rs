// UI Systems - Update systems for active slots, heat bar, etc.
// SOW-011-A Phase 4: Active slot population and heat bar updates

use bevy::prelude::*;
use crate::{HandState, CardType, Card};
use super::components::*;
use super::helpers;
use super::theme;

/// Update active slots with current Product/Location/Conviction/Insurance cards
pub fn update_active_slots_system(
    mut hand_state_query: Query<&mut HandState, Changed<HandState>>,
    slots_query: Query<(Entity, &ActiveSlot)>,
    mut commands: Commands,
    children_query: Query<&Children>,
    card_display_query: Query<Entity, With<PlayedCardDisplay>>,
    mut discard_text_query: Query<&mut Text, With<super::components::DiscardPile>>,
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

    // Update discard pile text
    if let Ok(mut text) = discard_text_query.get_single_mut() {
        text.sections[0].value = format!("Discard: {}", hand_state.discard_pile.len());
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

        // Spawn card or placeholder
        commands.entity(slot_entity).with_children(|parent| {
            if let Some(card) = active_card {
                // Spawn actual card with active state
                helpers::spawn_card_display_with_marker(
                    parent,
                    &card.name,
                    &card.card_type,
                    helpers::CardSize::Small,
                    helpers::CardDisplayState::Active,
                    true, // compact text
                    PlayedCardDisplay,
                );
            } else {
                // Spawn ghosted placeholder
                let (color, label) = match slot.slot_type {
                    SlotType::Location => (theme::LOCATION_CARD_COLOR, "Location"),
                    SlotType::Product => (theme::PRODUCT_CARD_COLOR, "Product"),
                    SlotType::Conviction => (theme::CONVICTION_CARD_COLOR, "Conviction"),
                    SlotType::Insurance => (theme::INSURANCE_CARD_COLOR, "Insurance"),
                };

                helpers::spawn_placeholder(
                    parent,
                    label,
                    helpers::CardSize::Small,
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
        text.sections[0].value = format!("{}/{}", current_heat, heat_threshold);
    }
}
