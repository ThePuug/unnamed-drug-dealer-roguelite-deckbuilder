# Core Gameplay Loop Specification

## Overview

Players build 15-card decks and play hands in a betting game structure reminiscent of poker. Each session (deck) consists of 3-5 hands, each hand has 3 rounds of betting, and players must constantly weigh risk versus reward: stay in for profit or fold to preserve cards and minimize Heat.

**Target Session Length:** 15 minutes per deck (3-5 hands)
**Core Tension:** Push your luck vs. fold early and bank safety

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

#### HAND (Like Poker)
**Player Experience:**
- 3 rounds of betting against AI opponents (Narc, Customer)
- Draw 3 cards, play cards face-down during betting
- Cards flip and resolve after betting closes
- Can fold anytime to preserve remaining cards

**Emotional Flow:**
- **Round 1:** Establishing position, initial strategy
- **Round 2:** Escalation, stakes rising, calculating risk
- **Round 3:** Final commitment, all-in or fold decision
- **Resolution:** Relief (safe) or panic (busted) or regret (folded)

**Key Decision Points:**
- Stay in or fold after each round reveal?
- Play high-value Product now or save for later?
- Override Location to reduce Evidence?
- Play insurance card preemptively?

---

#### ROUND (Betting Phase)
**Player Experience:**
- Turn order: Narc → Customer → Player (you go last)
- See what opponents play before deciding
- Can Check (stay in without playing), Raise (play card), or Fold (exit hand)

**Betting Mechanics:**
- First to raise gains **initiative** (can raise again after everyone calls)
- Maximum **3 raises per round** (across all players)
- All-in ends betting immediately (if someone plays their last card)

**Player Agency:**
- You see opponent actions before your turn
- You decide when to escalate vs. when to wait
- You control pacing (fast hands by checking, slow hands by raising)

**Tension Sources:**
- Narc plays Evidence card → Do you play Cover or fold?
- Customer plays Bulk Order → Big profit, but also big Evidence/Heat
- Running low on cards → All-in risk or fold early?

---

### Hand Structure (Detailed)

#### Setup Phase
1. All players draw 3 cards
2. Dealer flips Scenario Card (sets theme/context)
3. Betting begins

**Scenario Cards** (flavor only, no mechanics in MVP):
- "Late Night Meet" - Empty parking lot, minimal lighting
- "Police Nearby" - Patrol car visible one block away
- "School Zone" - Afternoon, kids getting out of school
- "Safe House" - Your established meeting spot

**Player Feedback:**
- Clear turn indicators (whose turn?)
- Card count visible for all players (who's close to all-in?)
- Running totals visible (Evidence, Cover, Heat, Profit - updated after each round)

---

#### Round Loop (Repeat 3 Times)

**1. Betting Phase**

Turn order: Narc → Customer → Player

**Available Actions:**
- **Check** - Stay in without playing a card (free action)
- **Raise** - Play a card face-down (commits card)
- **Fold** - Exit hand immediately (lose all cards played, discard remaining hand)

**Betting Rules:**
- First to raise gains initiative
- After all call, player with initiative can raise again
- Max 3 raises per round (prevents infinite loops)

**Player Experience:**
- Narc plays first: Telegraphs threat level (Surveillance? Warrant?)
- Customer plays second: Shows deal quality (Bulk Order? Haggling?)
- You play last: React with full information

**Feedback Needed:**
- Who has initiative? (visual indicator)
- How many raises left? (3/3 → 2/3 → 1/3 → betting closed)
- Can I raise? (only if have initiative and raises remaining)

---

**2. Cards Flip and Resolve**

All cards played this round flip face-up simultaneously.

**Calculate Running Totals:**
- Evidence = sum(all Evidence cards + Location base + modifiers)
- Cover = sum(all Cover cards + Location base + modifiers)
- Heat = sum(all Heat modifiers on cards)
- Profit = Product base price + all price modifiers

**Display:**
- All cards on table (grouped by player)
- Running totals updated
- Visual indicators: Safe (Evidence < Cover) or Danger (Evidence > Cover)

**Player Feedback:**
- Color-coded totals: Green (safe), Yellow (close), Red (busted)
- Evidence gap: "Cover +20" or "Evidence +15" (show margin)
- Heat accumulation: "+45 Heat this hand" (running count)

---

**3. Decision Point: Continue or Fold?**

After cards resolve, before next round:

**Options:**
- **Continue** - Draw back to 3 cards, play next round
- **Fold** - Exit hand, keep Heat accumulated so far, lose all cards played

**Why Fold?**
- Evidence climbing too high (can't win)
- Out of Cover cards (can't defend)
- Heat too high (not worth the profit)
- Preserving cards for next hand

**Why Continue?**
- Profit is worth the risk
- Have Cover cards to defend
- Insurance card in hand (Get Out of Jail)
- Only 1-2 rounds left (commitment)

**Feedback:**
- Show projected totals if you fold vs. continue
- "If you fold: Keep Heat +30, lose 6 cards"
- "If you continue: Need 25 Cover to stay safe"

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
