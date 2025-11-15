// UI Components - Marker structs for UI entities
// SOW-011-A Phase 1: Extracted from main.rs for modularity

use bevy::prelude::Component;

// ============================================================================
// Main UI Structure
// ============================================================================

#[derive(Component)]
pub struct UiRoot;

#[derive(Component)]
pub struct TotalsDisplay;

#[derive(Component)]
pub struct StatusDisplay;

#[derive(Component)]
pub struct BuyerScenarioCard;  // SOW-010: Oversized card displaying scenario info

#[derive(Component)]
pub struct BuyerScenarioCardText;  // SOW-010: Text content of scenario card

// ============================================================================
// Play Area Components
// ============================================================================

#[derive(Component)]
pub struct PlayAreaNarc;

#[derive(Component)]
pub struct PlayAreaDealer; // SOW-008: Shows dealer (now buyer) played cards

#[derive(Component)]
pub struct BuyerVisibleHand; // SOW-009: Displays Buyer's 3 visible cards (not yet played)

#[derive(Component)]
pub struct CardsContainer; // SOW-008: Container for cards within play area (horizontal layout)

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
pub struct BettingActionsContainer;

#[derive(Component)]
pub struct CheckButton;

#[derive(Component)]
pub struct RaiseButton;

#[derive(Component)]
pub struct FoldButton;

#[derive(Component)]
pub struct DecisionPointContainer;

#[derive(Component)]
pub struct BustContainer;

#[derive(Component)]
pub struct ContinueButton;

#[derive(Component)]
pub struct FoldDecisionButton;

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

#[derive(Component)]
pub struct SelectedDeckContainer;

#[derive(Component)]
pub struct DeckStatsDisplay;

#[derive(Component)]
pub struct DeckBuilderCardButton {
    pub card_id: u32,
    pub in_pool: bool,  // true = in available pool, false = in selected deck
}

// Note: DeckPreset enum temporarily in main.rs root, will be moved to game module
#[derive(Component)]
pub struct PresetButton {
    pub preset: crate::DeckPreset,
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
pub struct HeatBar;

#[derive(Component)]
pub struct HeatBarFill;

#[derive(Component)]
pub struct HeatBarText;

#[derive(Component)]
pub struct ActiveSlotsContainer;

#[derive(Component)]
pub struct EvidencePool;

#[derive(Component)]
pub struct CoverPool;

#[derive(Component)]
pub struct DealModPool;

#[derive(Component)]
pub struct DiscardPile;

#[derive(Component)]
pub struct BuyerDeckPanel;

#[derive(Component)]
pub struct NarcVisibleHand;

#[derive(Component)]
pub struct PlayerHandPanel;
