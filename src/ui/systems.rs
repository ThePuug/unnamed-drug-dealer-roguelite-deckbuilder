// UI Systems - Update systems for active slots, heat bar, etc.
// SOW-011-A Phase 4: Active slot population and heat bar updates
// SOW-011-B Phase 1: Resolution overlay system
// Updated for Bevy 0.17

use bevy::prelude::*;
use crate::{HandState, CardType, Card, HandPhase, HandOutcome, Owner};
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
    game_assets: Res<crate::assets::GameAssets>,
    emoji_font: Res<crate::EmojiFont>,
) {
    let Ok(mut hand_state) = hand_state_query.single_mut() else {
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
    if let Ok(discard_entity) = discard_pile_query.single() {
        // Clear old discard items (except header)
        if let Ok(children) = children_query.get(discard_entity) {
            // Skip first child (header "Discard Pile")
            for child in children.iter().skip(1) {
                commands.entity(child).despawn();
            }
        }

        // Add discarded cards (most recent first)
        commands.entity(discard_entity).with_children(|parent| {
            for card in hand_state.discard_pile.iter().rev() {
                parent.spawn((
                    Text::new(&card.name),
                    TextFont::from_font_size(11.0),
                    TextColor(theme::TEXT_SECONDARY),
                ));
            }
        });
    }

    // For each slot type, determine which card (if any) is active
    for (slot_entity, slot) in slots_query.iter() {
        // Clear ALL children from this slot (cards and placeholders)
        if let Ok(children) = children_query.get(slot_entity) {
            for child in children.iter() {
                commands.entity(child).despawn();
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
                // RFC-017: Get upgrade info for player cards
                // RFC-018: Get upgrade info for Narc cards (Evidence/Conviction)
                let upgrade_info = match card.card_type {
                    CardType::Product { .. } | CardType::Location { .. } | CardType::Insurance { .. } => {
                        let tier = hand_state.get_card_tier(&card.name);
                        let play_count = hand_state.card_play_counts.get(&card.name).copied().unwrap_or(0);
                        Some(helpers::UpgradeInfo {
                            tier_name: tier.name().to_string(),
                            plays: play_count,
                            plays_to_next: tier.plays_to_next(),
                            multiplier: tier.multiplier(),
                            star_color: tier.star_color(),
                            is_foil: tier.is_foil(),
                        })
                    }
                    CardType::Evidence { .. } | CardType::Conviction { .. } => {
                        // RFC-018: Narc cards use ⚖ (scales) with same color tiers
                        let tier = hand_state.narc_upgrade_tier;
                        if tier != crate::save::UpgradeTier::Base {
                            Some(helpers::UpgradeInfo {
                                tier_name: "⚖".to_string(), // Scales of Justice
                                plays: 0,
                                plays_to_next: None, // No progress for Narc cards
                                multiplier: tier.multiplier(),
                                star_color: tier.star_color(),
                                is_foil: tier.is_foil(),
                            })
                        } else {
                            None // Base tier shows no badge
                        }
                    }
                    _ => None, // Cover/DealModifier don't show tier badges
                };

                // Spawn actual card with Medium size, no margin - use template with upgrade
                helpers::spawn_card_display_with_upgrade(
                    parent,
                    &card.name,
                    &card.card_type,
                    helpers::CardSize::Medium,
                    helpers::CardDisplayState::Active,
                    PlayedCardDisplay,
                    game_assets.card_template.clone(),
                    &emoji_font,
                    upgrade_info,
                );
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
                    game_assets.card_placeholder.clone(),
                );
            }
        });
    }
}

/// Update heat bar fill and color based on current heat
pub fn update_heat_bar_system(
    hand_state_query: Query<&HandState, Changed<HandState>>,
    mut bar_fill_query: Query<&mut Node, (With<HeatBarFill>, Without<HeatBar>)>,
    mut bar_color_query: Query<&mut BackgroundColor, With<HeatBarFill>>,
    mut bar_text_query: Query<&mut Text, With<HeatBarText>>,
) {
    let Ok(hand_state) = hand_state_query.single() else {
        return;
    };

    // Heat is accumulated immediately when cards are played, use current_heat directly
    let current_heat = hand_state.current_heat;

    // Get threshold from buyer scenario (None = no limit, show as 999)
    let heat_threshold = hand_state.buyer_persona.as_ref()
        .and_then(|p| p.active_scenario_index)
        .and_then(|idx| hand_state.buyer_persona.as_ref()?.scenarios.get(idx))
        .and_then(|s| s.heat_threshold)
        .unwrap_or(999);

    // Update heat bar fill percentage
    let fill_percentage = if heat_threshold > 0 {
        ((current_heat as f32 / heat_threshold as f32) * 100.0).min(100.0)
    } else {
        0.0
    };

    if let Ok(mut node) = bar_fill_query.single_mut() {
        node.height = Val::Percent(fill_percentage);
    }

    // Update heat bar color based on percentage
    if let Ok(mut color) = bar_color_query.single_mut() {
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
    if let Ok(mut text) = bar_text_query.single_mut() {
        **text = format!("{current_heat}/{heat_threshold}");
    }
}

/// Show/hide resolution overlay and update results text
pub fn update_resolution_overlay_system(
    hand_state_query: Query<&HandState, Changed<HandState>>,
    mut overlay_query: Query<&mut Node, With<ResolutionOverlay>>,
    mut title_query: Query<(&mut Text, &mut TextColor), (With<ResolutionTitle>, Without<ResolutionResults>, Without<ResolutionStory>)>,
    mut story_query: Query<&mut Text, (With<ResolutionStory>, Without<ResolutionTitle>, Without<ResolutionResults>)>,
    mut results_query: Query<&mut Text, (With<ResolutionResults>, Without<ResolutionTitle>, Without<ResolutionStory>)>,
) {
    let Ok(hand_state) = hand_state_query.single() else {
        return;
    };

    let Ok(mut overlay_node) = overlay_query.single_mut() else {
        return;
    };

    // Show overlay when hand reaches Bust state
    if hand_state.current_state == HandPhase::Bust {
        overlay_node.display = Display::Flex;

        // Update title based on outcome
        let (mut title_text, mut title_color) = title_query.single_mut()
            .expect("Expected exactly one ResolutionTitle");

        **title_text = match hand_state.outcome {
            Some(HandOutcome::Safe) => "DEAL COMPLETE!".to_string(),
            Some(HandOutcome::Busted) => "BUSTED!".to_string(),
            Some(HandOutcome::Folded) => "HAND FOLDED".to_string(),
            Some(HandOutcome::InvalidDeal) => "INVALID DEAL".to_string(),
            Some(HandOutcome::BuyerBailed) => "BUYER BAILED".to_string(),
            None => "HAND COMPLETE".to_string(),
        };

        // Color code title
        title_color.0 = match hand_state.outcome {
            Some(HandOutcome::Safe) => theme::STATUS_SAFE,
            Some(HandOutcome::Busted) => theme::STATUS_BUSTED,
            Some(HandOutcome::Folded) => theme::STATUS_FOLDED,
            Some(HandOutcome::InvalidDeal) => theme::STATUS_INVALID,
            Some(HandOutcome::BuyerBailed) => theme::STATUS_BAILED,
            None => theme::TEXT_HEADER,
        };

        // SOW-012: Update story text
        let mut story_text = story_query.single_mut()
            .expect("Expected exactly one ResolutionStory");

        if let Some(story) = &hand_state.hand_story {
            **story_text = story.clone();
        } else {
            **story_text = "".to_string();
        }

        // Update results breakdown
        let mut results_text = results_query.single_mut()
            .expect("Expected exactly one ResolutionResults");

        let totals = hand_state.calculate_totals(true);
        // Heat is already accumulated when cards are played, use current_heat directly
        let cumulative_heat = hand_state.current_heat;
        let mut results = String::new();

        match hand_state.outcome {
            Some(HandOutcome::Safe) => {
                results.push_str(&format!("Evidence: {} ≤ Cover: {} ✓\n\n", totals.evidence, totals.cover));
                results.push_str(&format!("Profit: ${}\n", totals.profit));
                results.push_str(&format!("Deck Heat: {}\n", cumulative_heat));

                if hand_state.is_demand_satisfied() {
                    let multiplier = hand_state.get_profit_multiplier();
                    results.push_str(&format!("\nDemand Met! ×{multiplier:.1} multiplier"));
                } else {
                    results.push_str("\nDemand Not Met (reduced multiplier)");
                }
            }
            Some(HandOutcome::Busted) => {
                if hand_state.cards(Owner::Player).deck.len() < 3 {
                    results.push_str(&format!("Deck Exhausted: {} cards\n\nRun Ends", hand_state.cards(Owner::Player).deck.len()));
                } else {
                    results.push_str(&format!("Evidence: {} > Cover: {} ✗\n\n", totals.evidence, totals.cover));
                    results.push_str(&format!("You got caught!\nDeck Heat: {}", cumulative_heat));
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

        **results_text = results;
    } else {
        overlay_node.display = Display::None;
    }
}

/// Scale UI to fit screen while maintaining 16:9 aspect ratio
/// Designed for 1920x1080 (1080p) base layout with letterboxing/pillarboxing
pub fn scale_ui_to_fit_system(
    mut ui_scale: ResMut<UiScale>,
    mut ui_root_query: Query<&mut Node, With<super::components::UiRoot>>,
    mut deck_builder_root_query: Query<&mut Node, (With<super::components::DeckBuilderRoot>, Without<super::components::UiRoot>)>,
    windows: Query<&Window>,
) {
    let Ok(window) = windows.single() else {
        return;
    };

    // Early return if no UI roots exist yet (during initial setup)
    if ui_root_query.is_empty() && deck_builder_root_query.is_empty() {
        return;
    }

    const DESIGN_WIDTH: f32 = 1920.0;
    const DESIGN_HEIGHT: f32 = 1080.0;
    const DESIGN_ASPECT: f32 = DESIGN_WIDTH / DESIGN_HEIGHT;

    let window_width = window.width();
    let window_height = window.height();
    let window_aspect = window_width / window_height;

    // Calculate scale to fit screen while maintaining aspect ratio
    let scale = if window_aspect > DESIGN_ASPECT {
        // Window is wider - fit to height (pillarbox on sides)
        window_height / DESIGN_HEIGHT
    } else {
        // Window is taller - fit to width (letterbox top/bottom)
        window_width / DESIGN_WIDTH
    };

    // Apply uniform scale to all UI elements
    ui_scale.0 = scale;

    // Calculate the scaled UI size (in physical pixels, before UiScale is applied)
    let scaled_width = DESIGN_WIDTH * scale;
    let scaled_height = DESIGN_HEIGHT * scale;

    // Position the UI root centered (using unscaled offsets since UiScale will apply to these too)
    let offset_x = (window_width - scaled_width) / (2.0 * scale);
    let offset_y = (window_height - scaled_height) / (2.0 * scale);

    // Update UI root to be centered (position values will be scaled by UiScale)
    for mut node in ui_root_query.iter_mut() {
        node.position_type = PositionType::Absolute;
        node.left = Val::Px(offset_x);
        node.top = Val::Px(offset_y);
    }

    // Also update deck builder root if it exists
    for mut node in deck_builder_root_query.iter_mut() {
        node.position_type = PositionType::Absolute;
        node.left = Val::Px(offset_x);
        node.top = Val::Px(offset_y);
    }
}

/// POC: Update background image based on active location
/// Uses "cover" scaling: maintains aspect ratio, scales to fill screen
pub fn update_background_system(
    hand_state_query: Query<&HandState, Changed<HandState>>,
    mut background_image_query: Query<(&mut ImageNode, &mut Node), With<BackgroundImageNode>>,
    game_assets: Res<crate::assets::GameAssets>,
    images: Res<Assets<Image>>,
    windows: Query<&Window>,
    ui_scale: Res<UiScale>,
) {
    let Ok(hand_state) = hand_state_query.single() else {
        return;
    };

    let Ok((mut image_node, mut node)) = background_image_query.single_mut() else {
        return;
    };

    let Ok(window) = windows.single() else {
        return;
    };

    // Get the active location
    if let Some(location_card) = hand_state.active_location(true) {
        // Try to find the background image for this location
        if let Some(image_handle) = game_assets.background_images.get(&location_card.name) {
            // Check if image is loaded
            if let Some(image_asset) = images.get(image_handle) {
                image_node.image = image_handle.clone();

                // Account for UiScale when calculating dimensions
                let window_width = window.width() / ui_scale.0;
                let window_height = window.height() / ui_scale.0;
                let window_aspect = window_width / window_height;

                let image_width = image_asset.width() as f32;
                let image_height = image_asset.height() as f32;
                let image_aspect = image_width / image_height;

                // Scale to cover window (fills entire area, crops overflow)
                let (scaled_width, scaled_height) = if window_aspect > image_aspect {
                    // Window is wider - fit to width, crop height
                    (window_width, window_width / image_aspect)
                } else {
                    // Window is taller - fit to height, crop width
                    (window_height * image_aspect, window_height)
                };

                node.width = Val::Px(scaled_width);
                node.height = Val::Px(scaled_height);

                info!("Background: {}", location_card.name);
            }
        }
    } else {
        // No active location - clear the background image
        if image_node.image != Handle::default() {
            image_node.image = Handle::default();
            node.width = Val::Px(0.0);
            node.height = Val::Px(0.0);
            info!("Background cleared");
        }
    }
}

/// Handle mouse wheel scrolling for UI containers with ScrollPosition
/// Bevy 0.17 requires manual scroll handling
pub fn ui_scroll_system(
    mut mouse_wheel_events: EventReader<bevy::input::mouse::MouseWheel>,
    mut scroll_query: Query<(&Interaction, &mut ScrollPosition, &ComputedNode), With<CardPoolContainer>>,
) {
    use bevy::input::mouse::MouseScrollUnit;

    const LINE_HEIGHT: f32 = 21.0;

    for event in mouse_wheel_events.read() {
        for (interaction, mut scroll_position, computed) in scroll_query.iter_mut() {
            // Only scroll when hovering over the container
            if *interaction == Interaction::None || *interaction == Interaction::Hovered {
                let delta_y = match event.unit {
                    MouseScrollUnit::Line => event.y * LINE_HEIGHT,
                    MouseScrollUnit::Pixel => event.y,
                };

                // Calculate max scroll based on content size vs visible size
                let content_height = computed.content_size().y;
                let visible_height = computed.size().y;
                let max_scroll = (content_height - visible_height).max(0.0);

                // Update scroll position (negative because scroll down = positive y offset)
                let new_y = (scroll_position.y - delta_y).clamp(0.0, max_scroll);
                scroll_position.y = new_y;
            }
        }
    }
}
