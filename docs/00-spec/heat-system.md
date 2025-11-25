# Heat System Specification

## Overview

Heat is a **persistent character stat** that creates consequences spanning multiple play sessions. Heat represents law enforcement pressure and controls **Narc card power level** - higher Heat means the Narc's cards are upgraded to stronger versions. This creates meaningful long-term decisions: short-term profit vs. long-term survival.

**Core Tension:** Heat climbs faster than it decays (inevitable death spiral), but smart play can extend survival significantly.

**Key Design:** Heat controls *difficulty* (how strong Narc cards are). Higher Heat = Narc cards are upgraded to stronger versions, mirroring how player cards upgrade with use.

---

## Player Experience Goal

Players should feel:
- **Persistent consequences** - Today's choices affect tomorrow's difficulty
- **Strategic planning** - Manage Heat over days/weeks, not just one session
- **Tension escalation** - Early run is forgiving, late run is brutal
- **Inevitable doom** - Eventually everyone gets caught (no permanent victory)
- **Anti-binge incentive** - Playing multiple decks in one day = Heat accumulation without decay

This is NOT about punishment for playing - it's about **creating meaningful choices with long-term consequences**.

---

## Heat System

### What Is Heat?

**Player Mental Model:**
"Heat is how hot the cops are on my trail. The hotter I am, the harder the Narc tries next time. It goes up when I make risky deals and slowly cools down when I'm not playing."

**Mechanical Definition:**
- Persistent integer value (0-150+)
- Accumulates from cards played during hands
- Decays over real-world time (not game time)
- Determines Narc deck composition for NEXT session

---

### Heat Accumulation (During Play)

**Rule:** Heat delta = sum of all Heat modifiers on cards played during hand.

**Example Hand:**
```
Player plays Meth: +30 Heat
Player plays School Zone: +20 Heat
Buyer plays Bulk Order: +20 Heat
Narc plays Surveillance: +5 Heat
Player plays Alibi: -5 Heat
---
Total Heat delta: +70 Heat
```

**Application:**
- Heat applies at END of hand (if not busted)
- If folded mid-hand: Keep Heat from rounds played
- If busted: Heat applies before run ends (matters for leaderboards)

**Player Feedback:**
- Show running Heat total: "Heat this hand: +70"
- Show projected new Heat: "Current Heat: 40 → New Heat: 110"
- Alert on tier thresholds: "Warning: Entering HOT tier (51-75) - Narc much harder next deck"

---

### Heat Decay (Between Sessions)

**Rule:** Heat decreases by 1 per real-world hour.

**Examples:**
```
Monday 8pm: Play deck, end at Heat 60
Tuesday 8pm (24 hours): Heat now 36 (60 - 24)

Friday 10am: Play deck, end at Heat 90
Friday 2pm (4 hours): Heat now 86 (90 - 4)
```

**Design Intent:**
- **Encourages daily play** - One deck per day keeps Heat manageable
- **Punishes binging** - Multiple decks in one session = Heat accumulates without decay
- **Rewards patience** - Wait 24 hours, lose 24 Heat (significant)

**Strategic Implications:**
- High Heat run? Take a day off (let it cool)
- Need leaderboard push? Accept high Heat consequences
- Low Heat run? Can play multiple decks same day safely

**Player Feedback:**
- Show time until next decay: "Next Heat -1: in 23 minutes"
- Show projected Heat: "In 24 hours: Heat will be 45 (if you don't play)"
- Show decay rate: "-1 Heat/hour (-24 Heat/day)"

---

### Heat Tiers (Narc Card Upgrade Levels)

Heat determines how upgraded the Narc's cards are. This mirrors player card upgrades - the Narc gets stronger as you get more notorious.

| Heat Range | Tier | Narc Card Power | Player Experience |
|------------|------|-----------------|-------------------|
| 0-25 (Cold) | Base | Narc cards at base stats | "Cops barely care" |
| 26-50 (Warm) | Tier 1 | Narc cards upgraded once | "Cops are watching" |
| 51-75 (Hot) | Tier 2 | Narc cards upgraded twice | "Active investigation" |
| 76-100 (Scorching) | Tier 3 | Narc cards upgraded three times | "They have a file on me" |
| 101+ (Inferno) | Tier 4 | Narc cards fully upgraded | "Run ends soon" |

**Example - Surveillance Card:**
- Base: +15 Evidence, +3 Heat
- Tier 1: +20 Evidence, +5 Heat
- Tier 2: +25 Evidence, +7 Heat
- Tier 3: +30 Evidence, +10 Heat
- Tier 4: +40 Evidence, +15 Heat

**Strategic Implications:**
- **Cold:** Play aggressively, Narc is weak
- **Warm:** Include Cover cards, Narc is competent
- **Hot:** Insurance recommended, Narc is dangerous
- **Scorching:** Insurance mandatory, fold often
- **Inferno:** Death spiral, accept run ending soon

_Exact upgrade values TBD - playtest to determine._

---

### Heat Affects NEXT Deck (Not Current)

**Key Design Rule:** Your current Heat determines Narc deck for the NEXT deck you play.

**Example Timeline:**
```
Deck 1 Start: Heat 20 → Narc deck is COLD (easy)
Deck 1 End: Heat 65
---
Deck 2 Start: Heat 65 → Narc deck is HOT (hard)
Deck 2 End: Heat 95
---
Wait 24 hours (Heat decays to 71)
---
Deck 3 Start: Heat 71 → Narc deck is HOT (hard)
```

**Why This Design?**
- **Predictable difficulty** - You know how hot you are before starting
- **Consequence visibility** - High Heat this deck = harder next deck
- **Strategic planning** - Can prepare deck for expected Narc difficulty

**Player Feedback:**
- Before deck start: "Current Heat: 65 (HOT tier) - Narc will be aggressive"
- Show Narc deck preview: "Expected Evidence: 60-90 per hand"
- Show Make It Stick chance: "Warrant/DA Approval: ~33% chance"

---

## Run Progression Arc

### Early Run (Low Heat)
**Situation:** Heat 0-25

**Experience:**
- Easy Narc (low threat)
- **Strategy:** Play aggressively, build profit, accept Heat

**Emotional Arc:** "Grinding, learning, building foundation"

---

### Mid Run (Medium Heat)
**Situation:** Heat 26-50

**Experience:**
- Moderate Narc (need Cover)
- **Strategy:** Selective play, balance profit and safety

**Emotional Arc:** "Hitting stride, confident, profitable"

---

### Late Run (High Heat)
**Situation:** Heat 51-75

**Experience:**
- Hard Narc (active investigation)
- **Strategy:** High-risk, high-reward, insurance mandatory

**Emotional Arc:** "Peak performance, dangerous, thrilling"

---

### Death Spiral (Very High Heat)
**Situation:** Heat 76+

**Experience:**
- Extreme Narc (Make It Stick common)
- **Strategy:** Ultra-defensive OR go-for-broke finale

**Emotional Arc:** "Inevitable end approaching, final sprint"

---

## Fun Factor Analysis

### Why Is This Engaging?

**1. Long-Term Consequences**
- Today's choices affect tomorrow (not just this hand)
- Creates narrative arc (rise and fall)
- Persistence creates attachment to character

**2. Tension Escalation**
- Early run forgiving → Late run brutal
- Natural difficulty curve (no manual adjustment)
- Inevitable doom creates urgency

**3. Strategic Depth**
- Heat management (when to cool down?)
- Timing (when to push for profit?)
- Deck building based on Heat tier

**4. Anti-Binge Mechanics**
- Real-time decay rewards daily play
- Punishes binging (Heat accumulates)
- But allows play if desired (multiple characters)

**5. Emergent Stories**
- "Took 3 days off to cool Heat from 90 to 18"
- "One bad night, Heat 40 → 120, run ended"

---

### Potential Pain Points

**1. Death Spiral Feels Bad**
- Heat climbs faster than decay (inevitable bust)?
- **Mitigation:** Communicate upfront (no permanent victory), allow Heat reduction cards, player chose high-Heat deals

**2. Real-Time Decay Feels Mandatory**
- Pressure to play daily (FOMO)?
- **Mitigation:** Allow multiple characters (play different character if main too hot), cap max Heat (can't climb above 150)

**3. Narc RNG Variance**
- 60% Make It Stick chance still means 40% easy draws?
- **Mitigation:** Procedural deck generation (consistent difficulty at Heat tier), player sees deck preview before starting

---

## Balance Considerations

### Heat Decay Rate
**Current:** -1 Heat per hour (-24 per day)

**Tuning Questions:**
- Too fast? (Heat never dangerous)
- Too slow? (death spiral too quick)
- Should decay be nonlinear? (e.g., faster decay at very high Heat)

**Target Metrics:**
- Average run length: 15-25 decks
- Average Heat at bust: 80-100
- Days per run: 10-20 days

---

## Integration with Other Systems

**Requires:**
- Core Gameplay Loop (hand completion detection)
- Card System (Heat modifiers on cards)

**Feeds Into:**
- Bust Mechanics (Make It Stick tied to Heat threshold)
- Progression System (Heat stats on character profile)
- Narc card upgrades (Heat determines Narc card power level)

---

## MVP Scope

### Phase 1 (Core System)
- Heat accumulation (sum card modifiers)
- Heat decay (real-time, -1/hour)
- Basic Narc card upgrades (2-3 tiers)

### Phase 2 (Full Scaling)
- 5 Heat tiers with distinct upgrade levels
- Narc card upgrade display (show current tier)
- Heat preview before deck start

### Future Enhancements
- Nonlinear Heat decay (faster at very high Heat)
- Heat events (random "Heat Wave" days with +10 Heat)

---

## Open Questions

### Heat System
- Should Heat cap at 150? (prevent infinite scaling)
- Should Heat reset on "Go Home" early? (currently no)
- Should some cards reset Heat entirely? (Rat Out Partner currently does)

### Balance
- What's the target run length? (15-25 decks? more?)
- Should there be a "win condition"? (e.g., earn $100k and retire?)

---

## Success Criteria

### Engagement Metrics
- Daily return rate: 50%+ (Heat decay incentivizes)
- Average session gap: 20-28 hours (one deck per day pattern)
- Multi-deck sessions: 10-20% (binging occasionally, not always)

### Difficulty Scaling
- Average run length: 15-25 decks
- Bust rate by Heat tier: Cold <5%, Warm 10-15%, Hot 25-35%, Scorching 40-50%, Inferno 60%+

### Player Satisfaction
- Heat feels fair (escalation telegraphed)
- Death spiral feels inevitable but not cheap (player chose high Heat)
- Permadeath feels meaningful (losing Heat progress matters)
