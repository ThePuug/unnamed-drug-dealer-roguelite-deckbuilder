# ADR-005: Initiative System and Raise Control

## Status

**Proposed** - 2025-11-09

**Related RFC:** RFC-002 (Betting System and AI Opponents)

**Related ADRs:** ADR-002 (parent document), ADR-004 (Hand State Machine)

## Context

The betting phase needs to prevent stale "everyone Checks" rounds while also preventing infinite raise loops. We need a control mechanism that rewards aggression (first to raise) and limits abuse (max raises).

**Key Requirements:**
- Incentivize raising (create dynamic gameplay, not passive Checking)
- Prevent infinite loops (can't raise forever)
- Create positional advantage (poker-like "who acts first matters")
- Support re-raises (back-and-forth tension)
- Clear termination conditions (betting must end)

**Design Constraints:**
- Must work with fixed turn order (Narc → Customer → Player from ADR-002)
- Must integrate with hand state machine (ADR-004 Betting phase)
- Must be understandable (players need to predict when betting ends)
- Must prevent abuse (can't burn entire deck in one round)

## Decision

**We use an Initiative system where the first player to Raise gains control and can re-raise after all players respond, with a hard limit of 3 raises per round.**

### Initiative Mechanics

**Initiative = Right to act again after all other players respond**

```rust
struct BettingState {
    initiative_player: Option<Player>,  // Who has initiative (first to raise)
    raises_this_round: u8,               // Total raises so far (max 3)
    players_awaiting_action: Vec<Player>, // Who hasn't responded yet
    round_number: u8,                     // 1, 2, or 3
}
```

**Rules:**
1. **Gain Initiative:** First player to Raise (not Check) gains initiative
2. **Respond to Raise:** All other players must Check or Raise
3. **Re-raise Opportunity:** After all respond, initiative player can raise again
4. **Initiative Persists:** Continues until initiative player Checks or limit hit
5. **Initiative Resets:** Each round starts with no initiative (fresh slate)

### Betting Flow with Initiative

```
Example 1: Customer gains initiative

Round 1 Start (no initiative):
  Narc: Check
  Customer: Raise (card 1) → GAINS INITIATIVE
  Player: Check
  [All responded to initial raise]

  Customer (has initiative): Raise (card 2) → KEEPS INITIATIVE
  Narc: Check
  Player: Check
  [All responded again]

  Customer (has initiative): Check → ENDS BETTING
  → Cards flip
```

```
Example 2: Initiative changes hands on re-raise

Round 2 Start (no initiative):
  Narc: Raise (card 1) → GAINS INITIATIVE
  Customer: Check
  Player: Check
  [All responded to initial raise]

  Narc (has initiative): Check → LOSES INITIATIVE, ENDS BETTING
  → Cards flip
```

```
Example 3: Raise limit hit

Round 3 Start (no initiative):
  Narc: Raise (card 1) → GAINS INITIATIVE (raises: 1/3)
  Customer: Raise (card 2) → STEALS INITIATIVE? NO, responds to Narc's raise (raises: 2/3)
  Player: Raise (card 3) → (raises: 3/3) MAX HIT
  → Betting ends immediately, cards flip
```

### Initiative vs. Response Raises

**Key distinction:**

**Initiative Raise:**
- Player already has initiative
- Re-raising after all players responded
- Keeps initiative (can raise again after responses)

**Response Raise:**
- Player does NOT have initiative
- Raising in response to someone else's raise
- Does NOT gain initiative (original raiser keeps it)

```rust
fn handle_raise(state: &mut BettingState, player: Player) {
    state.raises_this_round += 1;

    // Only FIRST raise gains initiative
    if state.initiative_player.is_none() {
        state.initiative_player = Some(player);
    }

    // Response raises do NOT steal initiative
    // Original initiative holder keeps it
}
```

### Raise Limit (3 per round)

**Hard Cap:** Maximum 3 raises per round (total, not per player)

```rust
const MAX_RAISES_PER_ROUND: u8 = 3;

fn can_raise(state: &BettingState, player: &Player) -> bool {
    state.raises_this_round < MAX_RAISES_PER_ROUND
        && player.hand.len() > 0  // Has cards to play
}
```

**When limit hit:**
- Betting ends immediately (no more actions)
- Cards flip (proceed to Flip phase from ADR-004)
- Even if players want to keep raising (hard stop)

**Why 3?**
- Enough for back-and-forth (raise → response → re-raise)
- Prevents deck burning (3 cards per player × 3 rounds = 9 cards, half of 15-card deck)
- Poker parallel: Some poker variants cap raises

### Betting Termination Conditions

**Betting ends when:**

1. **All Check:** No one raises → Betting ends (no cards played this round)
2. **Initiative Checks:** Initiative player chooses not to re-raise → Betting ends
3. **Limit Hit:** 3 raises reached → Betting ends immediately
4. **All Fold:** Only one player remaining → Hand ends (not just round)

```rust
fn is_betting_complete(state: &BettingState) -> bool {
    // Limit hit
    if state.raises_this_round >= MAX_RAISES_PER_ROUND {
        return true;
    }

    // All players acted at least once
    if !all_players_acted(state) {
        return false;
    }

    // No initiative OR initiative player just Checked
    if state.initiative_player.is_none() {
        return true; // All Checked, no raises
    }

    // Initiative player acted and Checked (ended their re-raise opportunity)
    if initiative_player_just_checked(state) {
        return true;
    }

    false // Initiative player can still re-raise
}
```

### Turn Order Integration

**Fixed order:** Narc → Customer → Player (from ADR-002)

**Initiative breaks order:**
- Normal turns follow fixed order
- After all respond, initiative player acts again (out of order)
- Then back to fixed order for responses

```rust
fn next_player_to_act(state: &BettingState) -> Player {
    // If all players responded and initiative holder can act
    if all_players_responded(state) && state.initiative_player.is_some() {
        return state.initiative_player.unwrap();
    }

    // Otherwise, follow fixed turn order (Narc → Customer → Player)
    next_in_turn_order(state)
}
```

## Rationale

### Why Initiative System?

**Problem without initiative:**
- All players Check → No cards played → Stale round
- No incentive to raise first (why commit card if others don't?)

**Solution:**
- First to raise controls pacing (can keep raising after others respond)
- Creates "who blinks first?" tension
- Rewards aggression (initiative = control)

**Poker parallel:** Position advantage (button/dealer has control)

**Alternative Considered:** No initiative (everyone acts once per round)
- **Rejected:** Passive play dominant (wait for others to commit)
- **Rejected:** No back-and-forth tension (one raise per player max)

### Why First Raise Gains Initiative (not subsequent raises)?

**Design goal:** Reward first aggression, not response aggression

**Scenario:**
- Narc raises → Gains initiative
- Customer raises (response) → Does NOT steal initiative
- Narc still has initiative (can re-raise after all respond)

**Why this matters:**
- Prevents initiative ping-pong (who raised last has initiative)
- Clarifies who controls pacing (first aggressor)
- Simpler mental model ("first to commit controls")

**Alternative Considered:** Last raiser gains initiative
- **Rejected:** Initiative bounces between players (confusing)
- **Rejected:** Encourages "wait and respond" (not first aggression)

### Why Max 3 Raises?

**Problem without limit:**
- Infinite raise loops (A raises, B raises, A raises, ...)
- Deck burning (burn all 15 cards in Round 1)
- Stalling tactics (delay hand resolution indefinitely)

**Solution:**
- Hard cap at 3 raises (total for round, not per player)
- Forces betting to end
- Prevents abuse

**Why 3 specifically?**
- Enough for meaningful back-and-forth (raise → counter-raise → re-raise)
- Poker precedent (some variants cap raises at 3-4)
- Deck preservation (3 raises × 3 rounds = 9 cards max, not full deck)

**Alternative Considered:** No limit
- **Rejected:** Requires betting economics (pot odds, chip stacks)
- **Rejected:** Deck size becomes constraint (only 15 cards)

**Alternative Considered:** Higher limit (5-6 raises)
- **Rejected:** Too slow (pacing suffers)
- **Rejected:** Burns too many cards (hard to complete 3 rounds)

### Why Initiative Resets Each Round?

**Design goal:** Each round feels fresh (not dominated by Round 1 initiative)

**Alternative Considered:** Initiative carries across rounds
- **Rejected:** Round 1 winner dominates Rounds 2-3 (feels predetermined)
- **Rejected:** No comeback opportunity (lost initiative in Round 1 = lost hand)

**Chosen approach:**
- Reset each round (no initiative at round start)
- Creates 3 fresh contests (who gets initiative in Round 1? Round 2? Round 3?)

## Consequences

### Positive

- **Incentivizes raising:** Initiative is valuable (players want to raise first)
- **Creates tension:** Who raises first? Who re-raises?
- **Prevents stalling:** Max 3 raises = betting must end
- **Predictable termination:** Players can calculate "betting ends in X more raises"
- **Strategic depth:** Timing of raises matters (first = initiative, later = response)

### Negative

- **Complexity:** Initiative rules are non-obvious (requires learning)
- **Player confusion:** "Why can Customer raise again?" (initiative not visible)
- **Limit feels arbitrary:** Why 3 raises, not 2 or 4? (magic number)
- **Response raises feel weak:** Raising in response doesn't gain initiative (feels punishing)

### Mitigations

- **Complexity:** UI shows clear indicators (initiative badge, "X has initiative")
- **Confusion:** Tutorial/tooltips explain initiative ("First to raise can re-raise")
- **Arbitrary limit:** Playtest validates 3 is right balance (adjust if needed)
- **Weak response:** Response raises still pressure opponents (force them to commit cards)

## Implementation Notes

### File Structure (Rust/Bevy)

```
src/
  hand/
    betting.rs       - BettingState struct, initiative logic
    initiative.rs    - Initiative gain/loss, termination checks

  systems/
    betting_system.rs    - Handle Check/Raise/Fold actions
    initiative_ui.rs     - Display initiative badge, raises remaining
```

### Betting State Management

```rust
impl BettingState {
    fn handle_action(&mut self, player: Player, action: BettingAction) {
        match action {
            BettingAction::Check => {
                self.players_awaiting_action.remove(&player);

                // If player has initiative and Checks, lose initiative
                if self.initiative_player == Some(player) {
                    self.initiative_player = None;
                }
            },

            BettingAction::Raise => {
                self.raises_this_round += 1;

                // Only first raise gains initiative
                if self.initiative_player.is_none() {
                    self.initiative_player = Some(player);
                }

                self.players_awaiting_action.remove(&player);

                // Add all OTHER players back to awaiting (they must respond)
                self.players_awaiting_action = all_players_except(player);
            },

            BettingAction::Fold => {
                // Player exits hand (handled by hand state machine ADR-004)
            }
        }
    }

    fn is_complete(&self) -> bool {
        // Limit hit
        if self.raises_this_round >= MAX_RAISES_PER_ROUND {
            return true;
        }

        // No one waiting to act
        if !self.players_awaiting_action.is_empty() {
            return false;
        }

        // No initiative OR initiative player already acted
        if self.initiative_player.is_none() {
            return true;
        }

        // Initiative player is waiting to act (can re-raise)
        false
    }
}
```

### UI Indicators

**Initiative Badge:**
```
╔══════════════╗
║ CUSTOMER     ║
║ HAS INITIATIVE ║
╚══════════════╝
```

**Raises Remaining:**
```
Raises this round: 2 / 3
```

**Player Action Options:**
```
[Check] [Raise] [Fold]
               ↑
           (Disabled if raises >= 3)
```

### Integration Points

- **Hand State Machine (ADR-004):** Betting phase runs initiative logic, signals completion
- **AI System:** AI decides Check/Raise based on initiative status
- **UI System:** Display initiative badge, raises remaining, enabled actions

### Testing Strategy

**Unit Tests:**
- First raise gains initiative
- Response raise does NOT gain initiative
- Initiative player Checks → Loses initiative, betting ends
- Max 3 raises → Betting ends immediately
- All Check → No initiative, betting ends

**Integration Tests:**
- Full betting round: Narc raises → Customer/Player respond → Narc re-raises → Ends
- Limit hit: 3 raises → Betting ends before all Check
- Initiative reset: Round 1 initiative → Round 2 starts with no initiative

**Edge Cases:**
- Initiative player folds → Initiative lost, next player continues
- All-in on raise #3 → Limit hit, can't raise anymore

## Future Extensions (Post-MVP)

**Initiative Bonuses (Phase 3):**
- Initiative grants bonus (+10 Cover, +$50 profit)
- Rewards aggressive play mechanically (not just control)

**Initiative Persistence (Phase 3):**
- Initiative carries across rounds (Round 1 winner has advantage in Round 2)
- Creates momentum (early aggression pays off)
- Requires balancing (too strong? comeback impossible?)

**Variable Raise Limits (Phase 3):**
- Short rounds: Max 2 raises (fast pacing)
- Long rounds: Max 5 raises (extended tension)
- Event-driven: Special cards change limit mid-round

**Blind Raises (Phase 3):**
- Forced raises before cards drawn (ante system)
- Guarantees action (can't all Check)
- Poker parallel: Blinds in Texas Hold'em

## References

- **RFC-002:** Betting System and AI Opponents (initiative system, raise limits)
- **ADR-002:** Betting System and Hand Structure (parent document)
- **ADR-004:** Hand State Machine (Betting phase integration)
- **Poker Design:** Position advantage, raise caps (inspiration)

## Date

2025-11-09
