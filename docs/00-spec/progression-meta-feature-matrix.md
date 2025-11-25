# Progression & Meta-Game - Feature Matrix

Implementation tracking for Progression & Meta-Game specification.

**Spec:** [progression-meta.md](progression-meta.md)

**Last Updated:** 2025-11-25

---

## Summary

**Overall Completion:** 0/38 features (0%)

| Category | Complete | Partial | Not Started | Deferred |
|----------|----------|---------|-------------|----------|
| Cash System | 0 | 0 | 5 | 0 |
| Location System | 0 | 0 | 6 | 0 |
| Narc Variety | 0 | 0 | 0 | 1 |
| Card Unlock System | 0 | 0 | 6 | 0 |
| Per-Run Card Upgrades | 0 | 0 | 7 | 0 |
| Character System | 0 | 0 | 7 | 0 |
| Achievements | 0 | 0 | 4 | 0 |
| Leaderboards | 0 | 0 | 0 | 2 |
| **Total** | **0** | **0** | **35** | **3** |

---

## Cash System: 0/5 complete (0%)

| Feature | Status | RFC/ADR | Notes |
|---------|:------:|---------|-------|
| Cash on hand tracking | ❌ | - | Single account-wide pool |
| Cash persistence (account-wide) | ❌ | - | Survives permadeath, shared across characters |
| Cash earning (from deals) | ❌ | - | Add to pool when completing hands |
| Cash spending (at locations) | ❌ | - | Deduct when purchasing card unlocks |
| Revenue metric (separate) | ❌ | - | Tracks lifetime earned (for leaderboards), unaffected by spending |

---

## Location System: 0/6 complete (0%)

| Feature | Status | RFC/ADR | Notes |
|---------|:------:|---------|-------|
| Starting location (The Corner) | ❌ | - | Unlocked by default, basic card pool |
| Location as card shops | ❌ | - | Each location offers unique cards for purchase |
| Achievement-gated location unlocks | ❌ | - | Locations unlock via specific achievements |
| Location permanence | ❌ | - | Once unlocked, always accessible |
| Multi-location shopping | ❌ | - | Can buy from any unlocked location |
| Location UI (shop interface) | ❌ | - | Browse available cards, prices, purchase |

---

## Narc Variety System: 0/1 complete (TBD)

| Feature | Status | RFC/ADR | Notes |
|---------|:------:|---------|-------|
| Narc variety from locations | ⏸️ | - | TBD - Playtest to determine approach (profiles vs pool vs single) |

**MVP:** Single Narc profile. Heat controls difficulty via card upgrades. Variety deferred.

---

## Card Unlock System: 0/6 complete (0%)

| Feature | Status | RFC/ADR | Notes |
|---------|:------:|---------|-------|
| Starting collection (~15-20 cards) | ❌ | - | Basic products, locations, cover, modifiers |
| Location-specific card pools | ❌ | - | Each card available at one location |
| Cash purchase of cards | ❌ | - | Spend cash on hand to unlock |
| Permanent card unlocks | ❌ | - | Never lost, account-wide |
| Card pricing tiers | ❌ | - | $500-$1.5k (basic) to $20k+ (elite) |
| Card purchase UI | ❌ | - | Show card details, price, confirm purchase |

---

## Per-Run Card Upgrades: 0/7 complete (0%)

| Feature | Status | RFC/ADR | Notes |
|---------|:------:|---------|-------|
| Card play tracking (per character) | ❌ | - | Count times each card played |
| Upgrade thresholds | ❌ | - | Tier 1: 5 plays, Tier 2: 12, Tier 3: 25, Tier 4: 50 |
| Upgrade application | ❌ | - | Apply stat bonuses when threshold reached |
| Upgrade display on cards | ❌ | - | Show current tier and progress |
| Upgrade reset on permadeath | ❌ | - | All upgrades lost when character dies |
| Upgrade notification | ❌ | - | "Burner Phone upgraded to Tier 2!" |
| Upgrade feedback in UI | ❌ | - | Show upgraded stats vs base stats |

---

## Character System: 0/7 complete (0%)

| Feature | Status | RFC/ADR | Notes |
|---------|:------:|---------|-------|
| Single starting character slot | ❌ | - | One active character initially |
| Character slot unlocks | ❌ | - | Additional slots via achievements |
| Character profiles | ❌ | - | College Student, Widow, Cancer Patient, Mafia Member |
| Narrative framing (no mechanics) | ❌ | - | Profiles provide context only |
| Character permadeath | ❌ | - | Trigger on bust, character lost forever |
| Permadeath consequences | ❌ | - | Lose Heat, card upgrades; keep cash, unlocks |
| Character selection UI | ❌ | - | Choose profile for new character |

---

## Achievements: 0/4 complete (0%)

| Feature | Status | RFC/ADR | Notes |
|---------|:------:|---------|-------|
| Achievement tracking | ❌ | - | Track progress toward each achievement |
| Location unlock achievements | ❌ | - | Survive X decks, earn $Xk lifetime |
| Character slot unlock achievements | ❌ | - | Complete run, survive X decks total |
| Achievement notification | ❌ | - | "Achievement unlocked: Street Cred - The Block now available!" |

---

## Leaderboards: 0/0 complete (Deferred)

| Feature | Status | RFC/ADR | Notes |
|---------|:------:|---------|-------|
| Flash leaderboard (7-day revenue) | ⏸️ | - | Deferred to future phase |
| Kingpin/Survivor leaderboards | ⏸️ | - | Deferred to future phase |

---

## Implementation Deviations

_No implementations yet._

---

## Notes

- **Trust system removed** - Per-run card upgrades replace Trust as positive run progression
- Cash is ACCOUNT-WIDE (permanent, shared across characters, survives permadeath)
- Card unlocks are ACCOUNT-WIDE and PERMANENT
- Card upgrades are PER-CHARACTER and lost on permadeath
- Narc variety from locations TBD (playtest to determine approach)
- Leaderboards deferred - track revenue metric (not cash on hand)
- MVP Phase 1: Cash system, 2 locations, basic card upgrades (2 tiers)
- MVP Phase 2: 4 locations, character slots, full upgrades (4 tiers)
- MVP Phase 3: All locations, full card collection
