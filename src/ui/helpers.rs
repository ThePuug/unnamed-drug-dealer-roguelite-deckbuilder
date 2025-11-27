// UI Helpers - Reusable card display functions
// SOW-011-A Phase 2: Eliminates ~200 lines of duplicated card rendering logic
// Updated for Bevy 0.17

use bevy::prelude::*;
use crate::CardType;
use crate::EmojiFont;
use super::theme;
use super::foil_material::FoilCard;

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

/// RFC-017: Upgrade tier info for card display
#[derive(Clone)]
pub struct UpgradeInfo {
    pub tier_name: String,      // e.g., "★" for upgraded tiers
    pub plays: u32,             // Current play count
    pub plays_to_next: Option<u32>,  // Plays needed for next tier
    pub multiplier: f32,        // Stat multiplier (1.0 = no bonus, up to 1.5)
    pub star_color: (f32, f32, f32), // RGB color for star (grey/bronze/silver/gold)
    pub is_foil: bool,          // Whether card has foil effect (max tier)
}

impl Default for UpgradeInfo {
    fn default() -> Self {
        Self {
            tier_name: "Base".to_string(),
            plays: 0,
            plays_to_next: Some(1),
            multiplier: 1.0,
            star_color: (0.5, 0.5, 0.5),
            is_foil: false,
        }
    }
}

/// POC: Get stats to display for a card type (consistent ordering)
fn get_card_stats(card_type: &CardType) -> Vec<StatInfo> {
    get_card_stats_with_multiplier(card_type, 1.0)
}

/// RFC-017: Get stats with upgrade multiplier applied to primary stat
fn get_card_stats_with_multiplier(card_type: &CardType, multiplier: f32) -> Vec<StatInfo> {
    match card_type {
        CardType::Product { price, heat } => {
            // Product: +10% price
            let upgraded_price = (*price as f32 * multiplier).round() as i32;
            vec![
                StatInfo { emoji: "💰", label: "Price", value: format!("+${}", upgraded_price) },
                StatInfo { emoji: "🔥", label: "Heat", value: format!("{:+}", heat) },
            ]
        },
        CardType::Location { evidence, cover, heat } => {
            // Location: +10% cover, -10% evidence (reduced is good)
            let upgraded_cover = (*cover as f32 * multiplier).round() as i32;
            let evidence_reduction = 2.0 - multiplier; // 1.1 multiplier -> 0.9 evidence
            let upgraded_evidence = (*evidence as f32 * evidence_reduction).round() as i32;
            vec![
                StatInfo { emoji: "🔍", label: "Evidence", value: format!("{:+}", upgraded_evidence) },
                StatInfo { emoji: "🛡", label: "Cover", value: format!("{:+}", upgraded_cover) },
                StatInfo { emoji: "🔥", label: "Heat", value: format!("{:+}", heat) },
            ]
        },
        CardType::Evidence { evidence, heat } => {
            // RFC-018: Narc cards get upgrades based on character heat tier
            let upgraded_evidence = (*evidence as f32 * multiplier).round() as i32;
            let upgraded_heat = (*heat as f32 * multiplier).round() as i32;
            vec![
                StatInfo { emoji: "🔍", label: "Evidence", value: format!("{:+}", upgraded_evidence) },
                StatInfo { emoji: "🔥", label: "Heat", value: format!("{:+}", upgraded_heat) },
            ]
        },
        CardType::Conviction { heat_threshold } => {
            // RFC-018: Narc cards get upgrades based on character heat tier
            let upgraded_threshold = (*heat_threshold as f32 * multiplier).round() as u32;
            vec![
                StatInfo { emoji: "⚠", label: "Threshold", value: format!("+{}", upgraded_threshold) },
            ]
        },
        CardType::Cover { cover, heat } => {
            // Cover: +10% cover value
            let upgraded_cover = (*cover as f32 * multiplier).round() as i32;
            vec![
                StatInfo { emoji: "🛡", label: "Cover", value: format!("{:+}", upgraded_cover) },
                StatInfo { emoji: "🔥", label: "Heat", value: format!("{:+}", heat) },
            ]
        },
        CardType::Insurance { cover, cost, heat_penalty } => {
            // Insurance: +10% cover value
            let upgraded_cover = (*cover as f32 * multiplier).round() as i32;
            vec![
                StatInfo { emoji: "🛡", label: "Cover", value: format!("{:+}", upgraded_cover) },
                StatInfo { emoji: "💵", label: "Cost", value: format!("${}", cost) },
                StatInfo { emoji: "🔥", label: "Heat", value: format!("{:+}", heat_penalty) },
            ]
        },
        CardType::DealModifier { price_multiplier, evidence, cover, heat } => {
            // DealModifier: +10% to beneficial stats (price bonus, cover), -10% to negative stats (evidence)
            let mut stats = vec![];
            if *price_multiplier != 1.0 {
                let base_delta = (price_multiplier - 1.0) * 100.0;
                let upgraded_delta = (base_delta * multiplier).round() as i32;
                stats.push(StatInfo { emoji: "💰", label: "Price", value: format!("{:+}%", upgraded_delta) });
            }
            if *evidence != 0 {
                // Negative stat - reduce it (good for player)
                let evidence_reduction = 2.0 - multiplier;
                let upgraded_evidence = (*evidence as f32 * evidence_reduction).round() as i32;
                stats.push(StatInfo { emoji: "🔍", label: "Evidence", value: format!("{:+}", upgraded_evidence) });
            }
            if *cover != 0 {
                // Positive stat - increase it
                let upgraded_cover = (*cover as f32 * multiplier).round() as i32;
                stats.push(StatInfo { emoji: "🛡", label: "Cover", value: format!("{:+}", upgraded_cover) });
            }
            // Heat stays as-is (not a primary stat for modifiers)
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

        spawn_card_text_overlays(parent, card_name, &stats, category, scale, emoji_font, None);
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

        spawn_card_text_overlays(parent, card_name, &stats, category, scale, emoji_font, None);
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
    upgrade_info: Option<&UpgradeInfo>,
) {
    // RFC-017: Top-right tier badge (if upgraded)
    if let Some(info) = upgrade_info {
        if info.tier_name != "Base" {
            // Get star color from tier
            let (r, g, b) = info.star_color;
            let star_color = Color::srgb(r, g, b);

            // Badge background: dark for contrast, or special for foil
            let badge_bg = if info.is_foil {
                Color::srgb(0.2, 0.1, 0.3) // Purple-ish for foil
            } else {
                Color::srgb(0.15, 0.15, 0.15) // Dark background
            };

            // Upgraded card - show tier badge with colored star
            parent.spawn((
                Node {
                    position_type: PositionType::Absolute,
                    right: Val::Px(4.0 * scale),
                    top: Val::Px(4.0 * scale),
                    width: Val::Px(16.0 * scale),
                    height: Val::Px(16.0 * scale),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                BackgroundColor(badge_bg),
                BorderRadius::all(Val::Px(8.0 * scale)),
            ))
            .with_children(|parent| {
                // Use default font (DejaVuSans has filled star ★ U+2605)
                parent.spawn((
                    Text::new(&info.tier_name),
                    TextFont::from_font_size(14.0 * scale),
                    TextColor(star_color),
                ));
            });
        }

        // Show play count progress (bottom-right)
        if let Some(next_threshold) = info.plays_to_next {
            parent.spawn(Node {
                position_type: PositionType::Absolute,
                right: Val::Px(4.0 * scale),
                bottom: Val::Px(4.0 * scale),
                padding: UiRect::all(Val::Px(2.0 * scale)),
                ..default()
            })
            .with_children(|parent| {
                parent.spawn((
                    Text::new(format!("{}/{}", info.plays, next_threshold)),
                    TextFont::from_font_size(6.0 * scale),
                    TextColor(Color::srgb(0.7, 0.7, 0.7)),
                ));
            });
        }
    }

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
    spawn_card_display_with_upgrade(parent, card_name, card_type, size, state, marker, template_image, emoji_font, None);
}

/// RFC-017: Spawn a card display with upgrade tier info
pub fn spawn_card_display_with_upgrade<T: Component>(
    parent: &mut ChildSpawnerCommands,
    card_name: &str,
    card_type: &CardType,
    size: CardSize,
    state: CardDisplayState,
    marker: T,
    template_image: Handle<Image>,
    emoji_font: &EmojiFont,
    upgrade_info: Option<UpgradeInfo>,
) {
    let (width, _height) = size.dimensions();
    let card_color = get_card_color(card_type, state);
    let multiplier = upgrade_info.as_ref().map(|i| i.multiplier).unwrap_or(1.0);
    let stats = get_card_stats_with_multiplier(card_type, multiplier);
    let category = get_card_category(card_type);

    let template_aspect = 601.0 / 870.0;
    let height = width / template_aspect;
    let scale = width / 120.0;

    let is_foil = upgrade_info.as_ref().is_some_and(|i| i.is_foil);

    let mut entity_commands = parent.spawn((
        Node {
            width: Val::Px(width),
            height: Val::Px(height),
            position_type: PositionType::Relative,
            ..default()
        },
        marker,
    ));

    // RFC-017: Add FoilCard marker for max tier cards (shader overlay added by system)
    if is_foil {
        entity_commands.insert(FoilCard);
    }

    entity_commands.with_children(|parent| {
        parent.spawn((
            ImageNode::new(template_image).with_color(card_color),
            Node {
                position_type: PositionType::Absolute,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..default()
            },
        ));

        spawn_card_text_overlays(parent, card_name, &stats, category, scale, emoji_font, upgrade_info.as_ref());
    });
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
    spawn_card_button_with_upgrade(parent, card_name, card_type, size, state, marker, template_image, emoji_font, None);
}

/// RFC-017: Spawn a card button with upgrade tier display
pub fn spawn_card_button_with_upgrade<T: Component>(
    parent: &mut ChildSpawnerCommands,
    card_name: &str,
    card_type: &CardType,
    size: CardSize,
    state: CardDisplayState,
    marker: T,
    template_image: Handle<Image>,
    emoji_font: &EmojiFont,
    upgrade_info: Option<UpgradeInfo>,
) {
    let (width, _height) = size.dimensions();
    let card_color = get_card_color(card_type, state);
    // RFC-017: Apply upgrade multiplier to displayed stats
    let multiplier = upgrade_info.as_ref().map(|i| i.multiplier).unwrap_or(1.0);
    let stats = get_card_stats_with_multiplier(card_type, multiplier);
    let category = get_card_category(card_type);

    let template_aspect = 601.0 / 870.0;
    let height = width / template_aspect;
    let scale = width / 120.0;

    let is_foil = upgrade_info.as_ref().is_some_and(|i| i.is_foil);

    let mut entity_commands = parent.spawn((
        Button,
        Node {
            width: Val::Px(width),
            height: Val::Px(height),
            position_type: PositionType::Relative,
            ..default()
        },
        marker,
    ));

    // RFC-017: Add FoilCard marker for max tier cards (shader overlay added by system)
    if is_foil {
        entity_commands.insert(FoilCard);
    }

    entity_commands.with_children(|parent| {
        parent.spawn((
            ImageNode::new(template_image).with_color(card_color),
            Node {
                position_type: PositionType::Absolute,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..default()
            },
        ));

        spawn_card_text_overlays(parent, card_name, &stats, category, scale, emoji_font, upgrade_info.as_ref());
    });
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
