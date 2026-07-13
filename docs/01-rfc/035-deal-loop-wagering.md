# RFC-035: Core Deal-Loop Redesign — Wagering the Ante

## Status

**Draft** - 2026-07-13. Design converged with Reed in-session; this captures
the agreed shape so it stops living in chat. Precedes SOW-035. Blocks the
zone balance pass (that pass sits on top of this model, not the current one).

## 1. Motivation

The deal mini-game is solved. Today the only axis that matters is **get Cover
above Evidence**, the payout locks the instant you play a product, and hands
are a hard 3 rounds — so optimal play is "play your highest-cover card every
round," and the only interesting exception (holding a product for the next
hand) gets old fast. Reed's original vision was Texas-Hold'em-style **wagering**:
an escalating ante where pushing further grows the reward *and* the risk. That
tension doesn't exist because the pot never grows.

Three adjacent weaknesses fall out of the same root:
- **Insurance** is a passive backstop instead of a real mid-hand play.
- **Convictions** (insurance overrides gated on session-heat) mostly no-op —
  they only matter when you lost the cover battle *and* are insured *and* heat
  crossed a hidden threshold.
- **SOW-034 product stock** is a hidden charge counter on one card — correct
  economically, but not deckbuilder-native.

## 2. Current state (as-built, from the 2026-07-13 code sweep)

- **Hand = hard-coded 3 rounds** (`state_machine.rs:138`, literal `>= 3`; not
  data-driven). Turn order is a fixed `[Narc, Player]` each round
  (`get_turn_order` ignores the round arg); the Buyer plays in `DealerReveal`,
  outside the turn order.
- **Per round:** Narc plays hand **slot 0** (deterministic, no board awareness),
  Player plays ≤1 card or **PASS** or **BAIL OUT (fold)**, Buyer plays a
  **uniform-random** card. Progression is timer-gated (1s AI/dealer timers).
- **3-slot hands**; draw refills empty slots off the front of each owner's deck
  every round.
- **Deck lifecycle between hands** (`start_next_hand`): **Player** returns only
  *unplayed* cards (played cards gone → the run deck **depletes**, the soft
  clock); **Narc** `collect_all` → **resets to full every hand** (never
  depletes, composition constant all run); **Buyer** fully re-seeds each hand.
- **Narc difficulty** = deck composition, keyed **(dealer's station area ×
  career-heat tier)**. The tier is read from the persisted character's heat and
  **fixed at run start** (`input.rs:504-517`); session heat only drives the UI
  chip + buyer bail. Compositions authored per-tier in `narc_deck.ron`
  (base `default` ladder + per-area overrides).
- **Resolution** (`resolution.rs`): `Evidence ≤ Cover` → Safe; else Insurance
  saves you (pay cost + heat penalty + burn the card) *unless* an active
  Conviction whose `heat_threshold ≤ current_heat` overrides it → Bust. Cover =
  Location + Cover cards + Insurance-as-cover + modifiers; Evidence = Location +
  narc Evidence cards + modifiers. Conviction has **no** effect on totals.
- **Product stock (SOW-034)** = `AccountState.stock: HashMap<id,u32>` charge
  ledger; a Product card stays permanently in the deck and is drawable at 0
  charges (click is inert). Buying/fronting a batch adds `BATCH_SIZE=4` charges;
  a play **burns one charge at click time** (`input.rs:600-626`).
- Bust = jail the active dealer (kingpin bust = the one permadeath). Deck
  exhaustion is a neutral "go home", never a bust.

## 3. Proposed design

### 3a. Product is physical cards, not a charge counter
Revise SOW-034 storage: **buying/fronting a batch adds N Product *card copies*
to the deck.** Your deck *is* your inventory. Playing a product card antes a
unit into the pot. This is the deckbuilder-native version and it's the spine of
the wager. (SOW-034's economics, batch pricing, and the fronts reframe survive
unchanged — only the storage representation flips from ledger to copies.)

**Charges settle at resolution, not on play.** An ante you can grow, replace,
and fold on must be provisional: a **sale or bust burns the committed product**;
a **fold returns it to the deck**. (This reverses SOW-034's burn-at-click.)

### 3b. One product per deal; same bumps, different replaces
A deal is a single product type. Playing the **same** product again **bumps the
count** (antes another unit → bigger pot). Playing a **different** product
**replaces** it and **returns** the prior product to the deck (no burn) — the
existing "last product wins" override, made non-punishing by 3a. No mixed-product
pots (keeps the UI and payout math clean).

### 3c. The hand is a wager over N rounds (start at 5)
Each round the player chooses: **ante** (play another product — grow the pot),
**cover** (play a Cover/Location/modifier — defend), **insurance** (burn to
force a deal through), or **fold** (walk, keep uncommitted stock, lose the
committed). Round count is **data-driven, default 5** (replacing the hardcoded 3).

**The risk is the opportunity cost of the turn** — no new "evidence" stat on
product needed. A turn spent anteing is a turn *not* spent covering while the
narc keeps adding evidence. Since one product play = one unit, a fat pot costs
that many turns of not defending. This self-scales to zone difficulty for free:
- weak narc (Trailer Park): evidence trickles, one cover card stays ahead — you
  can ante most turns cheaply. Correct (forgiving zone).
- task-force narc (Red Light): evidence outruns a single cover card — you can't
  both push *and* stay covered, so past a point you fold. Correct (the
  "counterplay where you sometimes fold" Reed wanted).

The per-zone difficulty dial thus collapses to **narc evidence-density-per-round
vs. cover-per-card** — far simpler than bespoke per-tier decks.

### 3d. The Narc is a persistent session shoe
Replace narc `collect_all`-every-hand with a **deck that persists across the
run's hands** and cycles its discard back only when the draw pile empties
(standard deck cycling). This makes the narc a countable shoe: as it spends its
heavy cards over a few deals, your read sharpens into a "the deck's gone soft,
push now" window; a reshuffle (or going home) resets it. This is the loop that
makes "keep dealing to ride the soft window vs. bank and go home" a real
push-your-luck.

- **Small decks** (~7-ish), tuned so counting is doable **in memory** — bet on
  simple-and-fun before building any counting UI (§5).
- **Difficulty is a base deck + per-tier add/swap list**, not six bespoke decks.
  The tier is **fixed at run start** from career heat (as today) and **does not
  creep during the run** — the shoe is stable all run, so a count stays valid
  until a reshuffle; it "gets harder" *next* run as career heat climbs.
- Narc AI stays dumb (deck composition + draw order *is* the difficulty); no
  board-aware AI needed.

### 3e. Insurance and Convictions — mostly unchanged, for now
- **Insurance:** same mechanic, but the extra rounds turn it into a real play —
  *"evidence pulled ahead of my cover and I'm too deep to fold cheap → burn
  insurance to force it through."*
- **Convictions:** **left as-is** (override on session-heat threshold) —
  feel them out under the new rounds first. The ante model may rehabilitate them
  for free: bigger deals accrue more heat, pushing session heat toward the
  conviction thresholds faster, so they fire more often than they do today.
  Redesign deferred until we've felt the new loop.

## 4. Impact / what changes

- **Product storage (SOW-034):** ledger → card copies. Touches
  `AccountState.stock` (becomes deck copies), `buy_batch`/shop restock (adds
  copies), `input.rs` burn edge (moves to resolution), `front_card`/`seize_stock`
  (fronted copies), and `resolve_slot_click` (out-of-stock ⇒ simply "no product
  card in deck", so the inert-gate concept goes away).
- **Turn loop:** round count 3 → data-driven (5); the round becomes an
  ante/cover/insurance/fold choice; **settle-on-resolution** for product; fold
  returns committed product. `state_machine.rs` transitions + `input.rs`
  buttons.
- **Narc model:** persistent-across-hands shoe (change the Narc `collect_all`
  reset), small density decks, `narc_deck.ron` restructured to base + per-tier
  add/swap. `state_machine.rs` (`shuffle_cards_back`/`start_next_hand`),
  `narc_deck.rs`, `loader.rs`, `narc_deck.ron`.
- **Payout:** the pot = sum of anted product prices × modifiers × buyer
  multiplier at cashout (`card_engine.rs` `calculate_totals` — the active-product
  override becomes a count-aware sum for the single product type).
- **Reuses:** SOW-031 fronts, SOW-033 zones, SOW-034 economics; supersedes the
  betting shape of RFC-002/007/008.
- **Unblocks:** the zone balance pass, which then tunes the new dials.

## 5. Open questions / tuning knobs (for the SOW + playtest)

1. **Evidence-density-per-round vs. cover-per-card** — THE difficulty dial;
   must be tuned per zone so hard zones genuinely force the ante-vs-cover choice.
2. **Round count** — 5 to start; is it fixed or per-zone/situational?
3. **Batch size vs. deck bloat** — copies grow the deck; consuming-on-sale
   helps, but we likely want batch sizing + maybe a thinning outlet so a veteran
   deck isn't 40 Weeds.
4. **Does product ante add *any* exposure** beyond the turn cost? Leaning **no**
   — the opportunity cost is sufficient and cleaner. Confirm in playtest.
5. **Pot/payout math** — linear in count? buyer multiplier on the whole pot?
6. **Counting legibility** — start in-memory with small decks; add a
   seen/discard tracker only if small decks don't prove fun on their own.
7. **Buyer's role** — currently random reaction cards; does the buyer ante or
   haggle in the wager, or stay flavor? (Leaning: stays flavor for v1.)
8. **Deck-exhaustion pacing** — 5-round hands churn the player deck faster;
   watch session length.

## 6. Sequencing

RFC (this) → lock the shape → **SOW-035** implements (product-as-cards, the
N-round wager, the persistent narc shoe; convictions untouched) → **then** the
zone balance pass tunes the new dials. The balance pass explicitly waits — no
point tuning narc decks around a model we're replacing.
