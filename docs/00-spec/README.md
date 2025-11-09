# Game Design Specifications

This directory contains game design specifications that define what the game should be from the player's perspective.

## Specification Index

| Spec | Feature Matrix | Status | Features |
|------|---------------|--------|----------|
| [Core Gameplay Loop](core-gameplay-loop.md) | [Matrix](core-gameplay-loop-feature-matrix.md) | 0% (0/42) | Hand/Round/Deck/Run structure |
| [Card System](card-system.md) | [Matrix](card-system-feature-matrix.md) | 0% (0/48) | 7 card types, interactions |
| [Heat & Trust Systems](heat-trust-systems.md) | [Matrix](heat-trust-systems-feature-matrix.md) | 0% (0/32) | Persistent difficulty scaling |
| [Bust & Insurance Mechanics](bust-insurance-mechanics.md) | [Matrix](bust-insurance-mechanics-feature-matrix.md) | 0% (0/26) | Permadeath trigger & saves |
| [Progression & Meta](progression-meta.md) | [Matrix](progression-meta-feature-matrix.md) | 0% (0/35) | Card unlocks, leaderboards |

**Total Features:** 0/183 (0%)

---

## Specification Summaries

### Core Gameplay Loop
**Focus:** Hand/Round/Deck/Run hierarchy, betting mechanics, decision points

Players build 15-card decks and play 3-5 hands per session. Each hand has 3 rounds of betting (poker-like). Core tension: stay in for profit vs. fold to preserve cards and minimize Heat.

**Key Mechanics:**
- Turn-based betting (Check/Raise/Fold)
- 3 rounds per hand
- Running totals visible (Evidence, Cover, Heat, Profit)
- Fold anytime to preserve cards
- "Go Home" early option

---

### Card System
**Focus:** 7 card types, override rules, stacking mechanics

Cards modify four values: Evidence (risk), Cover (defense), Heat (difficulty), Profit (reward). Products and Locations use override system (last played wins). Modifiers stack additively or multiplicatively.

**Card Types:**
1. **Product** - What you're selling (override, base price, Heat)
2. **Location** - Where deal happens (override, Evidence/Cover bases)
3. **Deal Modifier** - Adjust terms (stack, affects any value)
4. **Evidence** - Narc investigation (additive, from AI)
5. **Cover** - Player defense (additive, player plays)
6. **Get Out of Jail** - Insurance (override, single use, costs)
7. **Make It Stick** - Conviction (override, Heat threshold, overrides insurance)

---

### Heat & Trust Systems
**Focus:** Persistent character stats, real-time decay, difficulty scaling

Heat represents law enforcement pressure (harder Narc decks). Trust represents customer relationships (better Customer decks). Both persist across sessions on same character.

**Heat System:**
- Accumulates from cards (+5 to +60 Heat per card)
- Decays in real-time (-1 Heat/hour, -24/day)
- 5 tiers (Cold → Warm → Hot → Scorching → Inferno)
- Determines Narc deck for NEXT session

**Trust System:**
- +1 per successful hand (stayed in all 3 rounds)
- -1 per fold (customer feels disrespected)
- 3 tiers (Low 0-3, Medium 4-10, High 11+)
- Determines Customer deck for CURRENT session

---

### Bust & Insurance Mechanics
**Focus:** Permadeath trigger, insurance saves, conviction override

Simple rule: **Evidence > Cover = Busted (run ends)**. Insurance cards (Get Out of Jail) can save you once per deck for a cost. Conviction cards (Make It Stick) override insurance at high Heat.

**Flow:**
1. If Evidence <= Cover → Safe (bank profit, continue)
2. If Evidence > Cover → Bust triggered
3. Check Make It Stick: If Heat >= threshold → Insurance overridden → Run ends
4. Check Get Out of Jail: If can afford → Pay cost, gain Heat, continue
5. No insurance or can't afford → Run ends

---

### Progression & Meta
**Focus:** Card unlocks, character permadeath, leaderboards

Account-wide progression through card unlocks (permanent). Character-specific runs with permadeath (lose Heat/Trust). Three leaderboards create distinct playstyles.

**Unlocks:**
- Starting: ~30 cards
- Full collection: ~80-100 cards
- Unlock via: Profit milestones, Deck milestones, Achievements
- Account-wide (never lost)

**Leaderboards:**
1. **Flash** - Highest profit in 7 days (sprint)
2. **Kingpin** - Highest profit single run (marathon)
3. **Survivor** - Most decks single run (endurance)

**Permadeath:**
- Lose: Heat, Trust, character profit
- Keep: Unlocked cards, account stats, achievements

---

## Feature Development Workflow

### As ARCHITECT Role

**When to Create/Update Feature Matrices:**

1. **RFC Approved** → Mark features "Planned" with RFC link
2. **SOW Created** → Add SOW link to relevant features
3. **Implementation Started** → Mark features "In Progress"
4. **Implementation Complete** → Mark features "Complete" with ADR links
5. **Deviations Found** → Document in "Implementation Deviations" section

**Feature Status Symbols:**
- ✅ **Complete** - Fully implemented per spec
- 🚧 **Partial** - MVP version only
- 🔄 **In Progress** - Currently being developed
- ❌ **Not Started** - Planned but not implemented
- ⏸️ **Deferred** - Intentionally postponed to post-MVP

---

## Next Steps (For ARCHITECT)

### Immediate Priority
1. Review specs with PLAYER role (validate player experience goals)
2. Create RFC-001 for MVP Core Prototype (foundational systems)
3. Identify architectural decisions requiring ADRs
4. Break down MVP into implementable SOWs (≤20 hours each)

### MVP Scope (Phase 1)
**Goal:** Playable prototype with core loop

**Systems Required:**
- Basic hand structure (3 rounds, betting, fold)
- Evidence > Cover bust mechanic
- 20 cards (5 Products, 3 Locations, 8 support, 4 insurance)
- Single character (no progression)
- Heat accumulation (no decay yet)

**Estimated Effort:** 60-80 hours (3-4 SOWs)

### Phase 2 (Systems Integration)
**Goal:** Complete game loop with persistence

**Systems Required:**
- Heat decay (real-time)
- Trust system + Customer deck scaling
- Narc deck scaling by Heat
- Character profiles (narrative framing)
- Save/load

**Estimated Effort:** 40-60 hours (2-3 SOWs)

### Phase 3 (Content & Polish)
**Goal:** Full collection and leaderboards

**Systems Required:**
- Full card collection (80-100 cards)
- Card unlock progression
- Leaderboards (Flash, Kingpin, Survivor)
- UI/UX polish
- Tutorial

**Estimated Effort:** 60-80 hours (3-4 SOWs)

---

## Spec Maintenance

### When to Update Specs
- Major design changes (PLAYER role decision)
- Discovered missing edge cases (during implementation)
- Balance adjustments (playtesting feedback)

### When to Update Feature Matrices
- RFC approved (mark features "Planned")
- Implementation complete (mark features "Complete")
- Deviations found (document in "Deviations" section)
- Recalculate totals (update percentages)

---

## Questions?

- **Where do RFCs come from?** PLAYER role creates based on spec gaps or new ideas
- **Who maintains matrices?** ARCHITECT role updates throughout implementation
- **What if implementation differs from spec?** Document deviation in matrix, decide: update spec, document as MVP, or reject implementation
- **How detailed should specs be?** Enough to guide RFC creation and validate implementations
