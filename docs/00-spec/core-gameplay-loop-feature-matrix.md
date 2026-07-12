# Core Gameplay Loop - Feature Matrix

**Spec:** [core-gameplay-loop.md](core-gameplay-loop.md)
**Last Updated:** 2026-07-12
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
| Turn indicator | ✅ | SOW-022: top-center round header + per-actor pill (NARC'S MOVE / YOUR MOVE / BUYER REACTING) with portrait spotlights |
| Card count per player | ✅ | SOW-022: narc/buyer count chips (buyer hand now intentionally hidden — see Implementation Deviations) |
| Running totals display | ✅ | SOW-022: EVIDENCE vs COVER balance bar + PAYOUT chip (replaces counters row) |
| Color-coded safety | ✅ | SOW-022: SAFE / AT RISK chip on the balance bar; heat gradient track in YOUR STANDING panel |
| Evidence gap display | ✅ | Balance bar split + resolution overlay |
| Heat accumulation | ✅ | SOW-022: YOUR STANDING heat track (0–100) with conviction-threshold tick marks; buyer cap on BAILS AT HEAT chip |
| Active Product highlight | ✅ | Active slot system ("THE DEAL ON THE TABLE") |
| Active Location highlight | ✅ | Active slot system ("THE DEAL ON THE TABLE") |
| Bust warning | ✅ | Real-time balance bar + AT RISK chip |
| Danger indicator | ✅ | Narc tier badge (RFC-018); narc intent bubble telegraph (SOW-022) |

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

## Implementation Deviations

| Deviation | Rationale |
|-----------|-----------|
| Buyer's face-up hand no longer displayed (SOW-022) | Game Play v2 design hides buyer cards behind a count chip — intentional hidden-information change supporting the July 2026 fun-assessment finding that nothing is uncertain at commitment |
| Narc telegraphs its pending card via intent bubble (SOW-022) | Game Play v2 design adds a roguelite-style intent preview while the narc is the pending actor; after acting it shows the card actually played |
| Played Evidence/Cover/Modifier cards render as aggregate (balance bar) + discard stack top card, not as an individual card pool (SOW-022) | v2 design removes the wrapping played pool; the last resolved card is visible face-up on the discard stack |

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
