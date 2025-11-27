# Core Gameplay Loop - Feature Matrix

**Spec:** [core-gameplay-loop.md](core-gameplay-loop.md)
**Last Updated:** 2025-11-27
**Overall Status:** 48/48 features complete (100%)

---

## Summary

| Category | Complete | Total | % |
|----------|:--------:|:-----:|:-:|
| Run (Character Lifecycle) | 6 | 6 | 100% |
| Deck (Session) | 7 | 7 | 100% |
| Hand Flow | 12 | 12 | 100% |
| Round Flow | 8 | 8 | 100% |
| Visual Indicators | 10 | 10 | 100% |
| Special Conditions | 5 | 5 | 100% |
| **Total** | **48** | **48** | **100%** |

---

## Run (Character Lifecycle) - 6/6 (100%)

| Feature | Status | Notes |
|---------|:------:|-------|
| Character persistence | ✅ | CharacterState in SaveData |
| Total profit tracking | ✅ | AccountState.lifetime_revenue |
| Decks played counter | ✅ | CharacterState.decks_played |
| Heat persistence | ✅ | CharacterState.heat |
| Heat real-time decay | ✅ | apply_decay() from last_played |
| Permadeath on bust | ✅ | character = None on Busted |

---

## Deck (Session) - 7/7 (100%)

| Feature | Status | Notes |
|---------|:------:|-------|
| 20-card deck pool | ✅ | RFC-005 |
| Deck building UI | ✅ | RFC-006 |
| Session play (multiple hands) | ✅ | Card retention between hands |
| "Go Home" early option | ✅ | GoHomeButton |
| Deck exhaustion handling | ✅ | deck.len() < 3 triggers end |
| Card counter display | ✅ | "Deck: X" above betting buttons |
| Post-session summary | ✅ | Session totals in resolution overlay |

---

## Hand Flow - 12/12 (100%)

| Feature | Status | Notes |
|---------|:------:|-------|
| 3-round hand structure | ✅ | Player Phase → Buyer Reveal × 3 |
| Turn order | ✅ | Fixed Narc→Player |
| Sequential card play | ✅ | Face-up, immediate reveal |
| Check action | ✅ | Skip playing card |
| Buyer card reveals | ✅ | Random from 3 visible |
| Player fold | ✅ | Available during PlayerPhase |
| Buyer cannot fold | ✅ | Plays via reaction deck |
| Narc cannot fold | ✅ | Hardcoded |
| Running totals | ✅ | Updated after each card |
| End of hand resolution | ✅ | Validity/Bail/Demand/Bust |
| Buyer reaction deck | ✅ | 7 cards per persona, 3 visible |
| Card retention | ✅ | Unplayed cards carry over |

---

## Round Flow - 8/8 (100%)

| Feature | Status | Notes |
|---------|:------:|-------|
| Sequential turn-based play | ✅ | Fixed Narc→Player order |
| Play card face-up | ✅ | Immediate reveal |
| Check action | ✅ | Skip card this turn |
| Fold action | ✅ | Exit hand any round |
| Cards visible immediately | ✅ | No simultaneous flip |
| Running totals per card | ✅ | Not per round |
| Buyer reveal after Player | ✅ | Random from visible |
| Fold during player turn | ✅ | With Play/Check actions |

---

## Visual Indicators - 10/10 (100%)

| Feature | Status | Notes |
|---------|:------:|-------|
| Turn indicator | ✅ | "Turn: Player" in status |
| Card count per player | ✅ | Visible hand displays |
| Running totals display | ✅ | Evidence/Cover/Multiplier |
| Color-coded safety | ✅ | Heat bar green/yellow/red |
| Evidence gap display | ✅ | In resolution overlay |
| Heat accumulation | ✅ | Heat bar with threshold |
| Active Product highlight | ✅ | Active slot system |
| Active Location highlight | ✅ | Active slot system |
| Bust warning | ✅ | Real-time totals comparison |
| Danger indicator | ✅ | Narc tier badge (RFC-018) |

---

## Special Conditions - 5/5 (100%)

| Feature | Status | Notes |
|---------|:------:|-------|
| "Go Home" early | ✅ | GoHomeButton |
| Deck exhaustion detection | ✅ | deck.len() < 3 |
| Deck exhaustion warning | ✅ | "Deck Exhausted" message |
| Fold preserves cards | ✅ | Unplayed cards kept |
| Fold loses profit | ✅ | "No profit, no risk" |

---

## Scrapped Features

| Feature | Reason |
|---------|--------|
| Customer Trust system | Replaced by card upgrades (RFC-017/019) |
| Hand history/replay | Not planned for MVP |
| Undo last action | Not planned for MVP |
| Turn order indicator UI | Unnecessary (fixed order) |
| Initiative badge | Betting removed |
| Raises remaining | Betting removed |
| Fold/Continue projections | Unnecessary complexity |
| "Go Home" projection | Unnecessary complexity |
| All-in trigger/effects | Betting removed |
| All players fold | N/A (2-player system) |
| Decision Support (4 features) | Unnecessary complexity |
| Balance Targets (4 features) | Not trackable metrics |
| Strategic deck building | Unnecessary complexity |

---

## Remaining Work

None - all features complete!
