// HandState module - Core game state machine for hand progression

use bevy::prelude::*;
use std::collections::HashMap;

use crate::models::card::*;
use crate::models::buyer::*;
use crate::models::cards::Cards;
use crate::data::*;

// Re-export implementations
pub mod state_machine;
pub mod resolution;
pub mod card_engine;

// ============================================================================
// CORE DEFINITIONS
// ============================================================================

/// States the hand can be in
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HandPhase {
    Draw,
    PlayerPhase,
    DealerReveal,
    FoldDecision,
    Resolve,
    Bust,
}

/// Outcome of hand resolution
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HandOutcome {
    Safe,
    Busted,
    Folded,
    InvalidDeal,
    BuyerBailed,
}

/// Hand state tracking
#[derive(Component)]
pub struct HandState {
    pub current_state: HandPhase,
    pub current_round: u8,
    pub owner_cards: HashMap<Owner, Cards>,
    pub cards_played: Vec<Card>,
    pub cards_played_this_round: Vec<Card>,
    pub discard_pile: Vec<Card>,
    pub outcome: Option<HandOutcome>,
    pub cash: u32,
    pub current_heat: u32,
    pub current_player_index: usize,
    pub checks_this_hand: Vec<(Owner, u8)>,
    pub buyer_persona: Option<BuyerPersona>,
    pub hand_story: Option<String>, // SOW-012: Generated narrative for this hand
    pub session_stories: Vec<String>, // Story history for this session
    pub last_profit: u32, // RFC-016: Profit from most recent hand resolution
    pub card_play_counts: HashMap<String, u32>, // RFC-017: Play counts for upgrade tiers
    pub card_upgrades: HashMap<String, crate::save::CardUpgrades>, // RFC-019: Per-card upgrade choices
    pub narc_upgrade_tier: crate::save::UpgradeTier, // RFC-018: Narc difficulty scaling
}

impl HandState {
    /// SOW-013-B: Create HandState from loaded assets
    /// RFC-018: Now takes heat_tier to set Narc difficulty scaling
    pub fn from_assets(assets: &crate::assets::GameAssets, heat_tier: crate::save::HeatTier) -> Self {
        let mut owner_cards = HashMap::new();
        owner_cards.insert(Owner::Narc, Cards::new(create_narc_deck(assets)));
        owner_cards.insert(Owner::Player, Cards::new(create_player_deck(assets)));
        owner_cards.insert(Owner::Buyer, Cards::empty());

        Self {
            current_state: HandPhase::Draw,
            current_round: 1,
            owner_cards,
            cards_played: Vec::new(),
            cards_played_this_round: Vec::new(),
            discard_pile: Vec::new(),
            outcome: None,
            cash: 0,
            current_heat: 0,
            current_player_index: 0,
            checks_this_hand: Vec::new(),
            buyer_persona: None,
            hand_story: None,
            session_stories: Vec::new(),
            last_profit: 0,
            card_play_counts: HashMap::new(), // RFC-017: Initialize empty
            card_upgrades: HashMap::new(), // RFC-019: Initialize empty
            narc_upgrade_tier: heat_tier.narc_upgrade_tier(), // RFC-018: Set from heat tier
        }
    }
}

// Keep Default for tests
impl Default for HandState {
    fn default() -> Self {
        // Tests use basic test decks
        #[cfg(test)]
        let (narc_deck, player_deck) = {
            use crate::models::test_helpers::*;
            let narc = vec![create_evidence("Evidence1", 5, 5), create_evidence("Evidence2", 10, 10)];
            let player = vec![create_product("Product1", 10, 0), create_location("Location1", 5, 10, 0)];
            (narc, player)
        };

        #[cfg(not(test))]
        let (narc_deck, player_deck) = (vec![], vec![]);

        let mut owner_cards = HashMap::new();
        owner_cards.insert(Owner::Narc, Cards::new(narc_deck));
        owner_cards.insert(Owner::Player, Cards::new(player_deck));
        owner_cards.insert(Owner::Buyer, Cards::empty());

        Self {
            current_state: HandPhase::Draw,
            current_round: 1,
            owner_cards,
            cards_played: Vec::new(),
            cards_played_this_round: Vec::new(),
            discard_pile: Vec::new(),
            outcome: None,
            cash: 0,
            current_heat: 0,
            current_player_index: 0,
            checks_this_hand: Vec::new(),
            buyer_persona: None,
            hand_story: None,
            session_stories: Vec::new(),
            last_profit: 0,
            card_play_counts: HashMap::new(), // RFC-017: Initialize empty
            card_upgrades: HashMap::new(), // RFC-019: Initialize empty
            narc_upgrade_tier: crate::save::UpgradeTier::Base, // RFC-018: Default to base (tests)
        }
    }
}

impl HandState {
    /// Get cards for an owner
    pub fn cards(&self, owner: Owner) -> &Cards {
        &self.owner_cards[&owner]
    }

    /// Get mutable cards for an owner
    pub fn cards_mut(&mut self, owner: Owner) -> &mut Cards {
        self.owner_cards.get_mut(&owner).unwrap()
    }

    /// RFC-017: Get upgrade tier for a card based on play counts
    pub fn get_card_tier(&self, card_name: &str) -> crate::save::UpgradeTier {
        let count = self.card_play_counts.get(card_name).copied().unwrap_or(0);
        crate::save::UpgradeTier::from_play_count(count)
    }

    /// RFC-017: Get play count for a card
    pub fn get_play_count(&self, card_name: &str) -> u32 {
        self.card_play_counts.get(card_name).copied().unwrap_or(0)
    }

    /// RFC-019: Get the stat multiplier for a specific stat on a card
    /// Each upgrade to this stat adds +10% (additive stacking)
    pub fn get_stat_multiplier(&self, card_name: &str, stat: crate::save::UpgradeableStat) -> f32 {
        self.card_upgrades
            .get(card_name)
            .map(|u| u.stat_multiplier(stat))
            .unwrap_or(1.0)
    }

    /// RFC-019: Get upgrade history for a card
    pub fn get_card_upgrades(&self, card_name: &str) -> Option<&crate::save::CardUpgrades> {
        self.card_upgrades.get(card_name)
    }
}
