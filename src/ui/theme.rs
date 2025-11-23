// UI Theme - Centralized Colors and Sizing Constants
// SOW-011-A Phase 1: Eliminates hardcoded RGB tuples throughout codebase

use bevy::prelude::Color;

// ============================================================================
// Card Type Colors (Bright - for hand/active display)
// ============================================================================

pub const PRODUCT_CARD_COLOR: Color = Color::srgb(0.9, 0.7, 0.2);      // Bright gold
pub const LOCATION_CARD_COLOR: Color = Color::srgb(0.3, 0.6, 0.9);     // Bright blue
pub const EVIDENCE_CARD_COLOR: Color = Color::srgb(0.8, 0.3, 0.3);     // Bright red
pub const COVER_CARD_COLOR: Color = Color::srgb(0.3, 0.8, 0.3);        // Bright green
pub const DEAL_MODIFIER_CARD_COLOR: Color = Color::srgb(0.7, 0.5, 0.9); // Purple
pub const INSURANCE_CARD_COLOR: Color = Color::srgb(0.2, 0.8, 0.8);    // Cyan
pub const CONVICTION_CARD_COLOR: Color = Color::srgb(0.9, 0.2, 0.2);   // Red
// SOW-AAA: Unused dim color constants removed

// Buyer Card Colors (specific for buyer deck display)
pub const BUYER_LOCATION_COLOR: Color = Color::srgb(0.5, 0.7, 1.0);
pub const BUYER_MODIFIER_COLOR: Color = Color::srgb(0.9, 0.7, 1.0);
// SOW-AAA: BUYER_VISIBLE_HAND_COLOR removed (unused)
pub const BUYER_DEFAULT_COLOR: Color = Color::srgb(0.5, 0.5, 0.5);

// ============================================================================
// UI Section Colors
// ============================================================================

pub const UI_ROOT_BG: Color = Color::srgb(0.1, 0.1, 0.15);             // Dark background
// SOW-AAA: PLAY_AREA_BG removed (unused)
pub const PLAYER_HAND_BG: Color = Color::srgb(0.2, 0.2, 0.25);         // Player hand panel

// Scenario Card (Buyer challenge card)
pub const SCENARIO_CARD_BG: Color = Color::srgb(0.2, 0.2, 0.3);        // Dark blue-gray
pub const SCENARIO_CARD_BORDER: Color = Color::srgb(0.9, 0.9, 0.4);    // Golden yellow
pub const SCENARIO_CARD_TEXT: Color = Color::srgb(0.9, 0.9, 0.4);      // Golden yellow text

// Play Area Colors (section backgrounds)
pub const NARC_SECTION_COLOR: Color = Color::srgb(0.8, 0.3, 0.3);      // Red (threat)
pub const BUYER_SECTION_COLOR: Color = Color::srgb(0.9, 0.9, 0.4);     // Yellow (buyer)

// Buyer Visible Hand
pub const BUYER_HAND_BORDER: Color = Color::srgb(1.0, 1.0, 0.0);       // Bright yellow border
// SOW-AAA: DEALER_CARD_BORDER removed (unused)

// ============================================================================
// State/Status Colors
// ============================================================================

// SOW-AAA: STATUS_PLAYING removed (unused)
pub const STATUS_SAFE: Color = Color::srgb(0.3, 1.0, 0.3);             // Green
pub const STATUS_BUSTED: Color = Color::srgb(1.0, 0.3, 0.3);           // Red
pub const STATUS_FOLDED: Color = Color::srgb(0.7, 0.7, 0.7);           // Gray
pub const STATUS_INVALID: Color = Color::srgb(1.0, 0.6, 0.0);          // Orange
pub const STATUS_BAILED: Color = Color::srgb(1.0, 0.8, 0.0);           // Yellow-orange

// ============================================================================
// Button Colors
// ============================================================================

pub const BUTTON_ENABLED_BG: Color = Color::srgb(0.3, 0.8, 0.3);       // Green
pub const BUTTON_DISABLED_BG: Color = Color::srgb(0.2, 0.2, 0.2);      // Dark gray
pub const BUTTON_NEUTRAL_BG: Color = Color::srgb(0.5, 0.5, 0.5);       // Gray
pub const CONTINUE_BUTTON_BG: Color = Color::srgb(0.3, 0.8, 0.3);      // Green
pub const RESTART_BUTTON_BG: Color = Color::srgb(0.8, 0.3, 0.3);       // Red
// SOW-AAA: GO_HOME_BUTTON_BG, PRESET_BUTTON_BG removed (unused)

// ============================================================================
// Border Colors
// ============================================================================

pub const CARD_BORDER_PLAYED: Color = Color::srgb(0.5, 0.5, 0.5);      // Dim border
pub const CARD_BORDER_SELECTED: Color = Color::srgb(1.0, 1.0, 0.5);    // Bright border
pub const CARD_BORDER_NORMAL: Color = Color::srgb(0.5, 0.5, 0.5);      // Normal border
pub const CARD_BORDER_BRIGHT: Color = Color::srgb(0.9, 0.9, 0.9);      // Bright white

// ============================================================================
// Text Colors
// ============================================================================

pub const TEXT_PRIMARY: Color = Color::srgb(0.9, 0.9, 0.9);            // Light gray (main text)
pub const TEXT_SECONDARY: Color = Color::srgb(0.8, 0.8, 0.8);          // Slightly dimmer
pub const TEXT_HEADER: Color = Color::srgb(0.9, 0.9, 0.4);             // Yellow (headers)

// ============================================================================
// Deck Builder Colors
// ============================================================================

pub const DECK_BUILDER_BG: Color = Color::srgb(0.1, 0.1, 0.1);         // Dark background
pub const CARD_POOL_BG: Color = Color::srgb(0.2, 0.2, 0.2);            // Pool background
pub const SELECTED_DECK_BG_VALID: Color = Color::srgb(0.2, 0.6, 0.2);  // Green (valid deck)
pub const SELECTED_DECK_BG_INVALID: Color = Color::srgb(0.8, 0.2, 0.2); // Red (invalid deck)
pub const CARD_AVAILABLE_BG: Color = Color::srgb(0.3, 0.3, 0.3);       // Available card
pub const CARD_UNAVAILABLE_BG: Color = Color::srgb(0.5, 0.2, 0.2);     // Unavailable (no copies)
// SOW-AAA: PRESET_BUTTON_BG removed (unused)

// ============================================================================
// Misc UI Colors
// ============================================================================

pub const PLACEHOLDER_BG: Color = Color::srgb(0.35, 0.35, 0.35);       // Placeholder card back
// SOW-AAA: PLACEHOLDER_BORDER, PILE_INDICATOR_BG, STAT_BOX_BG, STAT_BOX_BORDER removed (unused)

// ============================================================================
// Card Sizing Constants
// ============================================================================

pub const CARD_WIDTH_SMALL: f32 = 158.65;         // Small cards - played pool, deck builder
pub const CARD_HEIGHT_SMALL: f32 = 202.35;

pub const CARD_WIDTH_MEDIUM: f32 = 189.94;        // Medium cards - player hand, active slots, narc/buyer hands (275px height)
pub const CARD_HEIGHT_MEDIUM: f32 = 275.0;

// SOW-AAA: Unused card size constants removed (HAND, BUYER_VISIBLE, LARGE)

// ============================================================================
// Border/Spacing Constants
// ============================================================================

pub const CARD_BORDER_WIDTH: f32 = 2.0;
// SOW-AAA: CARD_BORDER_WIDTH_THICK, CARD_BORDER_RADIUS removed (unused)

// SOW-AAA: SPACING_TINY removed (unused)
pub const SPACING_SMALL: f32 = 8.0;
pub const SPACING_MEDIUM: f32 = 10.0;
// SOW-AAA: SPACING_MEDIUM_LARGE, SPACING_LARGE, SPACING_XLARGE removed (unused)

// ============================================================================
// Font Sizes
// ============================================================================
// SOW-AAA: All FONT_SIZE constants removed (font sizes defined in CardSize enum)

// ============================================================================
// Heat Bar Constants (SOW-011-A Phase 4)
// ============================================================================
// SOW-AAA: HEAT_BAR_WIDTH, HEAT_BAR_HEIGHT removed (unused)
pub const HEAT_BAR_BG: Color = Color::srgb(0.2, 0.2, 0.2);

// Heat bar gradient colors (stacked nodes)
pub const HEAT_BAR_GREEN: Color = Color::srgb(0.0, 1.0, 0.0);   // 0-50%
pub const HEAT_BAR_YELLOW: Color = Color::srgb(1.0, 1.0, 0.0);  // 50-80%
pub const HEAT_BAR_RED: Color = Color::srgb(1.0, 0.0, 0.0);     // 80-100%

// ============================================================================
// Helper Functions
// ============================================================================

/// Dim a color by both darkening and desaturating
/// factor: brightness factor (typically 0.4)
pub fn dim_color(color: Color, factor: f32) -> Color {
    if let Color::Srgba(srgba) = color {
        // Calculate luminance (perceived brightness)
        let luminance = srgba.red * 0.299 + srgba.green * 0.587 + srgba.blue * 0.114;

        // Desaturate: blend towards gray (luminance)
        let desaturate_amount = 0.5; // 50% desaturation
        let r = srgba.red * (1.0 - desaturate_amount) + luminance * desaturate_amount;
        let g = srgba.green * (1.0 - desaturate_amount) + luminance * desaturate_amount;
        let b = srgba.blue * (1.0 - desaturate_amount) + luminance * desaturate_amount;

        // Then darken
        Color::srgb(r * factor, g * factor, b * factor)
    } else {
        color
    }
}

// SOW-AAA: get_card_color_bright and get_card_color_dim removed (unused)