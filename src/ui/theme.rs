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
pub const DEFAULT_CARD_COLOR: Color = Color::srgb(0.5, 0.5, 0.5);      // Gray

// Card Type Colors (Dim - for played/inactive display)
pub const PRODUCT_CARD_COLOR_DIM: Color = Color::srgb(0.5, 0.4, 0.1);
pub const LOCATION_CARD_COLOR_DIM: Color = Color::srgb(0.2, 0.3, 0.5);
pub const EVIDENCE_CARD_COLOR_DIM: Color = Color::srgb(0.4, 0.2, 0.2);
pub const COVER_CARD_COLOR_DIM: Color = Color::srgb(0.2, 0.4, 0.2);
pub const DEAL_MODIFIER_CARD_COLOR_DIM: Color = Color::srgb(0.4, 0.3, 0.5);
pub const INSURANCE_CARD_COLOR_DIM: Color = Color::srgb(0.1, 0.4, 0.4);
pub const CONVICTION_CARD_COLOR_DIM: Color = Color::srgb(0.5, 0.1, 0.1);
pub const DEFAULT_CARD_COLOR_DIM: Color = Color::srgb(0.3, 0.3, 0.3);

// Buyer Card Colors (specific for buyer deck display)
pub const BUYER_LOCATION_COLOR: Color = Color::srgb(0.5, 0.7, 1.0);
pub const BUYER_MODIFIER_COLOR: Color = Color::srgb(0.9, 0.7, 1.0);
pub const BUYER_VISIBLE_HAND_COLOR: Color = Color::srgb(0.5, 0.7, 1.0); // Muted blue bg
pub const BUYER_DEFAULT_COLOR: Color = Color::srgb(0.5, 0.5, 0.5);

// ============================================================================
// UI Section Colors
// ============================================================================

pub const UI_ROOT_BG: Color = Color::srgb(0.1, 0.1, 0.15);             // Dark background
pub const PLAY_AREA_BG: Color = Color::srgb(0.15, 0.15, 0.2);          // Play area background
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
pub const DEALER_CARD_BORDER: Color = Color::srgb(1.0, 1.0, 0.8);      // Bright border for dealer reveals

// ============================================================================
// State/Status Colors
// ============================================================================

pub const STATUS_PLAYING: Color = Color::srgb(1.0, 1.0, 0.3);          // Yellow
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
pub const GO_HOME_BUTTON_BG: Color = Color::srgb(0.3, 0.8, 0.3);       // Green

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
pub const PRESET_BUTTON_BG: Color = Color::srgb(0.2, 0.2, 0.5);        // Blue preset button

// ============================================================================
// Misc UI Colors
// ============================================================================

pub const PLACEHOLDER_BG: Color = Color::srgb(0.35, 0.35, 0.35);       // Placeholder card back
pub const PLACEHOLDER_BORDER: Color = Color::srgb(0.5, 0.5, 0.5);      // Placeholder border
pub const PILE_INDICATOR_BG: Color = Color::srgb(0.6, 0.6, 0.6);       // Card pile counter bg
pub const STAT_BOX_BG: Color = Color::srgb(0.4, 0.4, 0.4);             // Stat box background
pub const STAT_BOX_BORDER: Color = Color::srgb(0.6, 0.6, 0.6);         // Stat box border

// ============================================================================
// Card Sizing Constants
// ============================================================================

pub const CARD_WIDTH_SMALL: f32 = 110.0;      // Small cards (narc/buyer hands, played pool)
pub const CARD_HEIGHT_SMALL: f32 = 140.0;

pub const CARD_WIDTH_MEDIUM: f32 = 120.0;     // Medium cards (player hand, override slots)
pub const CARD_HEIGHT_MEDIUM: f32 = 152.0;

pub const CARD_WIDTH_HAND: f32 = 120.0;       // Hand cards (deprecated, use MEDIUM)
pub const CARD_HEIGHT_HAND: f32 = 160.0;

pub const CARD_WIDTH_DECK_BUILDER: f32 = 110.0; // Deck builder cards
pub const CARD_HEIGHT_DECK_BUILDER: f32 = 140.0;

pub const CARD_WIDTH_BUYER_VISIBLE: f32 = 120.0; // Buyer visible hand
pub const CARD_HEIGHT_BUYER_VISIBLE: f32 = 140.0;

pub const CARD_WIDTH_LARGE: f32 = 180.0;      // Large cards (scenario card, etc.)
pub const CARD_HEIGHT_LARGE: f32 = 250.0;

// ============================================================================
// Border/Spacing Constants
// ============================================================================

pub const CARD_BORDER_WIDTH: f32 = 2.0;
pub const CARD_BORDER_WIDTH_THICK: f32 = 3.0;
pub const CARD_BORDER_RADIUS: f32 = 8.0;

pub const SPACING_TINY: f32 = 5.0;
pub const SPACING_SMALL: f32 = 8.0;
pub const SPACING_MEDIUM: f32 = 10.0;
pub const SPACING_MEDIUM_LARGE: f32 = 15.0;
pub const SPACING_LARGE: f32 = 20.0;
pub const SPACING_XLARGE: f32 = 30.0;

// ============================================================================
// Font Sizes
// ============================================================================

pub const FONT_SIZE_SMALL: f32 = 14.0;
pub const FONT_SIZE_MEDIUM: f32 = 18.0;
pub const FONT_SIZE_LARGE: f32 = 24.0;
pub const FONT_SIZE_HEADER: f32 = 28.0;

// ============================================================================
// Heat Bar Constants (SOW-011-A Phase 4)
// ============================================================================

pub const HEAT_BAR_WIDTH: f32 = 30.0;
pub const HEAT_BAR_HEIGHT: f32 = 220.0;
pub const HEAT_BAR_BG: Color = Color::srgb(0.2, 0.2, 0.2);

// Heat bar gradient colors (stacked nodes)
pub const HEAT_BAR_GREEN: Color = Color::srgb(0.0, 1.0, 0.0);   // 0-50%
pub const HEAT_BAR_YELLOW: Color = Color::srgb(1.0, 1.0, 0.0);  // 50-80%
pub const HEAT_BAR_RED: Color = Color::srgb(1.0, 0.0, 0.0);     // 80-100%

// ============================================================================
// Helper Functions
// ============================================================================

/// Dim a color by multiplying RGB values by factor (typically 0.4)
pub fn dim_color(color: Color, factor: f32) -> Color {
    if let Color::Srgba(srgba) = color {
        Color::srgb(srgba.red * factor, srgba.green * factor, srgba.blue * factor)
    } else {
        color
    }
}

/// Get card type color (bright version for hand/active display)
pub fn get_card_color_bright(card_type: &str) -> Color {
    match card_type {
        "Product" => PRODUCT_CARD_COLOR,
        "Location" => LOCATION_CARD_COLOR,
        "Evidence" => EVIDENCE_CARD_COLOR,
        "Cover" => COVER_CARD_COLOR,
        "DealModifier" => DEAL_MODIFIER_CARD_COLOR,
        "Insurance" => INSURANCE_CARD_COLOR,
        "Conviction" => CONVICTION_CARD_COLOR,
        _ => DEFAULT_CARD_COLOR,
    }
}

/// Get card type color (dim version for played/inactive display)
pub fn get_card_color_dim(card_type: &str) -> Color {
    match card_type {
        "Product" => PRODUCT_CARD_COLOR_DIM,
        "Location" => LOCATION_CARD_COLOR_DIM,
        "Evidence" => EVIDENCE_CARD_COLOR_DIM,
        "Cover" => COVER_CARD_COLOR_DIM,
        "DealModifier" => DEAL_MODIFIER_CARD_COLOR_DIM,
        "Insurance" => INSURANCE_CARD_COLOR_DIM,
        "Conviction" => CONVICTION_CARD_COLOR_DIM,
        _ => DEFAULT_CARD_COLOR_DIM,
    }
}
