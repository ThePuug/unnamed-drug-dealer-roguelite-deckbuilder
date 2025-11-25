# Bust & Insurance Mechanics Specification

## Overview

Bust mechanics determine when a run ends (character permadeath). The core rule is simple: **Evidence > Cover = Busted**. However, players can use **Get Out of Jail** insurance cards to survive busts, and the Narc can use **Make It Stick** conviction cards to override insurance at high Heat. This creates a high-stakes decision framework: is this hand worth risking your run?

**Core Tension:** Insurance is expensive but necessary at high Heat. Make It Stick overrides insurance, creating inevitable doom at extreme Heat.

---

## Player Experience Goal

Players should feel:
- **Fair consequence** - Bust is clear (Evidence > Cover), not hidden or random
- **Agency** - Can prevent bust (play Cover, fold early, use insurance)
- **Insurance value** - Get Out of Jail saves you, feels clutch, worth the cost
- **Escalating danger** - Make It Stick at high Heat creates dread
- **Informed risk** - Know when you're safe, when you're at risk, when you're doomed
- **Meaningful permadeath** - Losing character matters (Trust, Heat, run progress)

This is NOT about "gotcha" mechanics or hidden failure - it's about **transparent risk with counterplay options**.

---

## Core Bust Rule

### Simple Case: No Insurance, No Conviction

**Condition:** Evidence > Cover at end of hand

**Result:** Character busted (run ends, permadeath)

**Example:**
```
Final totals:
  Evidence: 75 (from Parking Lot base + Surveillance + Wiretap)
  Cover: 50 (from Parking Lot base + Alibi)

Evidence > Cover (75 > 50)
→ Busted
→ Run ends
→ Character arrested (permadeath)
```

**Player Feedback:**
- **During hand:** Color-coded totals (Green if safe, Yellow if close, Red if busted)
- **Before resolution:** "⚠️ EVIDENCE EXCEEDS COVER - You will be busted unless you fold or play Cover"
- **At bust:** "BUSTED - Evidence 75 > Cover 50 - Run Ends - Character Arrested"

---

## Insurance Cards (Get Out of Jail)

### What Are Insurance Cards?

**Player Mental Model:**
"My emergency escape. If I get busted, this card saves me... once. It costs me Heat and maybe money, but I keep my run alive."

**Mechanical Definition:**
- Card type: Get Out of Jail (7 card types)
- Acts as **Cover card** during hand (adds to Cover total)
- Acts as **insurance** at resolution (prevents bust if Evidence > Cover)
- **Single use per deck** (burned after use)
- Has **requirements** (pay money, take Heat penalty)

---

### Insurance Activation (Successful Save)

**Condition:** Evidence > Cover AND Get Out of Jail card played AND requirements met

**Process:**
1. Calculate Evidence overage: `overage = Evidence - Cover`
2. Check insurance requirements: `can_pay = (cash >= card_cost)`
3. If requirements met:
   - Pay cost (deduct from profit)
   - Gain Heat: `overage + card_penalty`
   - Bust negated (run continues)
   - Insurance burned (can't use again this deck)
4. If requirements NOT met:
   - Insurance fails
   - Run ends (busted)

**Example:**
```
Final totals:
  Evidence: 85
  Cover: 60

Evidence > Cover (85 > 60)
→ Bust triggered
→ Overage: 25

Player has Plea Bargain:
  Cost: $1,000
  Heat Penalty: +20

Player cash: $2,500 ✓ (can afford)

Resolution:
  Pay $1,000 (cash: $2,500 → $1,500)
  Gain Heat: 25 (overage) + 20 (penalty) = 45 Heat
  Bust negated
  Run continues
  Plea Bargain burned (can't use again)
```

**Player Feedback:**
- **Before bust:** "Insurance active: Plea Bargain (cost $1,000, Heat +20 on use)"
- **At bust:** "INSURANCE ACTIVATED - Plea Bargain"
- **After save:** "Paid $1,000, gained 45 Heat, run continues"
- **Status:** "Plea Bargain USED - cannot use again this deck"

---

### Insurance Failure (Can't Afford)

**Condition:** Evidence > Cover AND Get Out of Jail card played BUT requirements NOT met

**Result:** Insurance fails, run ends (busted)

**Example:**
```
Final totals:
  Evidence: 90
  Cover: 65

Evidence > Cover (90 > 65)
→ Bust triggered

Player has Plea Bargain:
  Cost: $1,000
  Heat Penalty: +20

Player cash: $800 ✗ (can't afford)

Resolution:
  Insurance fails (insufficient cash)
  Run ends (busted)
```

**Player Feedback:**
- **Before bust:** "⚠️ Plea Bargain requires $1,000 - you only have $800"
- **At bust:** "INSURANCE FAILED - Insufficient funds ($800 / $1,000)"
- **Result:** "Run Ends - Character Arrested"

---

### Insurance as Cover (Not Busted)

**Condition:** Evidence < Cover (or Evidence = Cover) AND Get Out of Jail card played

**Result:** Insurance NOT consumed (still available), acts as pure Cover card

**Example:**
```
Final totals:
  Evidence: 70
  Cover: 85 (includes +20 from Plea Bargain)

Evidence < Cover (70 < 85)
→ Safe (no bust)
→ Plea Bargain NOT consumed
→ Still available for future hands
```

**Player Feedback:**
- **At resolution:** "Safe - Evidence 70 < Cover 85"
- **Insurance status:** "Plea Bargain unused (still available)"

---

## Conviction Cards (Make It Stick)

### What Are Conviction Cards?

**Player Mental Model:**
"The Narc has a Warrant. If they bust me AND my Heat is high enough, my insurance won't work. I'm screwed if I get caught."

**Mechanical Definition:**
- Card type: Make It Stick (7 card types)
- Played by Narc AI
- Has **Heat threshold** (Warrant: 40, DA Approval: 60, Federal Case: 80, Caught Red-Handed: 0)
- **Overrides insurance** if threshold met (insurance fails)
- Only one active (override rule like Products/Locations)

---

### Conviction Activation (Insurance Override)

**Condition:** Evidence > Cover AND Make It Stick played AND current_heat >= threshold

**Process:**
1. Calculate bust: `Evidence > Cover`
2. Check Make It Stick threshold: `current_heat >= threshold`
3. If threshold met:
   - Insurance overridden (Get Out of Jail fails)
   - Run ends (busted)
4. If threshold NOT met:
   - Check insurance (Get Out of Jail can work)

**Example:**
```
Final totals:
  Evidence: 95
  Cover: 70
  Current Heat: 65

Evidence > Cover (95 > 70)
→ Bust triggered

Narc played DA Approval:
  Heat Threshold: 60

Player played Plea Bargain:
  Cost: $1,000
  Heat Penalty: +20

Check threshold:
  65 >= 60 ✓ (threshold met)

Resolution:
  DA Approval overrides insurance
  Plea Bargain FAILS
  Run ends (busted)
```

**Player Feedback:**
- **When Narc plays:** "⚠️ DA APPROVAL ACTIVE - Threshold: 60 Heat"
- **Warning:** "Your Heat (65) exceeds threshold - Insurance WILL NOT WORK if busted"
- **At bust:** "DA APPROVAL OVERRIDES INSURANCE - Plea Bargain FAILED"
- **Result:** "Run Ends - Character Convicted"

---

### Conviction Below Threshold (Insurance Works)

**Condition:** Evidence > Cover AND Make It Stick played BUT current_heat < threshold

**Result:** Insurance can work (Make It Stick inactive)

**Example:**
```
Final totals:
  Evidence: 85
  Cover: 60
  Current Heat: 35

Evidence > Cover (85 > 60)
→ Bust triggered

Narc played Warrant:
  Heat Threshold: 40

Player played Plea Bargain:
  Cost: $1,000
  Heat Penalty: +20
  Player cash: $2,000

Check threshold:
  35 < 40 ✗ (threshold NOT met)

Resolution:
  Warrant inactive (threshold not met)
  Check insurance: Plea Bargain works
  Pay $1,000, gain 45 Heat
  Run continues
```

**Player Feedback:**
- **When Narc plays:** "⚠️ WARRANT ACTIVE - Threshold: 40 Heat"
- **Status:** "Your Heat (35) is below threshold - Insurance WILL WORK if busted"
- **At bust:** "Warrant inactive (Heat below threshold)"
- **Insurance:** "Plea Bargain activated - Run continues"

---

## Resolution Flow (Complete Decision Tree)

### End of Hand Resolution

```
Calculate totals:
  evidence = sum(all Evidence sources)
  cover = sum(all Cover sources)

IF evidence <= cover:
  // Safe - no bust
  Bank profit
  Apply Heat delta
  Continue to next hand

ELSE:
  // Evidence > Cover - bust triggered

  IF Make It Stick card played:
    IF current_heat >= Make It Stick threshold:
      // Conviction applies - insurance overridden
      RUN ENDS (busted - conviction sticks)
      END
    ELSE:
      // Threshold not met - fall through to insurance check

  IF Get Out of Jail card played:
    IF can pay requirements (cash >= cost):
      // Insurance works
      Pay cost
      Gain Heat: (evidence - cover) + card_penalty
      Burn insurance card
      Run continues
      Continue to next hand
    ELSE:
      // Can't afford insurance
      RUN ENDS (busted - insurance failed)
      END

  // No insurance or insurance failed
  RUN ENDS (busted)
  END
```

---

## Edge Cases and Interactions

### Multiple Insurance Cards

**Scenario:** Player has Plea Bargain (Round 1) and Fake ID (Round 2).

**Resolution:**
- Override rule applies (only one insurance active)
- Fake ID replaces Plea Bargain
- Only Fake ID can save you
- Plea Bargain discarded (wasted)

**Player Feedback:**
- "Plea Bargain → Fake ID (previous insurance replaced)"
- "Warning: Only Fake ID active (Plea Bargain wasted)"

**Strategic Lesson:** Don't play multiple insurance cards.

---

### Multiple Conviction Cards

**Scenario:** Narc plays Warrant (threshold 40) in Round 1, then DA Approval (threshold 60) in Round 2.

**Resolution:**
- Override rule applies (only one conviction active)
- DA Approval replaces Warrant
- Only DA Approval threshold matters (60, not 40)

**Player Feedback:**
- "Warrant → DA Approval (threshold raised: 40 → 60)"

**Strategic Implication:** Narc can escalate conviction threshold mid-hand.

---

### Insurance Played After Fold Decision Point

**Scenario:** Player folds in Round 2, but had insurance in hand.

**Resolution:**
- Fold ends hand (no bust check)
- Insurance never used (still available)
- Cards in hand discarded

**Player Feedback:**
- "Folded - Insurance not needed (hand ended early)"

---

### Bust Check with Tie (Evidence = Cover)

**Scenario:** Evidence: 70, Cover: 70

**Resolution:**
- Evidence <= Cover → Safe (no bust)
- Ties go to player

**Player Feedback:**
- "Safe (tie) - Evidence 70 = Cover 70"

---

### Insurance Burned Mid-Deck

**Scenario:** Plea Bargain used in Hand 1, player draws new hand in Hand 2.

**Resolution:**
- Plea Bargain burned (discarded from deck)
- Hand 2: No insurance available (unless different card in deck)

**Player Feedback:**
- Deck counter: "11 cards remaining (Plea Bargain burned)"
- Insurance status: "No insurance in deck"

---

### Caught Red-Handed (Threshold 0)

**Scenario:** Narc plays Caught Red-Handed (threshold: 0), Evidence > Cover.

**Resolution:**
- Threshold always met (Heat always >= 0)
- Insurance ALWAYS overridden
- Instant bust (no save possible)

**Player Feedback:**
- "⚠️ CAUGHT RED-HANDED - Insurance WILL NOT WORK (no threshold)"
- "At bust: CAUGHT RED-HANDED - Instant conviction - Run Ends"

**Strategic Implication:** Nuclear option for Narc (very rare card, high-Heat only).

---

## Insurance Card Design

### Example Insurance Cards

**Plea Bargain**
- Cover: +20
- Cost: $1,000
- Heat Penalty: +20
- **Design:** Balanced (moderate cost, moderate Heat)

**Fake ID**
- Cover: +15
- Cost: $0
- Heat Penalty: +40
- **Design:** Free but expensive Heat (desperate play)

**Rat Out Partner**
- Cover: +30
- Cost: $500
- Heat Penalty: Reset Heat to 0
- **Design:** High-risk (lose profit) but resets Heat (long-term benefit)

**Witness Protection**
- Cover: +35
- Cost: $1,500
- Heat Penalty: +25
- **Design:** Expensive but strongest Cover (rich players)

**Bribe the Judge**
- Cover: +25
- Cost: $2,000
- Heat Penalty: +10
- **Design:** Very expensive cash but low Heat (profit-focused)

---

### Insurance Balance Considerations

**Cost vs. Heat Tradeoff:**
- High cash cost + low Heat = profit-focused (Bribe the Judge)
- Low/no cash cost + high Heat = survival-focused (Fake ID)

**Cover Value:**
- Insurance provides Cover (useful even if not busted)
- Higher Cover = more flexible (can save larger overages)

**Single Use Constraint:**
- Burned after use (deck management critical)
- Can't spam insurance (need multiple in deck for multi-hand safety)

---

## Conviction Card Design

### Example Conviction Cards

**Warrant**
- Heat Threshold: 40
- **Design:** Early Make It Stick (appears at Hot tier 51-75 Heat)

**DA Approval**
- Heat Threshold: 60
- **Design:** Mid-tier (appears at Scorching 76-100 Heat)

**Federal Case**
- Heat Threshold: 80
- **Design:** Late-tier (appears at Inferno 101+ Heat)

**Caught Red-Handed**
- Heat Threshold: 0 (always)
- **Design:** Nuclear option (very rare, Inferno only)

---

### Conviction Balance Considerations

**Heat Threshold Curve:**
- Thresholds match Heat tiers (Warrant: 40 at Hot, DA: 60 at Scorching, Federal: 80 at Inferno)
- Players at high Heat vulnerable (insurance less reliable)

**Conviction Frequency:**
- Heat 0-50: 0% chance (no Make It Stick in Narc deck)
- Heat 51-75: ~13% chance (2/15 Warrant)
- Heat 76-100: ~33% chance (3 Warrant, 2 DA Approval)
- Heat 101+: ~60% chance (3 DA, 2 Federal, 1 Caught Red-Handed)

**Design Intent:** Make It Stick escalates with Heat (insurance becomes unreliable at extreme Heat).

---

## Fun Factor Analysis

### Why Is This Engaging?

**1. Transparent Risk**
- Evidence and Cover visible (no hidden bust)
- Threshold clear (know when insurance works)
- Can calculate safety margin ("need 20 more Cover")

**2. Counterplay Options**
- Play Cover cards (prevent bust)
- Fold early (avoid bust check)
- Use insurance (survive bust)
- Reduce Heat (make insurance reliable)

**3. Insurance Feels Clutch**
- Saves you from permadeath (high-stakes)
- Single use creates tension (don't waste it)
- Cost creates trade-off (profit vs. safety)

**4. Escalating Danger**
- No conviction early (insurance reliable)
- Conviction at high Heat (insurance unreliable)
- Inevitable doom at extreme Heat (no safety)

**5. Informed Decisions**
- Know when you're safe (Evidence < Cover)
- Know when you're at risk (Evidence > Cover but have insurance)
- Know when you're doomed (Make It Stick + threshold met)

---

### Potential Pain Points

**1. Insurance Failure Feels Bad**
- Lose run despite having insurance (can't afford cost)?
- **Mitigation:** Clear cost display, warning before bust, player chose to stay in without enough cash

**2. Make It Stick Feels Unfair**
- Insurance overridden feels like "gotcha"?
- **Mitigation:** Telegraphed (Heat threshold visible), player chose to play at high Heat, rare at low Heat

**3. Single Use Limits Strategy**
- Can't rely on insurance for multiple hands?
- **Mitigation:** Include multiple insurance cards in deck (2-3), strategic deck building matters

**4. High Cost Reduces Profit**
- Insurance eats profit (pay $1k to save run, but lose money)?
- **Mitigation:** Surviving is worth it (can earn profit in future decks), alternative: Fake ID (free but high Heat)

---

## Balance Considerations

### Insurance Cost vs. Profit
**Design Question:** Should insurance cost be proportional to profit earned?

**Current Design:** Fixed cost (Plea Bargain: $1,000 regardless of hand profit)

**Alternative:** Percentage cost (Plea Bargain: 50% of hand profit)

**Pros:** Scales with risk (high-profit hands = high insurance cost)
**Cons:** Complex calculation, feels punishing on lucky hands

**Recommendation:** Keep fixed cost (simpler, more predictable).

---

### Make It Stick Frequency
**Current Design:**
- Heat 51-75: 13% chance
- Heat 76-100: 33% chance
- Heat 101+: 60% chance

**Tuning Questions:**
- Too common at high Heat? (insurance useless)
- Too rare at low Heat? (no threat)

**Target Metrics:**
- Insurance success rate: 70% at Heat 51-75, 50% at 76-100, 30% at 101+

---

### Insurance Burn Rate
**Current Design:** Single use per deck (burned after trigger)

**Alternative:** Cooldown (once per 3 hands)

**Pros:** More forgiving, allows multiple saves
**Cons:** Less tension, reduces deck-building strategy

**Recommendation:** Keep single use (creates tension, rewards deck planning).

---

## Integration with Other Systems

**Requires:**
- Core Gameplay Loop (hand resolution)
- Card System (Get Out of Jail and Make It Stick cards)
- Heat System (conviction threshold checks)

**Feeds Into:**
- Progression System (permadeath triggers character loss)
- Leaderboards (bust ends run, logs stats)

---

## MVP Scope

### Phase 1 (Core Bust)
- Evidence > Cover check
- Simple bust (run ends)
- No insurance, no conviction

### Phase 2 (Insurance)
- Get Out of Jail cards (2 types: Plea Bargain, Fake ID)
- Insurance activation (pay cost, gain Heat)
- Insurance failure (can't afford)

### Phase 3 (Conviction)
- Make It Stick cards (2 types: Warrant, DA Approval)
- Heat threshold checks
- Insurance override

### Future Enhancements
- More insurance types (8-10 total cards)
- More conviction types (6-8 total cards)
- Insurance cooldowns (can't use same card twice in 5 hands)
- Conviction escalation (Narc can upgrade Warrant → DA mid-hand)

---

## Open Questions

### Insurance Mechanics
- Should insurance provide Cover only if NOT triggered? (currently provides Cover always)
- Should insurance have limited charges? (e.g., 2 uses before burn)
- Should insurance cost scale with overage? (bigger bust = bigger cost)

### Conviction Mechanics
- Should Make It Stick have additional requirements? (e.g., "needs 2 Evidence cards played this hand")
- Should Caught Red-Handed be removable? (currently no counterplay)

### Balance
- What's the ideal insurance burn rate? (1 use per deck vs. 1 use per 3 hands)
- Should conviction be more/less common? (current frequency too high/low?)
- Should there be "anti-conviction" cards? (e.g., "Mistrial" negates Make It Stick)

---

## Success Criteria

### Player Understanding
- 100% of players understand Evidence > Cover = bust by 1st hand
- 90% of players understand insurance activation by 3rd use
- 80% of players understand Make It Stick override by 1st encounter

### Balance Metrics
- Insurance success rate: 70% overall (varies by Heat)
- Bust rate with insurance: 30% (Make It Stick or can't afford)
- Bust rate without insurance: 80%+ (Evidence > Cover common at high Heat)

### Player Satisfaction
- Bust feels fair (clear telegraphing)
- Insurance feels valuable (saves run, worth cost)
- Make It Stick feels dangerous (escalates threat at high Heat)
- Permadeath feels meaningful (losing character matters)
