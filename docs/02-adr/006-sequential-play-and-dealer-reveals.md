# ADR-006: Sequential Play and Progressive Dealer Reveals

## Status

**Approved** - 2025-11-10

**Related RFC:** RFC-008 (Sequential Play with Progressive Dealer Reveals)

**Supersedes:** ADR-002 (betting phase), ADR-005 (initiative system - marked obsolete)

## Context

Playtesting revealed that the simultaneous face-down card play with betting rounds (ADR-002/005) created a fundamental gameplay problem: **rounds felt meaningless**. Players would dump all cards blindly, then check math at the end. There was no reactive decision-making or progressive tension.

**Key Problems with Previous Architecture:**
- Rounds 1-2 felt like blind accumulation (no decisions)
- Only Round 3 mattered (final Evidence vs Cover check)
- Betting phase (Check/Raise/Fold) didn't create meaningful choices
- Initiative system added complexity without fun
- Optimal strategy: "spam all cards, fold if numbers bad"

**New Requirements:**
- Each round must create meaningful decisions (not just accumulation)
- Progressive information revelation (like Texas Hold'em flop/turn/river)
- Reactive gameplay (see threat, respond with appropriate card)
- Leverage unique override mechanic (Location battles)
- Create "river tension" (final reveals can save or doom you)

**Design Constraints:**
- Must integrate with existing features (insurance, conviction, deck building, card retention)
- Must work well vs AI (deterministic, not mind-reading)
- Must maintain performance (60fps with real-time totals updates)
- Must preserve card types and override rules (ADR-001)

## Decision

**We replace simultaneous betting with sequential turn-based card play, add Dealer community cards that reveal progressively, and allow folding after each Dealer reveal.**

### Sequential Turn-Based Play

**Turn Structure (per round):**
```
Round:
  1. Player Phase - Players play cards one at a time in turn order
     - Each player: Play 1 card face-up OR Check (skip)
     - Card flips immediately when played
     - Totals update after each card
     - Turn order rotates each round

  2. Dealer Reveal - One Dealer card flips (community card)
     - Affects all players
     - Totals update to include Dealer card

  3. Fold Decision - Player can fold (Rounds 1-2 only)
     - Keep unplayed cards
     - Discard played cards
     - Exit to next hand
```

**Turn Order Rotation:**
```rust
fn get_turn_order(round: u8) -> Vec<Owner> {
    match round {
        1 => vec![Owner::Narc, Owner::Customer, Owner::Player],  // Player last (info advantage)
        2 => vec![Owner::Customer, Owner::Player, Owner::Narc],  // Narc last
        3 => vec![Owner::Player, Owner::Narc, Owner::Customer],  // Customer last
        _ => unreachable!(),
    }
}
```

**Rationale:**
- Rotating turn order balances first-mover vs last-mover advantage
- Each player gets positional advantage once per hand
- Player goes last in Round 1 (learning advantage)

### Dealer Deck System

**Dealer as Community Cards:**
- Separate 20-card Dealer deck (scenario cards)
- 3 cards drawn at hand start (one per round)
- Cards reveal progressively (after player phase each round)
- Dealer cards affect ALL players (community effect)

**Dealer Card Types:**
```rust
enum CardType {
    // ... existing player card types (ADR-001)

    // NEW: Dealer card types
    DealerLocation {
        evidence: u32,
        cover: u32,
        heat: i32,
        // Can be overridden by player Location cards
    },

    DealerModifier {
        evidence: i32,  // Additive
        cover: i32,     // Additive
        heat: i32,      // Additive
        // Cannot be overridden
    },
}
```

**Dealer Deck Composition:**
- 8 Location cards (set base Evidence/Cover/Heat, can be overridden)
- 8 Modifier cards (adjust totals additively, cannot be overridden)
- 4 Wild cards (high-impact swings: ±significant values)

**Integration with HandState:**
```rust
struct HandState {
    // ... existing fields
    dealer_deck: Vec<Card>,    // 20 scenario cards (shuffled)
    dealer_hand: Vec<Card>,    // 3 cards drawn for this hand
}

impl HandState {
    fn new() -> Self {
        let mut dealer_deck = create_dealer_deck();
        dealer_deck.shuffle(&mut rand::thread_rng());
        let dealer_hand = dealer_deck.drain(0..3).collect();

        Self {
            // ... existing initialization
            dealer_deck,
            dealer_hand,
        }
    }

    fn dealer_reveal(&self, round: u8) -> &Card {
        &self.dealer_hand[(round - 1) as usize]
    }
}
```

### Fold Mechanics

**Player Fold (Rounds 1-2):**
- Available after Dealer reveal (see environment before deciding)
- Fold effects:
  - ❌ Discard `cards_played` (played cards lost)
  - ✅ Keep `player_hand` (unplayed cards retained)
  - ✅ Keep accumulated Heat (from played cards)
  - ❌ Lose profit (deal incomplete)
- Transition: Exit to next hand (or end run if needed)

**Customer Fold (AI-driven):**
- Threshold-based: Fold if `evidence > threshold`
- Thresholds: 50 (R1), 60 (R2), 80 (R3)
- Fold effects:
  - Remove customer cards from `cards_played`
  - Recalculate totals (Evidence reduced, profit multipliers lost)
  - Hand continues with remaining players

**Narc Behavior:**
- Narc CANNOT fold (thematically: law enforcement doesn't give up)
- Always plays through to resolution

### State Machine Changes

**Old State Flow (ADR-002/004):**
```
Round:
  Draw → Betting (Check/Raise/Fold) → Flip → DecisionPoint
```

**New State Flow:**
```
Round:
  Draw → PlayerPhase (sequential turns) → DealerReveal → FoldDecision → (next round or resolve)

Where PlayerPhase iterates:
  foreach player in turn_order:
    - Player plays card face-up OR checks
    - Card flips immediately
    - Totals update
```

**State Enum Modification:**
```rust
enum State {
    Draw,           // Draw cards to hand
    PlayerPhase,    // NEW: Sequential turn-based play
    DealerReveal,   // NEW: Flip Dealer community card
    FoldDecision,   // NEW: Player can fold after seeing Dealer card (R1-R2)
    Resolve,        // Calculate final totals, check bust (after R3)
    Bust,           // Terminal state (player busted)

    // REMOVED: Betting, Flip (obsolete from ADR-002)
    // REMOVED: DecisionPoint (replaced by FoldDecision)
}
```

### Running Totals Display

**Real-Time Updates:**
- Evidence bar (visual + number)
- Cover bar (visual + number)
- Profit calculation ($ amount with modifiers)
- Heat delta (cumulative, color-coded)
- Active Location highlight (shows which Location in effect)

**Update Frequency:**
- After each card played (9 updates per hand: 3 players × 3 rounds)
- After each Dealer reveal (3 updates per hand)
- Total: ~12 totals calculations per hand

**Performance Consideration:**
- Totals calculation is pure function: O(n) where n = cards played
- Max 12 cards per hand (3 players × 3 rounds + 3 Dealer cards)
- Negligible performance impact (<1ms per calculation)

## Rationale

### Why Sequential Play Instead of Simultaneous?

**Problem with simultaneous:**
- All cards played blind (no information)
- No reactive decisions (can't respond to threats)
- Flip happens after all committed (too late to adjust)

**Solution:**
- One card at a time, face-up immediately
- See opponent plays before deciding
- Running totals visible (know current Evidence/Cover state)

**Player Experience:**
- "Narc played Surveillance (+20 Evidence), I need Cover NOW"
- "Evidence is climbing, do I play my Location override or save it?"
- Creates moment-to-moment decisions (not just accumulation)

### Why Dealer Community Cards?

**Problem without Dealer cards:**
- Players control all cards (predictable)
- No external variance (same cards = same outcome)
- No "river" tension (Texas Hold'em-style drama)

**Solution:**
- Dealer deck as external factor (uncertainty)
- Reveals progressively (flop/turn/river structure)
- Affects all players (community cards like poker)

**Player Experience:**
- Round 1 reveal: "Private Residence (+10 Evidence, safe!)"
- Round 2 reveal: "Police Checkpoint (+30 Evidence, OH NO)"
- Round 3 reveal: "Please be helpful... Quiet Night (+10 Cover, SAVED!)"

**Poker Parallel:**
- Flop = Round 1 reveal (initial environment)
- Turn = Round 2 reveal (situation developing)
- River = Round 3 reveal (final card can save or doom you)

### Why Rotating Turn Order?

**Problem with fixed order (ADR-002: always Narc → Customer → Player):**
- Player always has last-mover advantage (too strong)
- No variety in positional play
- Narc always disadvantaged (goes first)

**Solution:**
- Turn order rotates each round
- R1: Narc/Customer/Player (Player learns patterns)
- R2: Customer/Player/Narc (Narc gets last-mover advantage)
- R3: Player/Narc/Customer (Player commits first, tension!)

**Balances advantage:**
- Each player gets first-mover once (initiative)
- Each player gets last-mover once (information)
- Creates variety (different strategies per round)

### Why Fold After Dealer Reveal (Not During Betting)?

**Problem with fold during betting (ADR-002):**
- Fold decision before seeing environment (blind choice)
- No progressive information (can't assess danger mid-round)

**Solution:**
- Fold option after Dealer reveal (see full picture)
- Players commit cards → see Dealer card → fold or continue
- Creates "river decision" moment (stay in or cut losses?)

**Player Experience:**
- "I played 2 cards, then Dealer revealed Police Checkpoint"
- "Evidence jumped to 75, my Cover is only 40"
- "Do I fold now (keep remaining cards) or push to Round 3?"

### Why Narc Can't Fold?

**Thematic Reason:**
- Narc is law enforcement (doesn't "give up" on investigations)
- Always pursues to conclusion (fits narrative)

**Mechanical Reason:**
- Guarantees at least one opponent (player vs Narc minimum)
- Prevents all-opponent-fold situations (boring)
- Creates consistent pressure (Narc always there)

## Consequences

### Positive

- **Meaningful rounds:** Each round creates decisions based on new information
- **Reactive gameplay:** Players respond to threats in real-time (not blind accumulation)
- **Progressive tension:** Dealer reveals create Texas Hold'em-style "river" moments
- **Override wars:** Location battles become central (visible, impactful)
- **Natural pacing:** Sequential reveals create escalating tension arc
- **Leverage existing systems:** Card types, override rules unchanged (ADR-001 preserved)
- **AI-friendly:** Deterministic play (no bluffing mind-reading needed)

### Negative

- **More UI updates:** 12 totals calculations per hand (vs 3 in old system)
- **Complexity shift:** From betting mechanics (Check/Raise/Fold) to timing strategy (when to play which card)
- **Balance unknown:** Dealer deck difficulty untested (needs tuning)
- **Turn order advantage:** Last-mover still has advantage (even with rotation)
- **Customer fold frequency:** May fold too often (annoying) or too rarely (mechanic wasted)

### Mitigations

- **UI updates:** Pure function calculation (<1ms), negligible performance impact
- **Complexity:** Simpler mental model (play card or skip vs betting rules), more intuitive
- **Balance:** Start with 50% safe / 30% neutral / 20% dangerous Dealer cards, tune based on playtesting
- **Turn order:** Rotation balances advantage (each player gets last-mover once)
- **Customer fold:** Target 20-30% fold rate, tune thresholds if needed

### Breaking Changes

**From ADR-002:**
- ❌ Betting phase (Check/Raise/Fold) removed
- ❌ Initiative system (ADR-005) removed
- ❌ Face-down card holding removed
- ❌ Simultaneous flip removed
- ❌ Fixed turn order (always Narc → Customer → Player) removed

**Preserved from ADR-001:**
- ✅ Card types unchanged (Product, Location, Evidence, Cover, Modifiers)
- ✅ Override rules unchanged (last Location played = active)
- ✅ Totals calculation unchanged (sum Evidence/Cover/Heat/Profit)
- ✅ Bust resolution unchanged (Evidence > Cover at end)

**Integration with Existing Features:**
- ✅ Insurance/Conviction (RFC-003): Still checks at resolution, no changes needed
- ✅ Card Retention (RFC-004): Unplayed cards still retained, fold preserves hand
- ✅ Deck Balance (RFC-005): Player deck unchanged, Dealer deck additive
- ✅ Deck Building (RFC-006): Must initialize Dealer deck in HandState

## Implementation Notes

### File Structure

**No new files required** (modification to existing `main.rs`):
```
src/
  main.rs
    - Modify HandState: Add dealer_deck, dealer_hand fields
    - Modify State enum: Replace Betting/Flip with PlayerPhase/DealerReveal/FoldDecision
    - Add create_dealer_deck() function
    - Add get_turn_order(round) function
    - Modify card play systems: Sequential instead of simultaneous
    - Add dealer reveal system
    - Add fold decision system
```

### Key Data Structures

```rust
struct HandState {
    current_state: State,
    current_round: u8,  // 1, 2, or 3
    cards_played: Vec<Card>,  // All cards played so far (cumulative)

    // NEW: Dealer deck
    dealer_deck: Vec<Card>,   // 20 scenario cards
    dealer_hand: Vec<Card>,   // 3 cards for this hand

    // Existing fields unchanged
    narc_deck: Vec<Card>,
    customer_deck: Vec<Card>,
    player_deck: Vec<Card>,
    narc_hand: Vec<Card>,
    customer_hand: Vec<Card>,
    player_hand: Vec<Card>,
    outcome: Option<HandOutcome>,
    cash: u32,
    current_heat: u32,
}

enum State {
    Draw,
    PlayerPhase,    // NEW: Sequential turn-based play
    DealerReveal,   // NEW: Community card flip
    FoldDecision,   // NEW: Player fold after Dealer reveal
    Resolve,
    Bust,
}

// NEW: Dealer card types
enum CardType {
    // ... existing types (ADR-001)
    DealerLocation { evidence: u32, cover: u32, heat: i32 },
    DealerModifier { evidence: i32, cover: i32, heat: i32 },
}
```

### System Ordering (Bevy)

**PlayerPhase:**
1. `turn_order_system` - Determine current player based on round
2. `player_input_system` - Handle player card click or Check button
3. `ai_card_play_system` - AI selects card to play (if current player is AI)
4. `card_reveal_system` - Flip card immediately (not face-down)
5. `totals_update_system` - Recalculate Evidence/Cover/Profit/Heat
6. `ui_update_system` - Display running totals
7. `next_turn_system` - Advance to next player or DealerReveal

**DealerReveal:**
1. `dealer_reveal_system` - Flip dealer_hand[round-1]
2. `totals_update_system` - Recalculate with Dealer card
3. `ui_update_system` - Display updated totals

**FoldDecision:**
1. `fold_button_system` - Handle player fold input
2. `customer_fold_system` - AI decides if customer folds
3. `fold_consequences_system` - Apply fold effects (discard played, keep unplayed)
4. `next_round_system` - Advance to next round or Resolve

### Integration with Existing Systems

**ADR-001 (Card System):**
- Card types preserved (add DealerLocation, DealerModifier)
- Override rules unchanged (last Location = active)
- Totals calculation unchanged (called more frequently)

**ADR-003 (Insurance/Conviction):**
- Resolution logic unchanged (still checks Evidence > Cover)
- Insurance activates at bust (after Round 3 resolution)
- Conviction overrides insurance (same logic)

**ADR-004 (Hand State Machine):**
- State enum modified (PlayerPhase/DealerReveal/FoldDecision replace Betting/Flip/DecisionPoint)
- Round structure preserved (3 rounds, then Resolve)
- State transitions updated to match new flow

**RFC-006 (Deck Building):**
- HandState initialization must create Dealer deck
- DeckBuilder unchanged (only manages player deck)
- Dealer deck separate from player deck selection

### Testing Strategy

**Unit Tests:**
- Turn order rotation correct (R1: N/C/P, R2: C/P/N, R3: P/N/C)
- Cards flip immediately (not face-down)
- Totals update after each card
- Dealer reveals correct card per round
- Player fold preserves unplayed cards
- Customer fold removes customer cards from totals
- Narc never folds (skip fold check)

**Integration Tests:**
- Full 3-round hand with sequential play
- Dealer reveals affect totals correctly
- Override wars work (Location replacement visible)
- Fold preserves card retention (RFC-004 integration)
- Insurance activates correctly (RFC-003 integration)

**Playtesting Validation (Critical):**

After 5-10 test hands, evaluate RFC-008 acceptance criteria:
- [ ] Did each round force a decision? (not just accumulation)
- [ ] Did Dealer reveals change strategy?
- [ ] Did I fold at least once because of progressive info?
- [ ] Did override mechanic matter? (changed outcome)
- [ ] Could I track Evidence/Cover totals easily?
- [ ] Did Customer folding create interesting choice?

**Target:** 4+ of 6 criteria = core loop works, game is fun

## Future Extensions (Post-MVP)

**Dealer Deck Customization (Phase 2):**
- Different Dealer decks per location/event (alley vs warehouse)
- Player chooses Dealer deck difficulty (easy/medium/hard)
- Deck composition affects run strategy

**Multi-Deck Runs (Phase 3):**
- Dealer deck persists across hands (20 cards for full run)
- Cards removed when played (deck depletion)
- Player learns Dealer deck composition over time

**Simultaneous Player Actions (Phase 3):**
- All players secretly select cards
- Reveal simultaneously (adds hidden information back)
- Hybrid: Secret selection + progressive Dealer reveals

**Dynamic Turn Order (Phase 3):**
- Turn order based on game state (highest Heat goes first)
- Winner of previous hand goes first
- Creates comeback mechanics or runaway leader dynamics

## References

- **RFC-008:** Sequential Play with Progressive Dealer Reveals (feature specification)
- **ADR-001:** Card Type System and Interaction Rules (card types, override rules preserved)
- **ADR-002:** Betting System and Hand Structure (superseded sections: betting phase, turn order)
- **ADR-003:** Insurance and Conviction System (integration preserved)
- **ADR-004:** Hand State Machine (state enum modified)
- **ADR-005:** Initiative System (obsolete, entire document superseded)
- **Poker Design:** Texas Hold'em progressive reveals (flop/turn/river inspiration)

## Date

2025-11-10
