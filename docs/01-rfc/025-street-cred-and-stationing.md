# RFC-025: Street Cred & Stationing

## Status

**Approved** — 2026-07-12 (design locked by Reed; see studio repo
`design-updates/2026-07-12-stationing-and-street-cred.md`)

## Problem

Areas exist (RFC-024) but dealers float: the run's territory is picked at
random and nothing ties a dealer to the ground they work. The map fantasy
needs dealers to be PLACED assets — working a territory, earning a
reputation there, and paying something to relocate. And area shops need a
progression key beyond cash, so that access to deeper stock is EARNED by
dealing ("to unlock Shrooms, you gotta deal in the Block").

## Design (all decisions locked by Reed, 2026-07-12)

### Stationing

- Each dealer has a **station** (`station: area_id`). A run happens in the
  active dealer's station — this replaces RFC-024's interim random pick.
- **Moving is a deliberate action costing cash + downtime**: a flat
  relocation fee (tunable constant) and one run of unavailability
  ("getting established" — reuses the sentence-ticking machinery). No heat
  effects from moving; heat stays **global per dealer**.
- The intended heat play is geographic: run hot territories while it pays,
  then station somewhere easy where authored low-heat content plus real-time
  decay cools you — the area ladder is the heat dial.

### Street cred

- `street_cred: Map<area_id, u32>` per dealer. **+1 cred per successful deal**
  (Safe resolution) in the run's area. **Cred never decays** — jail, moves,
  time: nothing erases reputation.
- **Shop unlock requirements**: shop items may carry a cred requirement
  alongside their price (authored in RON, validated at load). The requirement
  is satisfied by the ROSTER'S BEST cred for that area (any dealer's rep
  opens the door), and the shop UI shows **which dealer is effectively
  unlocking** — a credit line naming the highest-cred dealer ("unlocked by
  Ray"). This SOW ships the mechanism plus pilot content; the full
  requirement ladder lands in the authoring pass (SOW-026).

### Consequences

- Kingpin included: the kingpin has a station and earns cred like anyone.
- Jail hurts more than the sentence: a jailed dealer's cred is inert until
  they're out (requirements still count their cred — reputation persists —
  but they can't EARN more).
- HandState records the run's area (needed to credit cred at resolution).
- Fallen-empire reset wipes cred with the roster (a new empire starts unknown).
