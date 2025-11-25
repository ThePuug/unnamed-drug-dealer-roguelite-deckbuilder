// Save system data types.

use bevy::prelude::Resource;
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

/// Current save file format version
pub const SAVE_VERSION: u32 = 1;

/// Maximum sanity values for validation
const MAX_HEAT: u32 = 10_000;
const MAX_DECKS_PLAYED: u32 = 100_000;

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
#[derive(Resource, Serialize, Deserialize, Clone, Debug, Default)]
pub struct SaveData {
    /// Current character (None if permadeath occurred or new game)
    pub character: Option<CharacterState>,
    // Future: account: AccountState (account-wide cash, unlocks)
}

impl SaveData {
    pub fn new() -> Self {
        Self {
            character: None,
        }
    }

    /// Validate data sanity (defense in depth)
    pub fn validate(&self) -> Result<(), SaveError> {
        if let Some(ref character) = self.character {
            character.validate()?;
        }
        Ok(())
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
    // Future: card_play_counts for per-card upgrade tracking
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
}
