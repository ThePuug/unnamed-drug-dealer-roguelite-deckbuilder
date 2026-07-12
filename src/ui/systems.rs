// UI Systems - Update systems for active slots, heat bar, etc.
// SOW-011-A Phase 4: Active slot population and heat bar updates
// SOW-011-B Phase 1: Resolution overlay system
// Updated for Bevy 0.18

use bevy::prelude::*;
use crate::{HandState, CardType, Card, HandPhase, HandOutcome};
use super::components::*;
use super::helpers;
use super::theme;
use super::view;

/// Update active slots with current Product/Location/Conviction/Insurance cards
pub fn update_active_slots_system(
    mut hand_state_query: Query<&mut HandState, Changed<HandState>>,
    slots_query: Query<(Entity, &ActiveSlot)>,
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
    // (SOW-022: the discard STACK display derives from cards_played in
    // update_deck_discard_system; discard_pile remains the engine-facing record)
    for card in replaced_cards {
        hand_state.discard_pile.push(card);
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

        // Spawn card or placeholder into the table slot
        commands.entity(slot_entity).with_children(|parent| {
            if let Some(card) = active_card {
                // RFC-017/018 badge info (SOW-022: shared helper)
                let upgrade_info = helpers::upgrade_info_for(&hand_state, card);

                // SOW-022: Table-size card on "the deal on the table"
                helpers::spawn_card_display_with_upgrade(
                    parent,
                    &card.name,
                    &card.card_type,
                    helpers::CardSize::Table,
                    helpers::CardDisplayState::Active,
                    PlayedCardDisplay,
                    game_assets.card_template.clone(),
                    &emoji_font,
                    upgrade_info,
                );
            } else if slot.slot_type == SlotType::Insurance {
                // SOW-022: ghost "+ INSURANCE" slot invites the play
                let (width, height) = helpers::CardSize::Table.dimensions();
                parent.spawn((
                    Node {
                        width: Val::Px(width),
                        height: Val::Px(height),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        border: UiRect::all(Val::Px(2.0)),
                        border_radius: BorderRadius::all(Val::Px(12.0)),
                        ..default()
                    },
                    BackgroundColor(theme::GHOST_INSURANCE_BG),
                    BorderColor::all(theme::GHOST_INSURANCE_BORDER),
                    PlayedCardDisplay,
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Text::new("+ INSURANCE"),
                        TextFont::from_font_size(12.0),
                        TextColor(theme::GHOST_INSURANCE_TEXT),
                    ));
                });
            } else {
                // Ghosted placeholder communicates the slot type by color
                let color = match slot.slot_type {
                    SlotType::Location => theme::LOCATION_CARD_COLOR,
                    SlotType::Product => theme::PRODUCT_CARD_COLOR,
                    SlotType::Conviction => theme::CONVICTION_CARD_COLOR,
                    SlotType::Insurance => theme::INSURANCE_CARD_COLOR,
                };

                helpers::spawn_placeholder(
                    parent,
                    "",
                    helpers::CardSize::Table,
                    color,
                    game_assets.card_placeholder.clone(),
                );
            }
        });
    }
}

/// SOW-022: YOUR STANDING panel - session cash, heat value/tier, 0..100 heat
/// track with conviction-threshold tick marks
pub fn update_standing_panel_system(
    hand_state_query: Query<&HandState, Changed<HandState>>,
    mut texts: Query<&mut Text>,
    cash_query: Query<Entity, With<StandingCashText>>,
    heat_value_query: Query<Entity, With<StandingHeatValueText>>,
    tier_text_query: Query<Entity, With<StandingHeatTierText>>,
    mut text_colors: Query<&mut TextColor>,
    mut tier_chip_query: Query<&mut BorderColor, With<StandingHeatTierChip>>,
    mut fill_query: Query<&mut Node, With<StandingHeatBarFill>>,
    ticks_query: Query<Entity, With<StandingHeatTicks>>,
    tick_labels_query: Query<Entity, With<StandingHeatTickLabels>>,
    children_query: Query<&Children>,
    mut commands: Commands,
) {
    let Ok(hand_state) = hand_state_query.single() else {
        return;
    };

    let mut set_text = |entity: Option<Entity>, value: String| {
        if let Some(entity) = entity {
            if let Ok(mut text) = texts.get_mut(entity) {
                if **text != value {
                    **text = value;
                }
            }
        }
    };

    set_text(cash_query.single().ok(), view::format_cash(hand_state.cash));
    set_text(heat_value_query.single().ok(), hand_state.current_heat.to_string());

    // Heat tier chip (name + color from the shared HeatTier scale; session
    // heat is signed - negative reads as Cold)
    let tier = crate::save::HeatTier::from_heat(hand_state.current_heat.max(0) as u32);
    let (r, g, b) = tier.color();
    let tier_color = Color::srgb(r, g, b);
    set_text(tier_text_query.single().ok(), tier.name().to_uppercase());
    if let Ok(tier_entity) = tier_text_query.single() {
        if let Ok(mut color) = text_colors.get_mut(tier_entity) {
            color.0 = tier_color;
        }
    }
    if let Ok(mut border) = tier_chip_query.single_mut() {
        *border = BorderColor::all(tier_color.with_alpha(0.5));
    }

    // Track fill on the fixed 0..100 scale (negative heat shows an empty bar)
    if let Ok(mut node) = fill_query.single_mut() {
        let pct = hand_state.current_heat.clamp(0, view::HEAT_BAR_MAX as i32) as f32
            / view::HEAT_BAR_MAX as f32
            * 100.0;
        node.width = Val::Percent(pct);
    }

    // Conviction-threshold tick marks + labels (content-driven)
    let ticks = view::conviction_ticks(hand_state);

    if let Ok(ticks_entity) = ticks_query.single() {
        if let Ok(children) = children_query.get(ticks_entity) {
            for child in children.iter() {
                commands.entity(child).despawn();
            }
        }
        commands.entity(ticks_entity).with_children(|parent| {
            for (threshold, _) in &ticks {
                parent.spawn((
                    Node {
                        position_type: PositionType::Absolute,
                        left: Val::Percent(*threshold as f32 / view::HEAT_BAR_MAX as f32 * 100.0),
                        top: Val::Px(-2.0),
                        bottom: Val::Px(-2.0),
                        width: Val::Px(1.0),
                        ..default()
                    },
                    BackgroundColor(theme::STANDING_TICK),
                ));
            }
        });
    }

    if let Ok(labels_entity) = tick_labels_query.single() {
        if let Ok(children) = children_query.get(labels_entity) {
            for child in children.iter() {
                commands.entity(child).despawn();
            }
        }
        commands.entity(labels_entity).with_children(|parent| {
            for (index, (threshold, name)) in ticks.iter().enumerate() {
                // The nearest (lowest) threshold carries its card name
                let (label, color) = if index == 0 {
                    (format!("{name} {threshold}"), theme::STANDING_TICK_LABEL_FIRST)
                } else {
                    (threshold.to_string(), theme::STANDING_TICK_LABEL)
                };
                parent.spawn((
                    Text::new(label),
                    TextFont::from_font_size(9.0),
                    TextColor(color),
                    Node {
                        position_type: PositionType::Absolute,
                        left: Val::Percent(*threshold as f32 / view::HEAT_BAR_MAX as f32 * 100.0),
                        ..default()
                    },
                    // center the label on the tick
                    UiTransform::from_translation(Val2::percent(-50.0, 0.0)),
                ));
            }
        });
    }
}

/// SOW-022: Evidence vs Cover balance bar with SAFE/AT RISK + payout chips
pub fn update_balance_bar_system(
    hand_state_query: Query<&HandState, Changed<HandState>>,
    mut texts: Query<&mut Text>,
    evidence_text_query: Query<Entity, With<BalanceEvidenceText>>,
    cover_text_query: Query<Entity, With<BalanceCoverText>>,
    status_text_query: Query<Entity, With<BalanceStatusChipText>>,
    payout_text_query: Query<Entity, With<BalancePayoutChipText>>,
    mut text_colors: Query<&mut TextColor>,
    mut status_chip_query: Query<(&mut BackgroundColor, &mut BorderColor), With<BalanceStatusChip>>,
    mut fills: Query<&mut Node>,
    evidence_fill_query: Query<Entity, With<BalanceEvidenceFill>>,
    cover_fill_query: Query<Entity, With<BalanceCoverFill>>,
    divider_query: Query<Entity, With<BalanceDivider>>,
) {
    let Ok(hand_state) = hand_state_query.single() else {
        return;
    };

    let totals = hand_state.calculate_totals(true);

    let mut set_text = |entity: Option<Entity>, value: String| {
        if let Some(entity) = entity {
            if let Ok(mut text) = texts.get_mut(entity) {
                if **text != value {
                    **text = value;
                }
            }
        }
    };

    set_text(evidence_text_query.single().ok(), format!("EVIDENCE {}", totals.evidence));
    set_text(cover_text_query.single().ok(), format!("COVER {}", totals.cover));

    let multiplier = hand_state.get_profit_multiplier();
    set_text(payout_text_query.single().ok(), format!("PAYOUT ×{multiplier:.1}"));

    // SAFE / AT RISK chip (ties go to the player - resolution.rs)
    let is_safe = totals.evidence <= totals.cover;
    set_text(
        status_text_query.single().ok(),
        if is_safe { "SAFE".to_string() } else { "AT RISK".to_string() },
    );
    if let Ok(entity) = status_text_query.single() {
        if let Ok(mut color) = text_colors.get_mut(entity) {
            color.0 = if is_safe { theme::SAFE_CHIP_TEXT } else { theme::RISK_CHIP_TEXT };
        }
    }
    if let Ok((mut bg, mut border)) = status_chip_query.single_mut() {
        if is_safe {
            *bg = theme::SAFE_CHIP_BG.into();
            *border = BorderColor::all(theme::SAFE_CHIP_BORDER);
        } else {
            *bg = theme::RISK_CHIP_BG.into();
            *border = BorderColor::all(theme::RISK_CHIP_BORDER);
        }
    }

    // Bar split
    let evidence_pct = view::balance_split(totals.evidence, totals.cover);
    if let Ok(entity) = evidence_fill_query.single() {
        if let Ok(mut node) = fills.get_mut(entity) {
            node.width = Val::Percent(evidence_pct);
        }
    }
    if let Ok(entity) = cover_fill_query.single() {
        if let Ok(mut node) = fills.get_mut(entity) {
            node.width = Val::Percent(100.0 - evidence_pct);
        }
    }
    if let Ok(entity) = divider_query.single() {
        if let Ok(mut node) = fills.get_mut(entity) {
            node.left = Val::Percent(evidence_pct);
        }
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
                results.push_str(&format!("This Deal: +${}\n", totals.profit));

                if hand_state.is_demand_satisfied() {
                    let multiplier = hand_state.get_profit_multiplier();
                    results.push_str(&format!("Demand Met! ×{multiplier:.1} multiplier\n"));
                } else {
                    results.push_str("Demand Not Met (reduced multiplier)\n");
                }
            }
            Some(HandOutcome::Busted) => {
                // SOW-021: Busted now ONLY means a genuine bust (exhaustion no longer
                // fabricates this outcome), so always explain the actual cause.
                // The old deck.len() < 3 special case here mislabeled real late-run
                // busts as "Deck Exhausted" - exhaustion messaging lives on the
                // NEW DEAL button ("OUT OF CARDS") instead.
                results.push_str(&format!("Evidence: {} > Cover: {} ✗\n\n", totals.evidence, totals.cover));
                results.push_str("You got caught!");
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

        // Session totals (always show; heat is signed - a cooling session shows negative)
        results.push_str(&format!("\n\n─────────────────\nSession: ${} banked • {:+} Heat",
            hand_state.cash, cumulative_heat));

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

/// Update background image based on active location.
/// Uses "cover" scaling: maintains aspect ratio, scales to fill screen.
///
/// SOW-022 follow-up: runs every frame WITHOUT `Changed<HandState>`. Images
/// stream in asynchronously, and the old change-gated version consumed the
/// trigger even when the asset wasn't loaded yet - a location played while its
/// art was still loading left a stale background until the next state change
/// (or until NEW DEAL if it was the hand's final play). All writes below are
/// guarded by comparisons, so nothing is dirtied once the display is correct;
/// this also picks up window resizes, which the old version ignored.
pub fn update_background_system(
    hand_state_query: Query<&HandState>,
    mut background_image_query: Query<(&mut ImageNode, &mut Node), With<BackgroundImageNode>>,
    game_assets: Res<crate::assets::GameAssets>,
    images: Res<Assets<Image>>,
    windows: Query<&Window>,
    ui_scale: Res<UiScale>,
) {
    let Ok((mut image_node, mut node)) = background_image_query.single_mut() else {
        return;
    };

    let Ok(window) = windows.single() else {
        return;
    };

    let active_location = hand_state_query
        .single()
        .ok()
        .and_then(|hand_state| hand_state.active_location(true).cloned());

    let Some(location_card) = active_location else {
        // No active hand/location - clear the background image
        if image_node.image != Handle::default() {
            image_node.image = Handle::default();
            node.width = Val::Px(0.0);
            node.height = Val::Px(0.0);
            info!("Background cleared");
        }
        return;
    };

    let Some(image_handle) = game_assets.background_images.get(&location_card.name) else {
        return; // no art authored for this location - keep whatever is showing
    };

    // Not loaded yet: leave the current background and retry next frame
    let Some(image_asset) = images.get(image_handle) else {
        return;
    };

    if image_node.image != *image_handle {
        image_node.image = image_handle.clone();
        info!("Background: {}", location_card.name);
    }

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

    if node.width != Val::Px(scaled_width) {
        node.width = Val::Px(scaled_width);
    }
    if node.height != Val::Px(scaled_height) {
        node.height = Val::Px(scaled_height);
    }
}

/// Handle mouse wheel scrolling for UI containers with ScrollPosition
/// Bevy 0.18 requires manual scroll handling
pub fn ui_scroll_system(
    mut mouse_wheel_events: MessageReader<bevy::input::mouse::MouseWheel>,
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

/// SOW-021/SOW-022: Round header + actor pill - the player must always know
/// which of the 3 rounds they are in and whose action is in progress
pub fn update_turn_display_system(
    hand_state_query: Query<&HandState>,
    mut header_query: Query<&mut Text, (With<TurnIndicatorText>, Without<TurnPillText>)>,
    mut pill_text_query: Query<(&mut Text, &mut TextColor), (With<TurnPillText>, Without<TurnIndicatorText>)>,
    mut pill_query: Query<(&mut BackgroundColor, &mut BorderColor), (With<TurnPill>, Without<TurnPillDot>)>,
    mut dot_query: Query<&mut BackgroundColor, (With<TurnPillDot>, Without<TurnPill>)>,
) {
    let Ok(hand_state) = hand_state_query.single() else {
        return;
    };

    // Header line ("ROUND 2 / 3 · DEAL IN PROGRESS") - write only on change
    if let Ok(mut text) = header_query.single_mut() {
        let header = view::round_header(hand_state);
        if **text != header {
            **text = header;
        }
    }

    let (label, actor) = view::turn_pill(hand_state);
    let (bg, border, dot, text_color) = match actor {
        view::PillActor::Narc => (
            theme::PILL_NARC_BG,
            theme::PILL_NARC_BORDER,
            theme::PILL_NARC_DOT,
            theme::PILL_NARC_TEXT,
        ),
        view::PillActor::Player => (
            theme::PILL_PLAYER_BG,
            theme::PILL_PLAYER_BORDER,
            theme::PILL_PLAYER_DOT,
            theme::PILL_PLAYER_TEXT,
        ),
        view::PillActor::Buyer => (
            theme::PILL_BUYER_BG,
            theme::PILL_BUYER_BORDER,
            theme::PILL_BUYER_DOT,
            theme::PILL_BUYER_TEXT,
        ),
        view::PillActor::Neutral => (
            theme::PILL_NEUTRAL_BG,
            theme::PILL_NEUTRAL_BORDER,
            theme::PILL_NEUTRAL_DOT,
            theme::PILL_NEUTRAL_TEXT,
        ),
    };

    if let Ok((mut text, mut color)) = pill_text_query.single_mut() {
        // Label change gates all pill restyling (avoids per-frame dirtying)
        if **text != label {
            **text = label.to_string();
            color.0 = text_color;
            if let Ok((mut pill_bg, mut pill_border)) = pill_query.single_mut() {
                *pill_bg = bg.into();
                *pill_border = BorderColor::all(border);
            }
            if let Ok(mut dot_bg) = dot_query.single_mut() {
                *dot_bg = dot.into();
            }
        }
    }
}
