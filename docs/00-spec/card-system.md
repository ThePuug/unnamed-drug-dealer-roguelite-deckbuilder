# Card System Specification

## Overview

The game uses **7+ distinct card types** that interact to create drug deals. Cards affect four key values: **Evidence** (risk of arrest), **Cover** (defense against arrest), **Heat** (persistent difficulty), and **Profit** (money earned). Players, AI opponents (Narc, Customer), and the **Dealer** (community cards) all contribute cards, creating dynamic situations where everyone's cards matter.

**Core Mechanic:** Override system for Products and Locations (last played wins) + additive/multiplicative modifiers for everything else.

**Updated:** 2025-11-15 (Reflects RFC-010 product expansion and tagging system)

---

## Player Experience Goal

Players should feel:
- **Tactical depth** - Which card to play when matters immensely
- **Risk calculation** - Every card has tradeoffs (profit vs. evidence, cover vs. heat)
- **Counterplay** - Can react to opponent cards with smart plays
- **Build variety** - Different deck compositions support different strategies
- **Clear cause and effect** - Understand exactly why totals changed

This is NOT about memorizing combos or hidden interactions - it's about **transparent mechanics with meaningful choices**.

---

## Deck Distributions (Per RFC-005)

**Player Deck (20 cards - Dealer Theme):**
- ❌ **NO Evidence cards** (removed per RFC-005 - was creating anti-fun)
- ❌ **NO Conviction cards** (moved to Narc deck per RFC-005)
- Products: 9 distinct products (RFC-010: expanded variety with tagging system)
- Locations: 4 safe dealer locations (RFC-010: all have good Cover, low Evidence, negative Heat)
- Cover: Multiple defensive cards
- Insurance: Bust protection cards
- Deal Modifiers: Price multiplier cards

**Product Design Principles (RFC-010):**
- Each product has distinct identity and thematic purpose
- Tagged by Drug Class, Use Context, Legal Schedule, Risk Profile, Market Tier
- Range from Budget/Low-Heat (Weed) to Premium/High-Heat (Fentanyl)
- 9 total products providing variety without overwhelming choice

**Location Design Principles (RFC-010):**
- Player locations are dealer's SAFE choices (good Cover, low Evidence)
- All player locations have negative Heat (dealer picked these for a reason)
- Tagged by Privacy Level and Location Type
- Buyer can override with contextual locations (their turf, their terms)

**Narc Deck (25 cards - Law Enforcement Theme):**
- Evidence: 17 cards (variety of threat levels from Donut Break to Raid)
- Conviction: 8 cards (Warrant, DA Approval, RICO Case - prevent insurance)
- ✅ **Conviction moved here** from player deck

**Buyer Reaction Decks (RFC-009, RFC-010):**
- 7 cards per Buyer persona (3 visible, played randomly)
- Each deck follows structured template:
  - 1 Evidence modifier
  - 1 Cover modifier
  - 2 Location cards (1 safe, 1 risky - override player's choice)
  - 2 Price modifiers (1 up, 1 down - affect profit)
  - 1 Heat/thematic modifier
- ✅ **Replaces Customer + Dealer decks** - Simplified from 3 players to 2

**Buyer Scenarios (RFC-010):**
- Each Buyer has 2 scenarios (different Product/Location preferences)
- Scenario chosen at Buyer selection, persists across hands
- Scenarios have distinct Heat thresholds (same Buyer, different risk tolerance)

**Demand Validation (two-tier system):**
- **Deal Validity:** ANY Product + ANY Location makes deal valid ($0 → actual profit)
- **Demand Satisfaction:** Determines multiplier (base vs reduced)
  - Product demand satisfied: Active Product matches ANY of scenario's desired products
  - Location demand satisfied: Active Location matches ANY of scenario's preferred locations
  - BOTH satisfied → base_multiplier (×2.5, ×1.2, ×2.8)
  - Either not satisfied → reduced_multiplier (×1.0)

---

## Core Card Types

### 1. Product (What You're Selling)

**Player Experience:**
"This is what I'm actually selling. Weed is safe but low profit. Meth is dangerous but lucrative. I can change my mind mid-hand if risk gets too high."

**Mechanics:**
- Playing a Product **overrides** previous Product (only one active)
- Last Product played = what you're actually selling
- Has **base price** (modified by Customer/Deal Modifier cards)
- Has **Heat modifier** (Weed +5, Pills +15, Meth +30, Heroin +45)

**Strategic Use:**
- Early round: Play safe Product (Weed)
- Later round: Upgrade to risky Product (Meth) if Evidence is manageable
- Multiple Products in deck = flexibility to pivot

**Example Cards:**
```
Weed
  Base Price: $30
  Heat: +5

Pills
  Base Price: $60
  Heat: +15

Meth
  Base Price: $100
  Heat: +30

Heroin
  Base Price: $150
  Heat: +45
```

**Player Feedback:**
- "Current Product: Meth" (shows active Product)
- "If you played Weed: -$70 profit, -25 Heat" (show swap impact)

---

### 2. Location (Where Deal Happens)

**Player Experience:**
"This is where the deal goes down. Safe House is secure but low profit. School Zone is profitable but super risky. I can relocate mid-hand if Heat gets too hot."

**Mechanics:**
- Playing a Location **overrides** previous Location (only one active)
- Last Location played = where deal actually happens
- Has **base Evidence** and **base Cover**
- Has **Heat modifier**

**Strategic Use:**
- Early round: Risky Location (School Zone) for profit
- Later round: Safe Location (Safe House) if Evidence climbing
- Multiple Locations in deck = flexibility to escape danger

**Example Cards:**
```
Safe House
  Evidence: 10
  Cover: 30
  Heat: -5

Parking Lot
  Evidence: 25
  Cover: 15
  Heat: 0

School Zone
  Evidence: 40
  Cover: 5
  Heat: +20

Abandoned Warehouse
  Evidence: 15
  Cover: 25
  Heat: -10
```

**Player Feedback:**
- "Current Location: Parking Lot" (shows active Location)
- "If you played Safe House: -15 Evidence, +15 Cover" (show swap impact)

---

### 3. Deal Modifier (Adjust Deal Terms)

**Player Experience:**
"These cards tweak the deal. Disguise makes me safer. Bulk Sale Pressure increases profit but also risk. I stack these for compound effects."

**Mechanics:**
- Stack **additively** or **multiplicatively**
- Apply to Price, Evidence, Cover, Heat
- Can be played by any player (Player, Narc, Customer)

**Player Cards (Defensive):**
```
Disguise
  Cover: +20
  Heat: -5

Lookout
  Cover: +15
  Heat: 0

Bulk Sale Pressure (Player-initiated volume)
  Price: ×1.3
  Evidence: +10
```

**Narc Cards (Offensive):**
```
Heat Wave (increased pressure)
  Evidence: +15
  Heat: +10

Undercover Op
  Evidence: +20
  Heat: +5
```

**Customer Cards (Variable):**
```
Bulk Order (customer wants more)
  Price: ×1.5
  Evidence: +25
  Heat: +20

Haggling (customer wants discount)
  Price: -$30
  Evidence: +5

Premium Buyer (quality customer)
  Price: +$40
  Evidence: -10
  Heat: -5
```

**Strategic Use:**
- Early round: Play offensive modifiers (Bulk Sale Pressure) before opponents react
- Mid round: Play defensive modifiers (Disguise) to counter Narc Evidence
- Stack multiple: Disguise + Lookout = +35 Cover total

**Player Feedback:**
- Show modifier stack: "Price: $100 (base) × 1.5 (Bulk Order) × 1.3 (Bulk Sale Pressure) = $195"
- Show modifier sum: "Cover: 15 (Parking Lot) + 20 (Disguise) + 15 (Lookout) = 50"

---

### 4. Evidence (Narc's Investigation)

**Player Experience:**
"The Narc is building a case against me. Each Evidence card makes arrest more likely. I need Cover cards to defend."

**Mechanics:**
- Pure Evidence boost (additive only)
- Stacks with Location base Evidence
- Played by Narc AI

**Example Cards:**
```
Patrol
  Evidence: +5
  Heat: +2

Surveillance
  Evidence: +20
  Heat: +5

Wiretap
  Evidence: +30
  Heat: +10

Sting Operation
  Evidence: +40
  Heat: +20
```

**Strategic Response:**
- Counter with Cover cards
- Override Location to safer one (Safe House)
- Play insurance card (Get Out of Jail)

**Player Feedback:**
- "Evidence climbing: 70 total" (alert threshold: yellow at 50, red at 70+)
- "Need 20 more Cover to be safe" (show gap)

---

### 5. Cover (Player's Defense)

**Player Experience:**
"These cards protect me from arrest. Alibi reduces Heat too. Lawyer Up is expensive but bulletproof. I need to time these right."

**Mechanics:**
- Pure Cover boost (additive only)
- Offsets Evidence only (doesn't reduce Heat)
- Stacks with Location base Cover
- Played by Player

**Example Cards:**
```
Alibi
  Cover: +30
  Heat: -5

Lawyer Up
  Cover: +40
  Heat: 0

Lay Low
  Cover: +10
  Heat: -15

Bribe
  Cover: +25
  Heat: +10
```

**Strategic Use:**
- Hold until late round (see full Evidence threat first)
- Stack multiple if Evidence extreme (Alibi + Lawyer Up = +70 Cover)
- Consider Heat impact (Lay Low reduces Heat more than Cover)

**Player Feedback:**
- "Cover needed: 25" (show gap between Evidence and Cover)
- "Safe margin: +10 Cover" (show buffer)

---

### 6. Get Out of Jail (Player's Insurance)

**Player Experience:**
"My emergency escape. If I get busted, this saves me... once. Expensive but necessary at high Heat. I can only use it once per deck, so timing matters."

**Mechanics:**
- Playing one **overrides** previous Get Out of Jail (only one active)
- Acts as **both Cover card AND insurance**
- Negates bust if Evidence > Cover (unless Make It Stick overrides)
- **Single use per deck** (burned after use)
- Has **requirements** (pay money, take Heat penalty)

**Example Cards:**
```
Plea Bargain
  Cover: +20
  Heat: +20
  Requirement: Pay $1,000

Fake ID
  Cover: +15
  Heat: +40
  Requirement: None

Rat Out Partner
  Cover: +30
  Heat: Reset to 0
  Requirement: Pay $500

Witness Protection
  Cover: +35
  Heat: +25
  Requirement: Pay $1,500
```

**Strategic Use:**
- Include in deck when Heat 60+ (mandatory at high Heat)
- Play preemptively if Evidence high (acts as Cover)
- Save until hand resolution (insurance only triggers if busted)
- Understand cost (Plea Bargain costs $1k - worth it?)

**Player Feedback:**
- "Insurance active: Plea Bargain" (show active insurance)
- "Cost on bust: $1,000 + 20 Heat" (show penalty)
- "USED - cannot use again this deck" (after trigger)

---

### 7. Make It Stick (Narc's Conviction)

**Player Experience:**
"Oh no. The Narc has a Warrant. If my Heat is over 40 AND I get busted, it's game over. Insurance won't save me. I need to fold or reduce Evidence."

**Mechanics:**
- Playing one **overrides** previous Make It Stick (only one active)
- Makes bust **permanent** if Heat threshold met
- **Overrides Get Out of Jail cards** (insurance fails)
- Has **Heat threshold** (bust only if Heat >= threshold)

**Example Cards:**
```
Warrant
  Heat Threshold: 40

DA Approval
  Heat Threshold: 60

Federal Case
  Heat Threshold: 80

Caught Red-Handed
  Heat Threshold: 0 (always sticks)
```

**Strategic Response:**
- If threshold met: Fold immediately OR reduce Evidence to be safe
- If threshold not met: Continue (insurance will work)
- Narc deck scales with Heat (more Make It Stick at high Heat)

**Player Feedback:**
- "⚠️ WARRANT ACTIVE - Heat 65 / 40 threshold" (alert when threshold met)
- "Insurance WILL NOT WORK if busted" (clear warning)
- "Options: Fold now OR play 30+ Cover" (show outs)

---

### 8. Dealer Cards (Community/Scenario Cards) - NEW in RFC-008

**Player Experience:**
"The Dealer just revealed Police Checkpoint - Evidence jumped by 30! Do I fold now or push through? This is like the river card in poker - changes everything."

**Mechanics:**
- Dealer has separate 20-card deck
- 3 cards drawn per hand (face-down)
- One card revealed per round (after Player Phase)
- Affects ALL players (community cards)
- Dealer cards integrate into override system and totals

**Dealer Card Types:**

**Dealer Location Cards (8 cards):**
- Set base Evidence and Cover values
- Can be overridden by player Location cards (last played wins)
- Examples:
  - Private Residence (10 Evidence, 25 Cover, -10 Heat) [SAFE]
  - Parking Lot (25 Evidence, 15 Cover, 0 Heat) [NEUTRAL]
  - Police Checkpoint (30 Evidence, 0 Cover, +15 Heat) [DANGEROUS]
  - School Zone (35 Evidence, 5 Cover, +25 Heat) [VERY DANGEROUS]

**Dealer Modifier Cards (8 cards):**
- Adjust Evidence/Cover/Heat additively
- Cannot be overridden
- Examples:
  - Quiet Night (+5 Evidence, +10 Cover, -5 Heat) [HELPFUL]
  - Heat Wave (+15 Evidence, +0 Cover, +10 Heat) [HARMFUL]
  - Rival Dealer (if win: +30 Heat) [CONDITIONAL]

**Dealer Wild Cards (4 cards):**
- High-impact swings
- Examples:
  - Lucky Break (-20 Evidence) [VERY HELPFUL]
  - Bad Intel (+25 Evidence) [VERY HARMFUL]

**Strategic Use:**
- Dealer reveals create uncertainty (can't predict outcome)
- Can override Dealer Locations with player Locations
- Cannot counter Dealer Modifiers (must adapt)
- Creates "river tension" (final reveal can save or doom you)

**Player Feedback:**
- "Dealer reveals Police Checkpoint" (dramatic reveal)
- "Evidence: 45 → 75 (+30 from Police Checkpoint)"
- "Dealer has 2 cards remaining" (unrevealed count)
- Visual emphasis on Dealer reveals (different from player cards)

---

## Product and Location Tagging System (RFC-010)

### Purpose

Tags enable conditional logic, Buyer preferences, and future mechanics without hardcoding specific card names.

### Product Tags

**Drug Class:** Categorizes pharmacological effects
- Stimulant, Depressant, Psychedelic, Cannabis, Party

**Use Context:** Why people want this drug
- Party, Medical/Prescription, Street, Performance-Enhancing

**Legal Schedule:** DEA classification affects penalties
- Schedule I (highest), Schedule II, Schedule III

**Risk Profile:** How much Heat this drug generates
- HighHeat, ModerateHeat, LowHeat

**Market Tier:** Price range and accessibility
- Premium, MidTier, Budget

**Future Applications:**
- Buyer scenario preferences (wants Stimulants, avoids Psychedelics)
- Conditional modifiers (Public location + Schedule I = +Evidence)
- Special events (Fentanyl crisis increases Evidence for Prescription drugs)
- Deck building hints (show which products satisfy current Buyer)

### Location Tags

**Privacy Level:** How exposed the location is
- Private, SemiPrivate, Public

**Location Type:** Contextual category
- Residential, Industrial, Commercial, Educational, Recreational

**Future Applications:**
- Buyer scenario location preferences
- Conditional modifiers (Public + HighHeat product = extra Evidence)
- Heat decay (Private locations cool down faster)
- Special rules (Educational + Schedule I = severe penalties)

---

## Card Interaction Rules

### Override System (Products, Locations, Insurance, Conviction)

**Rule:** Playing a card of these types replaces the previous card of that type.

**Example:**
```
Round 1: Player plays Weed
Round 2: Player plays Meth
Result: Meth is active (Weed discarded)
```

**Why This Matters:**
- Flexibility (can change mind mid-hand)
- Tactical pivoting (upgrade Product if safe, downgrade if risky)
- Last-play advantage (can override opponent's Location)

**Player Feedback:**
- "Weed → Meth" (show replacement)
- "Previous card discarded" (make clear old card gone)

---

### Additive Stacking (Evidence, Cover, Deal Modifiers with +/-)

**Rule:** Cards with +/- values add together.

**Example:**
```
Parking Lot: 25 Evidence (base)
Narc plays Surveillance: +20 Evidence
Narc plays Wiretap: +30 Evidence
Total: 75 Evidence
```

**Player Feedback:**
- "Evidence: 25 (base) + 20 + 30 = 75"
- Color-code sources (Location = blue, Cards = red)

---

### Multiplicative Stacking (Deal Modifiers with ×)

**Rule:** Multipliers apply to base value from Product, then modifiers add/subtract.

**Example:**
```
Meth: $100 (base)
Customer plays Bulk Order: ×1.5
Player plays Bulk Sale Pressure: ×1.3
Result: $100 × 1.5 × 1.3 = $195
```

**Player Feedback:**
- "Profit: $100 (base) × 1.5 × 1.3 = $195"
- Show breakdown clearly

---

### Heat Accumulation (All Sources)

**Rule:** Sum all Heat modifiers on cards played (Products, Locations, Modifiers, Insurance).

**Example:**
```
Meth: +30 Heat
School Zone: +20 Heat
Bulk Order (Customer): +20 Heat
Alibi (Player): -5 Heat
Total: +65 Heat this hand
```

**Player Feedback:**
- "Heat delta: +65 this hand"
- "New Heat: 40 → 105" (show before/after)
- Alert if Heat over 100 ("INFERNO - Narc extremely aggressive next deck")

---

## Edge Cases and Interactions

### Multiple Products in Same Round

**Scenario:** Player plays Weed (Round 1), then plays Meth (Round 2).

**Resolution:**
- Weed discarded
- Meth active
- Only Meth's base price and Heat apply

**Player Feedback:** "Weed discarded (replaced by Meth)"

---

### Location Override After Evidence Accumulated

**Scenario:**
- Round 1: Parking Lot (25 Evidence base) + Surveillance (+20) = 45 Evidence
- Round 2: Player plays Safe House (10 Evidence base)

**Resolution:**
- Parking Lot base (25) replaced by Safe House base (10)
- Evidence cards (+20) still apply
- New total: 10 + 20 = 30 Evidence

**Player Feedback:** "Evidence: 45 → 30 (Safe House base 10, +20 from cards)"

---

### Insurance Played But Not Needed

**Scenario:** Player plays Plea Bargain (insurance + Cover), but Evidence < Cover at hand end.

**Resolution:**
- Acts as pure Cover card (+20 Cover)
- Insurance NOT consumed (still available for future hands)
- No penalty paid (only triggers on bust)

**Player Feedback:** "Plea Bargain active (insurance unused - still available)"

---

### Multiple Get Out of Jail Cards

**Scenario:** Player has Plea Bargain in Round 1, plays Fake ID in Round 2.

**Resolution:**
- Plea Bargain discarded (override rule)
- Only Fake ID active
- Both provide Cover while active, but only one insurance possible

**Player Feedback:** "Plea Bargain → Fake ID (only one insurance active)"

---

### Make It Stick Below Threshold

**Scenario:** Narc plays Warrant (threshold 40), Player's Heat is 35.

**Resolution:**
- If Evidence > Cover: Check insurance (Get Out of Jail works)
- Warrant inactive (threshold not met)

**Player Feedback:** "Warrant present but threshold not met (Heat 35 / 40)"

---

### Make It Stick AND Insurance

**Scenario:** Narc plays DA Approval (threshold 60), Player plays Plea Bargain, Heat is 65.

**Resolution:**
- Evidence > Cover → Bust triggered
- Make It Stick threshold met (65 >= 60) → Insurance overridden
- Plea Bargain fails
- Run ends (busted)

**Player Feedback:** "⚠️ DA APPROVAL OVERRIDES INSURANCE - Plea Bargain FAILED - RUN ENDS"

---

## Deck Building Implications (Per RFC-006)

**Deck Building System:**
- Players choose **10-20 cards** from the 20-card pool before each run
- Constraints: Minimum 10 cards, maximum 20 cards, must include at least 1 Product and 1 Location
- Presets available: Default (all 20), Aggro, Control, Balanced

### Aggressive Deck (High Risk/Reward)
```
10-12 cards total:
3× High-value Products (Meth, Heroin, Cocaine)
1× Risky Location (School Zone)
3× Cover (minimal defense)
2× Insurance (Plea Bargain, Fake ID)
2× Deal Modifiers
```

**Strategy:** High profit per hand, accept high Heat, rely on insurance. Small deck = see cards more frequently.

---

### Balanced Deck
```
15-16 cards total:
3× Mixed Products (Weed, Pills, Meth)
2× Mixed Locations (Safe House, Parking Lot)
5× Cover (moderate defense)
2× Insurance
3× Deal Modifiers
```

**Strategy:** Flexible, can pivot between aggressive and defensive. Medium deck = variety.

---

### Conservative Deck (Grind)
```
18-20 cards total:
2× Low-value Products (Weed, Pills)
3× Safe Locations (Safe House, Warehouse, Apartment)
8× Cover + Heat reduction (heavy defense)
2× Insurance
5× Deal Modifiers
```

**Strategy:** Low profit per hand, minimize Heat, survive longer. Large deck = more defensive options.

---

## Fun Factor Analysis

### Why Is This Engaging?

**1. Tactical Depth**
- 7 distinct card types with clear purposes
- Override system creates pivoting opportunities
- Stacking creates compound effects

**2. Clear Cause and Effect**
- Every card shows exactly what it does
- Running totals visible
- No hidden mechanics or gotchas

**3. Build Variety**
- Aggressive, Balanced, Conservative viable
- Different cards support different strategies
- No "optimal" deck (context-dependent)

**4. Counterplay Options**
- Narc plays Evidence → Player plays Cover
- Customer plays Bulk Order → Player overrides Location
- Make It Stick → Player folds early

**5. Risk Management**
- High-value Products = high profit + high Heat
- Insurance cards = safety + cost
- Cover cards = defense + deck slot cost

---

### Potential Pain Points

**1. Card Text Overload**
- Too many numbers per card?
- **Mitigation:** Icons, color-coding, tooltips on hover

**2. Override Confusion**
- Players forget which cards override?
- **Mitigation:** Clear visual indicators, "replaces X" text

**3. Multiplicative Math**
- Price calculation confusing? ($100 × 1.5 × 1.3 = ???)
- **Mitigation:** Show breakdown, auto-calculate, highlight final value

**4. Make It Stick Surprise**
- Players didn't realize threshold met?
- **Mitigation:** Big warning when played, alert when threshold met

---

## Balance Considerations

### Product Pricing Curve
- Weed: $30, +5 Heat → $6 per Heat
- Pills: $60, +15 Heat → $4 per Heat
- Meth: $100, +30 Heat → $3.33 per Heat
- Heroin: $150, +45 Heat → $3.33 per Heat

**Design Intent:** Higher-tier products slightly more efficient (encourage progression).

---

### Location Risk/Reward
- Safe House: 10 Evidence, 30 Cover, -5 Heat → Very safe, Heat reduction
- Parking Lot: 25 Evidence, 15 Cover, 0 Heat → Neutral
- School Zone: 40 Evidence, 5 Cover, +20 Heat → Very risky, profit potential

**Design Intent:** Riskier locations should be viable for profit builds (Customer loves risky deals).

---

### Insurance Cost/Benefit
- Plea Bargain: +20 Cover, pay $1k, +20 Heat
- Fake ID: +15 Cover, no payment, +40 Heat
- Rat Out Partner: +30 Cover, pay $500, reset Heat to 0

**Design Intent:** Expensive but necessary at high Heat (death without insurance).

---

## Integration with Other Systems

**Requires:**
- Core Gameplay Loop (hand/round structure)
- Bust Mechanics (Evidence > Cover resolution)

**Feeds Into:**
- Heat System (Heat modifiers on cards)
- Trust System (Customer card quality)
- Progression System (card unlocks)

---

## MVP Scope

### Phase 1 (Core Cards) - ✅ Implemented per RFC-005
**Player Pool (20 cards):**
- 4 Products (Weed, Meth, Heroin, Cocaine)
- 3 Locations (Safe House, Warehouse, Apartment)
- 10 Cover cards (Alibi, Bribe, various defensive)
- 2 Insurance (Plea Bargain, Fake ID)
- 1 Deal Modifier (Disguise)

**Narc Pool (25 cards):**
- 17 Evidence (Donut Break, Patrol, Surveillance, Stakeout, Raid, etc.)
- 8 Conviction (Warrant, DA Approval, RICO Case)

**Customer Pool (25 cards):**
- 5 Products (customer requests)
- 5 Locations (customer suggests)
- 15 Deal Modifiers (Bulk Order, Haggling, etc.)

**Dealer Pool (20 cards) - ✅ Implemented per RFC-008:**
- 8 Location cards (community cards)
- 8 Modifier cards
- 4 Wild cards

**Total: 90 cards across 4 decks**

### Phase 2 (Expanded Collection)
- More card variety within each deck
- Unlock system tied to progression
- Card synergies and combos

### Future Enhancements
- Card rarity tiers (Common, Rare, Legendary)
- Animated card effects
- Card cosmetics/art variants
- Player deck customization (unlock more cards over time)

---

## Open Questions

### Product Tiers
- Are 4 tiers enough? (Weed, Pills, Meth, Heroin)
- Should there be intermediate tiers? (e.g., "Premium Weed" between Weed and Pills)

### Location Variety
- How many Locations needed for deck diversity?
- Should Locations have unique mechanics? (e.g., "Airport" = no Make It Stick possible)

### Insurance Scaling
- Should insurance costs scale with Heat? (more expensive at high Heat)
- Should insurance have cooldowns? (once per 3 hands?)

### Card Balance
- How to balance new cards without power creep?
- Should cards have unlock conditions beyond profit? (e.g., "Survive 10 decks to unlock Heroin")

---

## Success Criteria

### Player Understanding
- 100% of players understand override system by 3rd hand
- 90% of players understand additive vs. multiplicative by 5th hand
- Players can predict totals within ±10% accuracy

### Deck Diversity
- No single "optimal" deck (multiple archetypes viable)
- Players experiment with different builds (change deck at least 20% per session)
- All card types used (no "dead" cards)

### Strategic Depth
- Skilled players demonstrate better card timing (fold less, bank more)
- Players understand Insurance value (use it appropriately)
- Players adapt decks to Heat level (more Cover at high Heat)
