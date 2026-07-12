# RFC-023: Kingpin & Dealers

## Status

**Proposed** — 2026-07-12 (authored by Claude as lead; open questions in SOW-023 Discussion)

## Problem

The player is currently the dealer: one `CharacterState`, and a bust is
permadeath. The July 2026 fun assessment found "nothing is wagered" and "no
goal"; the baseline playtest (Iteration 1) confirmed a bust is a wall, not a
setback. The vision is a **kingpin layer**: the player runs an operation,
hires dealers, absorbs busts as jail time instead of run-ending deletion, and
grows an empire — which gives losses a price (a jailed asset, idle time) and
wins a purpose (roster, territory, legacy).

## Design

### The roster

- `SaveData.dealers: Vec<DealerState>` — each dealer is today's
  `CharacterState` (heat, play counts, upgrade choices, decks_played,
  story_history, last_played) plus:
  - `name: String` — from a name pool; human-readable, displayed everywhere
  - `portrait: String` — actor-portrait key; the 9 currently unused actor
    portraits become the dealer face pool
  - `status: DealerStatus` — `Available` or `Jailed { until: u64 }`
- You start with **one free dealer**. Additional dealers are **hired with
  global cash**; cost scales with roster size so the roster is a progression
  sink (exact curve tuned in SOW-023).
- Exactly one dealer is selected for a run ("send someone out"). Their heat
  sets the narc tier (RFC-018 unchanged — per-dealer difficulty), their play
  counts/upgrades apply, their story history records the run.

### Jail (replaces permadeath) — REVISED per Reed, 2026-07-12

- Sentences are **turn-based**, not wall-clock:
  `Jailed { runs_remaining, sentence_total, heat_at_bust }` with
  `sentence = 1 + max(heat,0)/25` runs. Every completed run by any OTHER
  dealer ticks all sentences down — the empire keeps moving while someone sits.
- **Serving time reduces heat proportionally**: released after k of n runs →
  heat = heat_at_bust × (n−k)/n. A full sentence walks out at heat 0. Either
  release path adds `prior_convictions += 1` — the heat clears but the record
  doesn't (future difficulty hook). No decay while jailed (jail IS the valve).
- **Bail**: pay $300 × runs_remaining from global cash to release early —
  but the heat reduction stays proportional to time actually served.
- **The kingpin is `dealers[0]`** (`is_kingpin`): the game starts with you
  dealing yourself. The kingpin is never jailed and never hire-gated — but a
  KINGPIN bust ends the empire (full SaveData reset, the one remaining
  permadeath). Hired dealers absorb busts as jail time.

### Cash

- Already global (`AccountState.cash_on_hand`) — unchanged. Sessions bank
  profit to the account on GO HOME as today; hiring and (later) area unlocks
  spend from it.

### What this unlocks later

- P2: per-dealer heat means difficulty is a property of WHO you send.
- P3/P4: cash sinks (hires now, areas in RFC-024).
- P5: per-dealer story history feeds a kingpin ledger; a jailed dealer's
  stories read like a rap sheet.

## Consequences

- `SAVE_VERSION` bump with a fresh-save wipe (pre-release convention, SOW-021).
- `save_after_resolution_system` permadeath path (`character = None`) is
  removed; `character: Option<CharacterState>` becomes the dealers vec +
  active index. Every `save_data.character` touchpoint migrates.
- DeckBuilding screen gains an operations panel (roster select/hire) — full
  operations screen redesign can come later; SOW-023 does the minimum panel.
- e2e harness gains save isolation so scripted busts stop affecting real saves.
