# ADR-002: Betting System and Hand Structure

## Status

**Proposed** - 2025-11-09

**Related RFC:** RFC-002 (Betting System and AI Opponents)

**Note:** This ADR provides an overview of the betting system. Detailed architectural decisions extracted to:
- **ADR-004:** Hand State Machine and Round Structure
- **ADR-005:** Initiative System and Raise Control

## Context

The game's core loop needs to create **poker-like tension** through betting rounds, where players make sequential "stay in or fold" decisions as Evidence accumulates. This is the primary innovation differentiating this from traditional deckbuilders.

**Key Requirements:**
- 3-round structure for escalating tension (Round 1 = exploratory, Round 3 = climactic)
- Betting actions (Check, Raise, Fold) that feel meaningful
- Initiative system (first to raise controls pacing)
- Raise limits (prevent infinite stalling)
- Turn order (Player always goes last = most information)
- Clear fold decision points (when is it right to bail?)

**Design Constraints:**
- Must feel like poker (familiar mental model for "betting rounds")
- Must prevent infinite loops (raise limits)
- Must support 3 players (Narc, Customer, Player)
- Must integrate with card interaction system (ADR-001)

## Decision

**We use a 3-round betting structure with Check/Raise/Fold mechanics, initiative system, and max 3 raises per round.**

### Hand Structure

```
Hand = 3 Rounds

Round:
  1. Draw Phase    - All players draw to 3 cards
  2. Betting Phase - Sequential turns (Narc → Customer → Player)
  3. Flip Phase    - All cards flip simultaneously
  4. Decision Point - Continue (next round) or Fold (exit hand)

After Round 3:
  5. Resolution Phase - Calculate totals, check bust, bank profit
```

### Betting Actions

```rust
enum BettingAction {
    Check,  // Stay in without playing card
    Raise,  // Play card face-down
    Fold,   // Exit hand immediately
}
```

**Check:**
- Stay in current round without committing card
- Can still play cards in later rounds
- Passes turn to next player

**Raise:**
- Play 1 card face-down
- Gain initiative (if first to raise)
- Passes turn to next player
- If playing last card, mark as "all-in"

**Fold:**
- Exit hand immediately
- Keep unplayed cards + accumulated Heat
- Lose profit from current hand
- Can play next hand normally

### Initiative System

**Initiative = Right to re-raise after all players respond**

```rust
struct BettingState {
    initiative_player: Option<Player>,
    raises_this_round: u8,
    players_to_act: Vec<Player>,
}
```

**Rules:**
1. First player to Raise gains initiative
2. After all players Check or Raise (respond to initial raise)
3. Player with initiative can raise again
4. Continues until no one raises OR max 3 raises hit
5. Initiative resets each round

**Example:**
```
Round 1:
- Narc: Check (no initiative)
- Customer: Raise (gains initiative)
- Player: Check (responds)
- [Back to Customer - has initiative]
- Customer: Raise again (initiative still active)
- Player: Check (responds)
- [Back to Customer - has initiative]
- Customer: Check (ends betting)
→ Cards flip
```

### Raise Limits

**Maximum 3 raises per round (prevents infinite loops)**

```rust
const MAX_RAISES_PER_ROUND: u8 = 3;

fn can_raise(state: &BettingState) -> bool {
    state.raises_this_round < MAX_RAISES_PER_ROUND
        && has_cards_in_hand()
}
```

When limit hit → Betting ends immediately, cards flip

### Turn Order

**Fixed order: Narc → Customer → Player**

**Rationale:**
- Player goes last = Most information (sees Narc + Customer actions before deciding)
- Narc goes first = Applies pressure early (sets tone for round)
- Customer goes middle = Unpredictable (can check or raise, Player must react)

**Player advantage:** Always acts with full knowledge of opponent actions

### All-In Mechanic

**Playing last card ends betting for that player**

```rust
if player.hand.len() == 1 && action == Raise {
    player.all_in = true;
    // Player can't raise again, but betting continues for others
}
```

**All-in players:**
- Automatically Check on future turns this round
- Can still continue to next rounds (will draw back to 3 cards)
- Prevents "locked out" feel (can always draw more cards next round)

## Rationale

### Why 3 Rounds?

**Tested Pacing:**
- 1 round: No escalation (too fast, no tension build)
- 2 rounds: Awkward pacing (no middle ground)
- 3 rounds: Natural arc (setup → escalation → climax)
- 4+ rounds: Too slow (players lose interest)

**Poker parallel:** Pre-flop, Flop, Turn (3 betting rounds in Texas Hold'em)

**Player experience:**
- Round 1: "What's the Narc playing? What's the Customer offering?"
- Round 2: "Evidence is climbing... do I stay in?"
- Round 3: "All-in or fold? This is the moment."

### Why Initiative System?

**Problem without initiative:**
- All players just Check → No cards played → Boring
- Need incentive to raise (commit cards)

**Solution:**
- First to raise controls pacing (can re-raise after responses)
- Creates "who blinks first?" dynamic
- Rewards aggression (initiative = control)

**Poker parallel:** Position matters (button has advantage)

**Alternative Considered:** No initiative (fixed turns only)
- **Rejected:** No incentive to raise first (everyone waits)
- **Rejected:** Stale gameplay (predictable patterns)

### Why Max 3 Raises?

**Problem without limit:**
- Infinite raise loops (Player raises, Narc raises, Customer raises, repeat)
- Stalling (players burn cards to delay resolution)

**Solution:**
- 3 raises = enough for meaningful back-and-forth
- Prevents abuse (can't stall indefinitely)

**Poker parallel:** Some poker variants have raise caps (prevents infinite re-raising)

**Alternative Considered:** No limit
- **Rejected:** Requires complex betting economics (chip stacks, pot odds)
- **Rejected:** Deck size becomes problem (only 15 cards, can't burn 10 per round)

### Why Player Goes Last?

**Advantage for player:**
- Sees Narc action → Knows if Evidence coming
- Sees Customer action → Knows if deal good/bad
- Can make informed decision (fold if too risky)

**Intentional design:**
- Single-player game (player should have edge)
- Learning curve (player needs info to learn patterns)
- Fun factor (feeling clever > feeling blind)

**Alternative Considered:** Random turn order
- **Rejected:** Too confusing (who goes when?)
- **Rejected:** Sometimes player goes first (no info advantage, frustrating)

## Consequences

### Positive

- **Escalating tension:** 3 rounds create natural arc (validated in poker design)
- **Meaningful decisions:** Check/Raise/Fold each have value (not just "stay in")
- **Strategic depth:** Initiative creates positional advantage (like poker)
- **Prevents abuse:** Raise limit stops infinite loops
- **Player-friendly:** Always act last (informed decisions, learning-friendly)

### Negative

- **Complexity:** Initiative + raise limits + turn order = many rules to learn
- **Pacing variability:** Some rounds fast (all Check), some slow (3 raises)
- **Player advantage too strong?:** Always acting last may make game too easy
- **3 Rounds might be slow:** 9 player decisions per hand (concerns in RFC-002)

### Mitigations

- **Complexity:** UI shows clear indicators (turn highlight, "2 raises left", initiative badge)
- **Pacing variability:** Fast animations, auto-pass if Check (minimize wait time)
- **Player advantage:** Balance via AI aggression (Narc plays Evidence consistently)
- **Slow pacing:** Playtest in SOW-002, adjust if needed (could reduce to 2 rounds)

## Implementation Notes

### File Structure (Rust/Bevy)

```
src/
  hand/
    mod.rs           - Hand state machine
    betting.rs       - Betting phase logic, initiative system
    state.rs         - HandState enum, BettingState struct
    actions.rs       - BettingAction handling

  systems/
    betting_input.rs - Handle player Check/Raise/Fold input
    ai_betting.rs    - AI decision-making (Narc/Customer)
    initiative.rs    - Track and update initiative
```

### Hand State Machine

```rust
enum HandState {
    // Per-round states
    Draw,           // Draw cards to 3
    Betting(BettingState),  // Betting phase
    Flip,           // Cards flip simultaneously
    DecisionPoint,  // Continue or Fold prompt

    // After Round 3
    Resolution,     // Calculate totals, check bust
    Bust,           // Player busted
    Success,        // Player survived, bank profit
}

struct BettingState {
    current_round: u8,      // 1, 2, or 3
    current_player: Player, // Narc, Customer, or Player
    initiative: Option<Player>,
    raises_this_round: u8,
    players_acted: Vec<Player>,
}
```

### Betting Phase Logic

```rust
fn handle_betting_action(
    state: &mut BettingState,
    player: Player,
    action: BettingAction,
) -> BettingPhaseResult {
    match action {
        Check => {
            state.players_acted.push(player);
            advance_to_next_player(state);

            if all_players_acted(state) {
                if state.initiative.is_some() && !initiative_player_acted_last(state) {
                    // Initiative player can re-raise
                    set_current_player_to_initiative(state);
                } else {
                    // Betting ends → Flip
                    BettingPhaseResult::EndBetting
                }
            } else {
                BettingPhaseResult::Continue
            }
        }

        Raise => {
            if state.raises_this_round == 0 {
                state.initiative = Some(player);  // First to raise
            }

            state.raises_this_round += 1;
            state.players_acted.push(player);

            if state.raises_this_round >= MAX_RAISES_PER_ROUND {
                BettingPhaseResult::EndBetting  // Limit hit
            } else {
                advance_to_next_player(state);
                BettingPhaseResult::Continue
            }
        }

        Fold => {
            BettingPhaseResult::PlayerFolded(player)
        }
    }
}
```

### Integration Points

- **Card System (ADR-001):** Raise action plays card face-down, Flip phase reveals and calculates totals
- **AI System:** AI opponents choose Check/Raise/Fold based on simple heuristics (RFC-002 Narc/Customer strategies)
- **UI System:** Display turn indicator, initiative badge, raises remaining, fold button
- **Resolution System:** After Round 3 → calls `calculate_totals()` from ADR-001

### System Ordering (Bevy)

**Betting Phase:**
1. `BettingInputSystem` - Handle player input (Check/Raise/Fold buttons)
2. `AIBettingSystem` - AI decision-making (if current player is Narc/Customer)
3. `InitiativeSystem` - Update initiative state
4. `BettingStateMachine` - Advance betting state, check if betting ends
5. `UIUpdateSystem` - Render turn indicators, initiative badge

**Flip Phase:**
1. `CardFlipSystem` - Reveal cards
2. `TotalsCalculationSystem` - Calculate Evidence/Cover/Heat/Profit (ADR-001)
3. `UIUpdateSystem` - Display totals

### Testing Strategy

**Unit Tests:**
- First raise → Gains initiative
- All Check → Betting ends (no raises)
- 3 raises → Limit hit, betting ends
- Initiative re-raise → Can raise after all respond
- Fold → Player exits immediately, hand continues

**Integration Tests:**
- Full 3-round hand: Draw → Bet → Flip × 3 → Resolution
- Initiative flow: Customer raises → Player checks → Customer re-raises → Betting ends
- Raise limit: 3 raises hit → Betting ends, cards flip
- Fold timing: Player folds Round 2 → Keeps cards, loses profit

**Playtest Validations (Critical):**
- Does 3 rounds create escalation? (or too slow?)
- Do Check/Raise/Fold feel meaningful? (or just "stay in"?)
- Does initiative matter? (or ignored?)
- Does Player-last advantage feel fair? (or too easy?)

## Future Extensions (Post-MVP)

**Betting Economics (Phase 2):**
- Pot contributions (each raise adds to pot)
- Split pots (multiple players survive, split profit)
- Requires economic model (chip stacks, pot tracking)

**Variable Round Counts (Phase 3):**
- Short hands (2 rounds, quick decisions)
- Long hands (4 rounds, marathon tension)
- Event-driven (special cards change round count)

**Advanced Initiative (Phase 3):**
- Initiative carries across rounds (not just within round)
- Initiative bonuses (e.g., +10 Cover if you have initiative)
- Requires deeper strategic layer

## References

- **RFC-002:** Betting System and AI Opponents (3-round structure, Check/Raise/Fold, initiative)
- **ADR-001:** Card Type System and Interaction Rules (card flip → totals calculation)
- **ADR-004:** Hand State Machine and Round Structure (detailed state flow, extracted from this ADR)
- **ADR-005:** Initiative System and Raise Control (detailed initiative mechanics, extracted from this ADR)
- **Poker Design:** Texas Hold'em betting structure (inspiration for initiative + raise limits)

## Date

2025-11-09
