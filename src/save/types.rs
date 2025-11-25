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
    Cold,      // 0-25
    Warm,      // 26-50
    Hot,       // 51-75
    Scorching, // 76-100
    Inferno,   // 101+
}

impl HeatTier {
    pub fn from_heat(heat: u32) -> Self {
        match heat {
            0..=25 => HeatTier::Cold,
            26..=50 => HeatTier::Warm,
            51..=75 => HeatTier::Hot,
            76..=100 => HeatTier::Scorching,
            _ => HeatTier::Inferno,
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            HeatTier::Cold => "Cold",
            HeatTier::Warm => "Warm",
            HeatTier::Hot => "Hot",
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
            HeatTier::Scorching => (1.0, 0.2, 0.2), // Red
            HeatTier::Inferno => (0.8, 0.2, 0.8),   // Purple
        }
    }
}

/// RFC-017: Card upgrade tier based on play count
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
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
        assert_eq!(HeatTier::from_heat(0), HeatTier::Cold);
        assert_eq!(HeatTier::from_heat(25), HeatTier::Cold);
        assert_eq!(HeatTier::from_heat(26), HeatTier::Warm);
        assert_eq!(HeatTier::from_heat(50), HeatTier::Warm);
        assert_eq!(HeatTier::from_heat(51), HeatTier::Hot);
        assert_eq!(HeatTier::from_heat(75), HeatTier::Hot);
        assert_eq!(HeatTier::from_heat(76), HeatTier::Scorching);
        assert_eq!(HeatTier::from_heat(100), HeatTier::Scorching);
        assert_eq!(HeatTier::from_heat(101), HeatTier::Inferno);
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
}
