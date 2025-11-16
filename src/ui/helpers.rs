// UI Helpers - Reusable card display functions
// SOW-011-A Phase 2: Eliminates ~200 lines of duplicated card rendering logic

use bevy::prelude::*;
use crate::CardType;
use crate::EmojiFont;
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

/// Create a TextStyle with emoji font support
/// Use this for any text that contains emoji characters
pub fn text_style_with_emoji(font_size: f32, color: Color, emoji_font: &EmojiFont) -> TextStyle {
    TextStyle {
        font: emoji_font.0.clone(),
        font_size,
        color,
    }
}

/// Create a TextBundle with mixed text and emojis
/// Automatically separates emoji characters to use emoji font, regular text uses default font
/// Example: "🃏 DECK BUILDER 🎴" -> emoji font for 🃏🎴, default font for " DECK BUILDER "
pub fn text_bundle_with_emoji(text: impl Into<String>, font_size: f32, color: Color, emoji_font: &EmojiFont) -> TextBundle {
    let text_string: String = text.into();
    let mut sections = Vec::new();
    let mut current_text = String::new();
    let mut current_is_emoji = false;
    let mut first_char = true;

    for ch in text_string.chars() {
        let is_emoji = is_emoji_char(ch);

        if first_char {
            current_is_emoji = is_emoji;
            first_char = false;
        }

        // If we're switching between emoji/non-emoji, create a new section
        if is_emoji != current_is_emoji {
            if !current_text.is_empty() {
                let style = if current_is_emoji {
                    TextStyle {
                        font: emoji_font.0.clone(),
                        font_size,
                        color,
                    }
                } else {
                    TextStyle {
                        font_size,
                        color,
                        ..default()
                    }
                };
                sections.push(TextSection::new(current_text.clone(), style));
                current_text.clear();
            }
            current_is_emoji = is_emoji;
        }

        current_text.push(ch);
    }

    // Add the last section
    if !current_text.is_empty() {
        let style = if current_is_emoji {
            TextStyle {
                font: emoji_font.0.clone(),
                font_size,
                color,
            }
        } else {
            TextStyle {
                font_size,
                color,
                ..default()
            }
        };
        sections.push(TextSection::new(current_text, style));
    }

    TextBundle::from_sections(sections)
}

/// Check if a character is an emoji
/// This is a simplified check - covers most common emojis
fn is_emoji_char(ch: char) -> bool {
    matches!(ch as u32,
        0x1F300..=0x1F9FF | // Misc Symbols and Pictographs, Emoticons, Transport, etc.
        0x2600..=0x26FF |   // Misc symbols
        0x2700..=0x27BF |   // Dingbats
        0x1F000..=0x1F02F | // Mahjong Tiles, Domino Tiles
        0x1F0A0..=0x1F0FF | // Playing Cards
        0x1F100..=0x1F64F | // Enclosed Alphanumeric Supplement, Emoticons
        0x1F680..=0x1F6FF | // Transport and Map Symbols
        0x1F900..=0x1F9FF | // Supplemental Symbols and Pictographs
        0x1FA00..=0x1FA6F | // Chess Symbols, Symbols and Pictographs Extended-A
        0x1FA70..=0x1FAFF | // Symbols and Pictographs Extended-A
        0x2300..=0x23FF |   // Miscellaneous Technical
        0x2B50 | 0x2B55 |   // Star, Circle
        0x231A | 0x231B |   // Watch, Hourglass
        0x23E9..=0x23F3 |   // Play/Pause buttons
        0x25AA | 0x25AB |   // Squares
        0x25B6 | 0x25C0 |   // Play buttons
        0x25FB..=0x25FE     // Squares
    )
}
