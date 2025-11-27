# Progression & Meta-Game - Feature Matrix

**Spec:** [progression-meta.md](progression-meta.md)
**Last Updated:** 2025-11-27
**Overall Status:** 14/35 features complete (40%)

---

## Summary

| Category | Complete | Total | % |
|----------|:--------:|:-----:|:-:|
| Cash System | 4 | 5 | 80% |
| Location System | 0 | 6 | 0% |
| Card Unlock System | 0 | 6 | 0% |
| Per-Run Card Upgrades | 7 | 7 | 100% |
| Character System | 3 | 7 | 43% |
| Achievements | 0 | 4 | 0% |
| **Total** | **14** | **35** | **40%** |

---

## Cash System - 4/5 (80%)

| Feature | Status | Notes |
|---------|:------:|-------|
| Cash on hand tracking | ✅ | AccountState.cash_on_hand |
| Cash persistence (account-wide) | ✅ | Survives permadeath |
| Cash earning (from deals) | ✅ | Add profit on Safe outcome |
| Cash spending (at locations) | 🔄 | SOW-020 planned |
| Lifetime revenue metric | ✅ | AccountState.lifetime_revenue |

---

## Location System - 0/6 (0%)

| Feature | Status | Notes |
|---------|:------:|-------|
| Starting location (The Corner) | 🔄 | SOW-020 planned |
| Location as card shops | 🔄 | SOW-020 planned |
| Achievement-gated unlocks | ❌ | Deferred to future RFC |
| Location permanence | 🔄 | SOW-020 planned |
| Multi-location shopping | 🔄 | SOW-020 planned |
| Location UI (shop interface) | 🔄 | SOW-020 planned |

---

## Card Unlock System - 0/6 (0%)

| Feature | Status | Notes |
|---------|:------:|-------|
| Starting collection | 🔄 | SOW-020 planned |
| Location-specific card pools | 🔄 | SOW-020 planned |
| Cash purchase of cards | 🔄 | SOW-020 planned |
| Permanent card unlocks | 🔄 | SOW-020 planned |
| Card pricing tiers | 🔄 | SOW-020 planned |
| Card purchase UI | 🔄 | SOW-020 planned |

---

## Per-Run Card Upgrades - 7/7 (100%)

| Feature | Status | Notes |
|---------|:------:|-------|
| Card play tracking | ✅ | card_play_counts in HandState |
| Upgrade thresholds | ✅ | 6 tiers: 0, 3, 8, 15, 25, 40 plays |
| Upgrade application | ✅ | +10% per tier in calculate_totals() |
| Upgrade display on cards | ✅ | ★ badge with tier color |
| Upgrade reset on permadeath | ✅ | CharacterState deleted, counts lost |
| Upgrade notification | ✅ | GameState::UpgradeChoice UI |
| Upgrade stat choice | ✅ | Player picks stat to upgrade (RFC-019) |

---

## Character System - 3/7 (43%)

| Feature | Status | Notes |
|---------|:------:|-------|
| Single character slot | ✅ | CharacterState in SaveData |
| Character permadeath | ✅ | character = None on bust |
| Permadeath consequences | ✅ | Lose Heat/upgrades, keep cash/unlocks |
| Character slot unlocks | ❌ | Additional slots via achievements |
| Character profiles | ❌ | College Student, Widow, etc. |
| Narrative framing | ❌ | Profile-specific flavor text |
| Character selection UI | ❌ | Choose profile for new character |

---

## Achievements - 0/4 (0%)

| Feature | Status | Notes |
|---------|:------:|-------|
| Achievement tracking | ❌ | Track progress toward each |
| Location unlock achievements | ❌ | Survive X decks, earn $Xk |
| Character slot unlock achievements | ❌ | Complete run, survive X total |
| Achievement notification | ❌ | "Achievement unlocked!" UI |

---

## Scrapped Features

| Feature | Reason |
|---------|--------|
| Leaderboards (Flash/Kingpin/Survivor) | Deferred indefinitely |
| Narc variety from locations | Single Narc for MVP, Heat controls difficulty |

---

## Implementation Notes

- Cash system: `src/save/types.rs` (AccountState)
- Card upgrades: `src/systems/upgrade_choice.rs`, `src/save/types.rs` (UpgradeTier)
- Character state: `src/save/types.rs` (CharacterState)
- Permadeath: `src/systems/save_integration.rs:167`

---

## Remaining Work for Launch

**Priority 1 - Core Meta Loop:**
- Location System (6 features)
- Card Unlock System (6 features)
- Achievements (4 features)

**Priority 2 - Character Variety:**
- Character slots (1 feature)
- Character profiles (2 features)
- Character selection UI (1 feature)
