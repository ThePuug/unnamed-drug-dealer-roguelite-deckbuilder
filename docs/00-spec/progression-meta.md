# Progression & Meta-Game Specification

## Overview

Progression systems create long-term goals beyond individual runs. **Cash** is the universal currency for unlocking cards and content (account-wide). **Locations** gate card pools and unlock via achievements. **Narc tiers** escalate globally as locations unlock, creating difficulty progression. **Card upgrades** provide per-run power growth. **Character permadeath** creates meaningful stakes (lose Heat and card upgrades, keep cash and unlocks).

**Core Tension:** Unlocking new locations gives access to better cards but permanently increases Narc difficulty everywhere.

---

## Player Experience Goal

Players should feel:
- **Permanent progress** - Cash and unlocks persist forever, even through permadeath
- **Meaningful choice** - Spend cash on cards that fit your strategy
- **Risk/reward tradeoffs** - Chase achievements for new locations knowing Narcs get harder
- **In-run growth** - Cards get stronger the more you use them (this run only)
- **Meaningful loss** - Losing a character hurts (Heat gone, card upgrades gone) but not devastating
- **Replayability** - New runs feel fresh (different card upgrades, different strategy)

This is NOT about gating content behind paywalls or grinding - it's about **rewarding play with expanded strategic options**.

---

## Cash System

### Single Account-Wide Pool

**Player Mental Model:**
"I earn cash by playing. I spend cash to buy cards at locations. My wallet persists forever - even when my character dies, even across different characters."

**Mechanical Definition:**
- **One pool** - "Cash on hand" is the only spendable currency
- **Account-wide** - Shared across all characters (past, present, future)
- **Persistent** - Never lost, even on character permadeath
- **Earned** - By completing deals during gameplay
- **Spent** - At locations to purchase card unlocks

**Revenue Metric (Separate):**
- Tracks total cash ever earned (for leaderboards)
- Spending does NOT reduce this metric
- Used for competitive rankings only

**Example Flow:**
```
Play deals, earn $10k    → Cash on hand: $10k | Revenue: $10k
Buy cards for $3k        → Cash on hand: $7k  | Revenue: $10k
Character dies           → Cash on hand: $7k  | Revenue: $10k
New character earns $5k  → Cash on hand: $12k | Revenue: $15k
```

---

## Location System

### What Are Locations?

**Player Mental Model:**
"Locations are where I buy cards. Each location has different cards available. I unlock new locations by completing achievements. But unlocking harder locations makes Narcs tougher everywhere."

**Mechanical Definition:**
- **Card shops** - Each location offers a unique pool of cards for purchase
- **Achievement-gated** - Unlock new locations via specific accomplishments
- **Permanent unlocks** - Once unlocked, always accessible
- **Narc tier triggers** - Unlocking locations escalates global Narc difficulty

---

### Starting Location

**"The Corner"** (unlocked by default)
- Basic cards available
- Entry-level products, simple cover, basic modifiers
- Tier 1 Narcs (easiest)

---

### Example Location Progression

| Location | Unlock Achievement | Card Pool Theme | Narc Tier |
|----------|-------------------|-----------------|-----------|
| The Corner | Default | Basic products, simple cover | Tier 1 |
| The Block | Survive 5 decks | Better products, more variety | Tier 2 |
| Downtown | Earn $10k lifetime | Premium products, strong cover | Tier 3 |
| The Docks | Survive 15 decks | Import products, specialized cards | Tier 4 |
| The Tower | Earn $50k lifetime | Elite products, powerful modifiers | Tier 5 |

**Design Intent:**
- Early locations: Accessible achievements, basic cards
- Mid locations: Moderate challenge, strategic options
- Late locations: Aspirational goals, powerful cards, dangerous Narcs

---

### Location Access

- **Play any unlocked location** - Not locked to "current" location
- **Buy from any unlocked location** - Shop at multiple locations between decks
- **Balance deferred** - Risk/reward of location choice tuned later

---

## Narc Variety System

### How Location Unlocks Affect Narcs

**TBD - Playtest to determine best approach.**

Location unlocks should add *variety* to Narc encounters, not raw difficulty. Heat controls difficulty (Narc card upgrade levels). Location unlocks control variety (what types of Narcs/cards you might face).

**Possible approaches (decide after core loop is playable):**

| Approach | Description | Pros | Cons |
|----------|-------------|------|------|
| **Narc Profiles** | Each location unlocks a new Narc persona with own deck | Thematic, parallels Buyers | More content to design |
| **Card Pool Expansion** | Locations add new card types to single Narc | Simpler | Less personality |
| **Single Narc (MVP)** | One Narc, Heat controls everything | Simplest, ship faster | Less variety |

**MVP Recommendation:** Start with single Narc profile. Heat controls difficulty via card upgrades. Add variety later if playtesting shows it's needed.

**Key Principle:** New locations should NOT inherently make the game harder. Heat is the difficulty knob. Locations add variety and content.

---

## Card Unlock System

### Purchasing Cards

**Player Mental Model:**
"I spend cash at locations to unlock cards. Each location has different cards. Once I buy a card, it's mine forever."

**Mechanical Definition:**
- **Location-specific** - Each card is only available at one location
- **Cash purchase** - Spend cash on hand to unlock
- **Permanent** - Never lost, even on permadeath
- **Account-wide** - All characters can use unlocked cards

---

### Starting Collection

**Player starts with:** (~15-20 cards, enough for one viable deck)
- Basic Products (3-4 cards)
- Basic Locations (2-3 cards for deck building, not meta locations)
- Basic Cover (3-4 cards)
- Basic Modifiers (2-3 cards)

**Design Intent:** Enough to build one functional 15-card deck. Additional variety requires purchases.

---

### Card Pricing

_Specific prices TBD. General principles:_

| Card Power | Price Range | Availability |
|------------|-------------|--------------|
| Basic variants | $500 - $1,500 | Early locations |
| Standard options | $2,000 - $5,000 | Mid locations |
| Powerful cards | $8,000 - $15,000 | Late locations |
| Elite cards | $20,000+ | Final locations |

**Design Intent:**
- Steady unlock rate (~1 card per 2-4 decks played)
- Meaningful spending decisions (save up vs. buy now)
- Late-game cards are aspirational

---

## Per-Run Progression

### Card Upgrades

**Player Mental Model:**
"The more I use a card during my character's life, the stronger it gets. When my character dies, upgrades reset."

**Mechanical Definition:**
- **Usage-based** - Cards improve with repeated play
- **Per-character** - Upgrades tied to current character only
- **Lost on permadeath** - Reset to base when character dies
- **Incremental** - Multiple upgrade tiers per card

**Example Progression:**
```
Deck 1: Play "Burner Phone" 2 times
Deck 2: Play "Burner Phone" 3 times (5 total) → Upgrade to +5 Cover
Deck 4: Play "Burner Phone" 4 times (9 total) → Upgrade to +10 Cover
Deck 6: Play "Burner Phone" 3 times (12 total) → Upgrade to +15 Cover
Character dies: Burner Phone resets to base stats
```

**Design Intent:**
- Creates run identity (your deck evolves uniquely)
- Rewards card commitment without forcing it
- Meaningful loss on permadeath (lose upgrades)
- Fresh starts feel fresh (base cards again)

---

### Upgrade Mechanics

_Specific mechanics TBD. Possibilities:_

| Approach | Description |
|----------|-------------|
| Flat bonuses | +X to primary stat per tier |
| Percentage scaling | Stats increase by X% per tier |
| New effects | Gain additional abilities at thresholds |
| Cost reduction | Card becomes cheaper/easier to play |

**Upgrade Thresholds:**
- Tier 1: 5 plays
- Tier 2: 12 plays
- Tier 3: 25 plays
- Tier 4: 50 plays (max)

---

### Heat (Unchanged from Core)

- Accumulates during play
- Decays daily (encourages 1 deck/day pacing)
- Drives bust risk
- Resets on permadeath

Heat remains the primary tension mechanic. Card upgrades provide the "positive" per-run progression.

---

## Character System

### Character Slots

**Player Mental Model:**
"I start with one character slot. I can unlock more slots to run multiple characters."

**Mechanical Definition:**
- **Start with 1 slot** - One active character at a time initially
- **Unlock more** - Via achievements or purchases
- **Shared wallet** - All characters use same cash pool
- **Independent runs** - Each character has own Heat, card upgrades

**Example:**
```
Slot 1: "College Student" - Heat 45, Burner Phone at Tier 2
Slot 2: "Widow" - Heat 12, Burner Phone at Tier 1
Cash on hand: $8,500 (shared)
```

---

### Character Profiles

**Available Profiles:**
- **College Student** - "Tuition is due, rent is overdue, can't call mom for help"
- **Widow** - "Three kids to feed, insurance ran out, bills piling up"
- **Cancer Patient** - "Treatment costs more than I'll ever make, family doesn't know"
- **Mafia Member** - "Gotta prove I'm not just the boss's nephew, make my bones"

**Design Intent:**
- **Narrative framing only** - No mechanical differences
- **Emergent storytelling** - Player projects story onto run
- **Variety** - Different profiles for different runs

---

### Character Permadeath

**Trigger:** Character busted (Evidence > Cover, insurance failed/overridden)

**Lost:**
- Character itself (name, profile, stats)
- Heat (new character starts at 0)
- All card upgrades (reset to base)

**Preserved:**
- Cash on hand (account-wide)
- Unlocked cards (account-wide)
- Unlocked locations (account-wide)
- Unlocked character slots (account-wide)
- Narc tier (account-wide)
- Revenue metric (account-wide)

**Player Feedback:**
```
CHARACTER BUSTED - Run Ends

"College Student" arrested after 12 decks
Card upgrades lost: Burner Phone Tier 3, Meth Tier 2, ...

Cash on hand: $12,450 (kept)
Lifetime revenue: $45,230

[Create New Character]
```

---

## Achievements

### Purpose

Achievements unlock locations and character slots. They represent meaningful accomplishments that gate progression.

### Example Achievements

**Location Unlocks:**
| Achievement | Requirement | Unlocks |
|-------------|-------------|---------|
| Street Cred | Survive 5 decks (single character) | The Block |
| Money Talks | Earn $10k lifetime revenue | Downtown |
| Veteran | Survive 15 decks (single character) | The Docks |
| Kingpin | Earn $50k lifetime revenue | The Tower |

**Character Slot Unlocks:**
| Achievement | Requirement | Unlocks |
|-------------|-------------|---------|
| Back in Business | Complete a run (any length) | Slot 2 |
| Parallel Operations | Survive 10 decks total | Slot 3 |
| Empire Builder | Unlock 3 locations | Slot 4 |

**Design Intent:**
- Mix of survival and revenue achievements
- Some achievable quickly, some aspirational
- Clear path to new content

---

## Leaderboards (Future Feature)

### Purpose

Create competitive goals. Track player accomplishments.

### Planned Leaderboards

| Board | Measures | Strategy |
|-------|----------|----------|
| **Flash** | Highest revenue in 7-day window | Aggressive binge play |
| **Kingpin** | Highest revenue single run | Long patient runs |
| **Survivor** | Most decks single run | Ultra-conservative |

**Key Point:** Leaderboards track **revenue earned**, not cash on hand. Spending doesn't hurt your ranking.

_Full leaderboard implementation deferred to later phase._

---

## Meta-Game Loop

### The Cycles

**Per-Deck Loop:**
```
Build Deck (15 cards) → Play Hands → Earn Cash / Gain Heat →
Cards Played Gain Progress → End Deck → Repeat
```

**Per-Run Loop:**
```
Create Character → Play Decks → Cards Upgrade Over Time →
Heat Accumulates → Eventually Busted → Lose Character + Upgrades
```

**Meta Loop:**
```
Play Runs → Earn Cash → Buy Cards at Locations →
Chase Achievements → Unlock Locations (Narcs Get Harder) →
Unlock Character Slots → Start New Runs with More Options
```

---

### Progression Arc

**New Player:**
- 1 character slot
- ~15 starting cards
- Tier 1 Narcs
- The Corner only

**Mid Player:**
- 2-3 character slots
- ~40 cards unlocked
- Tier 3 Narcs
- 3 locations unlocked

**Veteran Player:**
- 4 character slots
- ~80+ cards unlocked
- Tier 5 Narcs
- All locations unlocked

---

## Fun Factor Analysis

### Why Is This Engaging?

**1. Permanent Progress**
- Cash never lost
- Card unlocks permanent
- Always moving forward (even through deaths)

**2. Meaningful Choices**
- Which cards to buy?
- When to push for achievement (harder Narcs)?
- Which cards to commit to (upgrades)?

**3. In-Run Investment**
- Card upgrades create attachment
- Losing upgrades hurts (but not devastating)
- Each run develops unique identity

**4. Clear Progression Path**
- Locations gate content visibly
- Achievements show what's possible
- Always something to work toward

**5. Difficulty Scaling**
- Narcs grow with player
- Can't out-level the game
- Mastery always relevant

---

### Potential Pain Points

**1. Narc Escalation Feels Punishing**
- "I unlocked a location and now everything is harder?"
- **Mitigation:** Clear warning before unlocking, better cards compensate

**2. Card Upgrades Lost on Death**
- "I had Tier 4 Burner Phone and now it's gone?"
- **Mitigation:** Clear telegraphing of bust risk, upgrades rebuild faster with skill

**3. Cash Spending Regret**
- "I bought the wrong card and wasted $5k"
- **Mitigation:** Show card details before purchase, no "trap" cards

---

## Balance Considerations

### Card Upgrade Pacing
- **Too slow:** Runs feel flat, no growth
- **Too fast:** Max upgrades too quickly, no progression
- **Target:** Reach Tier 2 on core cards by deck 5-8, Tier 4 is aspirational (deck 20+)

### Cash Economy
- **Too generous:** Unlock everything quickly, no decisions
- **Too stingy:** Grind forever for one card
- **Target:** Unlock ~1 card per 2-4 decks played

### Narc Tier Jumps
- **Too harsh:** New tier feels impossible
- **Too soft:** New tier feels the same
- **Target:** Noticeable but manageable increase, new cards help compensate

---

## MVP Scope

### Phase 1 (Core Progression)
- Cash on hand system (earn, spend, persist)
- Starting collection (~15 cards)
- 1 starting location (The Corner)
- 1 additional location unlock (The Block)
- Basic card purchasing
- Single character slot
- Basic card upgrade system (2 tiers)
- Tier 1-2 Narc differentiation

### Phase 2 (Expanded Content)
- 2 additional locations
- Character slot unlocks (up to 3)
- Achievement system
- Full card upgrade system (4 tiers)
- Tier 3-4 Narcs

### Phase 3 (Full Meta)
- All planned locations
- Full card collection (~80+ cards)
- All character slots
- All achievements
- Tier 5 Narcs

### Future Features
- Leaderboards (Flash, Kingpin, Survivor)
- Seasonal content
- Additional character profiles

---

## Open Questions

### Card Upgrades
- Exactly how do stats improve per tier?
- Do all cards upgrade the same way, or card-specific paths?
- Should some cards have unique upgrade effects?

### Locations
- How many locations total?
- What's the thematic progression?
- How many cards per location?

### Narc Tiers
- Specific mechanical differences per tier?
- How much harder is each tier?
- Any new Narc card types at higher tiers?

### Economy
- Exact card prices?
- Cash earned per deck (average)?
- Time to unlock all content?

---

## Success Criteria

### Engagement Metrics
- Cash unlock rate: ~1 card per 2-4 decks
- Average run length: 8-15 decks before bust
- Location unlock rate: New location every 5-10 hours

### Balance Metrics
- Card upgrade distribution: Most runs reach Tier 2-3 on core cards
- Narc tier feels fair: Players succeed at new tiers after adjustment period
- No dominant strategy: Multiple viable deck archetypes

### Player Satisfaction
- Unlocks feel rewarding (clear value from purchases)
- Permadeath feels fair (player had agency)
- Progression feels steady (always something close to unlock)
- Difficulty feels appropriate (challenge scales with skill)
