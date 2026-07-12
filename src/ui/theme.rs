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
// SOW-022: SCENARIO_CARD_BG/TEXT removed (scenario card replaced by buyer bubble);
// BORDER retained for the resolution overlay panel
pub const SCENARIO_CARD_BORDER: Color = Color::srgb(0.9, 0.9, 0.4);    // Golden yellow

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

// SOW-022: CARD_WIDTH/HEIGHT_MEDIUM removed (superseded by Table/Hand/Compact sizes)

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
// SOW-022: HEAT_BAR_BG/GREEN/YELLOW/RED removed (vertical heat bar replaced by
// the YOUR STANDING panel's gradient track - see STANDING_HEAT_* above)

// ============================================================================
// SOW-022: Game Play v2 Screen Palette
// ============================================================================
// Values transcribed from the "Game Play v2" design mockup (1920x1080).

// Canvas / vignette
pub const GAMEPLAY_CANVAS_BG: Color = Color::srgb(0.039, 0.047, 0.078);            // #0a0c14
pub const VIGNETTE_INNER: Color = Color::srgba(0.031, 0.039, 0.071, 0.35);
pub const VIGNETTE_MID: Color = Color::srgba(0.024, 0.027, 0.047, 0.82);
pub const VIGNETTE_OUTER: Color = Color::srgba(0.016, 0.020, 0.035, 0.95);

// Shared v2 text tones
pub const V2_LABEL: Color = Color::srgb(0.486, 0.525, 0.627);                      // #7c86a0 section labels

// YOUR STANDING panel
pub const STANDING_PANEL_BG: Color = Color::srgba(0.035, 0.043, 0.067, 0.88);
pub const STANDING_PANEL_BORDER: Color = Color::srgba(0.353, 0.392, 0.549, 0.4);
pub const STANDING_DIVIDER: Color = Color::srgba(0.353, 0.392, 0.549, 0.25);
pub const STANDING_CASH_LABEL: Color = Color::srgb(0.478, 0.659, 0.541);           // #7aa88a
pub const STANDING_CASH_VALUE: Color = Color::srgb(0.498, 0.910, 0.604);           // #7fe89a
pub const STANDING_HEAT_LABEL: Color = Color::srgb(1.0, 0.690, 0.478);             // #ffb07a
pub const STANDING_HEAT_VALUE: Color = Color::srgb(1.0, 0.824, 0.302);             // #ffd24d
pub const STANDING_HEAT_VALUE_DIM: Color = Color::srgb(0.478, 0.416, 0.322);       // #7a6a52 "/ 100"
pub const STANDING_HEAT_TRACK_BG: Color = Color::srgb(0.110, 0.082, 0.071);        // #1c1512
pub const STANDING_HEAT_TRACK_BORDER: Color = Color::srgba(0.471, 0.314, 0.235, 0.45);
pub const STANDING_HEAT_FILL_LOW: Color = Color::srgb(0.227, 0.820, 0.290);        // #3ad14a
pub const STANDING_HEAT_FILL_HIGH: Color = Color::srgb(0.902, 0.824, 0.302);       // #e6d24d
pub const STANDING_TICK: Color = Color::srgba(1.0, 0.431, 0.431, 0.6);
pub const STANDING_TICK_LABEL: Color = Color::srgb(0.541, 0.478, 0.416);           // #8a7a6a
pub const STANDING_TICK_LABEL_FIRST: Color = Color::srgb(0.851, 0.541, 0.541);     // #d98a8a

// Turn pill (per-actor variants)
pub const PILL_NARC_BG: Color = Color::srgba(0.110, 0.039, 0.047, 0.85);
pub const PILL_NARC_BORDER: Color = Color::srgba(0.863, 0.275, 0.275, 0.6);
pub const PILL_NARC_DOT: Color = Color::srgb(1.0, 0.353, 0.353);
pub const PILL_NARC_TEXT: Color = Color::srgb(1.0, 0.690, 0.690);
pub const PILL_PLAYER_BG: Color = Color::srgba(0.039, 0.110, 0.047, 0.85);
pub const PILL_PLAYER_BORDER: Color = Color::srgba(0.275, 0.863, 0.353, 0.6);
pub const PILL_PLAYER_DOT: Color = Color::srgb(0.353, 1.0, 0.478);
pub const PILL_PLAYER_TEXT: Color = Color::srgb(0.690, 1.0, 0.745);
pub const PILL_BUYER_BG: Color = Color::srgba(0.118, 0.102, 0.031, 0.9);
pub const PILL_BUYER_BORDER: Color = Color::srgba(0.902, 0.824, 0.314, 0.6);
pub const PILL_BUYER_DOT: Color = Color::srgb(1.0, 0.824, 0.302);
pub const PILL_BUYER_TEXT: Color = Color::srgb(0.941, 0.878, 0.541);
pub const PILL_NEUTRAL_BG: Color = Color::srgba(0.05, 0.055, 0.09, 0.85);
pub const PILL_NEUTRAL_BORDER: Color = Color::srgba(0.353, 0.392, 0.549, 0.5);
pub const PILL_NEUTRAL_DOT: Color = Color::srgb(0.608, 0.659, 0.784);
pub const PILL_NEUTRAL_TEXT: Color = Color::srgb(0.682, 0.714, 0.784);

// Narc intent bubble + spotlight
pub const NARC_BUBBLE_BG: Color = Color::srgba(0.118, 0.039, 0.047, 0.92);
pub const NARC_BUBBLE_BORDER: Color = Color::srgba(0.863, 0.314, 0.314, 0.65);
pub const NARC_BUBBLE_TITLE: Color = Color::srgb(1.0, 0.604, 0.604);               // #ff9a9a
pub const NARC_STAT_EVIDENCE: Color = Color::srgb(1.0, 0.420, 0.420);              // #ff6b6b
pub const NARC_STAT_HEAT: Color = Color::srgb(1.0, 0.702, 0.420);                  // #ffb36b
pub const NARC_SPOTLIGHT: Color = Color::srgba(1.0, 0.275, 0.275, 0.28);
pub const NARC_NAME: Color = Color::srgb(1.0, 0.604, 0.604);

// Buyer bubble + spotlight + chips
pub const BUYER_BUBBLE_BG: Color = Color::srgba(0.118, 0.102, 0.031, 0.94);
pub const BUYER_BUBBLE_BORDER: Color = Color::srgba(0.902, 0.824, 0.314, 0.6);
pub const BUYER_BUBBLE_TITLE: Color = Color::srgb(0.941, 0.878, 0.541);            // #f0e08a
pub const BUYER_BUBBLE_LABEL: Color = Color::srgb(0.604, 0.565, 0.376);            // #9a9060
pub const BUYER_BUBBLE_DEMAND: Color = Color::srgb(1.0, 0.914, 0.541);             // #ffe98a
pub const BUYER_BUBBLE_PAYOUT: Color = Color::srgb(1.0, 0.824, 0.302);             // #ffd24d
pub const BUYER_BUBBLE_HINT: Color = Color::srgb(0.518, 0.478, 0.322);             // #847a52
pub const BUYER_BUBBLE_DIVIDER: Color = Color::srgba(0.902, 0.824, 0.314, 0.2);
pub const BUYER_SPOTLIGHT: Color = Color::srgba(0.902, 0.784, 0.235, 0.14);
pub const BUYER_NAME: Color = Color::srgb(0.941, 0.878, 0.541);
pub const HEAT_CAP_CHIP_BG: Color = Color::srgba(0.118, 0.047, 0.047, 0.9);
pub const HEAT_CAP_CHIP_BORDER: Color = Color::srgba(0.863, 0.353, 0.353, 0.55);
pub const HEAT_CAP_CHIP_TEXT: Color = Color::srgb(1.0, 0.604, 0.604);
pub const HEAT_CAP_CHIP_VALUE: Color = Color::srgb(1.0, 0.824, 0.302);

// Actor card-count chips
pub const COUNT_CHIP_BG: Color = Color::srgba(0.047, 0.055, 0.086, 0.85);
pub const COUNT_CHIP_BORDER: Color = Color::srgba(0.471, 0.510, 0.627, 0.35);
pub const COUNT_CHIP_TEXT: Color = Color::srgb(0.682, 0.714, 0.784);               // #aeb6c8

// Deal table ghost insurance slot
pub const GHOST_INSURANCE_BORDER: Color = Color::srgba(0.2, 0.8, 0.8, 0.4);
pub const GHOST_INSURANCE_BG: Color = Color::srgba(0.2, 0.8, 0.8, 0.05);
pub const GHOST_INSURANCE_TEXT: Color = Color::srgba(0.471, 0.784, 0.784, 0.7);

// Evidence vs Cover balance bar
pub const BALANCE_EVIDENCE_TEXT: Color = Color::srgb(1.0, 0.420, 0.420);           // #ff6b6b
pub const BALANCE_COVER_TEXT: Color = Color::srgb(0.498, 0.910, 0.604);            // #7fe89a
pub const BALANCE_TRACK_BG: Color = Color::srgb(0.102, 0.078, 0.125);              // #1a1420
pub const BALANCE_TRACK_BORDER: Color = Color::srgba(0.353, 0.314, 0.431, 0.4);
pub const BALANCE_EVIDENCE_FILL_LOW: Color = Color::srgb(0.8, 0.302, 0.302);       // #cc4d4d
pub const BALANCE_EVIDENCE_FILL_HIGH: Color = Color::srgb(1.0, 0.420, 0.420);      // #ff6b6b
pub const BALANCE_COVER_FILL_LOW: Color = Color::srgb(0.227, 0.659, 0.290);        // #3aa84a
pub const BALANCE_COVER_FILL_HIGH: Color = Color::srgb(0.302, 0.8, 0.302);         // #4dcc4d
pub const SAFE_CHIP_BG: Color = Color::srgba(0.302, 0.8, 0.302, 0.15);
pub const SAFE_CHIP_BORDER: Color = Color::srgba(0.302, 0.8, 0.302, 0.6);
pub const SAFE_CHIP_TEXT: Color = Color::srgb(0.498, 0.910, 0.604);
pub const RISK_CHIP_BG: Color = Color::srgba(0.8, 0.302, 0.302, 0.15);
pub const RISK_CHIP_BORDER: Color = Color::srgba(0.8, 0.302, 0.302, 0.6);
pub const RISK_CHIP_TEXT: Color = Color::srgb(1.0, 0.420, 0.420);
pub const PAYOUT_CHIP_BG: Color = Color::srgba(0.702, 0.502, 0.902, 0.15);
pub const PAYOUT_CHIP_BORDER: Color = Color::srgba(0.702, 0.502, 0.902, 0.6);
pub const PAYOUT_CHIP_TEXT: Color = Color::srgb(0.788, 0.659, 0.941);              // #c9a8f0

// Deck / discard stacks
pub const STACK_PLATE_DEEP: Color = Color::srgb(0.071, 0.075, 0.110);              // #12131c
pub const STACK_PLATE_MID: Color = Color::srgb(0.090, 0.094, 0.149);               // #171826
pub const STACK_PLATE_BORDER_DEEP: Color = Color::srgba(0.353, 0.392, 0.549, 0.3);
pub const STACK_PLATE_BORDER_MID: Color = Color::srgba(0.353, 0.392, 0.549, 0.4);
pub const STACK_TOP_BORDER: Color = Color::srgba(0.471, 0.549, 0.745, 0.5);
pub const STACK_LABEL: Color = Color::srgb(0.604, 0.635, 0.729);                   // #9aa2ba

// Action buttons
pub const PASS_BUTTON_TOP: Color = Color::srgb(0.329, 0.839, 0.329);               // #54d654
pub const PASS_BUTTON_BOTTOM: Color = Color::srgb(0.227, 0.659, 0.227);            // #3aa83a
pub const PASS_BUTTON_TEXT: Color = Color::srgb(0.024, 0.129, 0.039);              // #06210a
pub const PASS_BUTTON_GLOW: Color = Color::srgba(0.275, 0.784, 0.314, 0.4);
pub const PASS_BUTTON_DISABLED_TOP: Color = Color::srgb(0.22, 0.24, 0.22);
pub const PASS_BUTTON_DISABLED_BOTTOM: Color = Color::srgb(0.16, 0.18, 0.16);
pub const PASS_BUTTON_TEXT_DISABLED: Color = Color::srgb(0.45, 0.5, 0.45);
pub const BAIL_BUTTON_BG: Color = Color::srgba(0.078, 0.086, 0.118, 0.9);
pub const BAIL_BUTTON_TEXT: Color = Color::srgb(0.788, 0.690, 0.690);              // #c9b0b0
pub const BAIL_BUTTON_BORDER: Color = Color::srgba(0.784, 0.353, 0.353, 0.5);
pub const BAIL_BUTTON_BORDER_DISABLED: Color = Color::srgba(0.353, 0.392, 0.549, 0.3);
pub const BAIL_BUTTON_TEXT_DISABLED: Color = Color::srgb(0.4, 0.42, 0.48);

// Hand fan hover glow
pub const HAND_HOVER_GLOW: Color = Color::srgba(0.471, 0.627, 1.0, 0.55);

// v2 card sizes (base 601:870 template aspect; heights derived at spawn)
pub const CARD_WIDTH_TABLE: f32 = 168.0;   // "the deal on the table" slots
pub const CARD_HEIGHT_TABLE: f32 = 243.2;
pub const CARD_WIDTH_HAND: f32 = 170.0;    // fanned player hand
pub const CARD_HEIGHT_HAND: f32 = 246.1;
pub const CARD_WIDTH_COMPACT: f32 = 134.0; // discard stack top card
pub const CARD_HEIGHT_COMPACT: f32 = 194.0;

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