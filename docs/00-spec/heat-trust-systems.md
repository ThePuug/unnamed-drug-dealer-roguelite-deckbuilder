# Heat & Trust Systems Specification

## Overview

Heat and Trust are **persistent character stats** that create consequences spanning multiple play sessions. Heat represents law enforcement pressure and increases difficulty (harder Narc decks). Trust represents customer relationships and affects deal quality (better Customer decks). Both create meaningful long-term decisions: short-term profit vs. long-term survival.

**Core Tension:** Heat climbs faster than it decays (inevitable death spiral), but smart play can extend survival significantly.

---

## Player Experience Goal

Players should feel:
- **Persistent consequences** - Today's choices affect tomorrow's difficulty
- **Strategic planning** - Manage Heat over days/weeks, not just one session
- **Tension escalation** - Early run is forgiving, late run is brutal
- **Earned rewards** - High Trust feels like an achievement (better deals)
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
Customer plays Bulk Order: +20 Heat
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
- Alert on tier thresholds: "⚠️ Entering HOT tier (51-75) - Narc much harder next deck"

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

### Heat Tiers (Difficulty Scaling)

Heat determines Narc deck composition for your NEXT deck.

#### Cold (0-25 Heat)
**Player Experience:** "Cops barely care. I can get away with anything."

**Narc Deck:**
- 10× Donut Break (0 Evidence, 0 Heat)
- 3× Patrol (+5 Evidence, +2 Heat)
- 2× Surveillance (+20 Evidence, +5 Heat)
- 0× Make It Stick cards

**Average Evidence per hand:** 20-40
**Make It Stick chance:** 0%

**Strategic Advice:**
- Build profit aggressively (no serious threat)
- Accept high-Heat deals (you'll cool down later)
- Focus on Trust building (safe to complete hands)

---

#### Warm (26-50 Heat)
**Player Experience:** "Cops are watching. I need to be more careful."

**Narc Deck:**
- 5× Donut Break
- 3× Patrol
- 5× Surveillance
- 2× Wiretap (+30 Evidence, +10 Heat)
- 0× Make It Stick cards

**Average Evidence per hand:** 40-70
**Make It Stick chance:** 0%

**Strategic Advice:**
- Include Cover cards in deck (3-4 minimum)
- Avoid School Zone (too risky at this Heat)
- Consider folding if Evidence climbs above 60

---

#### Hot (51-75 Heat)
**Player Experience:** "Cops are actively investigating. One mistake and I'm done."

**Narc Deck:**
- 2× Donut Break
- 2× Patrol
- 6× Surveillance
- 3× Wiretap
- 2× Warrant (Make It Stick, threshold 40)

**Average Evidence per hand:** 60-90
**Make It Stick chance:** ~13% (2/15 cards)

**Strategic Advice:**
- **Insurance mandatory** (include 2× Get Out of Jail)
- Heavy Cover in deck (5+ cards)
- Fold if Warrant played and Heat > 40
- Reduce Heat (use Lay Low, Safe House)

---

#### Scorching (76-100 Heat)
**Player Experience:** "Cops have a file on me. Every deal is life or death."

**Narc Deck:**
- 0× Donut Break
- 0× Patrol
- 4× Surveillance
- 6× Wiretap
- 3× Warrant (threshold 40)
- 2× DA Approval (Make It Stick, threshold 60)

**Average Evidence per hand:** 80-110
**Make It Stick chance:** ~33% (5/15 cards)

**Strategic Advice:**
- **Insurance absolutely mandatory**
- Ultra-defensive deck (7+ Cover cards)
- Fold often (50%+ fold rate)
- Prioritize Heat reduction over profit
- Consider taking a day off (let Heat decay)

---

#### Inferno (101+ Heat)
**Player Experience:** "Run ends soon. Every hand could be the last. High-risk, high-reward final sprint."

**Narc Deck:**
- 0× Donut Break
- 0× Patrol
- 2× Surveillance
- 4× Wiretap
- 4× Sting Operation (+40 Evidence, +20 Heat)
- 3× DA Approval (threshold 60)
- 2× Federal Case (Make It Stick, threshold 80)

**Average Evidence per hand:** 100-140
**Make It Stick chance:** ~60% (9/15 cards)

**Strategic Advice:**
- **Death spiral** - Heat climbs faster than it can decay
- Go for broke (max profit before inevitable bust)
- OR play ultra-conservative (fold 80%+ hands, grind Heat down)
- Insurance may not be enough (Federal Case threshold 80)
- Accept that run will end soon

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

## Trust System

### What Is Trust?

**Player Mental Model:**
"Trust is how much my customers like me. Low Trust = bad deals (they don't respect me). High Trust = great deals (they want to do business)."

**Mechanical Definition:**
- Persistent integer value (0-50+)
- Increases by +1 per successful hand (stayed in all 3 rounds, didn't get busted)
- Decreases by -1 per folded hand (customer feels disrespected)
- Determines Customer deck composition for CURRENT session

---

### Trust Gain/Loss

**+1 Trust per successful hand:**
- Stayed in all 3 rounds (didn't fold)
- Hand resolved Safe (Evidence < Cover) OR NotBusted (insurance worked)
- **Does NOT require profit** (just completion)

**-1 Trust per folded hand:**
- Player folds before Round 3 completes
- Customer feels disrespected (you backed out)

**No Trust change:**
- Busted and run ends (no Trust loss, character permadeath)
- "Go Home" early between hands (not mid-hand fold)

**Strategic Implications:**
- **Early run:** Fold often (bad deals at Low Trust anyway)
- **Mid run:** Selective play (some deals worth completing)
- **Late run:** Push through hands (High Trust deals are excellent)

**Player Feedback:**
- Show Trust change: "Trust: 5 → 6 (+1 for completing hand)"
- Show Trust change: "Trust: 10 → 9 (-1 for folding)"
- Show Trust tier: "Low Trust - Customer offers bad deals"

---

### Trust Tiers (Deal Quality Scaling)

Trust determines Customer deck composition for your CURRENT deck.

#### Low Trust (0-3 successful deals)
**Player Experience:** "Customer doesn't trust me yet. Terrible deals, barely worth my time."

**Customer Deck:**
- 6× Small Order (Price ×0.5, +5 Evidence, +5 Heat)
- 5× Haggling (Price -$30, +5 Evidence)
- 2× Just Weed (Price ×0.3 if Meth/Heroin, ×1.0 if Weed/Pills)
- 1× Regular Order (Price ×1.0, +10 Evidence, +10 Heat)
- 1× Fair Deal (Price +$0, +0 Evidence, +0 Heat)

**Average profit multiplier:** ×0.4 (terrible)
**Average Evidence:** +7 per hand
**Average Heat:** +7 per hand

**Strategic Advice:**
- **Fold often** (deals suck, not worth Heat)
- Focus on surviving to build Trust
- Use low-Heat products (Weed only)
- Accept low profit (building foundation)

---

#### Medium Trust (4-10 successful deals)
**Player Experience:** "Customer is warming up. Some deals are worth it, some aren't."

**Customer Deck:**
- 3× Small Order
- 4× Regular Order
- 3× Bulk Order (Price ×1.5, +25 Evidence, +20 Heat)
- 2× Haggling
- 2× Fair Deal
- 1× Premium Buyer (Price +$40, -10 Evidence, -5 Heat)

**Average profit multiplier:** ×1.1 (decent)
**Average Evidence:** +15 per hand
**Average Heat:** +15 per hand

**Strategic Advice:**
- **Selective play** (stay in if Bulk Order or Premium Buyer)
- Fold if Haggling + Small Order combo
- Balance profit and Trust building
- Medium-Heat products viable (Pills, some Meth)

---

#### High Trust (11+ successful deals)
**Player Experience:** "Customer loves me. Huge orders, premium buyers, excellent profit."

**Customer Deck:**
- 2× Regular Order
- 5× Bulk Order
- 3× VIP Client (Price ×2.0, +40 Evidence, +30 Heat)
- 2× Premium Buyer
- 2× Loyal Customer (Price +$50, -10 Evidence, -10 Heat)
- 1× Hook Me Up (Price ×1.3 if Meth/Heroin, ×1.0 otherwise)

**Average profit multiplier:** ×1.6 (excellent)
**Average Evidence:** +25 per hand
**Average Heat:** +20 per hand

**Strategic Advice:**
- **Stay in most hands** (deals are worth it)
- High-Heat products profitable (Meth, Heroin)
- Need strong Cover (Evidence high)
- Insurance mandatory (Heat accumulation fast)

---

### Trust Persistence

**Trust persists:**
- ✅ Across all decks on same character
- ✅ Until character busted (permadeath)
- ❌ Does NOT decay over time
- ❌ Does NOT transfer to new character

**Design Intent:**
- Reward consistent play (Trust builds over many decks)
- Create progression arc (Low → Medium → High Trust over ~10-15 decks)
- Loss on permadeath creates meaningful consequence

**Player Feedback:**
- Show Trust history: "Trust: 12 (built over 18 decks, 12 successful hands)"
- Show Trust tier: "High Trust - Customer offers premium deals"
- Warn on permadeath: "If busted: Lose Trust 12 progress"

---

## Interaction Between Heat and Trust

### Early Run (Low Heat, Low Trust)
**Situation:** Heat 0-25, Trust 0-3

**Experience:**
- Easy Narc (low threat)
- Bad Customer (low profit)
- **Strategy:** Fold often, build Trust slowly, stay cool

**Emotional Arc:** "Grinding, learning, building foundation"

---

### Mid Run (Medium Heat, Medium Trust)
**Situation:** Heat 26-50, Trust 4-10

**Experience:**
- Moderate Narc (need Cover)
- Decent Customer (some good deals)
- **Strategy:** Selective play, balance profit and safety

**Emotional Arc:** "Hitting stride, confident, profitable"

---

### Late Run (High Heat, High Trust)
**Situation:** Heat 51-75, Trust 11+

**Experience:**
- Hard Narc (active investigation)
- Excellent Customer (huge deals)
- **Strategy:** High-risk, high-reward, insurance mandatory

**Emotional Arc:** "Peak performance, dangerous, thrilling"

---

### Death Spiral (Very High Heat, Variable Trust)
**Situation:** Heat 76+, any Trust

**Experience:**
- Extreme Narc (Make It Stick common)
- Customer quality matters less (survival > profit)
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
- Trust building (worth folding for?)
- Timing (when to push for profit?)

**4. Anti-Binge Mechanics**
- Real-time decay rewards daily play
- Punishes binging (Heat accumulates)
- But allows play if desired (multiple characters)

**5. Emergent Stories**
- "Made it to Trust 15 before Heat caught up"
- "Took 3 days off to cool Heat from 90 to 18"
- "One bad night, Heat 40 → 120, run ended"

---

### Potential Pain Points

**1. Death Spiral Feels Bad**
- Heat climbs faster than decay (inevitable bust)?
- **Mitigation:** Communicate upfront (no permanent victory), allow Heat reduction cards, player chose high-Heat deals

**2. Trust Loss on Fold Discourages Cautious Play**
- Feels punishing to fold (even when smart)?
- **Mitigation:** Make Low Trust deals obviously bad (players WANT to fold), Trust builds quickly (fold 3 times = lose 3, but complete 3 = gain 3)

**3. Real-Time Decay Feels Mandatory**
- Pressure to play daily (FOMO)?
- **Mitigation:** Allow multiple characters (play different character if main too hot), cap max Heat (can't climb above 150)

**4. Narc RNG Variance**
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

### Trust Build Rate
**Current:** +1 per successful hand, -1 per fold

**Tuning Questions:**
- Too slow? (takes too long to reach High Trust)
- Too fast? (High Trust trivializes early run)
- Should Trust require consecutive successes? (e.g., +1 only if no folds last 3 hands)

**Target Metrics:**
- Decks to High Trust: 8-12 decks (assuming 50% fold rate early, 20% fold rate mid)
- Average Trust at bust: 12-18

---

### Heat vs. Trust Tension
**Design Intent:** High Trust should be MOST VALUABLE when Heat is HIGH (best deals when most dangerous).

**Current Balance:**
- High Trust = +60% profit multiplier
- High Heat = -50% survival rate (more busts)
- Net effect: High risk, high reward

**Tuning Question:** Is High Trust valuable enough to justify extreme Heat risk?

---

## Integration with Other Systems

**Requires:**
- Core Gameplay Loop (hand completion detection)
- Card System (Heat modifiers on cards)

**Feeds Into:**
- Bust Mechanics (Make It Stick tied to Heat threshold)
- Progression System (Trust/Heat stats on character profile)
- Leaderboards (Heat/Trust at bust shown on boards)

---

## MVP Scope

### Phase 1 (Core Systems)
- Heat accumulation (sum card modifiers)
- Heat decay (real-time, -1/hour)
- Trust gain/loss (+1 success, -1 fold)
- Basic Narc deck scaling (3 tiers: Cold, Warm, Hot)
- Basic Customer deck scaling (2 tiers: Low, High)

### Phase 2 (Full Scaling)
- 5 Heat tiers (Cold, Warm, Hot, Scorching, Inferno)
- 3 Trust tiers (Low, Medium, High)
- Procedural deck generation (exact card counts)
- Deck previews (show expected Narc difficulty)

### Future Enhancements
- Nonlinear Heat decay (faster at very high Heat)
- Trust milestones (special Customer cards at Trust 20, 30, etc.)
- Heat events (random "Heat Wave" days with +10 Heat)
- Trust perks (High Trust unlocks special cards)

---

## Open Questions

### Heat System
- Should Heat cap at 150? (prevent infinite scaling)
- Should Heat reset on "Go Home" early? (currently no)
- Should some cards reset Heat entirely? (Rat Out Partner currently does)

### Trust System
- Should Trust decay over real-world time? (currently no decay)
- Should Trust require PROFIT, not just completion? (currently just completion)
- Should High Trust have downsides? (e.g., +10 Evidence from "known associate")

### Balance
- What's the target run length? (15-25 decks? more?)
- Should there be a "win condition"? (e.g., earn $100k and retire?)
- Should Heat/Trust curves be symmetric? (currently Heat harder to manage)

---

## Success Criteria

### Engagement Metrics
- Daily return rate: 50%+ (Heat decay incentivizes)
- Average session gap: 20-28 hours (one deck per day pattern)
- Multi-deck sessions: 10-20% (binging occasionally, not always)

### Difficulty Scaling
- Average run length: 15-25 decks
- Bust rate by Heat tier: Cold <5%, Warm 10-15%, Hot 25-35%, Scorching 40-50%, Inferno 60%+
- Fold rate by Trust tier: Low 60%, Medium 30%, High 15%

### Player Satisfaction
- Heat feels fair (escalation telegraphed)
- Trust feels rewarding (High Trust worth the grind)
- Death spiral feels inevitable but not cheap (player chose high Heat)
- Permadeath feels meaningful (losing Trust/Heat progress matters)
