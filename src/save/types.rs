// Save system data types.

use bevy::prelude::Resource;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::time::{SystemTime, UNIX_EPOCH};

/// Current save file format version
/// SOW-021: Bumped 1 -> 2 (upgrade thresholds changed from testing to spec values;
/// older saves are rejected and the game starts a fresh account)
/// RFC-023: Bumped 2 -> 3 (single character replaced by dealer roster;
/// pre-release wipe convention - older saves start fresh)
// SOW-025: v4 adds dealer stations + street cred (pre-release wipe convention)
pub const SAVE_VERSION: u32 = 5; // SOW-026: lean starting collection

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
    /// RFC-023: The kingpin's dealer roster. Invariant: never empty - a
    /// fresh save recruits one starter dealer.
    pub dealers: Vec<DealerState>,
    /// Which dealer runs the next session
    pub active_dealer: usize,
    /// Account-wide state (persists forever - the kingpin's books)
    /// RFC-016: Account Cash System
    #[serde(default)]
    pub account: AccountState,
    /// SOW-023: arcade board of fallen empires. Survives `reset_empire` -
    /// the one thing a kingpin bust cannot erase. Stats are displayed on the
    /// game-over board; each epitaph also ARCHIVES the empire's full story
    /// history for the SOW-026 ledger (not displayed yet).
    #[serde(default)]
    pub fallen_empires: Vec<EmpireEpitaph>,
}

/// The tombstone of one empire: summary stats for the arcade leaderboard
/// plus the archived stories of everyone who worked it
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct EmpireEpitaph {
    pub ended_at: u64,
    pub lifetime_revenue: u64,
    pub cash_at_fall: u64,
    /// Roster size beyond the kingpin
    pub dealers_hired: u32,
    /// Times anyone in the roster went through the system
    pub total_prior_convictions: u32,
    /// Decks played across the whole roster
    pub decks_played: u32,
    /// Aggregate story history of every dealer (archive for SOW-026)
    pub stories: Vec<String>,
}

impl EmpireEpitaph {
    /// Summarize a dying empire. Pure given the timestamp.
    pub fn from_save(save: &SaveData, ended_at: u64) -> Self {
        Self {
            ended_at,
            lifetime_revenue: save.account.lifetime_revenue,
            cash_at_fall: save.account.cash_on_hand,
            dealers_hired: save.dealers.len().saturating_sub(1) as u32,
            total_prior_convictions: save.dealers.iter().map(|d| d.prior_convictions).sum(),
            decks_played: save.dealers.iter().map(|d| d.character.decks_played).sum(),
            stories: save
                .dealers
                .iter()
                .flat_map(|d| d.character.story_history.iter().cloned())
                .collect(),
        }
    }
}

/// Top-N fallen empires by lifetime revenue (indices into the input slice,
/// so callers can mark "this run" by comparing against the latest index)
pub fn leaderboard_top(fallen: &[EmpireEpitaph], n: usize) -> Vec<usize> {
    let mut order: Vec<usize> = (0..fallen.len()).collect();
    order.sort_by(|a, b| fallen[*b].lifetime_revenue.cmp(&fallen[*a].lifetime_revenue));
    order.truncate(n);
    order
}

impl SaveData {
    pub fn new() -> Self {
        Self {
            // RFC-023: every empire starts with the kingpin dealing in person
            dealers: vec![DealerState::kingpin()],
            active_dealer: 0,
            account: AccountState::new(),
            fallen_empires: Vec::new(),
        }
    }

    /// The dealer selected to run sessions (roster is never empty and the
    /// index is validated on load, so plain indexing is safe)
    pub fn active_dealer_state(&self) -> &DealerState {
        &self.dealers[self.active_dealer]
    }

    pub fn active_dealer_state_mut(&mut self) -> &mut DealerState {
        let idx = self.active_dealer;
        &mut self.dealers[idx]
    }

    /// The active dealer's career record (heat/upgrades/stories) - the
    /// drop-in replacement for the old singular `character`
    pub fn active_character(&self) -> &CharacterState {
        &self.active_dealer_state().character
    }

    pub fn active_character_mut(&mut self) -> &mut CharacterState {
        &mut self.active_dealer_state_mut().character
    }

    /// Cost to hire the next dealer at the current roster size
    pub fn next_hire_cost(&self) -> u64 {
        hire_cost(self.dealers.len())
    }

    /// Hire a new dealer, spending from the kingpin's account.
    /// Returns false (no mutation) if the account can't afford it.
    pub fn hire_dealer(&mut self) -> bool {
        let cost = self.next_hire_cost();
        if !self.account.spend(cost) {
            return false;
        }
        self.dealers.push(DealerState::recruit(&self.dealers));
        true
    }

    /// A run just completed somewhere in the empire: every jailed dealer's
    /// sentence ticks down, EXCEPT the runner's (a dealer jailed by this very
    /// run must not start serving on it). Returns the names released.
    pub fn complete_run_tick(&mut self, runner: usize) -> Vec<String> {
        self.dealers
            .iter_mut()
            .enumerate()
            .filter(|(idx, _)| *idx != runner)
            .filter_map(|(_, d)| d.tick_sentence().then(|| d.name.clone()))
            .collect()
    }

    /// SOW-025: relocate a dealer to another area. Costs the flat move fee
    /// from global cash plus one run of downtime ("getting established").
    /// The station changes immediately; the dealer is unavailable until the
    /// relocation ticks out. Returns false (no mutation) if the dealer is
    /// unavailable, already stationed there, or the fee is unaffordable.
    pub fn move_dealer(&mut self, dealer_idx: usize, to_area: &str) -> bool {
        let Some(dealer) = self.dealers.get(dealer_idx) else {
            return false;
        };
        if !dealer.is_available() || dealer.station == to_area {
            return false;
        }
        if !self.account.spend(MOVE_FEE) {
            return false;
        }
        let dealer = &mut self.dealers[dealer_idx];
        dealer.station = to_area.to_string();
        dealer.status = DealerStatus::Relocating { runs_remaining: 1 };
        true
    }

    /// SOW-025: the flat relocation fee (exposed for UI labels)
    pub fn move_fee(&self) -> u64 {
        MOVE_FEE
    }

    /// SOW-025: the roster's best street cred for an area - any dealer's
    /// reputation opens doors there. Returns (dealer index, cred); the shop
    /// shows WHO is effectively unlocking ("unlocked by <name>").
    pub fn best_cred(&self, area: &str) -> Option<(usize, u32)> {
        self.dealers
            .iter()
            .enumerate()
            .map(|(i, d)| (i, d.cred_in(area)))
            .max_by_key(|(_, cred)| *cred)
            .filter(|(_, cred)| *cred > 0)
    }

    /// Pay to release a jailed dealer before the sentence ends. Costs
    /// bail_cost(runs_remaining) from global cash; the heat reduction stays
    /// proportional to time actually served (the bail tradeoff).
    /// Returns false (no mutation) if not jailed or unaffordable.
    pub fn bail_out(&mut self, dealer_idx: usize) -> bool {
        let Some(dealer) = self.dealers.get(dealer_idx) else {
            return false;
        };
        let Some(remaining) = dealer.jail_remaining() else {
            return false;
        };
        if !self.account.spend(bail_cost(remaining)) {
            return false;
        }
        self.dealers[dealer_idx].release();
        true
    }

    /// RFC-023: the KINGPIN busting ends the empire - the one remaining
    /// permadeath. Everything resets, including the books - EXCEPT the
    /// arcade board: the falling empire's epitaph is appended first and the
    /// board carries into the fresh save (SOW-023 addendum).
    pub fn reset_empire(&mut self) {
        let mut fallen = std::mem::take(&mut self.fallen_empires);
        fallen.push(EmpireEpitaph::from_save(self, current_timestamp()));
        *self = SaveData::new();
        self.fallen_empires = fallen;
    }

    /// Validate data sanity (defense in depth)
    pub fn validate(&self) -> Result<(), SaveError> {
        if self.dealers.is_empty() {
            return Err(SaveError::ValidationError("Dealer roster is empty".into()));
        }
        if self.active_dealer >= self.dealers.len() {
            return Err(SaveError::ValidationError(format!(
                "Active dealer index {} out of range ({} dealers)",
                self.active_dealer,
                self.dealers.len()
            )));
        }
        for dealer in &self.dealers {
            dealer.validate()?;
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
    Base,   // 0-2 plays: No bonus
    Tier1,  // 3+ plays: +10% primary stat
    Tier2,  // Bronze: 8+ plays
    Tier3,  // Silver: 15+ plays
    Tier4,  // Gold: 25+ plays
    Tier5,  // Foil: 40+ plays
}

impl UpgradeTier {
    /// Calculate tier from play count
    /// SOW-021: Spec thresholds per card-system.md (0/3/8/15/25/40 plays)
    pub fn from_play_count(count: u32) -> Self {
        match count {
            0..=2 => UpgradeTier::Base,
            3..=7 => UpgradeTier::Tier1,
            8..=14 => UpgradeTier::Tier2,
            15..=24 => UpgradeTier::Tier3,
            25..=39 => UpgradeTier::Tier4,
            _ => UpgradeTier::Tier5,
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
            UpgradeTier::Base => Some(3),
            UpgradeTier::Tier1 => Some(8),
            UpgradeTier::Tier2 => Some(15),
            UpgradeTier::Tier3 => Some(25),
            UpgradeTier::Tier4 => Some(40),
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

    /// Apply a signed session heat delta to career heat (GO HOME transfer).
    /// Session heat can be negative (cooling run); career heat floors at 0.
    pub fn apply_session_heat(&mut self, session_heat: i32) {
        self.heat = (i64::from(self.heat) + i64::from(session_heat)).max(0) as u32;
    }

    /// Calculate and apply heat decay based on elapsed time
    /// Returns the amount of heat that decayed
    pub fn apply_decay(&mut self) -> u32 {
        let now = current_timestamp();
        let elapsed_secs = now.saturating_sub(self.last_played);
        let elapsed_hours = (elapsed_secs / 3600).min(168) as u32; // Cap at 1 week

        let decay = elapsed_hours.min(self.heat);
        self.heat = self.heat.saturating_sub(decay);
        // SOW-021: consume the decay window so repeated calls are idempotent.
        // OnEnter(DeckBuilding) fires more than once per launch (UpgradeChoice
        // round trips), which previously re-applied the same decay each time.
        if decay > 0 {
            self.last_played = now;
        }
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

    /// SOW-021: Apply an upgrade choice for a SPECIFIC pending card (batched
    /// upgrade screen lets the player resolve pending upgrades in any order).
    /// Returns true if the upgrade was applied.
    pub fn apply_upgrade_choice_for(&mut self, card_name: &str, stat: UpgradeableStat) -> bool {
        let Some(index) = self.pending_upgrades.iter().position(|p| p.card_name == card_name) else {
            return false;
        };

        // Verify the stat is one of this card's offered options
        let pending = &self.pending_upgrades[index];
        if pending.options[0] != stat && pending.options[1] != stat {
            return false;
        }

        self.add_card_upgrade(card_name, stat);
        self.pending_upgrades.remove(index);
        true
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

// ============================================================================
// RFC-023: Kingpin & Dealers
// ============================================================================

/// Jail sentences are TURN-BASED (Reed, 2026-07-12): one run per 25 heat at
/// bust, plus a mandatory minimum. Sentences tick down as OTHER dealers
/// complete runs - the empire keeps moving while someone sits.
const JAIL_HEAT_PER_RUN: u32 = 25;

/// Bail: $300 per remaining sentence run (early release costs money AND
/// forfeits the un-served heat reduction)
const BAIL_COST_PER_RUN: u64 = 300;

/// First hire beyond the kingpin costs $500; each subsequent hire doubles
const HIRE_BASE_COST: u64 = 500;

/// SOW-025: flat relocation fee (tuning candidate - see SOW-025 Discussion)
const MOVE_FEE: u64 = 250;

/// SOW-025: where every fresh dealer starts (the home turf; matches the
/// area flagged `unlocked: true` in shop_locations.ron)
pub const DEFAULT_STATION: &str = "the_corner";

fn default_station() -> String {
    DEFAULT_STATION.to_string()
}

/// Street names for recruited dealers
pub const DEALER_NAME_POOL: [&str; 12] = [
    "Slim", "Mouse", "Ghost", "Dice", "Rico", "Tex",
    "Lucky", "Smokes", "Blade", "Ace", "Vega", "Halo",
];

/// Actor portraits not used by the narc or any buyer persona - these faces
/// become the dealer roster (keys into GameAssets.actor_portraits)
pub const DEALER_PORTRAIT_POOL: [&str; 9] = [
    "Barista", "Displaced Patriot", "Flower Child", "Hells Angel", "Hippie",
    "Pimp", "Pretty Woman", "Street Walker", "Widow",
];

/// Whether a dealer can be sent out on a run
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum DealerStatus {
    Available,
    /// Busted: benched for a number of runs. Serving time reduces heat
    /// proportionally, so `heat_at_bust` is captured at sentencing.
    Jailed {
        runs_remaining: u32,
        sentence_total: u32,
        heat_at_bust: i32,
    },
    /// SOW-025: mid-move - getting established in the new station.
    /// Ticks down like a sentence; no heat effects (moving is cash + time).
    Relocating {
        runs_remaining: u32,
    },
}

/// A dealer in the kingpin's roster: an identity plus the career record that
/// used to be the singular CharacterState. `dealers[0]` is the KINGPIN -
/// you start the game dealing yourself. Non-kingpin busts mean jail;
/// a KINGPIN bust ends the empire (the only remaining permadeath).
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DealerState {
    pub name: String,
    /// Key into GameAssets.actor_portraits
    pub portrait: String,
    pub status: DealerStatus,
    /// The boss. Never jailed, never hire-gated; busting ends the game.
    #[serde(default)]
    pub is_kingpin: bool,
    /// Times this dealer has been through the system (release scar - the
    /// heat zeroes but the record doesn't; future difficulty hook)
    #[serde(default)]
    pub prior_convictions: u32,
    /// SOW-025: the area this dealer works - their runs happen here
    #[serde(default = "default_station")]
    pub station: String,
    /// SOW-025: reputation per area, earned +1 per successful deal there.
    /// NEVER decays - jail, moves, time: nothing erases what you built.
    #[serde(default)]
    pub street_cred: HashMap<String, u32>,
    /// Career record: heat, play counts, upgrades, stories (RFC-017/018/019)
    pub character: CharacterState,
}

/// Sentence length in runs for a bust at the given heat:
/// 1 mandatory run + 1 per 25 heat (negative heat can't shorten below 1)
pub fn jail_sentence_from_heat(heat: i32) -> u32 {
    1 + (heat.max(0) as u32) / JAIL_HEAT_PER_RUN
}

/// Cost to bail a dealer with the given remaining sentence
pub fn bail_cost(runs_remaining: u32) -> u64 {
    BAIL_COST_PER_RUN.saturating_mul(u64::from(runs_remaining))
}

/// Cost to hire the NEXT dealer given the current roster size (kingpin
/// included): $500, $1000, $2000, ... - the roster is a progression sink
pub fn hire_cost(roster_len: usize) -> u64 {
    HIRE_BASE_COST.saturating_mul(1u64 << (roster_len.saturating_sub(1)).min(40))
}

impl DealerState {
    /// The boss themselves - every fresh empire starts with the kingpin
    /// dealing in person
    pub fn kingpin() -> Self {
        Self {
            name: "The Kingpin".to_string(),
            portrait: DEALER_PORTRAIT_POOL[0].to_string(),
            status: DealerStatus::Available,
            is_kingpin: true,
            prior_convictions: 0,
            station: default_station(),
            street_cred: HashMap::new(),
            character: CharacterState::new(),
        }
    }

    /// Recruit a new dealer, picking the first name/portrait not already on
    /// the roster. Deterministic (pool order) so hiring is unit-testable;
    /// pools cycle if ever exhausted.
    pub fn recruit(existing: &[DealerState]) -> Self {
        let name = DEALER_NAME_POOL
            .iter()
            .find(|n| !existing.iter().any(|d| d.name == **n))
            .copied()
            .unwrap_or(DEALER_NAME_POOL[existing.len() % DEALER_NAME_POOL.len()]);
        let portrait = DEALER_PORTRAIT_POOL
            .iter()
            .find(|p| !existing.iter().any(|d| d.portrait == **p))
            .copied()
            .unwrap_or(DEALER_PORTRAIT_POOL[existing.len() % DEALER_PORTRAIT_POOL.len()]);

        Self {
            name: name.to_string(),
            portrait: portrait.to_string(),
            status: DealerStatus::Available,
            is_kingpin: false,
            prior_convictions: 0,
            station: default_station(),
            street_cred: HashMap::new(),
            character: CharacterState::new(),
        }
    }

    /// SOW-025: +1 street cred in an area (one successful deal there)
    pub fn add_cred(&mut self, area: &str) {
        *self.street_cred.entry(area.to_string()).or_insert(0) += 1;
    }

    /// SOW-025: this dealer's reputation in an area
    pub fn cred_in(&self, area: &str) -> u32 {
        self.street_cred.get(area).copied().unwrap_or(0)
    }

    /// SOW-025: remaining relocation downtime in runs (None if not moving)
    pub fn relocating_remaining(&self) -> Option<u32> {
        match self.status {
            DealerStatus::Relocating { runs_remaining } => Some(runs_remaining),
            _ => None,
        }
    }

    /// Sentence this dealer for a bust. Kingpins are never jailed - the
    /// caller handles a kingpin bust as game over.
    pub fn jail(&mut self) {
        debug_assert!(!self.is_kingpin, "kingpin busts end the empire, not jail");
        let heat_at_bust = self.character.heat as i32;
        let sentence = jail_sentence_from_heat(heat_at_bust);
        self.status = DealerStatus::Jailed {
            runs_remaining: sentence,
            sentence_total: sentence,
            heat_at_bust,
        };
    }

    /// Walk out of jail. Heat reduction is proportional to time served:
    /// full sentence -> heat 0; bailed after k of n runs -> keep the
    /// un-served share of heat_at_bust. Either way the record gains a
    /// prior conviction.
    pub fn release(&mut self) {
        if let DealerStatus::Jailed { runs_remaining, sentence_total, heat_at_bust } = self.status {
            let served = sentence_total.saturating_sub(runs_remaining);
            let reduction = if sentence_total > 0 {
                (i64::from(heat_at_bust) * i64::from(served) / i64::from(sentence_total)) as i32
            } else {
                heat_at_bust
            };
            self.character.heat = (heat_at_bust - reduction).max(0) as u32;
            self.prior_convictions += 1;
            self.status = DealerStatus::Available;
        }
    }

    /// One completed run elsewhere in the empire counts toward this
    /// dealer's downtime - a jail sentence OR a relocation (SOW-025).
    /// Auto-releases/arrives at zero. Returns true when the dealer
    /// becomes available.
    pub fn tick_sentence(&mut self) -> bool {
        match self.status {
            DealerStatus::Jailed { ref mut runs_remaining, .. } => {
                *runs_remaining = runs_remaining.saturating_sub(1);
                if *runs_remaining == 0 {
                    self.release();
                    return true;
                }
                false
            }
            DealerStatus::Relocating { ref mut runs_remaining } => {
                *runs_remaining = runs_remaining.saturating_sub(1);
                if *runs_remaining == 0 {
                    self.status = DealerStatus::Available;
                    return true;
                }
                false
            }
            DealerStatus::Available => false,
        }
    }

    /// Remaining sentence in runs (None if not jailed)
    pub fn jail_remaining(&self) -> Option<u32> {
        match self.status {
            DealerStatus::Jailed { runs_remaining, .. } => Some(runs_remaining),
            _ => None,
        }
    }

    pub fn is_available(&self) -> bool {
        self.status == DealerStatus::Available
    }

    pub fn validate(&self) -> Result<(), SaveError> {
        if self.name.is_empty() {
            return Err(SaveError::ValidationError("Dealer has empty name".into()));
        }
        // SOW-025: the kingpin can relocate like anyone - only jail is
        // impossible (kingpin busts end the empire instead)
        if self.is_kingpin && matches!(self.status, DealerStatus::Jailed { .. }) {
            return Err(SaveError::ValidationError("Kingpin cannot be jailed".into()));
        }
        if self.station.is_empty() {
            return Err(SaveError::ValidationError("Dealer has empty station".into()));
        }
        self.character.validate()
    }
}

/// Account-wide state that persists forever (survives permadeath)
///
/// RFC-016: Account Cash System
/// - cash_on_hand: Spendable currency for unlocks
/// - lifetime_revenue: Total ever earned (for achievements/leaderboards)
///
/// SOW-020: Location Card Shops
/// - unlocked_cards: Set of card IDs the player has purchased
/// - unlocked_locations: Set of shop location IDs the player can access
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AccountState {
    /// Spendable cash (reduced by purchases)
    pub cash_on_hand: u64,
    /// Total cash ever earned (never reduced)
    pub lifetime_revenue: u64,
    /// Total hands completed successfully
    pub hands_completed: u32,
    /// SOW-020: Card IDs the player has unlocked (includes starting collection)
    #[serde(default)]
    pub unlocked_cards: HashSet<String>,
    /// SOW-020: Shop location IDs the player can access
    #[serde(default)]
    pub unlocked_locations: HashSet<String>,
}

impl AccountState {
    pub fn new() -> Self {
        Self {
            cash_on_hand: 0,
            lifetime_revenue: 0,
            hands_completed: 0,
            unlocked_cards: Self::starting_collection(),
            unlocked_locations: HashSet::from(["the_corner".to_string()]),
        }
    }

    /// SOW-020: Starting card collection for new players
    /// SOW-026 lean start: WEED is the only starting product (Reed's
    /// authoring-first gradient - "starting off you only have access to
    /// weed"). Everything trimmed here was re-laddered into shop stock;
    /// the fresh collection must still build a legal deck (>=1 Product,
    /// >=1 Location) - asserted at load by validate_fresh_collection.
    pub fn starting_collection() -> HashSet<String> {
        HashSet::from([
            // Product: one. The first shop unlock (Shrooms) doubles your
            // productive hands per session - progression you can feel.
            "weed".to_string(),
            // Locations (simple spots)
            "dead_drop".to_string(),
            "parking_lot".to_string(),
            // Cover (basic)
            "alibi".to_string(),
            "fake_receipts".to_string(),
            // Insurance (basic)
            "fake_id".to_string(),
            // Modifiers (basic)
            "burner_phone".to_string(),
            "lookout".to_string(),
        ])
    }

    /// SOW-020: Check if a card is unlocked
    pub fn is_card_unlocked(&self, card_id: &str) -> bool {
        self.unlocked_cards.contains(card_id)
    }

    /// SOW-020: Unlock a card (called after purchase)
    pub fn unlock_card(&mut self, card_id: &str) {
        self.unlocked_cards.insert(card_id.to_string());
    }

    /// SOW-020: Check if a shop location is unlocked
    pub fn is_location_unlocked(&self, location_id: &str) -> bool {
        self.unlocked_locations.contains(location_id)
    }

    /// SOW-020: Unlock a shop location
    pub fn unlock_location(&mut self, location_id: &str) {
        self.unlocked_locations.insert(location_id.to_string());
    }

    /// SOW-024: Buy an area with global cash. Error strings are display-ready.
    pub fn purchase_location(&mut self, location_id: &str, price: u64) -> Result<(), &'static str> {
        if self.is_location_unlocked(location_id) {
            return Err("already unlocked");
        }
        if !self.spend(price) {
            return Err("insufficient funds");
        }
        self.unlock_location(location_id);
        Ok(())
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

    // ---- RFC-023: kingpin, dealers, jail, bail, hiring ----

    #[test]
    fn test_fresh_empire_starts_with_the_kingpin() {
        let data = SaveData::new();
        assert_eq!(data.dealers.len(), 1);
        assert_eq!(data.active_dealer, 0);
        assert!(data.dealers[0].is_kingpin);
        assert!(data.dealers[0].is_available());
        assert!(data.validate().is_ok());
    }

    #[test]
    fn test_jail_sentence_scales_with_heat() {
        assert_eq!(jail_sentence_from_heat(0), 1); // mandatory minimum
        assert_eq!(jail_sentence_from_heat(-20), 1); // cooling can't shorten it
        assert_eq!(jail_sentence_from_heat(24), 1);
        assert_eq!(jail_sentence_from_heat(25), 2);
        assert_eq!(jail_sentence_from_heat(100), 5);
    }

    #[test]
    fn test_full_sentence_zeroes_heat_and_scars_the_record() {
        let mut dealer = DealerState::recruit(&[]);
        dealer.character.heat = 50; // sentence: 1 + 50/25 = 3 runs

        dealer.jail();
        assert_eq!(dealer.jail_remaining(), Some(3));

        assert!(!dealer.tick_sentence()); // 2 left
        assert!(!dealer.tick_sentence()); // 1 left
        assert_eq!(dealer.character.heat, 50); // unchanged while inside
        assert!(dealer.tick_sentence()); // released

        assert!(dealer.is_available());
        assert_eq!(dealer.character.heat, 0); // full serve -> full reduction
        assert_eq!(dealer.prior_convictions, 1); // the record remains
    }

    #[test]
    fn test_bail_reduction_is_proportional_to_time_served() {
        let mut data = SaveData::new();
        data.account.cash_on_hand = 10_000;
        assert!(data.hire_dealer());
        data.dealers[1].character.heat = 100; // sentence: 5 runs
        data.dealers[1].jail();

        // Serve 2 of 5 runs (ticked by other dealers' completed runs)
        data.complete_run_tick(0);
        data.complete_run_tick(0);
        assert_eq!(data.dealers[1].jail_remaining(), Some(3));

        // Bail: $300 x 3 remaining
        let cash_before = data.account.cash_on_hand;
        assert!(data.bail_out(1));
        assert_eq!(data.account.cash_on_hand, cash_before - 900);
        assert!(data.dealers[1].is_available());
        // Served 2/5 -> reduction 40 of 100 -> walks out at heat 60
        assert_eq!(data.dealers[1].character.heat, 60);
        assert_eq!(data.dealers[1].prior_convictions, 1);

        // Bailing an available dealer is refused
        assert!(!data.bail_out(1));
    }

    #[test]
    fn test_run_tick_excludes_the_runner_and_releases_at_zero() {
        let mut data = SaveData::new();
        data.account.cash_on_hand = 10_000;
        assert!(data.hire_dealer());
        data.dealers[1].character.heat = 10; // sentence: 1 run
        data.dealers[1].jail();

        // The just-jailed dealer's own run must not count toward the sentence
        let released = data.complete_run_tick(1);
        assert!(released.is_empty());
        assert_eq!(data.dealers[1].jail_remaining(), Some(1));

        // Someone else runs: sentence served, auto-release
        let released = data.complete_run_tick(0);
        assert_eq!(released, vec![data.dealers[1].name.clone()]);
        assert!(data.dealers[1].is_available());
    }

    #[test]
    fn test_hire_cost_doubles_with_roster_size() {
        assert_eq!(hire_cost(1), 500); // kingpin only -> first hire
        assert_eq!(hire_cost(2), 1000);
        assert_eq!(hire_cost(3), 2000);
        assert_eq!(hire_cost(4), 4000);
        assert_eq!(bail_cost(3), 900);
    }

    #[test]
    fn test_hire_dealer_spends_and_recruits_unique_identity() {
        let mut data = SaveData::new();
        data.account.cash_on_hand = 1400;

        // First hire: $500
        assert!(data.hire_dealer());
        assert_eq!(data.dealers.len(), 2);
        assert_eq!(data.account.cash_on_hand, 900);
        assert!(!data.dealers[1].is_kingpin);
        assert_ne!(data.dealers[0].name, data.dealers[1].name);
        assert_ne!(data.dealers[0].portrait, data.dealers[1].portrait);

        // Second hire: $1000 > $900 - refused, no mutation
        assert!(!data.hire_dealer());
        assert_eq!(data.dealers.len(), 2);
        assert_eq!(data.account.cash_on_hand, 900);
    }

    #[test]
    fn test_kingpin_bust_resets_the_empire() {
        let mut data = SaveData::new();
        data.account.cash_on_hand = 50_000;
        assert!(data.hire_dealer());
        data.active_character_mut().heat = 90;

        data.reset_empire();
        assert_eq!(data.dealers.len(), 1);
        assert!(data.dealers[0].is_kingpin);
        assert_eq!(data.active_character().heat, 0);
        assert_eq!(data.account.cash_on_hand, AccountState::new().cash_on_hand);
    }

    #[test]
    fn test_roster_roundtrip_with_jailed_dealer() {
        let mut data = SaveData::new();
        data.account.cash_on_hand = 500;
        assert!(data.hire_dealer());
        data.dealers[1].character.heat = 40;
        data.dealers[1].jail();
        data.active_dealer = 0;

        // Serialize through the same path the save file uses
        let bytes = bincode::serialize(&data).unwrap();
        let loaded: SaveData = bincode::deserialize(&bytes).unwrap();
        assert_eq!(loaded.dealers.len(), 2);
        assert_eq!(
            loaded.dealers[1].status,
            DealerStatus::Jailed { runs_remaining: 2, sentence_total: 2, heat_at_bust: 40 }
        );
        assert!(loaded.validate().is_ok());
    }

    #[test]
    fn test_validate_rejects_bad_active_index_and_jailed_kingpin() {
        let mut data = SaveData::new();
        data.active_dealer = 3;
        assert!(data.validate().is_err());

        let mut data = SaveData::new();
        data.dealers[0].status = DealerStatus::Jailed {
            runs_remaining: 1,
            sentence_total: 1,
            heat_at_bust: 0,
        };
        assert!(data.validate().is_err());
    }

    #[test]
    fn test_epitaph_summarizes_roster() {
        let mut save = SaveData::new();
        save.account.lifetime_revenue = 12_400;
        save.account.cash_on_hand = 900;
        save.dealers[0].character.decks_played = 5;
        save.dealers[0].character.story_history.push("The boss's tale".into());
        let mut hired = DealerState::recruit(&save.dealers);
        hired.prior_convictions = 2;
        hired.character.decks_played = 4;
        hired.character.story_history.push("Slim's tale".into());
        save.dealers.push(hired);

        let epitaph = EmpireEpitaph::from_save(&save, 1234);
        assert_eq!(epitaph.ended_at, 1234);
        assert_eq!(epitaph.lifetime_revenue, 12_400);
        assert_eq!(epitaph.cash_at_fall, 900);
        assert_eq!(epitaph.dealers_hired, 1);
        assert_eq!(epitaph.total_prior_convictions, 2);
        assert_eq!(epitaph.decks_played, 9);
        assert_eq!(epitaph.stories.len(), 2); // full ledger archived
    }

    #[test]
    fn test_fallen_empires_survive_reset_and_accumulate() {
        let mut save = SaveData::new();
        save.account.lifetime_revenue = 1000;
        save.reset_empire();
        assert_eq!(save.fallen_empires.len(), 1);
        assert_eq!(save.fallen_empires[0].lifetime_revenue, 1000);
        // Fresh empire otherwise
        assert_eq!(save.dealers.len(), 1);
        assert!(save.dealers[0].is_kingpin);
        assert_eq!(save.account.lifetime_revenue, 0);

        save.account.lifetime_revenue = 5000;
        save.reset_empire();
        assert_eq!(save.fallen_empires.len(), 2); // the board is forever
        assert_eq!(save.fallen_empires[1].lifetime_revenue, 5000);
    }

    #[test]
    fn test_leaderboard_top_sorted_by_revenue() {
        let mut save = SaveData::new();
        for revenue in [300u64, 900, 100, 600] {
            save.account.lifetime_revenue = revenue;
            save.reset_empire();
        }
        let top = leaderboard_top(&save.fallen_empires, 3);
        let revenues: Vec<u64> = top.iter().map(|i| save.fallen_empires[*i].lifetime_revenue).collect();
        assert_eq!(revenues, vec![900, 600, 300]);
        // latest entry (index 3, revenue 600) can be identified for the marker
        assert!(top.contains(&3));
    }

    #[test]
    fn test_purchase_location() {
        // SOW-024: buy once, double-buy rejected, insufficient funds rejected
        let mut account = AccountState::new();
        account.cash_on_hand = 2500;

        assert!(!account.is_location_unlocked("the_block"));
        assert_eq!(account.purchase_location("the_block", 2000), Ok(()));
        assert!(account.is_location_unlocked("the_block"));
        assert_eq!(account.cash_on_hand, 500);

        // Double purchase: rejected, cash untouched
        assert_eq!(account.purchase_location("the_block", 2000), Err("already unlocked"));
        assert_eq!(account.cash_on_hand, 500);

        // Unaffordable: rejected, nothing unlocked
        assert_eq!(account.purchase_location("downtown", 2000), Err("insufficient funds"));
        assert!(!account.is_location_unlocked("downtown"));
        assert_eq!(account.cash_on_hand, 500);
    }

    #[test]
    fn test_empire_reset_wipes_area_unlocks() {
        // SOW-024: a fallen empire re-expands - unlocks die with the account
        let mut save = SaveData::new();
        save.account.cash_on_hand = 5000;
        save.account.purchase_location("the_block", 2000).unwrap();
        assert!(save.account.is_location_unlocked("the_block"));

        save.reset_empire();
        assert!(!save.account.is_location_unlocked("the_block"));
        assert!(save.account.is_location_unlocked("the_corner")); // fresh default
    }

    #[test]
    fn test_move_dealer_costs_fee_and_downtime() {
        // SOW-025: relocation = cash + 1 run of unavailability
        let mut save = SaveData::new();
        save.account.cash_on_hand = 1000;

        assert!(save.move_dealer(0, "the_block"));
        assert_eq!(save.account.cash_on_hand, 750); // $250 fee
        assert_eq!(save.dealers[0].station, "the_block"); // station changes immediately
        assert_eq!(save.dealers[0].relocating_remaining(), Some(1));
        assert!(!save.dealers[0].is_available()); // can't be sent out mid-move

        // One completed run elsewhere -> arrived
        assert!(save.dealers[0].tick_sentence());
        assert!(save.dealers[0].is_available());
        assert_eq!(save.dealers[0].station, "the_block");
    }

    #[test]
    fn test_move_dealer_rejections() {
        let mut save = SaveData::new();
        save.account.cash_on_hand = 100; // can't afford the $250 fee
        assert!(!save.move_dealer(0, "the_block"));
        assert_eq!(save.dealers[0].station, DEFAULT_STATION);

        save.account.cash_on_hand = 1000;
        assert!(!save.move_dealer(0, DEFAULT_STATION)); // already stationed there
        assert!(!save.move_dealer(9, "the_block")); // out of range

        assert!(save.move_dealer(0, "the_block"));
        assert!(!save.move_dealer(0, "the_corner")); // mid-relocation = unavailable
        assert_eq!(save.account.cash_on_hand, 750); // only one fee charged
    }

    #[test]
    fn test_street_cred_accrues_and_never_decays() {
        // SOW-025: +1 per successful deal; nothing (jail, moves) erases it
        let mut dealer = DealerState::recruit(&[]);
        dealer.add_cred("the_block");
        dealer.add_cred("the_block");
        dealer.add_cred("the_corner");
        assert_eq!(dealer.cred_in("the_block"), 2);
        assert_eq!(dealer.cred_in("the_corner"), 1);
        assert_eq!(dealer.cred_in("nowhere"), 0);

        // Jail round-trip: cred untouched
        dealer.character.heat = 50;
        dealer.jail();
        dealer.release();
        assert_eq!(dealer.cred_in("the_block"), 2);

        // Move: cred untouched (reputation, not presence)
        let mut save = SaveData::new();
        save.account.cash_on_hand = 1000;
        save.dealers.push(dealer);
        assert!(save.move_dealer(1, "the_block"));
        assert_eq!(save.dealers[1].cred_in("the_block"), 2);
    }

    #[test]
    fn test_best_cred_names_the_unlocking_dealer() {
        // SOW-025: any dealer's cred opens the door; the highest is credited
        let mut save = SaveData::new();
        assert!(save.best_cred("the_block").is_none()); // nobody known yet

        save.dealers.push(DealerState::recruit(&save.dealers));
        save.dealers[0].add_cred("the_block");
        save.dealers[1].add_cred("the_block");
        save.dealers[1].add_cred("the_block");
        let (idx, cred) = save.best_cred("the_block").unwrap();
        assert_eq!((idx, cred), (1, 2));
    }

    #[test]
    fn test_kingpin_can_relocate_but_never_jail() {
        let mut save = SaveData::new();
        save.account.cash_on_hand = 1000;
        assert!(save.move_dealer(0, "the_block"));
        assert!(save.validate().is_ok()); // relocating kingpin is legal

        let mut jailed_kingpin = DealerState::kingpin();
        jailed_kingpin.status = DealerStatus::Jailed {
            runs_remaining: 1,
            sentence_total: 1,
            heat_at_bust: 10,
        };
        assert!(jailed_kingpin.validate().is_err()); // jailed kingpin is not
    }

    #[test]
    fn test_empire_reset_returns_stations_and_wipes_cred() {
        let mut save = SaveData::new();
        save.account.cash_on_hand = 1000;
        save.dealers[0].add_cred("the_block");
        assert!(save.move_dealer(0, "the_block"));

        save.reset_empire();
        assert_eq!(save.dealers[0].station, DEFAULT_STATION);
        assert_eq!(save.dealers[0].cred_in("the_block"), 0);
        assert!(save.dealers[0].is_available());
    }

    #[test]
    fn test_apply_session_heat_signed_with_floor() {
        // SOW-022 follow-up: session heat is signed; a cooling run reduces
        // career heat, floored at 0
        let mut state = CharacterState::new();
        state.heat = 50;
        state.apply_session_heat(20);
        assert_eq!(state.heat, 70);
        state.apply_session_heat(-30);
        assert_eq!(state.heat, 40);
        state.apply_session_heat(-100);
        assert_eq!(state.heat, 0); // floors, never wraps
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
    fn test_decay_not_reapplied_on_repeat_calls() {
        // SOW-021: OnEnter(DeckBuilding) fires more than once per launch
        // (UpgradeChoice round trips) - decay must consume its window
        let mut state = CharacterState::new();
        state.heat = 100;
        state.last_played = current_timestamp().saturating_sub(10 * 3600); // 10h ago

        assert_eq!(state.apply_decay(), 10);
        assert_eq!(state.heat, 90);

        // Second call in the same session decays nothing further
        assert_eq!(state.apply_decay(), 0);
        assert_eq!(state.heat, 90);
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
        let data = SaveData::new();

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

        assert!(data.validate().is_ok());
        assert_eq!(data.account.cash_on_hand, 500);
    }

    // ========================================================================
    // UpgradeTier Tests (RFC-017)
    // ========================================================================

    #[test]
    fn test_upgrade_tier_from_play_count() {
        // SOW-021: Spec thresholds (card-system.md): 0/3/8/15/25/40
        assert_eq!(UpgradeTier::from_play_count(0), UpgradeTier::Base);
        assert_eq!(UpgradeTier::from_play_count(2), UpgradeTier::Base);
        assert_eq!(UpgradeTier::from_play_count(3), UpgradeTier::Tier1);
        assert_eq!(UpgradeTier::from_play_count(7), UpgradeTier::Tier1);
        assert_eq!(UpgradeTier::from_play_count(8), UpgradeTier::Tier2);
        assert_eq!(UpgradeTier::from_play_count(14), UpgradeTier::Tier2);
        assert_eq!(UpgradeTier::from_play_count(15), UpgradeTier::Tier3);
        assert_eq!(UpgradeTier::from_play_count(24), UpgradeTier::Tier3);
        assert_eq!(UpgradeTier::from_play_count(25), UpgradeTier::Tier4);
        assert_eq!(UpgradeTier::from_play_count(39), UpgradeTier::Tier4);
        assert_eq!(UpgradeTier::from_play_count(40), UpgradeTier::Tier5);
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
        // SOW-021: Spec thresholds (absolute play counts for the next tier)
        assert_eq!(UpgradeTier::Base.plays_to_next(), Some(3));
        assert_eq!(UpgradeTier::Tier1.plays_to_next(), Some(8));
        assert_eq!(UpgradeTier::Tier2.plays_to_next(), Some(15));
        assert_eq!(UpgradeTier::Tier3.plays_to_next(), Some(25));
        assert_eq!(UpgradeTier::Tier4.plays_to_next(), Some(40));
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
    fn test_apply_upgrade_choice_for_any_order() {
        // SOW-021: batched upgrade screen resolves pending upgrades in any order
        let mut state = CharacterState::new();
        let product = crate::models::card::CardType::Product { price: 30, heat: 5 };

        state.pending_upgrades.push(PendingUpgrade {
            card_name: "Card A".to_string(),
            card_type: product.clone(),
            tier: UpgradeTier::Tier1,
            options: [UpgradeableStat::Price, UpgradeableStat::Heat],
        });
        state.pending_upgrades.push(PendingUpgrade {
            card_name: "Card B".to_string(),
            card_type: product,
            tier: UpgradeTier::Tier1,
            options: [UpgradeableStat::Price, UpgradeableStat::Heat],
        });

        // Resolve the SECOND pending first
        assert!(state.apply_upgrade_choice_for("Card B", UpgradeableStat::Heat));
        assert_eq!(state.pending_upgrades.len(), 1);
        assert_eq!(state.pending_upgrades[0].card_name, "Card A");
        assert!(state.card_upgrades.contains_key("Card B"));

        // Stat not among the card's offered options is rejected
        assert!(!state.apply_upgrade_choice_for("Card A", UpgradeableStat::Cover));
        assert_eq!(state.pending_upgrades.len(), 1);

        // Unknown card is rejected
        assert!(!state.apply_upgrade_choice_for("Card C", UpgradeableStat::Price));

        // Resolve the remaining card
        assert!(state.apply_upgrade_choice_for("Card A", UpgradeableStat::Price));
        assert!(state.pending_upgrades.is_empty());
    }

    #[test]
    fn test_character_get_card_tier() {
        let mut state = CharacterState::new();

        // SOW-021: Spec thresholds (0/3/8/15/25/40)
        // No plays = Base tier
        assert_eq!(state.get_card_tier("Test Card"), UpgradeTier::Base);

        // 2 plays still Base; 3rd play reaches Tier 1
        state.increment_play_count("Test Card");
        state.increment_play_count("Test Card");
        assert_eq!(state.get_card_tier("Test Card"), UpgradeTier::Base);
        state.increment_play_count("Test Card");
        assert_eq!(state.get_card_tier("Test Card"), UpgradeTier::Tier1);

        // 8 plays = Tier 2
        for _ in 0..5 {
            state.increment_play_count("Test Card");
        }
        assert_eq!(state.get_card_tier("Test Card"), UpgradeTier::Tier2);

        // 40 plays = Tier 5 (max); more plays stay at max
        for _ in 0..32 {
            state.increment_play_count("Test Card");
        }
        assert_eq!(state.get_card_tier("Test Card"), UpgradeTier::Tier5);
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
