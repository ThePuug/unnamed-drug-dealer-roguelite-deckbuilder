# Progression & Meta-Game - Feature Matrix

**Spec:** [progression-meta.md](progression-meta.md)
**Last Updated:** 2025-11-29
**Overall Status:** 26/35 features complete (74%)

---

## Summary

| Category | Complete | Total | % |
|----------|:--------:|:-----:|:-:|
| Cash System | 5 | 5 | 100% |
| Location System | 5 | 6 | 83% |
| Card Unlock System | 6 | 6 | 100% |
| Per-Run Card Upgrades | 7 | 7 | 100% |
| Character System | 3 | 7 | 43% |
| Achievements | 0 | 4 | 0% |
| **Total** | **26** | **35** | **74%** |

---

## Cash System - 5/5 (100%)

| Feature | Status | Notes |
|---------|:------:|-------|
| Cash on hand tracking | ✅ | AccountState.cash_on_hand |
| Cash persistence (account-wide) | ✅ | Survives permadeath |
| Cash earning (from deals) | ✅ | Add profit on Safe outcome |
| Cash spending (at locations) | ✅ | SOW-020 |
| Lifetime revenue metric | ✅ | AccountState.lifetime_revenue |

---

## Location System - 5/6 (83%)

| Feature | Status | Notes |
|---------|:------:|-------|
| Starting location (The Corner) | ✅ | SOW-020 |
| Location as card shops | ✅ | SOW-020 |
| Achievement-gated unlocks | ❌ | Deferred to future RFC |
| Location permanence | ✅ | SOW-020 |
| Multi-location shopping | ✅ | SOW-020 |
| Location UI (shop interface) | ✅ | SOW-020 |

---

## Card Unlock System - 6/6 (100%)

| Feature | Status | Notes |
|---------|:------:|-------|
| Starting collection | ✅ | SOW-020 |
| Location-specific card pools | ✅ | SOW-020 |
| Cash purchase of cards | ✅ | SOW-020 |
| Permanent card unlocks | ✅ | SOW-020 |
| Card pricing tiers | ✅ | SOW-020 |
| Card purchase UI | ✅ | SOW-020 |

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
- ~~Location System~~ ✅ (5/6 complete, achievement-gating deferred)
- ~~Card Unlock System~~ ✅ (6/6 complete)
- Achievements (4 features)

**Priority 2 - Character Variety:**
- Character slots (1 feature)
- Character profiles (2 features)
- Character selection UI (1 feature)
