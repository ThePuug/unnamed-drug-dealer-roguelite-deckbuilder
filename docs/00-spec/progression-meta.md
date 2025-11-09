# Progression & Meta-Game Specification

## Overview

Progression systems create long-term goals beyond individual runs. **Card unlocks** expand strategic options (account-wide). **Leaderboards** create competitive goals (Flash, Kingpin, Survivor). **Character permadeath** creates meaningful stakes (lose Heat/Trust progress). **Character profiles** provide narrative framing (no mechanical differences). The meta-game loop: play run → get busted → start fresh → repeat with more cards and knowledge.

**Core Tension:** Permadeath creates stakes, but account-wide unlocks ensure permanent progress.

---

## Player Experience Goal

Players should feel:
- **Permanent progress** - Unlocking cards feels rewarding and persistent
- **Meaningful loss** - Losing character hurts (Heat/Trust gone), but not devastating (cards remain)
- **Long-term goals** - Leaderboards create aspirational targets
- **Replayability** - New runs feel fresh (different character, different strategy)
- **Skill expression** - Better players climb leaderboards, unlock cards faster
- **No grind wall** - Progression feels natural (unlock cards by playing, not grinding)

This is NOT about gating content behind paywalls or grinding - it's about **rewarding play with expanded strategic options**.

---

## Card Unlock System

### What Are Card Unlocks?

**Player Mental Model:**
"As I play and earn profit, I unlock more cards. These cards are mine forever, even if my character dies. More cards = more deck-building options."

**Mechanical Definition:**
- **Account-wide** (all characters share unlocked cards)
- **Permanent** (never lost, even on character death)
- Unlocked via **profit milestones**, **deck milestones**, **achievements**
- Starting collection: ~30 basic cards (enough for viable decks)
- Full collection: ~80-100 cards (complete strategic depth)

---

### Starting Collection (~30 Cards)

**Player starts with:**
- 5 Products (Weed, Pills, Meth, Heroin, one basic variant)
- 3 Locations (Safe House, Parking Lot, School Zone)
- 12 Support cards (4 Deal Modifiers, 4 Evidence, 4 Cover)
- 6 Evidence cards (Narc cards - procedurally used)
- 4 Insurance cards (2 Get Out of Jail, 2 Make It Stick - Narc uses)

**Design Intent:** Enough cards to build functional 15-card decks with strategic variety.

**Deck Building Viability:**
- Can build Aggressive deck (high-value Products, light Cover)
- Can build Balanced deck (mix of Products/Locations/Cover)
- Can build Conservative deck (low-value Products, heavy Cover)

---

### Unlock Categories

#### 1. Profit Milestones (Account Lifetime Profit)

**Tracks:** Total profit earned across all characters, all runs, all time

**Example Milestones:**
- $5,000 total → Unlock "Premium Weed" (Product: $45, +8 Heat)
- $10,000 total → Unlock "Cocaine" (Product: $120, +35 Heat)
- $25,000 total → Unlock "Safehouse Network" (Location: 5 Evidence, 35 Cover, -10 Heat)
- $50,000 total → Unlock "Federal Immunity" (Get Out of Jail: +50 Cover, $5k cost, +5 Heat)
- $100,000 total → Unlock "Cartel Connection" (Product: $200, +60 Heat)

**Design Intent:**
- Rewards consistent play (profit accumulates forever)
- No punishment for character death (profit doesn't reset)
- High-value unlocks at extreme milestones (aspirational)

**Player Feedback:**
- Show lifetime profit: "$12,450 / $25,000 (next unlock: Safehouse Network)"
- Show unlock progress bar
- Celebrate unlock: "NEW CARD UNLOCKED: Safehouse Network"

---

#### 2. Deck Milestones (Account Lifetime Decks)

**Tracks:** Total decks played across all characters, all runs, all time

**Example Milestones:**
- 5 decks → Unlock "Burner Phone" (Cover: +25, -10 Heat)
- 10 decks → Unlock "Launder Money" (Deal Modifier: Price ×1.2, +5 Evidence, -15 Heat)
- 25 decks → Unlock "Offshore Account" (Cover: +30, -20 Heat)
- 50 decks → Unlock "Political Connections" (Cover: +40, -10 Heat, costs $2k)
- 100 decks → Unlock "Untouchable" (Get Out of Jail: +60 Cover, $10k cost, Heat reset to 0)

**Design Intent:**
- Rewards consistency (decks accumulate forever)
- Encourages daily play (1 deck/day = steady unlocks)
- Defensive cards at high milestones (help long runs)

**Player Feedback:**
- Show deck counter: "38 / 50 decks (next unlock: Political Connections)"
- Show unlock progress bar
- Celebrate unlock: "NEW CARD UNLOCKED: Political Connections"

---

#### 3. Achievements (Specific Accomplishments)

**Tracks:** Specific gameplay feats

**Example Achievements:**
- "Survive 10 decks (single character)" → Unlock "Safe Haven" (Location: 8 Evidence, 32 Cover, -8 Heat)
- "Earn $5k in one deck" → Unlock "High Roller" (Deal Modifier: Price ×1.4, +15 Evidence, +15 Heat)
- "Reach Heat 100+" → Unlock "Heat Seeker" (Product: $180, +50 Heat, but +30 Cover)
- "Reach Trust 20+" → Unlock "VIP Network" (Deal Modifier: Price +$100, -15 Evidence, -10 Heat)
- "Get busted with Federal Case" → Unlock "Federal Case" (Make It Stick card now available to player... as knowledge?)

**Design Intent:**
- Rewards specific playstyles (survival, profit, risk-taking)
- Creates optional challenges (not required for progression)
- Unique cards reflect achievement theme

**Player Feedback:**
- Show achievement list with progress
- Celebrate unlock: "ACHIEVEMENT UNLOCKED: Survive 10 Decks - NEW CARD: Safe Haven"

---

### Unlock Pacing

**Target Unlock Rate:**
- 1 new card every 1-2 hours of play (~4-8 decks)
- 50% of cards unlocked by 20-30 hours (core collection)
- 100% of cards unlocked by 60-80 hours (completionist)

**Unlock Distribution:**
- Early unlocks: Basic variants (more Products, more Locations)
- Mid unlocks: Strategic options (specialized Deal Modifiers, strong Cover)
- Late unlocks: High-risk/high-reward (extreme Products, expensive Insurance)

**Design Intent:**
- No grind wall (unlocks come naturally from playing)
- Steady drip of new content (always something to unlock soon)
- Late-game unlocks reward mastery (not required for viability)

---

## Character System

### Character Profiles

**Player Mental Model:**
"Each character is a different person with their own story. They play the same mechanically, but the narrative changes."

**Available Profiles:**
- **College Student** - "Tuition is due, rent is overdue, can't call mom for help"
- **Widow** - "Three kids to feed, insurance ran out, bills piling up"
- **Cancer Patient** - "Treatment costs more than I'll ever make, family doesn't know"
- **Mafia Member** - "Gotta prove I'm not just the boss's nephew, make my bones"

**Design Intent:**
- **Narrative framing only** (no mechanical differences)
- Player projects story onto gameplay (emergent storytelling)
- Different profiles for different runs (variety)

**Example Story Emergence:**
```
College Student:
  Run length: 8 decks
  Total profit: $3,200
  Bust: Warrant at Heat 55
  Story: "Couldn't keep up with school and dealing. Got caught making a risky sale in the campus parking lot. Expelled and arrested."

Widow:
  Run length: 22 decks
  Total profit: $18,500
  Bust: DA Approval at Heat 88
  Story: "Provided for the kids for months. Got greedy, started selling Meth to rich clients. DA built case and convicted her."
```

---

### Character Persistence (Per-Run)

**What Persists on a Character:**
- ✅ Total profit accumulated
- ✅ Total decks played
- ✅ Current Heat level (with decay)
- ✅ Current Trust level (no decay)
- ✅ Run history (hands played, profit per deck, etc.)

**What Resets Each Deck:**
- ❌ Current deck (cards return to collection)
- ❌ Cards in hand (discard after deck)

**What Doesn't Transfer to New Character:**
- ❌ Heat (starts at 0)
- ❌ Trust (starts at 0)
- ❌ Profit (starts at $0)
- ✅ Unlocked cards (account-wide, kept forever)

---

### Character Permadeath

**Trigger:** Character busted (Evidence > Cover, insurance failed/overridden)

**Consequences:**
- Character gone forever (name, profile, stats lost)
- Heat lost (new character starts at 0)
- Trust lost (new character starts at 0)
- Profit on that character logged (leaderboards) but not transferred

**Preserved:**
- Unlocked cards (account-wide)
- Account lifetime profit (continues accumulating)
- Account lifetime decks (continues accumulating)
- Achievements (permanent)

**Player Feedback:**
- "CHARACTER BUSTED - Run Ends"
- "College Student arrested after 12 decks, $8,430 profit, Heat 72, Trust 14"
- "Lifetime profit: $45,230 (+$8,430)"
- "Create new character to continue"

**Design Intent:**
- Meaningful loss (lose Heat/Trust/character-specific profit)
- Not devastating (keep cards and account progress)
- Encourages fresh start (new character, new strategy)

---

## Leaderboards

### Purpose

**Create competitive goals beyond individual runs:**
- Flash: Sprint strategy (high profit in 7 days)
- Kingpin: Marathon strategy (high profit on one character)
- Survivor: Endurance strategy (most decks on one character)

**Design Intent:**
- Encourage different playstyles (aggressive, balanced, conservative)
- Create aspirational targets (top 10, top 100)
- Provide bragging rights (screenshot and share)

---

### 1. "Flash" Leaderboard - Highest Profit in 7 Days

**Measures:** Total profit earned across all characters/decks in any 7-day rolling window

**Resets:** Never (all-time high tracked)

**Calculation:**
```
For each 7-day window:
  profit = sum(profit from all decks played in window)

Flash score = max(profit across all 7-day windows)
```

**Example:**
```
Player A:
  Monday-Sunday (Week 1): $12,000 profit (10 decks played)
  Monday-Sunday (Week 2): $18,000 profit (15 decks played)
  Flash score: $18,000

Player B:
  Friday-Thursday: $25,000 profit (20 decks played, multiple characters)
  Flash score: $25,000 (best 7-day window)
```

**Strategic Implications:**
- **Binge-friendly** - Play many decks in one week
- **High-risk acceptable** - Character death doesn't end window (start new character, keep grinding)
- **Aggressive builds** - Maximize profit per deck (accept high Heat)

**Player Feedback:**
- "Current 7-day window: $9,200 (4 decks)"
- "Best 7-day window: $18,000 (Week 2)"
- "Leaderboard rank: #45"

---

### 2. "Kingpin" Leaderboard - Highest Profit Single Run

**Measures:** Total profit earned on one character before bust

**Resets:** Never (all-time high tracked)

**Calculation:**
```
For each character:
  profit = sum(profit from all decks before bust)

Kingpin score = max(profit across all characters)
```

**Example:**
```
Player A:
  Character 1: $3,200 (busted after 8 decks)
  Character 2: $18,500 (busted after 22 decks)
  Kingpin score: $18,500

Player B:
  Character 1: $45,000 (busted after 50 decks)
  Kingpin score: $45,000
```

**Strategic Implications:**
- **Patience rewarded** - Long runs = high profit
- **Heat management critical** - Must survive many decks
- **1 deck/day optimal** - Heat decay maximizes run length
- **Insurance mandatory** - Can't afford early bust

**Player Feedback:**
- "Current character: $12,450 (18 decks)"
- "Best character: $18,500 (Character 2, 22 decks)"
- "Leaderboard rank: #120"

---

### 3. "Survivor" Leaderboard - Most Decks Played Single Run

**Measures:** Number of decks played on one character before bust

**Resets:** Never (all-time high tracked)

**Calculation:**
```
For each character:
  decks = count(decks played before bust)

Survivor score = max(decks across all characters)
```

**Example:**
```
Player A:
  Character 1: 8 decks
  Character 2: 22 decks
  Survivor score: 22

Player B:
  Character 1: 50 decks
  Survivor score: 50
```

**Strategic Implications:**
- **Ultra-conservative** - Minimize profit per deck (fold often)
- **Heat management extreme** - Use all Heat-reduction cards
- **1 deck/day mandatory** - Heat decay critical for survival
- **Trust less important** - Fold often anyway

**Player Feedback:**
- "Current character: 18 decks"
- "Best character: 22 decks (Character 2)"
- "Leaderboard rank: #88"

---

### Leaderboard Display

**Format:**
```
FLASH LEADERBOARD (Highest 7-Day Profit)
---
1. [PlayerName] - $52,000 (18 decks, Week 12)
2. [PlayerName] - $48,500 (20 decks, Week 8)
3. [PlayerName] - $45,200 (15 decks, Week 5)
...
45. [You] - $18,000 (10 decks, Week 2)
```

**Metadata:**
- Rank (1-1000)
- Player name
- Score (profit or decks)
- Additional info (decks played, week number, character name)

---

## Meta-Game Loop

### The Cycle

```
1. Create Character (choose profile)
   ↓
2. Build Deck (15 cards from collection)
   ↓
3. Play Deck (3-5 hands, make profit, accumulate Heat/Trust)
   ↓
4. Decision Point:
   a) Go Home (end session, bank profit) → Go to step 2
   b) Get Busted (run ends) → Go to step 5
   ↓
5. Character Death (permadeath)
   ↓
6. Review Stats (leaderboard updates, lifetime profit, unlocks)
   ↓
7. Go to step 1 (create new character)
```

---

### Short-Term Loop (Deck to Deck)

**Same Character, Multiple Decks:**
- Deck 1: Start at Heat 0, Trust 0 → End at Heat 40, Trust 3, Profit $600
- Wait 24 hours (Heat decays to 16)
- Deck 2: Start at Heat 16, Trust 3 → End at Heat 55, Trust 7, Profit $1,200
- Wait 24 hours (Heat decays to 31)
- Deck 3: Start at Heat 31, Trust 7 → End at Heat 70, Trust 10, Profit $1,800
- ...

**Strategic Arc:**
- Early decks: Easy (low Heat, but low Trust)
- Mid decks: Balanced (medium Heat, medium Trust)
- Late decks: Dangerous (high Heat, but high Trust = big profit)

---

### Long-Term Loop (Run to Run)

**Across Multiple Characters:**
- Character 1: 8 decks, $3,200 profit (busted at Heat 55)
- Character 2: 22 decks, $18,500 profit (busted at Heat 88)
- Character 3: 15 decks, $10,200 profit (busted at Heat 72)
- ...

**Account Progress:**
- Lifetime profit: $31,900 (unlocked ~15 additional cards)
- Lifetime decks: 45 (unlocked ~8 additional cards)
- Achievements: 5 (unlocked ~5 unique cards)
- Total collection: ~58/100 cards unlocked

---

## Fun Factor Analysis

### Why Is This Engaging?

**1. Permanent Progress (No Loss)**
- Unlocked cards forever (never lost)
- Account lifetime stats accumulate (always growing)
- Permadeath hurts, but not devastating

**2. Diverse Goals**
- Flash: Sprint (play hard for one week)
- Kingpin: Marathon (one long run)
- Survivor: Endurance (conservative play)

**3. Emergent Stories**
- Character profiles provide narrative framing
- Run stats tell story (8 decks, $3,200, busted at School Zone)
- Players create their own narratives

**4. Replayability**
- New character = fresh start (Heat 0, Trust 0)
- Different playstyle each run (aggressive, balanced, conservative)
- Expanded collection = new deck options

**5. Skill Expression**
- Better players: Higher leaderboards, faster unlocks
- Mastery: Longer runs, more profit, better Heat management

---

### Potential Pain Points

**1. Permadeath Feels Bad**
- Lose 20-deck character to one bad hand?
- **Mitigation:** Clear telegraphing (Evidence visible), counterplay (fold, insurance), player chose to stay in

**2. Grind for Unlocks**
- Need to play 50 decks to unlock all cards?
- **Mitigation:** Steady unlock rate (1 card per 1-2 hours), core collection unlocked early (50% by 20-30 hours), late unlocks optional (not required)

**3. Leaderboard Feels Inaccessible**
- Top 10 unreachable for casual players?
- **Mitigation:** Show personal bests (not just global rank), multiple leaderboards (find one that fits playstyle), achievements provide goals too

**4. Character Loss Resets Progress**
- Heat/Trust gone on permadeath?
- **Mitigation:** Unlocked cards remain (account progress), lifetime stats continue, new character can unlock more cards

---

## Balance Considerations

### Unlock Pacing
**Current Target:** 1 card per 1-2 hours (4-8 decks)

**Tuning Questions:**
- Too slow? (players feel gated)
- Too fast? (unlock all cards in 20 hours)
- Should unlocks accelerate? (more cards late-game)

**Metrics:**
- 50% cards unlocked by 20-30 hours
- 80% cards unlocked by 40-50 hours
- 100% cards unlocked by 60-80 hours

---

### Leaderboard Balance
**Current Design:** 3 leaderboards (Flash, Kingpin, Survivor)

**Tuning Questions:**
- Are all 3 viable? (or does one dominate)
- Should there be more leaderboards? (e.g., "Highest single deck profit")
- Should leaderboards reset? (monthly, seasonal)

**Metrics:**
- Player participation: 30%+ on at least one leaderboard
- Distribution: No single strategy dominates (33/33/33 split across Flash/Kingpin/Survivor)

---

### Permadeath Severity
**Current Design:** Lose Heat/Trust/character profit, keep cards/account stats

**Alternative:** Lose nothing (keep Heat/Trust across characters)

**Pros:** Less punishing, encourages continued play
**Cons:** No stakes, reduces tension

**Recommendation:** Keep permadeath (creates stakes, rewards mastery).

---

## Integration with Other Systems

**Requires:**
- Core Gameplay Loop (deck/hand completion detection)
- Card System (unlocked cards available for deck building)
- Heat & Trust (persist on character, lost on permadeath)
- Bust Mechanics (permadeath trigger)

**Feeds Into:**
- UI/UX (leaderboard display, unlock celebrations, character creation)
- Save System (account progress, character stats, unlocked cards)

---

## MVP Scope

### Phase 1 (Core Progression)
- Starting collection (30 cards)
- Account lifetime profit (tracks across characters)
- Account lifetime decks (tracks across characters)
- 5 profit milestones (unlock 5 cards)
- 5 deck milestones (unlock 5 cards)
- Character permadeath (lose Heat/Trust, keep cards)

### Phase 2 (Leaderboards)
- Flash leaderboard (7-day rolling window)
- Kingpin leaderboard (single run profit)
- Survivor leaderboard (single run decks)
- Leaderboard display (top 100 + personal rank)

### Phase 3 (Full Collection)
- Full collection (80-100 cards)
- 20+ profit milestones
- 20+ deck milestones
- 10+ achievements (unique unlocks)

### Future Enhancements
- Seasonal leaderboards (monthly reset)
- Character customization (cosmetic only)
- Leaderboard rewards (unique cards for top 10)
- Social features (share runs, compare with friends)

---

## Open Questions

### Unlock System
- Should unlocks require both profit AND decks? (e.g., "$10k profit AND 15 decks")
- Should unlocks have prerequisites? (e.g., "unlock Meth before Cocaine")
- Should unlocks be reversible? (e.g., "lock card if not used in 10 decks")

### Leaderboards
- Should leaderboards reset? (monthly, seasonal, never)
- Should there be more leaderboards? (e.g., "Lowest Heat at bust")
- Should leaderboards have rewards? (cosmetics, unique cards)

### Character System
- Should characters have mechanical differences? (e.g., College Student starts with +10 Cover)
- Should characters have unique starting decks? (e.g., Mafia Member starts with different cards)
- Should characters level up? (e.g., gain bonuses after 10 decks)

---

## Success Criteria

### Engagement Metrics
- Unlock rate: 1 card per 1-2 hours (sustained over 60+ hours)
- Character survival: 15-25 decks average per character
- Leaderboard participation: 30%+ players on at least one board

### Balance Metrics
- Leaderboard distribution: No single strategy dominates (33/33/33 across Flash/Kingpin/Survivor)
- Unlock distribution: 50% cards by 20-30 hours, 100% by 60-80 hours
- Permadeath impact: 70%+ players create new character immediately (not quit)

### Player Satisfaction
- Unlocks feel rewarding (celebrate each unlock)
- Permadeath feels fair (player had control)
- Leaderboards feel achievable (personal bests matter)
- Replayability high (new character = fresh experience)
