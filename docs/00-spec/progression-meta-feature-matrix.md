# Progression & Meta-Game - Feature Matrix

**Spec:** [progression-meta.md](progression-meta.md)
**Last Updated:** 2026-07-13
**Overall Status:** 41/49 features complete (84%)

---

## Summary

| Category | Complete | Total | % |
|----------|:--------:|:-----:|:-:|
| Cash System | 5 | 5 | 100% |
| Location System | 10 | 10 | 100% |
| Card Unlock System | 6 | 6 | 100% |
| Supplier System | 5 | 5 | 100% |
| Per-Run Card Upgrades | 7 | 7 | 100% |
| Character System | 3 | 7 | 43% |
| Achievements | 0 | 4 | 0% |
| History & Legacy | 5 | 5 | 100% |
| **Total** | **41** | **49** | **84%** |

(Summary repaired 2026-07-13: History & Legacy existed as a section since
SOW-030 but was missing from this table.)

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

## Location System - 10/10 (100%)

| Feature | Status | Notes |
|---------|:------:|-------|
| Starting location (The Corner) | ✅ | SOW-020 |
| Location as card shops | ✅ | SOW-020 |
| Area (territory) purchase unlocks | ✅ | SOW-024: cash purchase supersedes the achievement placeholder (RFC-024); buyers area-gated, two-stage run selection. SOW-028: three-zone city — The Strip $1,200 spliced between the free Corner and the $2,000 Block |
| Location permanence | ✅ | SOW-020 |
| Multi-location shopping | ✅ | SOW-020 |
| Location UI (shop interface) | ✅ | SOW-020 |
| Dealer stationing (runs happen where the dealer stands) | ✅ | SOW-025: station per dealer, move = $250 + 1-run downtime, replaces SOW-024's interim random pick |
| Street cred per dealer per area | ✅ | SOW-025: +1 per Safe deal in the run's area, never decays; roster card shows "STATION · CRED n" |
| Cred-gated shop stock | ✅ | SOW-025: `shop_cred_required` in card RON (pilots: Storage Unit 3, Heroin 5); roster's best cred opens the door, shop credits the dealer by name |
| City map screen (area picker + move UI) | ✅ | SOW-029: full-screen overlay with three zone node cards (clientele + payout band, products, narc hint, stationed-dealer chips, ★ best cred); zone unlocks and dealer relocation both live on the map |

---

## Card Unlock System - 6/6 (100%)

| Feature | Status | Notes |
|---------|:------:|-------|
| Starting collection | ✅ | SOW-026: lean 8-card start (Weed the only product); shop ladder carries everything else with cash+cred requirements |
| Location-specific card pools | ✅ | SOW-020 |
| Cash purchase of cards | ✅ | SOW-020 |
| Permanent card unlocks | ✅ | SOW-020 |
| Card pricing tiers | ✅ | SOW-020 |
| Card purchase UI | ✅ | SOW-020 |

---

## Supplier System - 5/5 (100%)

| Feature | Status | Notes |
|---------|:------:|-------|
| Named supplier per zone | ✅ | SOW-031: authored in shop_locations.ron with voice (Lil Smoke / Miss Velvet / The Broker); header on each zone's shop tab |
| Fronts against product cards | ✅ | SOW-031: take now at 25% vig, lump due in 4 runs on the run ticker (runner's own run counts); card playable while fronted, owned forever on payoff |
| Escalation ladder | ✅ | SOW-031: CutOff (stock locked, one final window) → muscle (20% cash seizure, or active dealer benched 1 run when broke) + repossession → Soured (permanent, cash only) |
| Front visibility surfaces | ✅ | SOW-031: hub clock next to START RUN, PAY on the shop header, supplier line on map nodes, OWED TO SUPPLIERS on the ledger strip |
| Zone flavor authored in content | ✅ | SOW-031 (SOW-029 carry): identity + narc_hint moved from code into shop_locations.ron, required at load |

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

## History & Legacy (P5) - 5/5 (100%)

| Feature | Status | Notes |
|---------|:------:|-------|
| Story history per dealer | ✅ | CharacterState.story_history (narrative engine) |
| Empire epitaphs (fallen-empires archive) | ✅ | SOW-023: EmpireEpitaph frozen at kingpin bust, survives reset, stories archived |
| Kingpin Ledger overlay | ✅ | SOW-030: empire strip + roster dossiers + fallen board, stories one click deeper; everything DERIVED from existing save state (zero schema changes) |
| Arcade board browsable outside game over | ✅ | SOW-030: living empire slotted unranked "IN PROGRESS" at its would-be rank; ties go to the dead record |
| Per-zone history on the city map | ✅ | SOW-030: "n deals closed · best: <dealer>" derived from street cred, same numbers as the ledger |

---

## Scrapped Features

| Feature | Reason |
|---------|--------|
| Leaderboards (Flash/Kingpin/Survivor) | Superseded: the SOW-023/030 fallen-empires arcade board (lifetime revenue) is the leaderboard |
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
- ~~Location System~~ ✅ (9/9 complete - SOW-024 areas + SOW-025 stationing/cred)
- ~~Card Unlock System~~ ✅ (6/6 complete)
- Achievements (4 features)

**Priority 2 - Character Variety:**
- Character slots (1 feature)
- Character profiles (2 features)
- Character selection UI (1 feature)
