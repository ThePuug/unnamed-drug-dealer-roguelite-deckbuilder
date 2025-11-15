# Core Gameplay Loop Specification

## Overview

Players build 10-20 card decks (choosing from a 20-card pool) and play hands in a sequential reveal structure with progressive information. Each session (deck) consists of 3-5 hands, each hand has 3 rounds where players play cards one-at-a-time (face-up), followed by a Dealer card reveal. Players must constantly weigh risk versus reward: stay in for profit or fold to preserve cards and minimize Heat.

**Target Session Length:** 15 minutes per deck (3-5 hands)
**Core Tension:** Progressive information reveals force reactive decisions each round

**Updated:** 2025-11-10 (Reflects RFC-005 deck sizes, RFC-006 deck building, RFC-008 sequential play)

---

## Player Experience Goal

Players should feel:
- **Calculated tension** - Each decision matters, but never feels random
- **Agency** - Clear information, conscious choices, no "gotcha" moments
- **Meaningful consequences** - Today's choices affect tomorrow's difficulty
- **Push-your-luck thrill** - The temptation to stay in "one more round"
- **Regret or triumph** - Strong emotional response to getting caught or banking profit

This is NOT about twitch reflexes or hidden information - it's about **knowing the odds and choosing your risk**.

---

## Core Mechanics

### Game Structure Hierarchy

#### RUN (Character Lifecycle)
**Player Experience:**
- You ARE this character until busted
- All decks played accumulate on one character
- Every decision contributes to total profit and survival time
- Permadeath when busted - start fresh with new character

**What Persists:**
- ✅ Total profit accumulated
- ✅ Number of decks played
- ✅ Heat level (with real-time decay)
- ✅ Customer Trust level
- ❌ Current deck (resets after each session)

**Emotional Arc:**
- Start: Cautious, learning the ropes
- Mid-run: Confident, building trust and profit
- Late-run: High stakes, dangerous, managing Heat carefully
- End: Either triumphant high-profit exit OR dramatic bust

---

#### DECK (Session)
**Player Experience:**
- 15 minutes of concentrated decision-making
- Build your deck before starting
- Play hands until cards exhausted (or "Go Home" early)
- Deck resets after session, but Heat/Trust/Profit persist

**Key Moments:**
- **Pre-session:** Deck building based on current Heat and strategy
- **During session:** Playing 3-5 hands, each a risk/reward decision
- **Post-session:** Banking profit, seeing Heat delta, planning next deck

**Strategic Depth:**
- Higher Heat = need more defensive cards
- Low Trust = might want to fold more (bad customer deals)
- Late in run = need insurance cards (Get Out of Jail)

---

#### HAND (Sequential Reveal Structure)
**Player Experience:**
- 3 rounds against AI opponents (Narc, Customer)
- Each round: Players play cards one-at-a-time **face-up**, then Dealer reveals community card
- **Turn order rotates** per round (Round 1: Narc→Customer→Player, Round 2: Customer→Player→Narc, Round 3: Player→Narc→Customer)
- Can **Check** (skip playing card) or play one card face-up
- Can fold after Dealer reveal (Rounds 1-2 only)
- Running totals visible after each card played

**Emotional Flow:**
- **Round 1:** Information gathering - see what everyone plays, then Dealer reveals first community card
- **Round 2:** Escalation - react to previous cards, Dealer reveals second community card, fold pressure builds
- **Round 3:** Final commitment - no fold option, Dealer reveals final card, resolution
- **Resolution:** Relief (safe) or panic (busted) or regret (folded early)

**Key Decision Points:**
- Which card to play when turn order favors me?
- Fold now after dangerous Dealer reveal, or push to next round?
- Override opponent's Location to reduce Evidence?
- Play insurance card preemptively?
- Check (play no card) or commit another card?

---

#### ROUND (Player Phase → Dealer Reveal)
**Player Experience:**
- Turn order **rotates** per round (gives each player last-mover advantage once)
  - Round 1: Narc → Customer → Player
  - Round 2: Customer → Player → Narc
  - Round 3: Player → Narc → Customer
- Each player plays **one card face-up** OR checks (plays no card)
- Cards flip **immediately** when played (everyone sees them)
- After all players act: **Dealer reveals one community card**
- Running totals update after each card (Evidence, Cover, Heat, Profit)

**Player Agency:**
- See what previous players played before your turn
- React to opponent cards in real-time
- Can check (play no card) to conserve hand
- Fold option appears after Dealer reveal (Rounds 1-2 only)

**Tension Sources:**
- Narc plays Surveillance (+20 Evidence) → Do you play Cover or override Location?
- Customer plays Bulk Order (+25 Evidence, ×1.5 profit) → Worth the risk?
- Dealer reveals Police Checkpoint (+30 Evidence) → Fold now or push through?
- Override wars → Narc plays School Zone, you counter with Warehouse, Dealer reveals another Location

---

### Hand Structure (Detailed)

#### Setup Phase
1. Dealer draws 3 community cards (face-down) from Dealer deck
2. Players draw cards from their hand (carrying over unplayed cards from previous hands per RFC-004)
3. Round 1 begins

**Dealer Cards** (Community cards affecting all players):
- **Dealer deck:** 20 scenario cards separate from player decks
- **3 cards drawn per hand:** Revealed one per round (progressive information)
- **Card types:**
  - Location cards (8): Set base Evidence/Cover (can be overridden by player Locations)
  - Modifier cards (8): Adjust Evidence/Cover/Heat additively (cannot be overridden)
  - Wild cards (4): High-impact swings (Lucky Break, Bad Intel, etc.)

**Player Feedback:**
- Clear turn indicators (whose turn, turn order for this round)
- Card count visible for all players
- **Running totals visible** (Evidence, Cover, Heat, Profit - **updated after each card played**)
- Active Location highlighted (shows which Location is in effect)
- Dealer cards remaining to reveal (Round 1: 2 unrevealed, Round 2: 1 unrevealed, Round 3: all revealed)

---

#### Round Loop (Repeat 3 Times)

**1. Player Phase (Sequential Play)**

Turn order rotates per round (see above).

**Each Player's Turn:**
- **Play one card face-up** - Card flips immediately, totals update
- **Check** - Skip playing a card (conserve hand)
- Cards visible to all players immediately upon play

**Player Experience:**
- See previous players' cards before your turn
- React to threats in real-time
- Running totals update after each card (progressive information)
- Can check to save cards for later rounds

**Feedback During Player Phase:**
- Turn indicator (whose turn)
- Running totals after each card
- Active Location highlight (if Location override occurred)
- Cards played so far this round (visible on table)

---

**2. Dealer Reveal**

After all players act, Dealer reveals one community card.

**Dealer Card Effects:**
- **Location cards:** Override previous Location if no player Location played after it
- **Modifier cards:** Add to Evidence/Cover/Heat (cannot be overridden)
- **Wild cards:** Large swings in totals
- Totals update immediately after Dealer reveal

**Player Feedback:**
- Dealer card flips with visual emphasis
- Totals update showing impact of Dealer card
- "Dealer reveals Police Checkpoint (+30 Evidence, 0 Cover, +15 Heat)"
- Color-coded alert if totals shift to dangerous range

---

**3. Fold Option (Available on Player's Turn)**

Player can fold during their turn in any round (1, 2, or 3):

**Fold Mechanics:**
- Available as action during player's turn (alongside Play Card and Check)
- Can fold in any round (including Round 3 before committing final card)
- Exit hand immediately when chosen
- Keep unplayed cards, lose cards played so far, keep Heat accumulated
- Narc CANNOT fold (always plays through)
- Buyer plays via reaction deck (no fold decision)

**Strategic Timing:**
- **Round 1**: Fold after seeing Narc's opening threat
- **Round 2**: Fold after seeing Narc's escalation + Buyer's Round 1 card impact
- **Round 3**: Fold even in final round if situation becomes unwinnable

**Why Fold?**
- Narc played high Evidence cards
- Buyer revealed dangerous card in previous round
- Running low on defensive cards
- Evidence exceeds Cover with no outs

**Why Continue?**
- Totals still manageable
- Have Cover/Location cards to defend
- Insurance card in hand
- Profit worth the risk

**Key Design:**
Sequential turn order means player always sees current game state (Narc's cards, running totals, Buyer's previous reveals) before their turn, providing natural information-based fold decision points without needing separate phases.

---

#### End of Hand Resolution

**If you stayed in all 3 rounds:**
- Calculate final totals from all cards across rounds
- Apply bust check (see Bust Mechanics spec)
- Bank profit (if not busted)
- Apply Heat delta

**If you folded:**
- Keep Heat from rounds played
- Lose all cards played
- No profit (nothing banked)
- Cards discarded

---

### Decision-Making Framework

Every round, player evaluates:

**1. Risk Assessment**
- What's Evidence vs. Cover gap?
- How many Cover cards do I have left?
- Is insurance card in hand?

**2. Reward Evaluation**
- What's current profit potential?
- Worth the Heat accumulation?
- Need profit for leaderboard push?

**3. Card Management**
- How many cards left in deck?
- Can I play 3 more hands?
- Need to preserve specific cards?

**4. Strategic Position**
- What's my current Heat level?
- What's my Trust with Customer?
- How many decks survived this run?

---

## Edge Cases and Polish

### "Go Home" Early

**When Available:**
- Between hands (not mid-hand)
- When you have cards remaining in deck

**What Happens:**
- End session immediately
- Bank all profit earned so far
- Apply Heat accumulated
- Deck resets, Heat/Trust/Profit persist

**Why Use It?**
- Heat too high to continue safely
- Profit target reached
- Real-world time constraint (need to stop)

**Player Feedback:**
- Show projected stats: "If you go home: $450 banked, Heat 65, 8 cards unused"

---

### All-In Scenarios

**Triggers:**
- Player plays their last card during betting

**Effect:**
- Betting ends immediately for that hand
- All remaining players must call or fold (no more raising)
- Hand proceeds to resolution

**Strategic Implications:**
- Forces early resolution (good or bad)
- Opponent can't escalate further
- Risky but decisive

---

### Deck Exhaustion

**Minimum Hands Per Deck:**
- 5 hands (draw 3, fold immediately = 15 cards)

**Realistic Hands Per Deck:**
- 3-4 hands (playing cards during rounds)

**What Happens When Deck Empty:**
- Can't draw cards (hand ends if need to draw)
- Must "Go Home" (session ends)
- Bank profit and apply Heat

**Player Feedback:**
- Card counter: "9 cards remaining (2-3 hands)"
- Warning: "Last hand - deck nearly empty"

---

### Scenario Card Integration (Future)

**MVP Scope:**
- Flavor only (no mechanical effect)
- Set theme/atmosphere

**Future Enhancement:**
- Mechanical effects (e.g., "Police Nearby" +10 Evidence)
- Player can react (e.g., "Lay Low" card negates Scenario)
- Dynamic difficulty adjustment

---

## Fun Factor Analysis

### Why Is This Engaging?

**1. Informed Risk-Taking**
- You know the odds (Evidence, Cover, Heat visible)
- You choose when to push luck
- Failure feels fair (you made the call)

**2. Meaningful Choices**
- Stay in or fold? (every round)
- Which card to play? (tactical)
- When to escalate? (betting strategy)

**3. Consequence Tension**
- Today's Heat affects tomorrow's difficulty
- One bad hand can end a 20-deck run
- High stakes, but you control them

**4. Strategic Depth**
- Deck building before session
- Card management during session
- Heat/Trust management across sessions

**5. Emotional Highs and Lows**
- Relief when Evidence < Cover
- Panic when Narc plays Warrant
- Triumph when banking huge profit
- Regret when busted with insurance in deck

---

### Potential Pain Points

**1. Analysis Paralysis**
- Too much information to process?
- **Mitigation:** Clear visual indicators (safe/danger zones), running totals, color-coded feedback

**2. Feel-Bad Moments**
- Getting busted on last hand of deck?
- **Mitigation:** Insurance cards (Get Out of Jail), clear bust warnings, player chose to stay in

**3. Grinding Feels Tedious**
- Playing 5 hands to exhaust deck?
- **Mitigation:** "Go Home" option, faster hand resolution, fewer hands needed

**4. RNG Frustration**
- Narc draws all Warrants?
- **Mitigation:** Procedural deck generation (Heat-based), no true RNG during hand

---

## Balance Considerations

### Session Length Target
- **Goal:** 15 minutes per deck
- **Hands per deck:** 3-5 hands
- **Time per hand:** ~3-4 minutes
- **Breakdown:** 1 min betting per round × 3 rounds + 30s resolution

### Risk vs. Reward Curve
- **Early run:** Low Heat, easy Narc, good profit opportunities
- **Mid run:** Medium Heat, harder Narc, need strategic play
- **Late run:** High Heat, extreme Narc, insurance mandatory

### Fold Frequency
- **Desired:** 20-30% fold rate per hand
- **Too low:** Players taking too much risk (bust rate high)
- **Too high:** Players being too cautious (profit too low)

---

## Integration with Other Systems

**Requires:**
- Card System (all 7 card types)
- Heat System (accumulation and decay)
- Trust System (Customer deck generation)
- Bust Mechanics (Evidence > Cover resolution)

**Feeds Into:**
- Progression System (profit → unlocks)
- Leaderboards (profit, decks survived)
- Meta Game (character permadeath)

---

## MVP Scope

### Phase 1 (Core Experience)
- Hand structure (3 rounds, betting, fold)
- Turn order (Narc → Customer → Player)
- Check/Raise/Fold actions
- Running totals display (Evidence, Cover, Heat, Profit)
- End-of-hand resolution
- "Go Home" option
- Basic 20-card collection

### Future Enhancements
- Scenario Cards with mechanical effects
- Animation and visual polish
- Hand history/replay
- Undo last action
- Tutorial/guided first hand

---

## Open Questions

### Pacing
- Is 3-4 minutes per hand too long?
- Should rounds auto-advance if all players check?
- Should there be a "fast mode" for experienced players?

### Feedback
- How much information to show during betting? (opponent card backs visible?)
- Should projected totals show "what if" scenarios?
- Alert players before bust threshold?

### Difficulty
- Is 3 rounds per hand right? (2 rounds too fast? 4 rounds too slow?)
- Should max raises per round scale with difficulty?
- Should "Go Home" have any penalty? (currently free)

---

## Success Criteria

### Engagement Metrics
- Average session length: ~15 minutes
- Fold rate per hand: 20-30%
- Hands per deck: 3-5 average
- Player retention: Want to play "one more deck"

### Player Satisfaction
- Decisions feel meaningful (not random)
- Bust feels fair (player had control)
- Profit milestones feel rewarding
- High-risk plays feel exciting (not punishing)

### Skill Expression
- Skilled players survive longer (more decks per run)
- Skilled players earn more (better profit/deck ratio)
- Skilled players manage Heat better (controlled risk)
