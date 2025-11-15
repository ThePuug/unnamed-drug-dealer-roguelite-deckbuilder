// UI Helpers - Reusable card display functions
// SOW-011-A Phase 2: Eliminates ~200 lines of duplicated card rendering logic

use bevy::prelude::*;
use crate::CardType;
use super::theme;

/// Card display size variants
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CardSize {
    Small,        // 110x140 (narc/buyer hands, played pool)
    Medium,       // 120x152 (player hand, override slots)
    DeckBuilder,  // 110x140 (deck builder)
}

impl CardSize {
    pub fn dimensions(&self) -> (f32, f32) {
        match self {
            CardSize::Small => (theme::CARD_WIDTH_SMALL, theme::CARD_HEIGHT_SMALL),
            CardSize::Medium => (theme::CARD_WIDTH_MEDIUM, theme::CARD_HEIGHT_MEDIUM),
            CardSize::DeckBuilder => (theme::CARD_WIDTH_DECK_BUILDER, theme::CARD_HEIGHT_DECK_BUILDER),
        }
    }

    pub fn font_size(&self) -> f32 {
        match self {
            CardSize::Small => 10.0,
            CardSize::Medium => 12.0,
            CardSize::DeckBuilder => 11.0,
        }
    }
}

/// Card display state - affects visual styling
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CardDisplayState {
    Active,      // Bright colors, highlighted border
    Inactive,    // Dim colors, normal border
    Selected,    // Bright border (for deck builder selection)
}

/// Get color for a card type based on display state
pub fn get_card_color(card_type: &CardType, state: CardDisplayState) -> Color {
    let base_color = match card_type {
        CardType::Product { .. } => theme::PRODUCT_CARD_COLOR,
        CardType::Location { .. } => theme::LOCATION_CARD_COLOR,
        CardType::Evidence { .. } => theme::EVIDENCE_CARD_COLOR,
        CardType::Cover { .. } => theme::COVER_CARD_COLOR,
        CardType::DealModifier { .. } => theme::DEAL_MODIFIER_CARD_COLOR,
        CardType::Insurance { .. } => theme::INSURANCE_CARD_COLOR,
        CardType::Conviction { .. } => theme::CONVICTION_CARD_COLOR,
    };

    match state {
        CardDisplayState::Active | CardDisplayState::Selected => base_color,
        CardDisplayState::Inactive => theme::dim_color(base_color, 0.6),
    }
}

/// Get buyer-specific color for a card type (for buyer visible hand)
pub fn get_buyer_card_color(card_type: &CardType) -> Color {
    match card_type {
        CardType::Location { .. } => theme::BUYER_LOCATION_COLOR,
        CardType::DealModifier { .. } => theme::BUYER_MODIFIER_COLOR,
        _ => theme::BUYER_DEFAULT_COLOR,
    }
}

// SOW-AAA: get_card_color_dim removed (unused)

/// Get border color based on display state
pub fn get_border_color(state: CardDisplayState) -> Color {
    match state {
        CardDisplayState::Active => theme::CARD_BORDER_BRIGHT,
        CardDisplayState::Selected => theme::CARD_BORDER_SELECTED,
        CardDisplayState::Inactive => theme::CARD_BORDER_PLAYED,
    }
}

/// Format card text based on card type
pub fn format_card_text(card_name: &str, card_type: &CardType) -> String {
    match card_type {
        CardType::Product { price, heat } =>
            format!("{card_name}\n${price} | Heat: {heat}"),
        CardType::Location { evidence, cover, heat } =>
            format!("{card_name}\nE:{evidence} C:{cover} H:{heat}"),
        CardType::Evidence { evidence, heat } =>
            format!("{card_name}\nEvidence: {evidence} | Heat: {heat}"),
        CardType::Cover { cover, heat } =>
            format!("{card_name}\nCover: {cover} | Heat: {heat}"),
        CardType::DealModifier { price_multiplier, evidence, cover, heat } =>
            format!("{card_name}\n×{price_multiplier:.1} | E:{evidence} C:{cover} H:{heat}"),
        CardType::Insurance { cover, cost, heat_penalty } =>
            format!("{card_name}\nCover: {cover} | Cost: ${cost} | Heat: {heat_penalty}"),
        CardType::Conviction { heat_threshold } =>
            format!("{card_name}\nThreshold: {heat_threshold}"),
    }
}

/// Compact format for small cards (played cards display)
pub fn format_card_text_compact(card_name: &str, card_type: &CardType) -> String {
    match card_type {
        CardType::Product { price, heat } =>
            format!("{card_name}\n${price} H:{heat}"),
        CardType::Location { evidence, cover, heat } =>
            format!("{card_name}\nE:{evidence} C:{cover} H:{heat}"),
        CardType::Evidence { evidence, heat } =>
            format!("{card_name}\nE:{evidence} H:{heat}"),
        CardType::Cover { cover, heat } =>
            format!("{card_name}\nC:{cover} H:{heat}"),
        CardType::DealModifier { price_multiplier, evidence, cover, heat } =>
            format!("{card_name}\n×{price_multiplier:.1} E:{evidence} C:{cover} H:{heat}"),
        CardType::Insurance { cover, cost, heat_penalty } =>
            format!("{card_name}\nC:{cover} ${cost} H:{heat_penalty}"),
        CardType::Conviction { heat_threshold } =>
            format!("{card_name}\nT:{heat_threshold}"),
    }
}

// SOW-AAA: spawn_card_display removed (unused, use spawn_card_display_with_marker)

/// Spawn a card display node with a marker component
/// Use this when you need to query for specific card entities
pub fn spawn_card_display_with_marker<T: Component>(
    parent: &mut ChildBuilder,
    card_name: &str,
    card_type: &CardType,
    size: CardSize,
    state: CardDisplayState,
    compact_text: bool,
    marker: T,
) {
    let (width, height) = size.dimensions();
    let font_size = size.font_size();
    let card_color = get_card_color(card_type, state);
    let border_color = get_border_color(state);

    let card_text = if compact_text {
        format_card_text_compact(card_name, card_type)
    } else {
        format_card_text(card_name, card_type)
    };

    parent.spawn((
        NodeBundle {
            style: Style {
                width: Val::Px(width),
                height: Val::Px(height),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                padding: UiRect::all(Val::Px(8.0)),
                border: UiRect::all(Val::Px(theme::CARD_BORDER_WIDTH)),
                margin: UiRect::all(Val::Px(theme::SPACING_MEDIUM)), // Small cards have margin
                ..default()
            },
            background_color: card_color.into(),
            border_color: border_color.into(),
            ..default()
        },
        marker,
    ))
    .with_children(|parent| {
        parent.spawn(TextBundle::from_section(
            card_text,
            TextStyle {
                font_size,
                color: theme::TEXT_PRIMARY,
                ..default()
            },
        ));
    });
}

/// Spawn a card button (interactive, clickable)
/// Use this for player hand cards, deck builder cards
pub fn spawn_card_button<T: Component>(
    parent: &mut ChildBuilder,
    card_name: &str,
    card_type: &CardType,
    size: CardSize,
    state: CardDisplayState,
    marker: T,
) {
    let (width, height) = size.dimensions();
    let font_size = size.font_size();
    let card_color = get_card_color(card_type, state);
    let border_color = get_border_color(state);

    let card_text = format_card_text(card_name, card_type);

    parent.spawn((
        ButtonBundle {
            style: Style {
                width: Val::Px(width),
                height: Val::Px(height),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                padding: UiRect::all(Val::Px(8.0)),
                border: UiRect::all(Val::Px(theme::CARD_BORDER_WIDTH)),
                ..default()
            },
            background_color: card_color.into(),
            border_color: border_color.into(),
            ..default()
        },
        marker,
    ))
    .with_children(|parent| {
        parent.spawn(TextBundle::from_section(
            card_text,
            TextStyle {
                font_size,
                color: Color::BLACK, // Buttons use black text for readability
                ..default()
            },
        ).with_text_justify(JustifyText::Center));
    });
}

/// Spawn a ghosted placeholder for an empty slot
pub fn spawn_placeholder(
    parent: &mut ChildBuilder,
    placeholder_text: &str,
    size: CardSize,
    color_hint: Color,
) {
    let (width, height) = size.dimensions();
    let font_size = size.font_size();

    parent.spawn(NodeBundle {
        style: Style {
            width: Val::Px(width),
            height: Val::Px(height),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            padding: UiRect::all(Val::Px(8.0)),
            border: UiRect::all(Val::Px(theme::CARD_BORDER_WIDTH)),
            // Margin only for Small size (narc/buyer/played pool), not Medium (player hand/slots)
            ..default()
        },
        background_color: theme::PLACEHOLDER_BG.into(),
        border_color: theme::dim_color(color_hint, 0.5).into(), // Dashed effect via dim color
        ..default()
    })
    .with_children(|parent| {
        parent.spawn(TextBundle::from_section(
            placeholder_text,
            TextStyle {
                font_size,
                color: theme::TEXT_SECONDARY,
                ..default()
            },
        ).with_text_justify(JustifyText::Center));
    });
}
