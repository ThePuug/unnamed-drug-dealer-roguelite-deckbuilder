# ADR-004: Hand State Machine and Round Structure

## Status

**Proposed** - 2025-11-09

**Related RFC:** RFC-002 (Betting System and AI Opponents)

**Related ADRs:** ADR-002 (parent document), ADR-005 (Initiative System)

## Context

The game needs a clear state machine to orchestrate the flow of a hand through multiple rounds of betting, card reveals, and decision points. This is fundamental to the game loop and affects every system.

**Key Requirements:**
- Support 3 rounds per hand for escalating tension
- Clear phase transitions (Draw → Bet → Flip → Decide)
- Decision points between rounds (continue or fold?)
- Resolution phase after Round 3 (calculate totals, check bust)
- Extensible for future phases (e.g., special events, combo resolution)

**Design Constraints:**
- Must integrate with betting system (ADR-002, ADR-005)
- Must integrate with card interactions (ADR-001)
- Must support AI decision-making at each phase
- Must be deterministic and testable

## Decision

**We use a hierarchical state machine: Hand contains Rounds, Rounds contain Phases (Draw → Betting → Flip → Decision Point).**

### Hand State Machine

```rust
enum HandState {
    // Round states (repeat 3 times)
    Round(RoundState),

    // After Round 3
    Resolution,
    Bust,
    Success,
}

enum RoundState {
    Draw,          // All players draw to 3 cards
    Betting,       // Betting phase (see ADR-005 for details)
    Flip,          // Cards reveal simultaneously
    DecisionPoint, // Continue or Fold prompt (only between rounds, not after Round 3)
}

struct Hand {
    current_round: u8,      // 1, 2, or 3
    state: HandState,
    cards_played: Vec<Card>,
    players_active: Vec<Player>, // Players still in hand
}
```

### Round Flow

```
Round 1:
  Draw → Betting → Flip → DecisionPoint → (continue or fold)
                                         ↓
Round 2:                                 ↓
  Draw → Betting → Flip → DecisionPoint → (continue or fold)
                                         ↓
Round 3:                                 ↓
  Draw → Betting → Flip → Resolution (no decision point after Round 3)
                           ↓
                    Calculate Totals
                           ↓
                    Check Bust (Evidence > Cover?)
                           ↓
                    Success or Bust
```

### State Transitions

```rust
impl Hand {
    fn advance_state(&mut self) {
        self.state = match self.state {
            // Within a round
            RoundState::Draw => RoundState::Betting,
            RoundState::Betting => RoundState::Flip,
            RoundState::Flip => {
                if self.current_round < 3 {
                    RoundState::DecisionPoint
                } else {
                    HandState::Resolution  // After Round 3, no decision point
                }
            },
            RoundState::DecisionPoint => {
                // Player continues to next round
                self.current_round += 1;
                RoundState::Draw
            },

            // After Round 3
            HandState::Resolution => {
                if is_busted() {
                    HandState::Bust
                } else {
                    HandState::Success
                }
            },

            HandState::Bust | HandState::Success => {
                // Terminal states, hand ends
                panic!("Cannot advance from terminal state")
            }
        }
    }
}
```

### Decision Point Behavior

**Between Rounds (after Round 1, Round 2):**
- Player can **Continue** (advance to next round) or **Fold** (exit hand immediately)
- AI opponents make Continue/Fold decision based on heuristics
- If player folds: Keep unplayed cards, lose profit, hand ends
- If all players fold: Hand ends immediately

**After Round 3:**
- NO decision point (no "Fold" option after Round 3)
- Always proceed to Resolution
- Rationale: Final commitment (all cards played, calculate results)

### Phase Responsibilities

**Draw Phase:**
- All active players draw cards to hand size 3
- Uses deck system (draw from player's deck)
- Skipped if player already has 3 cards

**Betting Phase:**
- Sequential turns (Narc → Customer → Player)
- Check/Raise/Fold actions (see ADR-002)
- Initiative system active (see ADR-005)
- Ends when all players respond and initiative exhausted

**Flip Phase:**
- All cards played this round reveal simultaneously
- Totals update (running totals, not final yet)
- UI displays cards and intermediate totals

**Decision Point Phase (Rounds 1-2 only):**
- Prompt: "Continue to Round X?" or "Fold now?"
- Show current totals (Evidence, Cover, Heat, Profit)
- Show safety margin (how much Cover needed to be safe)
- AI makes Continue/Fold decision

**Resolution Phase (After Round 3 only):**
- Calculate final totals using ADR-001 algorithm
- Check bust: Evidence > Cover?
- Check insurance activation (ADR-003)
- Check conviction override (ADR-003)
- Transition to Success or Bust

## Rationale

### Why 3 Rounds?

**Escalation Arc:**
- Round 1: Exploratory (low commitment, feeling out opponents)
- Round 2: Escalation (Evidence climbing, decision weight increasing)
- Round 3: Climax (final commitment, all-or-nothing)

**Poker parallel:** Pre-flop, Flop, Turn (3 betting rounds in Texas Hold'em)

**Playtesting target:** Each round ~1 minute → 3 minutes per hand → 15 minutes for 5 hands

**Alternative Considered:** 2 rounds
- **Rejected:** No middle ground, awkward pacing (setup → climax, no escalation)

**Alternative Considered:** 4+ rounds
- **Rejected:** Too slow, players lose interest (9+ decisions per hand)

### Why Decision Points Between Rounds?

**Problem without decision points:**
- Can't fold mid-hand (forced to play all 3 rounds even if Evidence too high)
- No strategic retreat (must commit all cards)

**Solution:**
- Decision points allow folding after Round 1 or Round 2
- Preserve cards for next hand (only played 1-2 rounds worth)
- Minimize Heat gain (only accumulate Heat from cards played so far)

**Why NO decision point after Round 3:**
- Round 3 is final commitment (all cards played, point of no return)
- Prevents "fold at last second" feel (anti-climactic)
- Forces tension peak (Round 3 flip is dramatic climax)

### Why Hierarchical State Machine?

**Alternative Considered:** Flat state machine (11 states: Draw1, Bet1, Flip1, Decision1, Draw2, ...)
- **Rejected:** Repetitive code, hard to maintain, doesn't capture round structure

**Hierarchical approach:**
- Round contains Phases (4-5 states)
- Hand tracks current_round (1, 2, 3)
- Clear structure (easier to understand, maintain)

**Benefits:**
- Easy to add new phases (e.g., "Combo Resolution" phase after Flip)
- Round logic reusable (same phase flow for all 3 rounds)
- Clear separation of concerns (round flow vs. hand flow)

## Consequences

### Positive

- **Clear structure:** State machine is explicit (no implicit state in flags)
- **Easy to test:** Can test each phase transition independently
- **Extensible:** Easy to add new phases or change round count
- **AI-friendly:** AI can make decisions at clear decision points
- **UI-friendly:** UI can render based on current state (show appropriate controls)

### Negative

- **Complexity:** Hierarchical state machine is more complex than flat
- **Round count fixed:** Changing from 3 rounds requires refactoring
- **Decision point timing fixed:** Can't fold mid-round (only between rounds)

### Mitigations

- **Complexity:** Encapsulate state transitions in methods (advance_state(), can_fold(), etc.)
- **Fixed round count:** Use const ROUNDS_PER_HAND = 3 (easy to change if needed)
- **Fixed timing:** Intentional design (decision points between rounds creates pacing)

## Implementation Notes

### File Structure (Rust/Bevy)

```
src/
  hand/
    mod.rs           - Hand struct, public API
    state.rs         - HandState, RoundState enums
    transitions.rs   - State transition logic
    decision_point.rs - Continue/Fold decision handling

  systems/
    hand_state_machine.rs - Bevy system to advance hand state
    decision_point_ui.rs  - UI for Continue/Fold prompt
```

### State Machine System (Bevy)

```rust
fn hand_state_machine_system(
    mut hand: ResMut<Hand>,
    betting_complete: Res<BettingComplete>,
    player_decision: Option<Res<PlayerDecision>>,
) {
    match hand.state {
        RoundState::Draw => {
            draw_cards_for_all_players();
            hand.advance_state(); // → Betting
        },

        RoundState::Betting => {
            // Wait for betting phase to complete (see ADR-005)
            if betting_complete.is_true() {
                hand.advance_state(); // → Flip
            }
        },

        RoundState::Flip => {
            flip_all_cards();
            hand.advance_state(); // → DecisionPoint or Resolution
        },

        RoundState::DecisionPoint => {
            // Wait for player Continue/Fold decision
            if let Some(decision) = player_decision {
                if decision.fold {
                    hand.state = HandState::Success; // Fold = exit hand
                } else {
                    hand.advance_state(); // Continue → next round Draw
                }
            }
        },

        HandState::Resolution => {
            let totals = calculate_totals(&hand.cards_played); // ADR-001
            let result = resolve_bust(totals, ...); // ADR-003
            hand.state = match result {
                BustResult::Busted | BustResult::ConvictionOverride => HandState::Bust,
                _ => HandState::Success,
            };
        },

        HandState::Bust | HandState::Success => {
            // Terminal states, end hand
        }
    }
}
```

### Integration Points

- **Betting System (ADR-002, ADR-005):** Betting phase calls betting logic, waits for completion
- **Card Interactions (ADR-001):** Flip phase reveals cards, Resolution calculates totals
- **Insurance/Conviction (ADR-003):** Resolution phase calls bust resolution logic
- **AI System:** Decision points trigger AI Continue/Fold decisions

### Testing Strategy

**Unit Tests:**
- State transitions: Draw → Betting → Flip → DecisionPoint → Draw (Round 2)
- Round progression: Round 1 → Round 2 → Round 3 → Resolution
- Decision point: Fold after Round 1 → HandState::Success
- No decision point after Round 3: Flip → Resolution (skip DecisionPoint)

**Integration Tests:**
- Full 3-round hand: Draw → Bet → Flip × 3 → Resolution → Success
- Fold after Round 1: Only 1 round of cards played
- Fold after Round 2: Only 2 rounds of cards played

**State Machine Invariants:**
- Current round never > 3
- DecisionPoint never after Round 3
- Terminal states (Bust, Success) never transition

## Future Extensions (Post-MVP)

**Variable Round Counts (Phase 3):**
- Short hands: 2 rounds (quick decisions)
- Long hands: 4 rounds (marathon tension)
- Event-driven: Special cards change round count mid-hand

**New Phases (Phase 3):**
- Combo Resolution: After Flip, check for Product/Location combos
- Heat Decay: After Resolution, apply Heat decay based on time
- Trust Update: After Success, update Customer/Narc trust

**Branching States (Phase 3):**
- Special events: Raid (skip betting, forced flip)
- Emergency fold: Mid-round fold (costs extra Heat)

## References

- **RFC-002:** Betting System and AI Opponents (3-round structure, decision points)
- **ADR-002:** Betting System and Hand Structure (parent document)
- **ADR-005:** Initiative System (betting phase logic)
- **ADR-001:** Card Type System (totals calculation in Resolution)
- **ADR-003:** Insurance and Conviction System (bust resolution in Resolution)

## Date

2025-11-09
