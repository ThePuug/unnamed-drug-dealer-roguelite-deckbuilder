// UI Helpers - Reusable card display functions
// SOW-011-A Phase 2: Eliminates ~200 lines of duplicated card rendering logic
// Updated for Bevy 0.17

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
    parent: &mut ChildSpawnerCommands,
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
        Button,
        Node {
            width: Val::Px(width),
            height: Val::Px(height),
            position_type: PositionType::Relative,
            ..default()
        },
        marker,
    ))
    .with_children(|parent| {
        // Template background image (colorized, maintains aspect ratio)
        parent.spawn((
            ImageNode::new(template_image.clone()).with_color(card_color),
            Node {
                position_type: PositionType::Absolute,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..default()
            },
        ));

        spawn_card_text_overlays(parent, card_name, &stats, category, scale, emoji_font);
    });
}

/// POC: Spawn a card display using the template image with text overlays
pub fn spawn_card_with_template<T: Component>(
    parent: &mut ChildSpawnerCommands,
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
        Node {
            width: Val::Px(width),
            height: Val::Px(height),
            position_type: PositionType::Relative,
            ..default()
        },
        marker,
    ))
    .with_children(|parent| {
        // Template background image (colorized, maintains aspect ratio)
        parent.spawn((
            ImageNode::new(template_image).with_color(card_color),
            Node {
                position_type: PositionType::Absolute,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..default()
            },
        ));

        spawn_card_text_overlays(parent, card_name, &stats, category, scale, emoji_font);
    });
}

/// POC: Helper to spawn text overlays on card template
fn spawn_card_text_overlays(
    parent: &mut ChildSpawnerCommands,
    card_name: &str,
    stats: &[StatInfo],
    category: &str,
    scale: f32,
    emoji_font: &EmojiFont,
) {
    // Top banner: Card type/category
    parent.spawn(Node {
        position_type: PositionType::Absolute,
        left: Val::Percent(10.0),
        right: Val::Percent(10.0),
        top: Val::Px(7.0 * scale),
        justify_content: JustifyContent::Center,
        ..default()
    })
    .with_children(|parent| {
        parent.spawn((
            Text::new(category),
            TextFont::from_font_size(8.0 * scale),
            TextColor(Color::srgb(0.8, 0.8, 0.8)),
        ));
    });

    // Stats (up to 4 rows) - adjusted for actual template layout
    let stat_start_y = 34.0 * scale;  // Start of first stat row
    let stat_row_height = 19.0 * scale; // Spacing between rows (reduced by 1px)

    for (idx, stat) in stats.iter().enumerate().take(4) {
        let y_pos = stat_start_y + (idx as f32 * stat_row_height);

        // Emoji icon (left circle) - shifted 1px left
        parent.spawn(Node {
            position_type: PositionType::Absolute,
            left: Val::Px(8.0 * scale),
            top: Val::Px(y_pos),
            width: Val::Px(18.0 * scale),
            height: Val::Px(18.0 * scale),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        })
        .with_children(|parent| {
            parent.spawn((
                Text::new(stat.emoji),
                TextFont {
                    font: emoji_font.0.clone(),
                    font_size: 10.0 * scale,
                    ..default()
                },
                TextColor(Color::WHITE),
            ));
        });

        // Stat label + value (right box) - moved 4px left total
        parent.spawn(Node {
            position_type: PositionType::Absolute,
            left: Val::Px(27.0 * scale),
            right: Val::Px(10.0 * scale),
            top: Val::Px(y_pos),
            flex_direction: FlexDirection::Column,
            ..default()
        })
        .with_children(|parent| {
            // Stat label (small)
            parent.spawn((
                Text::new(stat.label),
                TextFont::from_font_size(7.0 * scale),
                TextColor(Color::srgb(0.7, 0.7, 0.7)),
            ));
            // Stat value (larger) - use default font, not emoji font
            parent.spawn((
                Text::new(&stat.value),
                TextFont::from_font_size(10.0 * scale),
                TextColor(Color::WHITE),
            ));
        });
    }

    // Bottom banner: Card name (font size reduced for Bevy 0.17)
    parent.spawn(Node {
        position_type: PositionType::Absolute,
        left: Val::Percent(8.0),
        right: Val::Percent(8.0),
        bottom: Val::Px(41.0 * scale),
        justify_content: JustifyContent::Center,
        ..default()
    })
    .with_children(|parent| {
        parent.spawn((
            Text::new(card_name),
            TextFont::from_font_size(8.0 * scale),
            TextColor(Color::WHITE),
            TextLayout::new_with_linebreak(bevy::text::LineBreak::NoWrap),
        ));
    });
}

/// Spawn a card display node with a marker component
/// Use this when you need to query for specific card entities
/// POC: Now uses template image rendering
pub fn spawn_card_display_with_marker<T: Component>(
    parent: &mut ChildSpawnerCommands,
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
    parent: &mut ChildSpawnerCommands,
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
    parent: &mut ChildSpawnerCommands,
    _placeholder_text: &str,
    size: CardSize,
    color_hint: Color,
    placeholder_image: Handle<Image>,
) {
    let (width, _height) = size.dimensions();

    // POC: Match template aspect ratio (601x870)
    let template_aspect = 601.0 / 870.0;
    let height = width / template_aspect;

    parent.spawn(Node {
        width: Val::Px(width),
        height: Val::Px(height),
        position_type: PositionType::Relative,
        ..default()
    })
    .with_children(|parent| {
        // Placeholder background image (colorized, maintains aspect ratio)
        parent.spawn((
            ImageNode::new(placeholder_image).with_color(theme::dim_color(color_hint, 0.5)),
            Node {
                position_type: PositionType::Absolute,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..default()
            },
        ));
    });
}
