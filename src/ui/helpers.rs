// UI Helpers - Reusable card display functions
// SOW-011-A Phase 2: Eliminates ~200 lines of duplicated card rendering logic

use bevy::prelude::*;
use crate::CardType;
use crate::EmojiFont;
use super::theme;

/// Card display size variants
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CardSize {
    Small,    // Played pool cards, deck builder cards
    Medium,   // Player hand, active slots, narc/buyer hands
}

impl CardSize {
    pub fn dimensions(&self) -> (f32, f32) {
        match self {
            CardSize::Small => (theme::CARD_WIDTH_SMALL, theme::CARD_HEIGHT_SMALL),
            CardSize::Medium => (theme::CARD_WIDTH_MEDIUM, theme::CARD_HEIGHT_MEDIUM),
        }
    }

    pub fn font_size(&self) -> f32 {
        match self {
            CardSize::Small => 9.5,
            CardSize::Medium => 12.0,
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

/// POC: Stat display info for card template rendering
#[derive(Clone)]
struct StatInfo {
    emoji: &'static str,
    label: &'static str,
    value: String,
}

/// POC: Get stats to display for a card type (consistent ordering)
fn get_card_stats(card_type: &CardType) -> Vec<StatInfo> {
    match card_type {
        CardType::Product { price, heat } => vec![
            StatInfo { emoji: "💰", label: "Price", value: format!("+${}", price) },
            StatInfo { emoji: "🔥", label: "Heat", value: format!("{:+}", heat) },
        ],
        CardType::Location { evidence, cover, heat } => vec![
            StatInfo { emoji: "🔍", label: "Evidence", value: format!("{:+}", evidence) },
            StatInfo { emoji: "🛡", label: "Cover", value: format!("{:+}", cover) },
            StatInfo { emoji: "🔥", label: "Heat", value: format!("{:+}", heat) },
        ],
        CardType::Evidence { evidence, heat } => vec![
            StatInfo { emoji: "🔍", label: "Evidence", value: format!("{:+}", evidence) },
            StatInfo { emoji: "🔥", label: "Heat", value: format!("{:+}", heat) },
        ],
        CardType::Conviction { heat_threshold } => vec![
            StatInfo { emoji: "⚠", label: "Threshold", value: format!("+{}", heat_threshold) },
        ],
        CardType::Cover { cover, heat } => vec![
            StatInfo { emoji: "🛡", label: "Cover", value: format!("{:+}", cover) },
            StatInfo { emoji: "🔥", label: "Heat", value: format!("{:+}", heat) },
        ],
        CardType::Insurance { cover, cost, heat_penalty } => vec![
            StatInfo { emoji: "🛡", label: "Cover", value: format!("{:+}", cover) },
            StatInfo { emoji: "💵", label: "Cost", value: format!("${}", cost) },
            StatInfo { emoji: "🔥", label: "Heat", value: format!("{:+}", heat_penalty) },
        ],
        CardType::DealModifier { price_multiplier, evidence, cover, heat } => {
            let mut stats = vec![];
            if *price_multiplier != 1.0 {
                let delta = ((price_multiplier - 1.0) * 100.0) as i32;
                stats.push(StatInfo { emoji: "💰", label: "Price", value: format!("{:+}%", delta) });
            }
            if *evidence != 0 {
                stats.push(StatInfo { emoji: "🔍", label: "Evidence", value: format!("{:+}", evidence) });
            }
            if *cover != 0 {
                stats.push(StatInfo { emoji: "🛡", label: "Cover", value: format!("{:+}", cover) });
            }
            stats.push(StatInfo { emoji: "🔥", label: "Heat", value: format!("{:+}", heat) });
            stats
        }
    }
}

/// POC: Get category name for a card type
fn get_card_category(card_type: &CardType) -> &'static str {
    match card_type {
        CardType::Product { .. } => "PRODUCT",
        CardType::Location { .. } => "LOCATION",
        CardType::Evidence { .. } => "EVIDENCE",
        CardType::Conviction { .. } => "CONVICTION",
        CardType::Cover { .. } => "COVER",
        CardType::Insurance { .. } => "INSURANCE",
        CardType::DealModifier { .. } => "MODIFIER",
    }
}

/// POC: Spawn a card button using the template image with text overlays
pub fn spawn_card_button_with_template<T: Component>(
    parent: &mut ChildBuilder,
    card_name: &str,
    card_type: &CardType,
    size: CardSize,
    state: CardDisplayState,
    marker: T,
    template_image: Handle<Image>,
    emoji_font: &EmojiFont,
) {
    let (width, _height) = size.dimensions();
    let card_color = get_card_color(card_type, state);
    let stats = get_card_stats(card_type);
    let category = get_card_category(card_type);

    // Template aspect ratio: Actual card_template.png dimensions (601x870)
    let template_aspect = 601.0 / 870.0;
    let height = width / template_aspect;

    // Scale factor for text positioning based on card width
    let scale = width / 120.0; // Base measurements for 120px width

    parent.spawn((
        ButtonBundle {
            style: Style {
                width: Val::Px(width),
                height: Val::Px(height),
                position_type: PositionType::Relative,
                ..default()
            },
            ..default()
        },
        marker,
    ))
    .with_children(|parent| {
        // Template background image (colorized, maintains aspect ratio)
        parent.spawn((
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    ..default()
                },
                ..default()
            },
            UiImage {
                texture: template_image.clone(),
                color: card_color,
                ..default()
            },
        ));

        spawn_card_text_overlays(parent, card_name, &stats, category, scale, emoji_font);
    });
}

/// POC: Spawn a card display using the template image with text overlays
pub fn spawn_card_with_template<T: Component>(
    parent: &mut ChildBuilder,
    card_name: &str,
    card_type: &CardType,
    size: CardSize,
    state: CardDisplayState,
    marker: T,
    template_image: Handle<Image>,
    emoji_font: &EmojiFont,
) {
    let (width, _height) = size.dimensions();
    let card_color = get_card_color(card_type, state);
    let stats = get_card_stats(card_type);
    let category = get_card_category(card_type);

    // Template aspect ratio: Actual card_template.png dimensions (601x870)
    let template_aspect = 601.0 / 870.0;
    let height = width / template_aspect;

    // Scale factor for text positioning based on card width
    let scale = width / 120.0; // Base measurements for 120px width

    parent.spawn((
        NodeBundle {
            style: Style {
                width: Val::Px(width),
                height: Val::Px(height),
                position_type: PositionType::Relative,
                ..default()
            },
            ..default()
        },
        marker,
    ))
    .with_children(|parent| {
        // Template background image (colorized, maintains aspect ratio)
        parent.spawn((
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    ..default()
                },
                ..default()
            },
            UiImage {
                texture: template_image,
                color: card_color,
                ..default()
            },
        ));

        spawn_card_text_overlays(parent, card_name, &stats, category, scale, emoji_font);
    });
}

/// POC: Helper to spawn text overlays on card template
fn spawn_card_text_overlays(
    parent: &mut ChildBuilder,
    card_name: &str,
    stats: &[StatInfo],
    category: &str,
    scale: f32,
    emoji_font: &EmojiFont,
) {
    // Top banner: Card type/category
    parent.spawn(NodeBundle {
        style: Style {
            position_type: PositionType::Absolute,
            left: Val::Percent(10.0),
            right: Val::Percent(10.0),
            top: Val::Px(7.0 * scale),
            justify_content: JustifyContent::Center,
            ..default()
        },
        ..default()
    })
    .with_children(|parent| {
        parent.spawn(TextBundle::from_section(
            category,
            TextStyle {
                font_size: 8.0 * scale,
                color: Color::srgb(0.8, 0.8, 0.8),
                ..default()
            },
        ));
    });

    // Stats (up to 4 rows) - adjusted for actual template layout
    let stat_start_y = 34.0 * scale;  // Start of first stat row (moved 2px down)
    let stat_row_height = 20.0 * scale; // Spacing between rows

    for (idx, stat) in stats.iter().enumerate().take(4) {
        let y_pos = stat_start_y + (idx as f32 * stat_row_height);

        // Emoji icon (left circle)
        parent.spawn(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                left: Val::Px(9.0 * scale),
                top: Val::Px(y_pos),
                width: Val::Px(18.0 * scale),
                height: Val::Px(18.0 * scale),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent.spawn(text_bundle_with_emoji(
                stat.emoji,
                10.0 * scale,
                Color::WHITE,
                emoji_font,
            ));
        });

        // Stat label + value (right box) - moved 4px left total
        parent.spawn(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                left: Val::Px(27.0 * scale),
                right: Val::Px(10.0 * scale),
                top: Val::Px(y_pos),
                flex_direction: FlexDirection::Column,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            // Stat label (small)
            parent.spawn(TextBundle::from_section(
                stat.label,
                TextStyle {
                    font_size: 7.0 * scale,
                    color: Color::srgb(0.7, 0.7, 0.7),
                    ..default()
                },
            ));
            // Stat value (larger)
            parent.spawn(text_bundle_with_emoji(
                &stat.value,
                10.0 * scale,
                Color::WHITE,
                emoji_font,
            ));
        });
    }

    // Bottom banner: Card name (moved 1px down, font size reduced by 1)
    parent.spawn(NodeBundle {
        style: Style {
            position_type: PositionType::Absolute,
            left: Val::Percent(10.0),
            right: Val::Percent(10.0),
            bottom: Val::Px(41.0 * scale), // Moved 1px down (was 42.0 originally)
            justify_content: JustifyContent::Center,
            ..default()
        },
        ..default()
    })
    .with_children(|parent| {
        parent.spawn(TextBundle::from_section(
            card_name,
            TextStyle {
                font_size: 10.0 * scale, // Reduced from 11.0
                color: Color::WHITE,
                ..default()
            },
        ));
    });
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
/// POC: Now uses template image rendering
pub fn spawn_card_display_with_marker<T: Component>(
    parent: &mut ChildBuilder,
    card_name: &str,
    card_type: &CardType,
    size: CardSize,
    state: CardDisplayState,
    _compact_text: bool,
    marker: T,
    template_image: Handle<Image>,
    emoji_font: &EmojiFont,
) {
    spawn_card_with_template(parent, card_name, card_type, size, state, marker, template_image, emoji_font);
}

/// Spawn a card button (interactive, clickable)
/// Use this for player hand cards, deck builder cards
/// POC: Now uses template image rendering
pub fn spawn_card_button<T: Component>(
    parent: &mut ChildBuilder,
    card_name: &str,
    card_type: &CardType,
    size: CardSize,
    state: CardDisplayState,
    marker: T,
    template_image: Handle<Image>,
    emoji_font: &EmojiFont,
) {
    spawn_card_button_with_template(parent, card_name, card_type, size, state, marker, template_image, emoji_font);
}

/// Spawn a ghosted placeholder for an empty slot
pub fn spawn_placeholder(
    parent: &mut ChildBuilder,
    _placeholder_text: &str,
    size: CardSize,
    color_hint: Color,
    placeholder_image: Handle<Image>,
) {
    let (width, _height) = size.dimensions();

    // POC: Match template aspect ratio (601x870)
    let template_aspect = 601.0 / 870.0;
    let height = width / template_aspect;

    parent.spawn(NodeBundle {
        style: Style {
            width: Val::Px(width),
            height: Val::Px(height),
            position_type: PositionType::Relative,
            ..default()
        },
        ..default()
    })
    .with_children(|parent| {
        // Placeholder background image (colorized, maintains aspect ratio)
        parent.spawn((
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    ..default()
                },
                ..default()
            },
            UiImage {
                texture: placeholder_image,
                color: theme::dim_color(color_hint, 0.5), // Dimmed version of card type color
                ..default()
            },
        ));
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
