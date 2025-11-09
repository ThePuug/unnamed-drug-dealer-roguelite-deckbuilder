# Progression & Meta-Game - Feature Matrix

Implementation tracking for Progression & Meta-Game specification.

**Spec:** [progression-meta.md](progression-meta.md)

**Last Updated:** 2025-11-09

---

## Summary

**Overall Completion:** 0/35 features (0%)

| Category | Complete | Partial | Not Started | Deferred |
|----------|----------|---------|-------------|----------|
| Card Unlocks | 0 | 0 | 8 | 0 |
| Character System | 0 | 0 | 6 | 0 |
| Leaderboards | 0 | 0 | 9 | 0 |
| Meta-Game Loop | 0 | 0 | 5 | 0 |
| Player Feedback | 0 | 0 | 7 | 0 |
| **Total** | **0** | **0** | **35** | **0** |

---

## Card Unlocks: 0/8 complete (0%)

| Feature | Status | RFC/ADR | Notes |
|---------|:------:|---------|-------|
| Starting collection (~30 cards) | ❌ | - | 5 Products, 3 Locations, 12 support, 6 Narc, 4 insurance |
| Account-wide unlock tracking | ❌ | - | Unlocks persist across all characters |
| Profit milestones | ❌ | - | $5k, $10k, $25k, $50k, $100k total profit |
| Deck milestones | ❌ | - | 5, 10, 25, 50, 100 total decks played |
| Achievements | ❌ | - | Specific accomplishments (survive 10 decks, etc.) |
| Unlock notifications | ❌ | - | "NEW CARD UNLOCKED: [Card Name]" |
| Unlock progress tracking | ❌ | - | "$12,450 / $25,000 (next unlock)" |
| Full collection (~80-100 cards) | ❌ | - | Phase 2 expansion |

---

## Character System: 0/6 complete (0%)

| Feature | Status | RFC/ADR | Notes |
|---------|:------:|---------|-------|
| Character profiles | ❌ | - | College Student, Widow, Cancer Patient, Mafia Member |
| Narrative framing (no mechanics) | ❌ | - | Profiles provide context only |
| Character persistence | ❌ | - | Heat, Trust, Profit persist per-character |
| Character permadeath | ❌ | - | Trigger on bust, character lost forever |
| Character stat logging | ❌ | - | Track decks, profit, Heat, Trust on death |
| New character creation | ❌ | - | Start fresh after bust |

---

## Leaderboards: 0/9 complete (0%)

| Feature | Status | RFC/ADR | Notes |
|---------|:------:|---------|-------|
| Flash leaderboard (7-day profit) | ❌ | - | Rolling window, max profit in any 7 days |
| Kingpin leaderboard (single run) | ❌ | - | Total profit on one character before bust |
| Survivor leaderboard (deck count) | ❌ | - | Most decks played on one character |
| 7-day window calculation | ❌ | - | Track all 7-day windows, find max |
| Leaderboard ranking (top 1000) | ❌ | - | Show global rank |
| Personal best tracking | ❌ | - | Show own best across all categories |
| Leaderboard display UI | ❌ | - | Rank, name, score, metadata |
| Leaderboard updates | ❌ | - | Update on character bust / deck completion |
| Leaderboard persistence | ❌ | - | Never reset (all-time highs) |

---

## Meta-Game Loop: 0/5 complete (0%)

| Feature | Status | RFC/ADR | Notes |
|---------|:------:|---------|-------|
| Deck-to-deck loop | ❌ | - | Same character, Heat/Trust persist |
| Run-to-run loop | ❌ | - | New character, fresh start |
| Account progress accumulation | ❌ | - | Lifetime profit, decks, achievements |
| Strategic arc (early/mid/late run) | ❌ | - | Difficulty escalates with Heat |
| Emergent storytelling | ❌ | - | Stats create narrative ("8 decks, $3.2k, busted at School Zone") |

---

## Player Feedback: 0/7 complete (0%)

| Feature | Status | RFC/ADR | Notes |
|---------|:------:|---------|-------|
| Lifetime stats display | ❌ | - | Total profit, decks across all characters |
| Current character stats | ❌ | - | Heat, Trust, profit, decks on current character |
| Unlock progress bars | ❌ | - | Visual progress to next unlock |
| Unlock celebrations | ❌ | - | "NEW CARD UNLOCKED" popup |
| Leaderboard rank display | ❌ | - | "Flash: #45, Kingpin: #120, Survivor: #88" |
| Permadeath summary | ❌ | - | "Character busted after 12 decks, $8.4k profit" |
| Account progress on death | ❌ | - | "Lifetime profit: $45k (+$8.4k)" |

---

## Implementation Deviations

_No implementations yet._

---

## Notes

- Card unlocks are ACCOUNT-WIDE (permanent progress)
- Character death loses Heat/Trust/character-specific profit
- Leaderboards create 3 distinct playstyles (Flash/Kingpin/Survivor)
- MVP: Focus on core unlock system (profit + deck milestones)
- Phase 2: Full leaderboards + achievements
- Phase 3: Expanded card collection (80-100 cards)
- Target unlock rate: 1 card per 1-2 hours of play
