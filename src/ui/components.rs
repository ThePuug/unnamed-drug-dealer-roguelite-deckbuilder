// UI Components - Marker structs for UI entities
// SOW-011-A Phase 1: Extracted from main.rs for modularity

use bevy::prelude::Component;

// ============================================================================
// Main UI Structure
// ============================================================================

#[derive(Component)]
pub struct UiRoot;

#[derive(Component)]
pub struct BackgroundImage; // POC: Location background image container (clips overflow)

#[derive(Component)]
pub struct BackgroundImageNode; // POC: Actual image node inside container

// ============================================================================
// Play Area Components
// ============================================================================

#[derive(Component)]
pub struct BuyerPortrait; // Buyer actor portrait image

#[derive(Component)]
pub struct NarcPortrait; // Narc actor portrait image

#[derive(Component)]
pub struct PlayerHandDisplay;

#[derive(Component)]
pub struct CardButton {
    pub card_index: usize,
}

#[derive(Component)]
pub struct PlayedCardDisplay;

// ============================================================================
// Betting UI Components (SOW-002 Phase 5)
// ============================================================================

#[derive(Component)]
pub struct CheckButton;

// SOW-AAA: RaiseButton removed (obsolete - ADR-006)

#[derive(Component)]
pub struct FoldButton;

// ============================================================================
// Restart/Navigation Buttons (SOW-004)
// ============================================================================

#[derive(Component)]
pub struct RestartButton; // "NEW DEAL" button

#[derive(Component)]
pub struct GoHomeButton; // "GO HOME" button

// ============================================================================
// Deck Builder UI Components (SOW-006)
// ============================================================================

#[derive(Component)]
pub struct DeckBuilderRoot;

#[derive(Component)]
pub struct CardPoolContainer;

// SOW-AAA: SelectedDeckContainer removed (unused)

#[derive(Component)]
pub struct DeckStatsDisplay;

#[derive(Component)]
pub struct DeckBuilderCardButton {
    pub card_id: String,
}


#[derive(Component)]
pub struct StartRunButton;

// ============================================================================
// SOW-011-A Phase 4: Active Slot System
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SlotType {
    Product,
    Location,
    Conviction,
    Insurance,
}

#[derive(Component)]
pub struct ActiveSlot {
    pub slot_type: SlotType,
}

#[derive(Component)]
pub struct ActiveSlotsContainer;

/// SOW-021/SOW-022: Round header text ("ROUND 2 / 3 · DEAL IN PROGRESS")
#[derive(Component)]
pub struct TurnIndicatorText;

#[derive(Component)]
pub struct DeckCounter; // Deck cards remaining counter ("DECK · n")

// ============================================================================
// SOW-022: Game Play v2 Screen
// ============================================================================

/// Radial vignette over the location background
#[derive(Component)]
pub struct ScreenVignette;

// -- YOUR STANDING panel --

#[derive(Component)]
pub struct StandingCashText;

#[derive(Component)]
pub struct StandingHeatValueText;

/// Tier chip container (border color follows heat tier)
#[derive(Component)]
pub struct StandingHeatTierChip;

#[derive(Component)]
pub struct StandingHeatTierText;

#[derive(Component)]
pub struct StandingHeatBarFill;

/// Container for conviction-threshold tick marks inside the heat track
#[derive(Component)]
pub struct StandingHeatTicks;

/// Container for the tick labels row under the heat track
#[derive(Component)]
pub struct StandingHeatTickLabels;

// -- Turn pill --

#[derive(Component)]
pub struct TurnPill;

#[derive(Component)]
pub struct TurnPillDot;

#[derive(Component)]
pub struct TurnPillText;

// -- Narc character cluster --

#[derive(Component)]
pub struct NarcIntentBubble;

#[derive(Component)]
pub struct NarcIntentTitleText;

/// Row that holds the intent stat entries (rebuilt on change)
#[derive(Component)]
pub struct NarcIntentStatsRow;

#[derive(Component)]
pub struct NarcSpotlight;

// (hand-size count chips removed - they added little over the action bubbles)

// -- Buyer character cluster --

#[derive(Component)]
pub struct BuyerSpotlight;

#[derive(Component)]
pub struct BuyerNameText;

/// Hoverable wants bubble (carries Interaction)
#[derive(Component)]
pub struct BuyerBubble;

#[derive(Component)]
pub struct BuyerScenarioNameText;

#[derive(Component)]
pub struct BuyerDemandText;

#[derive(Component)]
pub struct BuyerPayoutText;

/// Expanded detail shown while hovering the wants bubble
#[derive(Component)]
pub struct BuyerDetailPanel;

#[derive(Component)]
pub struct BuyerDetailText;

/// Confidence face on the scenario placard - how close the buyer is to
/// bailing (replaces the BAILS AT HEAT chip; exact thresholds live in the
/// hover detail)
#[derive(Component)]
pub struct BuyerConfidenceEmoji;

#[derive(Component)]
pub struct BuyerConfidenceText;

/// "PLAYED · <card>" speech bubble for buyer reactions, symmetric with the
/// narc intent bubble (SOW-022 follow-up: buyer plays had no on-screen feedback)
#[derive(Component)]
pub struct BuyerPlayedBubble;

#[derive(Component)]
pub struct BuyerPlayedTitleText;

#[derive(Component)]
pub struct BuyerPlayedStatsRow;

// -- Evidence vs Cover balance bar --

#[derive(Component)]
pub struct BalanceEvidenceText;

#[derive(Component)]
pub struct BalanceCoverText;

#[derive(Component)]
pub struct BalanceStatusChip;

#[derive(Component)]
pub struct BalanceStatusChipText;

#[derive(Component)]
pub struct BalancePayoutChipText;

#[derive(Component)]
pub struct BalanceEvidenceFill;

#[derive(Component)]
pub struct BalanceCoverFill;

#[derive(Component)]
pub struct BalanceDivider;

// -- Deck / discard stacks --

/// Top face of the deck stack (card-back image filled at runtime)
#[derive(Component)]
pub struct DeckStackImage;

#[derive(Component)]
pub struct DiscardCountText;

/// Slot the discard stack's face-up top card is spawned into
#[derive(Component)]
pub struct DiscardTopCardSlot;

// -- Hand fan --

/// Positioned wrapper around one fanned hand card; hover adjusts transform/z
#[derive(Component)]
pub struct HandCardWrapper {
    pub angle_deg: f32,
    pub base_z: i32,
}

// ============================================================================
// SOW-011-B: Hand Resolution Overlay
// ============================================================================

#[derive(Component)]
pub struct ResolutionOverlay;

#[derive(Component)]
pub struct ResolutionBackdrop;

#[derive(Component)]
pub struct ResolutionPanel;

#[derive(Component)]
pub struct ResolutionTitle;

#[derive(Component)]
pub struct ResolutionStory; // SOW-012: Narrative story text

#[derive(Component)]
pub struct ResolutionResults;

// ============================================================================
// Character Heat Display (Persistent Heat from Save System)
// ============================================================================

// SOW-023: CharacterHeatDisplay/CharacterHeatText/CharacterTierText removed -
// per-dealer heat displays on the operations roster panel (the old stats-block
// line duplicated it)

#[derive(Component)]
pub struct DecayInfoDisplay;

// ============================================================================
// Account Cash Display (RFC-016: Account Cash System)
// ============================================================================

#[derive(Component)]
pub struct AccountCashDisplay;

#[derive(Component)]
pub struct AccountCashText;

#[derive(Component)]
pub struct LifetimeRevenueText;

// ============================================================================
// Story History Overlay (Narrative Log)
// ============================================================================

#[derive(Component)]
pub struct StoryHistoryButton;

#[derive(Component)]
pub struct StoryHistoryOverlay;

#[derive(Component)]
pub struct StoryHistoryCloseButton;

#[derive(Component)]
pub struct StoryHistoryText;

// ============================================================================
// SOW-023: Operations roster panel (deck-builder screen)
// ============================================================================

/// Strip under the tabs listing the kingpin's dealers (children rebuilt on
/// save change by populate_roster_panel_system)
#[derive(Component)]
pub struct RosterPanel;

/// Clickable dealer card - selects who runs the next session
#[derive(Component)]
pub struct RosterDealerButton {
    pub dealer_index: usize,
}

/// Pay bail for a jailed dealer
#[derive(Component)]
pub struct RosterBailButton {
    pub dealer_index: usize,
}

/// Hire the next recruit
#[derive(Component)]
pub struct RosterHireButton;

/// SOW-025: relocate a dealer to another unlocked area (cash + downtime)
#[derive(Component)]
pub struct RosterMoveButton {
    pub dealer_index: usize,
    pub to_area: String,
}

// ============================================================================
// SOW-020: Shop UI Components
// ============================================================================

/// Tab button container at top of deck builder
#[derive(Component)]
pub struct ShopTabsContainer;

/// Individual tab button (Your Cards / Shop)
#[derive(Component)]
pub struct ShopTab {
    pub is_shop: bool,
}

/// Container for shop location buttons
#[derive(Component)]
pub struct ShopLocationSelector;

/// Button to select a shop location
#[derive(Component)]
pub struct ShopLocationButton {
    pub location_id: String,
}

/// Container for shop cards display
#[derive(Component)]
pub struct ShopCardsContainer;

/// Card display in shop showing price and locked/unlocked status
#[derive(Component)]
pub struct ShopCardDisplay {
    pub card_id: String,
    pub price: u32,
    pub is_unlocked: bool,
}

/// Purchase button on a shop card
#[derive(Component)]
pub struct ShopPurchaseButton {
    pub card_id: String,
    pub price: u32,
}

/// Text showing the selected location name
#[derive(Component)]
pub struct ShopLocationNameText;

/// SOW-024: Purchase button for a locked area ("THE BLOCK — $2,000")
#[derive(Component)]
pub struct ShopAreaUnlockButton {
    pub location_id: String,
    pub price: u32,
}

/// SOW-024: One-line expansion feedback ("New turf: The Block")
#[derive(Component)]
pub struct ShopFeedbackText;
