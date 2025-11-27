// Save system data types.

use bevy::prelude::Resource;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

/// Current save file format version
pub const SAVE_VERSION: u32 = 1;

/// Maximum sanity values for validation
const MAX_HEAT: u32 = 10_000;
const MAX_DECKS_PLAYED: u32 = 100_000;
const MAX_CASH: u64 = 999_999_999_999; // ~1 trillion cap

/// Errors that can occur during save/load operations
#[derive(Debug, Clone, PartialEq)]
pub enum SaveError {
    /// Save file does not exist
    NotFound,
    /// Save file signature invalid (tampered or corrupted)
    TamperedOrCorrupted,
    /// Save file version not supported
    UnsupportedVersion(u32),
    /// Serialization/deserialization failed
    SerializationError(String),
    /// File I/O error
    IoError(String),
    /// Data validation failed
    ValidationError(String),
}

impl std::fmt::Display for SaveError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SaveError::NotFound => write!(f, "Save file not found"),
            SaveError::TamperedOrCorrupted => write!(f, "Save file tampered or corrupted"),
            SaveError::UnsupportedVersion(v) => write!(f, "Unsupported save version: {}", v),
            SaveError::SerializationError(e) => write!(f, "Serialization error: {}", e),
            SaveError::IoError(e) => write!(f, "I/O error: {}", e),
            SaveError::ValidationError(e) => write!(f, "Validation error: {}", e),
        }
    }
}

impl std::error::Error for SaveError {}

/// The actual save file format (with signature for tamper detection)
#[derive(Serialize, Deserialize)]
pub struct SaveFile {
    /// Format version for migration support
    pub version: u32,
    /// Serialized SaveData payload
    pub data: Vec<u8>,
    /// HMAC-SHA256 signature over data
    pub signature: [u8; 32],
}

/// The game's persistent state
#[derive(Resource, Serialize, Deserialize, Clone, Debug)]
pub struct SaveData {
    /// Current character (None if permadeath occurred or new game)
    pub character: Option<CharacterState>,
    /// Account-wide state (persists forever, survives permadeath)
    /// RFC-016: Account Cash System
    #[serde(default)]
    pub account: AccountState,
}

impl SaveData {
    pub fn new() -> Self {
        Self {
            character: None,
            account: AccountState::new(),
        }
    }

    /// Validate data sanity (defense in depth)
    pub fn validate(&self) -> Result<(), SaveError> {
        if let Some(ref character) = self.character {
            character.validate()?;
        }
        self.account.validate()?;
        Ok(())
    }
}

impl Default for SaveData {
    fn default() -> Self {
        Self::new()
    }
}

/// Character profile (MVP placeholder)
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub enum CharacterProfile {
    #[default]
    Default,
    // Future: Named profiles with narrative flavor
}

/// Heat tier based on current heat value
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HeatTier {
    Cold,      // 0-29
    Warm,      // 30-59
    Hot,       // 60-89
    Blazing,   // 90-119
    Scorching, // 120-149
    Inferno,   // 150+
}

impl HeatTier {
    pub fn from_heat(heat: u32) -> Self {
        match heat {
            0..=29 => HeatTier::Cold,
            30..=59 => HeatTier::Warm,
            60..=89 => HeatTier::Hot,
            90..=119 => HeatTier::Blazing,
            120..=149 => HeatTier::Scorching,
            _ => HeatTier::Inferno,
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            HeatTier::Cold => "Cold",
            HeatTier::Warm => "Warm",
            HeatTier::Hot => "Hot",
            HeatTier::Blazing => "Blazing",
            HeatTier::Scorching => "Scorching",
            HeatTier::Inferno => "Inferno",
        }
    }

    /// Get color for UI display (RGB)
    pub fn color(&self) -> (f32, f32, f32) {
        match self {
            HeatTier::Cold => (0.2, 0.8, 0.3),      // Green
            HeatTier::Warm => (0.9, 0.9, 0.2),      // Yellow
            HeatTier::Hot => (1.0, 0.6, 0.1),       // Orange
            HeatTier::Blazing => (1.0, 0.4, 0.1),   // Deep Orange
            HeatTier::Scorching => (1.0, 0.2, 0.2), // Red
            HeatTier::Inferno => (0.8, 0.2, 0.8),   // Purple
        }
    }

    /// RFC-018: Get Narc upgrade tier for this Heat tier
    /// Higher Heat = stronger Narc cards
    pub fn narc_upgrade_tier(&self) -> UpgradeTier {
        match self {
            HeatTier::Cold => UpgradeTier::Base,      // No bonus
            HeatTier::Warm => UpgradeTier::Tier1,     // +10%
            HeatTier::Hot => UpgradeTier::Tier2,      // +20%
            HeatTier::Blazing => UpgradeTier::Tier3,  // +30%
            HeatTier::Scorching => UpgradeTier::Tier4, // +40%
            HeatTier::Inferno => UpgradeTier::Tier5,  // +50% with foil effect
        }
    }

    /// RFC-018: Get danger level description for UI
    pub fn danger_name(&self) -> &'static str {
        match self {
            HeatTier::Cold => "Relaxed",
            HeatTier::Warm => "Alert",
            HeatTier::Hot => "Dangerous",
            HeatTier::Blazing => "Severe",
            HeatTier::Scorching => "Intense",
            HeatTier::Inferno => "Deadly",
        }
    }
}

/// RFC-017: Card upgrade tier based on play count
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum UpgradeTier {
    Base,   // 0-4 plays: No bonus
    Tier1,  // 5+ plays: +10% primary stat
    Tier2,  // Bronze: 12+ plays (TESTING: 2+)
    Tier3,  // Silver: 25+ plays (TESTING: 3+)
    Tier4,  // Gold: 50+ plays (TESTING: 4+)
    Tier5,  // Foil: 100+ plays (TESTING: 5+)
}

impl UpgradeTier {
    /// Calculate tier from play count
    /// TESTING: Using 1/2/3/4/5 thresholds (production: 5/12/25/50/100)
    pub fn from_play_count(count: u32) -> Self {
        match count {
            0 => UpgradeTier::Base,
            1 => UpgradeTier::Tier1,   // TESTING (production: 5..=11)
            2 => UpgradeTier::Tier2,   // TESTING (production: 12..=24)
            3 => UpgradeTier::Tier3,   // TESTING (production: 25..=49)
            4 => UpgradeTier::Tier4,   // TESTING (production: 50..=99)
            _ => UpgradeTier::Tier5,   // TESTING: 5+ (production: 100+)
        }
    }

    /// Get the stat multiplier for this tier
    pub fn multiplier(&self) -> f32 {
        match self {
            UpgradeTier::Base => 1.0,
            UpgradeTier::Tier1 => 1.1,  // +10%
            UpgradeTier::Tier2 => 1.2,  // +20%
            UpgradeTier::Tier3 => 1.3,  // +30%
            UpgradeTier::Tier4 => 1.4,  // +40%
            UpgradeTier::Tier5 => 1.5,  // +50%
        }
    }

    /// Get plays needed for next tier (None if max tier)
    pub fn plays_to_next(&self) -> Option<u32> {
        match self {
            UpgradeTier::Base => Some(1),   // TESTING (production: 5)
            UpgradeTier::Tier1 => Some(2),  // TESTING (production: 12)
            UpgradeTier::Tier2 => Some(3),  // TESTING (production: 25)
            UpgradeTier::Tier3 => Some(4),  // TESTING (production: 50)
            UpgradeTier::Tier4 => Some(5),  // TESTING (production: 100)
            UpgradeTier::Tier5 => None,     // Max tier
        }
    }

    /// Display name for UI (star emoji for all upgraded tiers)
    pub fn name(&self) -> &'static str {
        match self {
            UpgradeTier::Base => "Base",
            _ => "★",  // Filled star (U+2605)
        }
    }

    /// Get star color as RGB tuple
    /// Grey -> Bronze -> Silver -> Gold -> Gold (foil uses gold star)
    pub fn star_color(&self) -> (f32, f32, f32) {
        match self {
            UpgradeTier::Base => (0.5, 0.5, 0.5),   // Grey (shouldn't show)
            UpgradeTier::Tier1 => (0.6, 0.6, 0.6),  // Dull grey
            UpgradeTier::Tier2 => (0.8, 0.5, 0.2),  // Bronze
            UpgradeTier::Tier3 => (0.75, 0.75, 0.8), // Silver
            UpgradeTier::Tier4 => (1.0, 0.84, 0.0), // Gold
            UpgradeTier::Tier5 => (1.0, 0.84, 0.0), // Gold (card gets foil)
        }
    }

    /// Whether this tier has the foil effect on the whole card
    pub fn is_foil(&self) -> bool {
        matches!(self, UpgradeTier::Tier5)
    }
}

impl Default for UpgradeTier {
    fn default() -> Self {
        UpgradeTier::Base
    }
}

/// RFC-019: Which stat was upgraded on a card
/// Used for player-chosen upgrade paths
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum UpgradeableStat {
    /// Product: increase price
    Price,
    /// Location/Cover/Insurance: increase cover
    Cover,
    /// Location/DealModifier: decrease evidence
    Evidence,
    /// All types: decrease heat
    Heat,
    /// Insurance: decrease heat_penalty
    HeatPenalty,
    /// DealModifier: increase price_mult
    PriceMultiplier,
}

impl UpgradeableStat {
    /// Display name for UI
    pub fn name(&self) -> &'static str {
        match self {
            Self::Price => "Price",
            Self::Cover => "Cover",
            Self::Evidence => "Evidence",
            Self::Heat => "Heat",
            Self::HeatPenalty => "Heat Penalty",
            Self::PriceMultiplier => "Price Bonus",
        }
    }

    /// Whether this stat improves by increasing (true) or decreasing (false)
    pub fn improves_by_increase(&self) -> bool {
        matches!(self, Self::Price | Self::Cover | Self::PriceMultiplier)
    }

    /// Get available upgrade stats for a card type
    /// Returns the stats that can be upgraded for this card type
    pub fn available_for(card_type: &crate::models::card::CardType) -> Vec<Self> {
        use crate::models::card::CardType;
        match card_type {
            CardType::Product { .. } => vec![Self::Price, Self::Heat],
            CardType::Location { .. } => vec![Self::Evidence, Self::Cover, Self::Heat],
            CardType::Cover { .. } => vec![Self::Cover, Self::Heat],
            CardType::Insurance { .. } => vec![Self::Cover, Self::HeatPenalty],
            CardType::DealModifier { .. } => vec![Self::PriceMultiplier, Self::Evidence, Self::Cover, Self::Heat],
            // Evidence and Conviction cards are not player-upgradeable
            CardType::Evidence { .. } => vec![],
            CardType::Conviction { .. } => vec![],
        }
    }

    /// Select 2 random stats for upgrade choice
    /// For cards with exactly 2 stats, returns both (no randomness needed)
    /// For cards with 3+ stats, randomly selects 2
    pub fn random_pair(card_type: &crate::models::card::CardType) -> Option<[Self; 2]> {
        use rand::prelude::*;
        let available = Self::available_for(card_type);

        match available.len() {
            0 | 1 => None, // Can't offer choice with less than 2 options
            2 => Some([available[0], available[1]]),
            _ => {
                // Randomly select 2 from available
                let mut shuffled = available;
                shuffled.shuffle(&mut rand::rng());
                Some([shuffled[0], shuffled[1]])
            }
        }
    }
}

/// RFC-019: Upgrade choices for a single card
/// Tracks which stat was chosen at each tier upgrade
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CardUpgrades {
    /// Stats chosen at each tier (index 0 = Tier1, index 1 = Tier2, etc.)
    pub upgrades: Vec<UpgradeableStat>,
}

impl CardUpgrades {
    /// Create new empty upgrade history
    pub fn new() -> Self {
        Self { upgrades: Vec::new() }
    }

    /// Add an upgrade choice
    pub fn add_upgrade(&mut self, stat: UpgradeableStat) {
        self.upgrades.push(stat);
    }

    /// Count how many times a specific stat was upgraded
    pub fn count_stat(&self, stat: UpgradeableStat) -> usize {
        self.upgrades.iter().filter(|&&s| s == stat).count()
    }

    /// Get the multiplier for a specific stat based on upgrade count
    /// Each upgrade adds +10% (additive stacking)
    pub fn stat_multiplier(&self, stat: UpgradeableStat) -> f32 {
        1.0 + (self.count_stat(stat) as f32 * 0.1)
    }

    /// Current tier based on number of upgrades
    pub fn current_tier(&self) -> UpgradeTier {
        match self.upgrades.len() {
            0 => UpgradeTier::Base,
            1 => UpgradeTier::Tier1,
            2 => UpgradeTier::Tier2,
            3 => UpgradeTier::Tier3,
            4 => UpgradeTier::Tier4,
            _ => UpgradeTier::Tier5,
        }
    }
}

/// RFC-019: A pending upgrade choice waiting for player input
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PendingUpgrade {
    /// Card name that earned the upgrade
    pub card_name: String,
    /// Card type (for determining available stats)
    pub card_type: crate::models::card::CardType,
    /// The tier being upgraded to
    pub tier: UpgradeTier,
    /// Two stat options randomly selected for player choice
    pub options: [UpgradeableStat; 2],
}

impl PendingUpgrade {
    /// Create a new pending upgrade with random stat options
    pub fn new(card_name: String, card_type: crate::models::card::CardType, tier: UpgradeTier) -> Option<Self> {
        let options = UpgradeableStat::random_pair(&card_type)?;
        Some(Self {
            card_name,
            card_type,
            tier,
            options,
        })
    }
}

/// Character state that persists across sessions until permadeath
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CharacterState {
    /// Character profile (narrative only)
    pub profile: CharacterProfile,
    /// Current heat level
    pub heat: u32,
    /// Unix timestamp of last deck completion
    pub last_played: u64,
    /// Total decks played with this character
    pub decks_played: u32,
    /// Unix timestamp when character was created
    pub created_at: u64,
    /// RFC-017: Per-card play counts for upgrade tracking
    /// Key is card name, value is times played
    #[serde(default)]
    pub card_play_counts: HashMap<String, u32>,
    /// RFC-019: Per-card upgrade choices
    /// Key is card name, value is upgrade history
    #[serde(default)]
    pub card_upgrades: HashMap<String, CardUpgrades>,
    /// RFC-019: Pending upgrade choices waiting for player input
    /// These persist across sessions so player can make choice on next load
    #[serde(default)]
    pub pending_upgrades: Vec<PendingUpgrade>,
    /// Story history from completed deals (narrative log)
    #[serde(default)]
    pub story_history: Vec<String>,
}

impl CharacterState {
    pub fn new() -> Self {
        let now = current_timestamp();
        Self {
            profile: CharacterProfile::Default,
            heat: 0,
            last_played: now,
            decks_played: 0,
            created_at: now,
            card_play_counts: HashMap::new(),
            card_upgrades: HashMap::new(),
            pending_upgrades: Vec::new(),
            story_history: Vec::new(),
        }
    }

    /// Get current heat tier
    pub fn heat_tier(&self) -> HeatTier {
        HeatTier::from_heat(self.heat)
    }

    /// Calculate and apply heat decay based on elapsed time
    /// Returns the amount of heat that decayed
    pub fn apply_decay(&mut self) -> u32 {
        let now = current_timestamp();
        let elapsed_secs = now.saturating_sub(self.last_played);
        let elapsed_hours = (elapsed_secs / 3600).min(168) as u32; // Cap at 1 week

        let decay = elapsed_hours.min(self.heat);
        self.heat = self.heat.saturating_sub(decay);
        decay
    }

    /// Update last_played timestamp (called at deck end)
    pub fn mark_deck_completed(&mut self) {
        self.last_played = current_timestamp();
        self.decks_played += 1;
    }

    /// Add heat from hand resolution
    pub fn add_heat(&mut self, amount: i32) {
        if amount >= 0 {
            self.heat = self.heat.saturating_add(amount as u32);
        } else {
            self.heat = self.heat.saturating_sub((-amount) as u32);
        }
    }

    /// RFC-017: Increment play count for a card
    pub fn increment_play_count(&mut self, card_name: &str) {
        let count = self.card_play_counts.entry(card_name.to_string()).or_insert(0);
        *count = count.saturating_add(1);
    }

    /// RFC-017: Get play count for a card
    pub fn get_play_count(&self, card_name: &str) -> u32 {
        *self.card_play_counts.get(card_name).unwrap_or(&0)
    }

    /// RFC-017: Get upgrade tier for a card based on play count
    pub fn get_card_tier(&self, card_name: &str) -> UpgradeTier {
        UpgradeTier::from_play_count(self.get_play_count(card_name))
    }

    /// RFC-019: Get upgrade history for a card
    pub fn get_card_upgrades(&self, card_name: &str) -> Option<&CardUpgrades> {
        self.card_upgrades.get(card_name)
    }

    /// RFC-019: Add an upgrade choice for a card
    pub fn add_card_upgrade(&mut self, card_name: &str, stat: UpgradeableStat) {
        let upgrades = self.card_upgrades
            .entry(card_name.to_string())
            .or_insert_with(CardUpgrades::new);
        upgrades.add_upgrade(stat);
    }

    /// RFC-019: Get the stat multiplier for a specific stat on a card
    pub fn get_stat_multiplier(&self, card_name: &str, stat: UpgradeableStat) -> f32 {
        self.card_upgrades
            .get(card_name)
            .map(|u| u.stat_multiplier(stat))
            .unwrap_or(1.0)
    }

    /// RFC-019: Check if a card has earned an upgrade that hasn't been applied yet
    /// Returns the tier to upgrade to, if any
    pub fn check_pending_upgrade(&self, card_name: &str) -> Option<UpgradeTier> {
        let play_count = self.get_play_count(card_name);
        let tier_from_plays = UpgradeTier::from_play_count(play_count);

        let upgrade_count = self.card_upgrades
            .get(card_name)
            .map(|u| u.upgrades.len())
            .unwrap_or(0);
        let tier_from_upgrades = match upgrade_count {
            0 => UpgradeTier::Base,
            1 => UpgradeTier::Tier1,
            2 => UpgradeTier::Tier2,
            3 => UpgradeTier::Tier3,
            4 => UpgradeTier::Tier4,
            _ => UpgradeTier::Tier5,
        };

        // If play count tier is higher than upgrade tier, we have a pending upgrade
        if tier_from_plays > tier_from_upgrades {
            // Return the next tier to upgrade to
            Some(match tier_from_upgrades {
                UpgradeTier::Base => UpgradeTier::Tier1,
                UpgradeTier::Tier1 => UpgradeTier::Tier2,
                UpgradeTier::Tier2 => UpgradeTier::Tier3,
                UpgradeTier::Tier3 => UpgradeTier::Tier4,
                UpgradeTier::Tier4 | UpgradeTier::Tier5 => UpgradeTier::Tier5,
            })
        } else {
            None
        }
    }

    /// RFC-019: Queue a pending upgrade for a card
    pub fn queue_pending_upgrade(&mut self, card_name: &str, card_type: &crate::models::card::CardType) -> bool {
        // Check if this card already has a pending upgrade in the queue
        if self.pending_upgrades.iter().any(|p| p.card_name == card_name) {
            return false;
        }

        // Check if there's actually a pending upgrade
        if let Some(tier) = self.check_pending_upgrade(card_name) {
            if let Some(pending) = PendingUpgrade::new(card_name.to_string(), card_type.clone(), tier) {
                self.pending_upgrades.push(pending);
                return true;
            }
        }
        false
    }

    /// RFC-019: Get the next pending upgrade (if any)
    pub fn next_pending_upgrade(&self) -> Option<&PendingUpgrade> {
        self.pending_upgrades.first()
    }

    /// RFC-019: Has pending upgrades
    pub fn has_pending_upgrades(&self) -> bool {
        !self.pending_upgrades.is_empty()
    }

    /// RFC-019: Apply an upgrade choice and remove from pending queue
    /// Returns true if the upgrade was applied
    pub fn apply_upgrade_choice(&mut self, stat: UpgradeableStat) -> bool {
        if let Some(pending) = self.pending_upgrades.first() {
            // Verify the stat is one of the options
            if pending.options[0] != stat && pending.options[1] != stat {
                return false;
            }

            // Apply the upgrade
            let card_name = pending.card_name.clone();
            self.add_card_upgrade(&card_name, stat);

            // Remove from pending queue
            self.pending_upgrades.remove(0);
            true
        } else {
            false
        }
    }

    /// Validate character state sanity
    pub fn validate(&self) -> Result<(), SaveError> {
        if self.heat > MAX_HEAT {
            return Err(SaveError::ValidationError(format!(
                "Heat {} exceeds maximum {}",
                self.heat, MAX_HEAT
            )));
        }
        if self.decks_played > MAX_DECKS_PLAYED {
            return Err(SaveError::ValidationError(format!(
                "Decks played {} exceeds maximum {}",
                self.decks_played, MAX_DECKS_PLAYED
            )));
        }
        Ok(())
    }
}

impl Default for CharacterState {
    fn default() -> Self {
        Self::new()
    }
}

/// Account-wide state that persists forever (survives permadeath)
///
/// RFC-016: Account Cash System
/// - cash_on_hand: Spendable currency for unlocks
/// - lifetime_revenue: Total ever earned (for achievements/leaderboards)
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AccountState {
    /// Spendable cash (reduced by purchases)
    pub cash_on_hand: u64,
    /// Total cash ever earned (never reduced)
    pub lifetime_revenue: u64,
    /// Total hands completed successfully
    pub hands_completed: u32,
    // Future: unlocked_cards, unlocked_locations, achievements
}

impl AccountState {
    pub fn new() -> Self {
        Self {
            cash_on_hand: 0,
            lifetime_revenue: 0,
            hands_completed: 0,
        }
    }

    /// Add profit from a successful hand
    pub fn add_profit(&mut self, profit: u32) {
        let profit = profit as u64;
        self.cash_on_hand = self.cash_on_hand.saturating_add(profit).min(MAX_CASH);
        self.lifetime_revenue = self.lifetime_revenue.saturating_add(profit).min(MAX_CASH);
        self.hands_completed = self.hands_completed.saturating_add(1);
    }

    /// Spend cash on an unlock (returns false if insufficient funds)
    pub fn spend(&mut self, amount: u64) -> bool {
        if self.cash_on_hand >= amount {
            self.cash_on_hand -= amount;
            true
        } else {
            false
        }
    }

    /// Validate account state sanity
    pub fn validate(&self) -> Result<(), SaveError> {
        if self.cash_on_hand > MAX_CASH {
            return Err(SaveError::ValidationError(format!(
                "Cash on hand {} exceeds maximum {}",
                self.cash_on_hand, MAX_CASH
            )));
        }
        if self.lifetime_revenue > MAX_CASH {
            return Err(SaveError::ValidationError(format!(
                "Lifetime revenue {} exceeds maximum {}",
                self.lifetime_revenue, MAX_CASH
            )));
        }
        // Lifetime revenue should always be >= cash on hand
        // (but allow some slack for edge cases during migration)
        Ok(())
    }
}

impl Default for AccountState {
    fn default() -> Self {
        Self::new()
    }
}

/// Get current Unix timestamp in seconds
pub fn current_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_heat_tier_boundaries() {
        // Cold: 0-29
        assert_eq!(HeatTier::from_heat(0), HeatTier::Cold);
        assert_eq!(HeatTier::from_heat(29), HeatTier::Cold);
        // Warm: 30-59
        assert_eq!(HeatTier::from_heat(30), HeatTier::Warm);
        assert_eq!(HeatTier::from_heat(59), HeatTier::Warm);
        // Hot: 60-89
        assert_eq!(HeatTier::from_heat(60), HeatTier::Hot);
        assert_eq!(HeatTier::from_heat(89), HeatTier::Hot);
        // Blazing: 90-119
        assert_eq!(HeatTier::from_heat(90), HeatTier::Blazing);
        assert_eq!(HeatTier::from_heat(119), HeatTier::Blazing);
        // Scorching: 120-149
        assert_eq!(HeatTier::from_heat(120), HeatTier::Scorching);
        assert_eq!(HeatTier::from_heat(149), HeatTier::Scorching);
        // Inferno: 150+
        assert_eq!(HeatTier::from_heat(150), HeatTier::Inferno);
        assert_eq!(HeatTier::from_heat(1000), HeatTier::Inferno);
    }

    #[test]
    fn test_character_state_new() {
        let state = CharacterState::new();
        assert_eq!(state.heat, 0);
        assert_eq!(state.decks_played, 0);
        assert_eq!(state.profile, CharacterProfile::Default);
        assert!(state.created_at > 0);
        assert!(state.last_played > 0);
    }

    #[test]
    fn test_add_heat_positive() {
        let mut state = CharacterState::new();
        state.add_heat(10);
        assert_eq!(state.heat, 10);
        state.add_heat(5);
        assert_eq!(state.heat, 15);
    }

    #[test]
    fn test_add_heat_negative() {
        let mut state = CharacterState::new();
        state.heat = 20;
        state.add_heat(-5);
        assert_eq!(state.heat, 15);
    }

    #[test]
    fn test_add_heat_no_underflow() {
        let mut state = CharacterState::new();
        state.heat = 5;
        state.add_heat(-10);
        assert_eq!(state.heat, 0);
    }

    #[test]
    fn test_decay_calculation() {
        let mut state = CharacterState::new();
        state.heat = 100;
        // Simulate 10 hours ago
        state.last_played = current_timestamp().saturating_sub(10 * 3600);

        let decay = state.apply_decay();
        assert_eq!(decay, 10);
        assert_eq!(state.heat, 90);
    }

    #[test]
    fn test_decay_capped_at_168_hours() {
        let mut state = CharacterState::new();
        state.heat = 200;
        // Simulate 1 year ago (way more than 168 hours)
        state.last_played = current_timestamp().saturating_sub(365 * 24 * 3600);

        let decay = state.apply_decay();
        assert_eq!(decay, 168); // Capped at 168
        assert_eq!(state.heat, 32); // 200 - 168
    }

    #[test]
    fn test_decay_does_not_go_below_zero() {
        let mut state = CharacterState::new();
        state.heat = 5;
        state.last_played = current_timestamp().saturating_sub(100 * 3600);

        let decay = state.apply_decay();
        assert_eq!(decay, 5); // Only 5 available to decay
        assert_eq!(state.heat, 0);
    }

    #[test]
    fn test_mark_deck_completed() {
        let mut state = CharacterState::new();
        let initial_decks = state.decks_played;
        let initial_time = state.last_played;

        std::thread::sleep(std::time::Duration::from_millis(10));
        state.mark_deck_completed();

        assert_eq!(state.decks_played, initial_decks + 1);
        assert!(state.last_played >= initial_time);
    }

    #[test]
    fn test_validation_rejects_excessive_heat() {
        let mut state = CharacterState::new();
        state.heat = 20_000; // Way over limit

        let result = state.validate();
        assert!(result.is_err());
    }

    #[test]
    fn test_save_data_validation() {
        let mut data = SaveData::new();
        data.character = Some(CharacterState::new());

        assert!(data.validate().is_ok());
    }

    // ========================================================================
    // AccountState Tests (RFC-016)
    // ========================================================================

    #[test]
    fn test_account_state_new() {
        let account = AccountState::new();
        assert_eq!(account.cash_on_hand, 0);
        assert_eq!(account.lifetime_revenue, 0);
        assert_eq!(account.hands_completed, 0);
    }

    #[test]
    fn test_account_add_profit() {
        let mut account = AccountState::new();
        account.add_profit(100);

        assert_eq!(account.cash_on_hand, 100);
        assert_eq!(account.lifetime_revenue, 100);
        assert_eq!(account.hands_completed, 1);

        account.add_profit(50);
        assert_eq!(account.cash_on_hand, 150);
        assert_eq!(account.lifetime_revenue, 150);
        assert_eq!(account.hands_completed, 2);
    }

    #[test]
    fn test_account_spend_success() {
        let mut account = AccountState::new();
        account.add_profit(1000);

        let spent = account.spend(400);
        assert!(spent);
        assert_eq!(account.cash_on_hand, 600);
        // Lifetime revenue unchanged by spending
        assert_eq!(account.lifetime_revenue, 1000);
    }

    #[test]
    fn test_account_spend_insufficient_funds() {
        let mut account = AccountState::new();
        account.add_profit(100);

        let spent = account.spend(500);
        assert!(!spent);
        // Cash unchanged when spend fails
        assert_eq!(account.cash_on_hand, 100);
        assert_eq!(account.lifetime_revenue, 100);
    }

    #[test]
    fn test_account_validation() {
        let account = AccountState::new();
        assert!(account.validate().is_ok());
    }

    #[test]
    fn test_account_validation_rejects_excessive_cash() {
        let mut account = AccountState::new();
        account.cash_on_hand = MAX_CASH + 1;

        let result = account.validate();
        assert!(result.is_err());
    }

    #[test]
    fn test_save_data_with_account() {
        let mut data = SaveData::new();
        data.account.add_profit(500);
        data.character = Some(CharacterState::new());

        assert!(data.validate().is_ok());
        assert_eq!(data.account.cash_on_hand, 500);
    }

    // ========================================================================
    // UpgradeTier Tests (RFC-017)
    // ========================================================================

    #[test]
    fn test_upgrade_tier_from_play_count() {
        // TESTING MODE: Tiers at 0/1/2/3/4/5 plays (production: 0/5/12/25/50/100)
        assert_eq!(UpgradeTier::from_play_count(0), UpgradeTier::Base);
        assert_eq!(UpgradeTier::from_play_count(1), UpgradeTier::Tier1);
        assert_eq!(UpgradeTier::from_play_count(2), UpgradeTier::Tier2);
        assert_eq!(UpgradeTier::from_play_count(3), UpgradeTier::Tier3);
        assert_eq!(UpgradeTier::from_play_count(4), UpgradeTier::Tier4);
        assert_eq!(UpgradeTier::from_play_count(5), UpgradeTier::Tier5);
        assert_eq!(UpgradeTier::from_play_count(100), UpgradeTier::Tier5);
    }

    #[test]
    fn test_upgrade_tier_multiplier() {
        assert!((UpgradeTier::Base.multiplier() - 1.0).abs() < f32::EPSILON);
        assert!((UpgradeTier::Tier1.multiplier() - 1.1).abs() < f32::EPSILON);
        assert!((UpgradeTier::Tier2.multiplier() - 1.2).abs() < f32::EPSILON);
        assert!((UpgradeTier::Tier3.multiplier() - 1.3).abs() < f32::EPSILON);
        assert!((UpgradeTier::Tier4.multiplier() - 1.4).abs() < f32::EPSILON);
        assert!((UpgradeTier::Tier5.multiplier() - 1.5).abs() < f32::EPSILON);
    }

    #[test]
    fn test_upgrade_tier_plays_to_next() {
        // TESTING MODE thresholds for next tier
        assert_eq!(UpgradeTier::Base.plays_to_next(), Some(1));
        assert_eq!(UpgradeTier::Tier1.plays_to_next(), Some(2));
        assert_eq!(UpgradeTier::Tier2.plays_to_next(), Some(3));
        assert_eq!(UpgradeTier::Tier3.plays_to_next(), Some(4));
        assert_eq!(UpgradeTier::Tier4.plays_to_next(), Some(5));
        assert_eq!(UpgradeTier::Tier5.plays_to_next(), None); // Max tier
    }

    #[test]
    fn test_upgrade_tier_name() {
        assert_eq!(UpgradeTier::Base.name(), "Base");
        // All upgraded tiers show star (color varies but name is same)
        assert_eq!(UpgradeTier::Tier1.name(), "★");
        assert_eq!(UpgradeTier::Tier5.name(), "★");
    }

    #[test]
    fn test_upgrade_tier_star_color() {
        // Grey for base/tier1
        assert_eq!(UpgradeTier::Base.star_color(), (0.5, 0.5, 0.5));
        assert_eq!(UpgradeTier::Tier1.star_color(), (0.6, 0.6, 0.6));
        // Bronze for tier2
        assert_eq!(UpgradeTier::Tier2.star_color(), (0.8, 0.5, 0.2));
        // Silver for tier3
        assert_eq!(UpgradeTier::Tier3.star_color(), (0.75, 0.75, 0.8));
        // Gold for tier4 and tier5
        assert_eq!(UpgradeTier::Tier4.star_color(), (1.0, 0.84, 0.0));
        assert_eq!(UpgradeTier::Tier5.star_color(), (1.0, 0.84, 0.0));
    }

    #[test]
    fn test_upgrade_tier_is_foil() {
        assert!(!UpgradeTier::Base.is_foil());
        assert!(!UpgradeTier::Tier1.is_foil());
        assert!(!UpgradeTier::Tier4.is_foil());
        assert!(UpgradeTier::Tier5.is_foil());
    }

    // ========================================================================
    // Play Count Tracking Tests (RFC-017)
    // ========================================================================

    #[test]
    fn test_character_increment_play_count() {
        let mut state = CharacterState::new();

        state.increment_play_count("Test Card");
        assert_eq!(state.get_play_count("Test Card"), 1);

        state.increment_play_count("Test Card");
        assert_eq!(state.get_play_count("Test Card"), 2);

        // Different card starts at 0
        assert_eq!(state.get_play_count("Other Card"), 0);
    }

    #[test]
    fn test_character_play_count_multiple_cards() {
        let mut state = CharacterState::new();

        state.increment_play_count("Card A");
        state.increment_play_count("Card A");
        state.increment_play_count("Card B");
        state.increment_play_count("Card A");

        assert_eq!(state.get_play_count("Card A"), 3);
        assert_eq!(state.get_play_count("Card B"), 1);
        assert_eq!(state.get_play_count("Card C"), 0);
    }

    #[test]
    fn test_character_get_card_tier() {
        let mut state = CharacterState::new();

        // No plays = Base tier
        assert_eq!(state.get_card_tier("Test Card"), UpgradeTier::Base);

        // TESTING MODE: 1st play = Tier 1
        state.increment_play_count("Test Card");
        assert_eq!(state.get_card_tier("Test Card"), UpgradeTier::Tier1);

        // 2nd play = Tier 2 (TESTING MODE)
        state.increment_play_count("Test Card");
        assert_eq!(state.get_card_tier("Test Card"), UpgradeTier::Tier2);

        // Continue to max tier
        state.increment_play_count("Test Card"); // 3 plays = Tier3
        state.increment_play_count("Test Card"); // 4 plays = Tier4
        state.increment_play_count("Test Card"); // 5 plays = Tier5 (max)
        assert_eq!(state.get_card_tier("Test Card"), UpgradeTier::Tier5);

        // More plays stay at Tier5 (max)
        state.increment_play_count("Test Card");
        assert_eq!(state.get_card_tier("Test Card"), UpgradeTier::Tier5);
    }

    #[test]
    fn test_character_play_count_saturating() {
        let mut state = CharacterState::new();
        // Directly set to near-max to test saturating behavior
        state.card_play_counts.insert("Test".to_string(), u32::MAX - 1);

        state.increment_play_count("Test");
        assert_eq!(state.get_play_count("Test"), u32::MAX);

        // Should not overflow
        state.increment_play_count("Test");
        assert_eq!(state.get_play_count("Test"), u32::MAX);
    }

    #[test]
    fn test_character_play_counts_default() {
        // Test that default HashMap initializes correctly (backward compatibility)
        let state = CharacterState::new();
        assert!(state.card_play_counts.is_empty());
    }

    // ========================================================================
    // RFC-018: Heat Tier → Narc Upgrade Tier Tests
    // ========================================================================

    #[test]
    fn test_heat_tier_narc_upgrade_mapping() {
        // Cold → Base (no bonus)
        assert_eq!(HeatTier::Cold.narc_upgrade_tier(), UpgradeTier::Base);

        // Warm → Tier1 (+10%)
        assert_eq!(HeatTier::Warm.narc_upgrade_tier(), UpgradeTier::Tier1);

        // Hot → Tier2 (+20%)
        assert_eq!(HeatTier::Hot.narc_upgrade_tier(), UpgradeTier::Tier2);

        // Blazing → Tier3 (+30%)
        assert_eq!(HeatTier::Blazing.narc_upgrade_tier(), UpgradeTier::Tier3);

        // Scorching → Tier4 (+40%)
        assert_eq!(HeatTier::Scorching.narc_upgrade_tier(), UpgradeTier::Tier4);

        // Inferno → Tier5 (+50% with foil effect)
        assert_eq!(HeatTier::Inferno.narc_upgrade_tier(), UpgradeTier::Tier5);
    }

    #[test]
    fn test_heat_tier_danger_names() {
        assert_eq!(HeatTier::Cold.danger_name(), "Relaxed");
        assert_eq!(HeatTier::Warm.danger_name(), "Alert");
        assert_eq!(HeatTier::Hot.danger_name(), "Dangerous");
        assert_eq!(HeatTier::Blazing.danger_name(), "Severe");
        assert_eq!(HeatTier::Scorching.danger_name(), "Intense");
        assert_eq!(HeatTier::Inferno.danger_name(), "Deadly");
    }

    #[test]
    fn test_heat_to_narc_tier_via_character() {
        let mut character = CharacterState::new();

        // At 0 heat (Cold), Narc should be Base
        character.heat = 0;
        assert_eq!(character.heat_tier().narc_upgrade_tier(), UpgradeTier::Base);

        // At 30 heat (Warm), Narc should be Tier1
        character.heat = 30;
        assert_eq!(character.heat_tier().narc_upgrade_tier(), UpgradeTier::Tier1);

        // At 60 heat (Hot), Narc should be Tier2
        character.heat = 60;
        assert_eq!(character.heat_tier().narc_upgrade_tier(), UpgradeTier::Tier2);

        // At 90 heat (Blazing), Narc should be Tier3
        character.heat = 90;
        assert_eq!(character.heat_tier().narc_upgrade_tier(), UpgradeTier::Tier3);

        // At 120 heat (Scorching), Narc should be Tier4
        character.heat = 120;
        assert_eq!(character.heat_tier().narc_upgrade_tier(), UpgradeTier::Tier4);

        // At 150 heat (Inferno), Narc should be Tier5 with foil
        character.heat = 150;
        assert_eq!(character.heat_tier().narc_upgrade_tier(), UpgradeTier::Tier5);
    }
}
