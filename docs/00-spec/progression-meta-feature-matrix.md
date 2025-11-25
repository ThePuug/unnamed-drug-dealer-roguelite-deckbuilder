# Progression & Meta-Game - Feature Matrix

Implementation tracking for Progression & Meta-Game specification.

**Spec:** [progression-meta.md](progression-meta.md)

**Last Updated:** 2025-11-25

---

## Summary

**Overall Completion:** 9/38 features (24%)

| Category | Complete | Partial | Not Started | Deferred |
|----------|----------|---------|-------------|----------|
| Cash System | 4 | 0 | 1 | 0 |
| Location System | 0 | 0 | 6 | 0 |
| Narc Variety | 0 | 0 | 0 | 1 |
| Card Unlock System | 0 | 0 | 6 | 0 |
| Per-Run Card Upgrades | 5 | 0 | 2 | 0 |
| Character System | 0 | 0 | 7 | 0 |
| Achievements | 0 | 0 | 4 | 0 |
| Leaderboards | 0 | 0 | 0 | 2 |
| **Total** | **9** | **0** | **26** | **3** |

---

## Cash System: 4/5 complete (80%)

| Feature | Status | RFC/ADR | Notes |
|---------|:------:|---------|-------|
| Cash on hand tracking | ✅ | RFC-016 | AccountState.cash_on_hand in save system |
| Cash persistence (account-wide) | ✅ | RFC-016 | Survives permadeath via account state (separate from character) |
| Cash earning (from deals) | ✅ | RFC-016 | Add profit to account on Safe outcome |
| Cash spending (at locations) | ❌ | - | Deferred to location unlock RFC |
| Revenue metric (separate) | ✅ | RFC-016 | AccountState.lifetime_revenue (never decreases) |

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

## Per-Run Card Upgrades: 5/7 complete (71%)

| Feature | Status | RFC/ADR | Notes |
|---------|:------:|---------|-------|
| Card play tracking (per character) | ✅ | RFC-017 | Counts stored in CharacterState.card_play_counts |
| Upgrade thresholds | ✅ | RFC-017 | MVP: Tier 1 at 5 plays (+10% primary stat) |
| Upgrade application | ✅ | RFC-017 | Applied in calculate_totals() during play |
| Upgrade display on cards | ✅ | RFC-017 | Infrastructure added (UpgradeInfo passed to card render) |
| Upgrade reset on permadeath | ✅ | RFC-017 | Play counts in CharacterState, lost when character dies |
| Upgrade notification | ❌ | - | Deferred - "Burner Phone upgraded to Tier 2!" |
| Upgrade feedback in UI | ❌ | - | Deferred - Show upgraded stats vs base stats |

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

**RFC-016 (Account Cash System):**
- Cash display added to deck builder UI (shows cash on hand and lifetime revenue)
- No separate UI for cash earned "this hand" - only account totals displayed
- Profit earned in final hand before permadeath still added to account (cash survives death)

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
