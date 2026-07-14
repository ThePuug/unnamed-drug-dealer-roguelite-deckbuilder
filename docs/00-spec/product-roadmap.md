# Product Roadmap — Path to Playable

**Owner:** Claude (lead), directed by Reed
**Last Updated:** 2026-07-13
**Cadence:** playtest (e2e driver) → design update → SOW → implement + tests → repeat.
Each iteration appends to the Iteration Log below.

## Vision (from Reed, 2026-07-12)

You are the **kingpin**, not the dealer. You hire dealers who run sessions on
your behalf; they carry their own heat and decks, and can be **jailed** for a
period when busted. **Cash is global.** The product needs:

| Pillar | One-liner | Today's foundation |
|---|---|---|
| P1 Kingpin & dealers | Roster of hired dealers, each with own heat/deck/story; jail replaces permadeath; global cash | Single `CharacterState` + permadeath; `AccountState.cash_on_hand` already global |
| P2 Heat difficulty | Rising heat makes the world harder | RFC-018 narc tiers from career heat (works — baseline playtest busted hand 1 vs a high-tier narc) |
| P3 Progression & unlocks | Cards and capabilities unlock over time | RFC-017/019 upgrades, SOW-020 shops; **The Block is authored but permanently locked** (`unlock_location` has zero callers) |
| P4 Unlockable areas | New areas gate shops/customers/cards | `shop_locations.ron` (the_corner, the_block); buyers not yet area-gated |
| P5 History | Stories accumulate into a legacy | `story_history` per character + narrative engine; only visible in a deck-builder overlay |

## Sequencing (revised 2026-07-12 after the map/stationing design iteration)

1. ✅ **SOW-023 Kingpin & Dealers Foundation** (P1) — merged.
2. ✅ **SOW-024 Areas & Unlocks** (P3+P4) — merged; territories with interim
   random-area run selection.
3. ✅ **SOW-025 Street Cred & Stationing** — merged; stationed runs, cred
   accrual, cash+downtime moves, cred-gated shops with credit line.
4. ✅ **SOW-026 Content Authoring Pass** — merged; Weed-only lean start, full
   cash+cred shop ladder, measured pacing (Shrooms session 2-3).
5. ✅ **SOW-027 Heat Economy** — merged; per-area × per-tier narc
   compositions (multipliers retired), Lay Low + Crooked Lawyer, Heat
   upgrade wired, fresh floor ≤ Hot, ZERO-warnings baseline.
6. ✅ **SOW-028 Third Zone: The Strip + zone coherence** — merged; Pimp
   persona, Ecstasy/Ice re-zoned to the Strip, Housewife → Block first
   rung (Wolf ×2.8 gap closed), Corner fresh floor re-tuned to Cold.
7. ✅ **SOW-029 City Map Screen** — merged; map overlay with three live
   node cards, unlock + relocation from the map, two-click rotation.
8. ✅ **SOW-030 Kingpin Ledger** (P5) — merged; empire strip, dossiers
   with story feeds, browsable board with IN PROGRESS row, map zone
   history line, zero schema changes.
9. ✅ **SOW-031 Suppliers & Fronts** — merged; Lil Smoke / Miss Velvet /
   The Broker, 25% vig, 4-run windows on the run-ticker, cut off →
   muscle → soured, surfaces on shop/hub/map/ledger; zone strings moved
   to RON; SAVE_VERSION 6.
10. **SOW-032 Starter/Tutorial Arc — "Road to Your First Dealer"** —
   authored, NOT started; DEFERRED behind the higher-value seeds below.
   OPTIONAL at empire start; skipping confers NO benefit.
11. ✅ **SOW-033 Zone Re-Theme** — merged 2026-07-13 (jumped the queue).
   Three tier-labels → three low-heat neighborhoods (Trailer Park →
   Suburbia → Red Light District), themed across clientele/narcs/supplier/
   unlockables; 2 products & 3 buyers per zone; premium tier + Wall Street
   Wolf shelved; actor-art moved to RON with per-area narc art (E3 paid
   down); SAVE_VERSION 6 → 7.

12. ✅ **SOW-034 Limited-Use Cards** — merged 2026-07-13. Products are
   consumable stock: unlock = permanent access, stock = batches
   bought/fronted, each play burns a charge (fold-free, bust loses it).
   The "nothing wagered" fix; fronts reframed as batch-fronting.
   SAVE_VERSION 7 → 8.

**Next seeds (Reed, 2026-07-13, in priority):** (1) **Art polish batch** —
wire the new Tweaker/Deadbeat/John faces + escort rename (queued), then the
**background pass** (watermark strip, gibberish fixes, Red Light
backgrounds — studio art-backlog). (2) **Unlockable dealers per area**.
(3) Utility-card consumables. (4) **"Widen the margins" mechanic** (the
limited-use progression reward). (5) SOW-032 Tutorial Arc.

**Update (2026-07-14, autonomous run):** delivering (2) unlockable dealers
[SOW-038], the generic-hire retirement [SOW-039], and (5) the tutorial
[SOW-032] this evening. **(3) Utility-card consumables is REVERSED** — Reed
decided only PRODUCTS are consumable stock; utility cards (cover/insurance/
modifier/location) stay free-infinite, so SOW-040 is dropped. Next mechanic
seed after the batch: (4) "widen the margins."

**Roadmap adds (Reed, 2026-07-13):**
- **Retire the generic hire pool** — DECIDED. Signature-per-zone hiring
  (SOW-036) becomes the ONLY way to hire a dealer. Remove the generic
  `hire_dealer()` / `recruit()` / `DEALER_PORTRAIT_POOL` path and the
  roster HIRE button; the transitional **Gladys-only** pool that SOW-036
  shipped goes away with it. This resolves SOW-036's open
  generic-hiring-fate question. (Small SOW; touches save/types.rs +
  roster UI; likely SAVE_VERSION bump.)
- **`back_of_the_club` → shop-buyable location** — ✅ PREMISE ALREADY MET
  (verified 2026-07-14). The alley was dropped only from the Pimp's
  *reaction deck* (SOW-036 boot fix), NOT from the shop pool: it is still a
  purchasable Red Light District location card (`locations.ron`:
  `shop_location: red_light_district`, $800, cred 1) and its id appears in
  no reaction deck. Remaining work (SOW-037) is small: a regression guard
  test so the shop hook can't be silently dropped again, plus a live
  buy→play e2e proof (it panicked on boot before the SOW-036 fix).

**Closed threads (Reed, 2026-07-12):** dev save wipes are a non-concern for
the leaderboard; Lay Low stays committed (no cancel); heat stays global per
dealer. Original debt list fully absorbed: jail-as-wager shipped (023),
RFC-019 resolved (027), harness isolation/outcome-awareness shipped (023/024).

## Iteration Log

### Iteration 14 — 2026-07-14

- **Autonomous delivery campaign started** (Reed: deliver seeds #2/#3/#5 + the
  two small SOWs; work async in isolated worktrees, main session does doc
  housekeeping; I self-verify via adversarial review + the e2e driver and merge
  autonomously — Reed reviews outcomes). Sequence (save-version bumps force it
  sequential): SOW-037 → 038 → 039 → 040 → 032.
- **SOW-037 `back_of_the_club` guard — MERGED** (2177bf5): the "orphaned" premise
  was stale — the alley is already a Red Light shop card ($800 / cred 1), dropped
  only from the Pimp reaction deck by the SOW-036 boot fix. Added the missing
  regression guard test (the old orphan test passed vacuously) + SOW doc; no RON
  change, no save bump. 286 tests, zero warnings; adversarial verify confirmed the
  guard bites.
- **Build reliability fix** (5deaa30): `Cargo.lock` is gitignored and `hmac = "0"`
  floated to a breaking 0.13 in fresh worktrees — pinned `hmac` to 0.12.
  (Open recommendation: commit `Cargo.lock` for full reproducibility.)
- **Doc housekeeping pass:** reconciled the SOW index (033/034/036), CLAUDE.md
  (retired the "early development" scaffold — real commands / module map /
  patterns, fixed `docs/adr` → `docs/02-adr` links), the progression-meta matrix
  (zone renames, Broker → Deb, ladder, SOW-034 fronting), and a full README
  rewrite (was frozen at SOW-014). Docs now track current `main`.
- **In flight:** SOW-038 Unlockable Dealers — cred-gated additional dealers per
  zone (additive over signatures); rehomes Gladys to Trailer Park, pre-clearing
  the generic-hire retirement (SOW-039).

### Iteration 13 — 2026-07-13

- **SOW-036 Signature Dealers merged** (284 tests, zero warnings): hiring is
  now a **per-zone** act. Each zone authors one named **signature dealer** you
  hire AT that zone on the shared hire-cost ladder (`next_hire_cost()` =
  `500 * 2^(len-1)`, no cred gate); the hire lands **stationed at that zone**
  and can be hired **once**. Faces: **Trailer Park → Bubba**, **Suburbia →
  Roxanne**, **Red Light District → Marcus**. The generic hire pool
  (`DEALER_PORTRAIT_POOL`) shrank to **Gladys-only** — the three signature
  faces are reserved — with load-time validation that every zone authors a
  non-empty signature. Model guard + map view-model (`signature_status`) both
  key off `account.unlocked_locations`; map surfaces a "HIRE `<NAME>` — $X"
  button on unlocked nodes (mirrors the UNLOCK / SEND button patterns).
  SAVE_VERSION 8 → 9 (serde-default `signature_of` field; the bump wipes old
  saves per the SOW-021 policy).
- **P0 boot fix (on-branch):** the Pimp's reaction deck referenced
  `back_of_the_club`, which no longer resolved to a playable location and
  panicked on boot — **dropped the alley** from the Pimp's deck, leaving it at
  **7 cards**. (Follow-up captured above: make `back_of_the_club` a shop-buyable
  location so the scene is obtainable rather than orphaned.)
- **Art:** three **regenerated dealer faces** (bubba / marcus / roxanne) wired
  through the RON portrait mappings and the loud disk-existence check.
- **Generic-hire fate DECIDED (Reed):** retire the generic hire pool entirely —
  signature-per-zone becomes the only hire path — scheduled as a follow-up SOW
  (see Roadmap adds above). SOW-036 ships **as-is** with the transitional
  Gladys-only pool.
- Adversarial review: **safe to merge**, no blocker/major; 1 minor applied at
  closeout (`signature_status` tightened to the account's `unlocked_locations`
  set alone, matching the model guard, + a regression test), two nits accepted
  as matching established button patterns. **Live boot verified:** the Pimp
  boot fix boots and the signature-dealer map UI renders. Merged `--no-ff`,
  pushed assets → game.

### Iteration 12 — 2026-07-13

- **SOW-034 Limited-Use Cards merged** (273 tests, zero warnings): the
  "nothing wagered" fix. Products became **consumable stock** — unlock is
  permanent access (`unlocked_cards` untouched), stock is a new
  `AccountState.stock` ledger of charges bought or fronted in batches of 4.
  Each product play burns one charge at the commit edge (fold-before-play
  free; a bust loses that one charge, not the batch); 0 charges = out of
  stock, inert but the deck stays legal. **Fronts reframed** to batch-
  fronting — `take_front` requires access + grants a batch, souring seizes
  unsold charges instead of revoking access (reusing ~90% of SOW-031).
  Per-zone `restock_margin` (0.35/0.50/0.65) prices restock off base sale
  price; economy above water (Weed batch $44 → Coke $312). SAVE_VERSION
  7 → 8.
- Architecture nailed by a 5-agent code-map sweep first (deck rep = one
  card + save-side ledger; commit-time burn; fronts reuse; the
  `shop_price`-isn't-restock economy correction).
- Adversarial review (28 agents): **0 sustained**, but I applied one
  dismissed footgun (`restock_unit` could round to 0 → free product +
  access; floored at 1) and documented the accepted flat-ledger souring
  simplification + the ECS-wiring coverage gap (rests on playtest). Merged
  `--no-ff`, pushed assets → game → studio.
- **Open for Reed:** the art-face batch is queued (Tweaker/Deadbeat/John +
  escort rename) as the immediate next commit; then the background art
  pass; and **watch a charge burn** on playtest to confirm the ECS wiring.

### Iteration 11 — 2026-07-13

- **SOW-033 Zone Re-Theme merged** (256 tests, zero warnings): the three
  tier-named areas became three low-heat neighborhoods —
  **Trailer Park → Suburbia → Red Light District** — each themed across
  clientele, narcs, supplier, and unlockables. Area IDs renamed
  (`the_corner`/`the_strip`/`the_block` → `trailer_park`/`suburbia`/
  `red_light_district`), the RON array reordered (Red Light is now the
  TOP rung), unlock ladder $0 → $1,200 → $2,500. Shop stock cut to exactly
  **2 products/zone** (Weed/Shrooms · Codeine/**Xanax**[new] ·
  Ecstasy/Coke), clientele grown to **3 buyers/zone** — 9 personas:
  Biker/Tweaker/Deadbeat, Frat Bro[re-homed]/Housewife/Widow,
  Pimp/Working Girl/Club Kid. Premium tier (Acid/Ice/Heroin/Fentanyl) +
  Wall Street Wolf **shelved** for a future Business District arc.
  SAVE_VERSION 6 → 7 (live save resets).
- **Actor-art system (E3 paid down):** portrait mappings moved from
  hard-coded `loader.rs` into RON (`portrait` per buyer, `narc_portrait`
  per area) with a LOUD missing-file panic; `<role>-<slug>.png` naming
  template; **per-area narc art** — Reed dropped Trailer Park + Suburbia
  narc faces mid-flight, Red Light uses the original as `narc-default`.
  Art→persona assignments are shuffleable RON one-liners.
- Adversarial review (25 agents, 4 dims × find→3-skeptic-refute):
  1 sustained (2-products/zone had no test guard — fixed `01583ec`),
  6 refuted. One reviewer died on a connection error; that dimension
  re-verified by hand. Merged `--no-ff`, pushed assets → game → studio.
- **New design captured (next SOWs):** limited-use cards (product as
  consumable stock — the "nothing wagered" fix; design doc written),
  unlockable dealers per area, utility consumables; SOW-032 Tutorial Arc
  deferred behind them.
- **Open for Reed:** art-face reassignment batch (Bubba → a Trailer
  buyer; dedupe the two redundant hippie faces; rework barista/julie —
  all shuffleable RON); limited-use forks (fixed batch vs variable qty;
  broke-with-no-stock as a fail-state); narc-difficulty tuning (Suburbia
  under-differentiated from Trailer at high tiers); the fresh-empire e2e
  screenshot walk.

### Iteration 10 — 2026-07-13

- SOW-031 merged (255 tests, zero warnings): the supply side exists.
  One named supplier per zone (Lil Smoke / Miss Velvet / The Broker with
  voice lines), fronts against cards at 25% vig due in 4 runs — the
  runner's OWN run counts, so unproductive runs spend real ticks (Reed's
  run-quality pressure, now measured: blind dud play burned a window in
  3 runs and got cut off; targeted play serviced the same debt). PAY any
  time; escalation CutOff → muscle (20% seizure / bench) → Soured
  (permanent). Surfaces: shop header + FRONT/PAY, hub due-clock beside
  START RUN, map node supplier lines, ledger OWED stat. Zone identity/
  narc-hint strings moved into shop_locations.ron (SOW-029 carry
  closed). SAVE_VERSION 5 → 6 (bincode; v5 saves reset — Reed flagged).
- Reed art drop wired mid-flight: five dealer portraits (julie, marcus,
  gladys, bubba, roxanne — pool 8 → 13, no face duplication until
  roster 14) and his silhouette.png as the player character's
  placeholder ("Silhouette" key; deliberately generic pending character
  customization; legacy Barista kingpins normalize at load).
- Adversarial review (44 agents): 4 distinct defects, 2 HIGH — a
  repossessed card kept playing all session (stale DeckBuilder
  snapshot), and the broke-muscle bench could permanently softlock a
  solo empire. Both fixed + unit-pinned same-day. Pattern recorded:
  derived resources drift from SaveData — resync at every mutation
  site (GUIDANCE.md).
- SOW-032 Tutorial Arc authored (design: studio repo
  2026-07-13-tutorial-arc.md — guided play, not a mode; six beats;
  graduation at the first $500 hire) but NOT started: paused for the
  night at Reed's direction. It is the LAST item on the current
  roadmap — next-arc planning follows Reed's playtest of the full loop.
- Open for Reed: front window feel (4 vs 5 runs), muscle-bench flavor,
  arcade score formula, epitaph naming, Strip session-3 heat.

### Iteration 9 — 2026-07-12

- SOW-030 merged (225 tests, zero warnings, save byte-identical —
  derive-not-record held): LEDGER overlay with the empire strip (the six
  numbers the epitaph freezes), roster dossiers + story feeds, fallen-
  empires board browsable in play with the living empire slotted IN
  PROGRESS at its would-be rank (ties go to the dead), map zone history
  line, E1 portrait-pool fix.
- Adversarial review: 4 LOW findings, fixed same-day — panel caps with
  the IN PROGRESS row pinned (was clipping off a full board), epitaph
  feeds now honest archive-order (flat archives carry no global
  chronology — a real timeline needs the future event-log SOW), cap/tail
  logic moved into the tested view layer, harness -Hire learned 2-dealer
  scenarios. Trend: SOW-029 review caught 2 HIGH, this one only LOWs —
  the recorded lessons held by instruction.
- Open for Reed: arcade score formula (lifetime revenue for now);
  epitaph naming at game over (schema addition — awaiting nod).
- Next: SOW-031 Suppliers & Fronts (design confirmed: fronts against
  cards, 1 supplier/zone, due-dates on the run ticker, cut off → muscle
  → soured; zone strings move to RON per the SOW-029 carry).

### Iteration 8 — 2026-07-12

- SOW-029 merged (196 tests, zero warnings, +21 pure view-model tests):
  CITY MAP overlay with three node cards (status/price with live
  affordability, clientele + payout band, fiction-voice narc hints,
  stationed-dealer chips with heat/cred, best-cred ★), zone unlock and
  dealer relocation from the map through the existing SOW-024/025 code
  paths, two-click burn-then-cool rotation. e2e: full
  unlock→relocate→run-at-new-station loop verified live.
- Pre-merge adversarial review (16 agents) caught 2 HIGH defects before
  they shipped: a launch panic on pending-upgrade saves (bare ResMut on
  a conditionally-inserted resource) and overlay click fall-through
  (Node defaults FocusPolicy::Pass — the opaque canvas was visual only).
  Both fixed + re-verified live. Lesson recorded: overlays over a live
  screen need FocusPolicy::Block; state-wide resources need
  init_resource, not OnEnter inserts that can early-return.
- Process: all three repos pushed to GitHub (Reed caught local-only
  history); push is now part of every closeout. "Fork" jargon dropped.
- Art asks ledger created (studio repo art-backlog.md, full 4-way
  sweep): 13 items — notable finds beyond the known asks: the Kingpin
  wears barista.png, pimp.png double-booked buyer/dealer-pool (fix
  queued SOW-030), watermark retouch pass, per-card art as the largest
  untouched surface.
- Carried: StoryHistoryOverlay focus gap (pre-existing) → stabilization;
  zone strings → RON in SOW-031; Strip session-3 heat awaits Reed's
  playtest.
- Next: SOW-030 Kingpin Ledger (design: studio repo
  2026-07-12-kingpin-ledger.md — derive-not-record, summary-first,
  browsable fallen-empires board, map node history line).

### Iteration 7 — 2026-07-12

- SOW-028 merged (175 tests, zero warnings): The Strip (nightlife,
  $1,200) with the Pimp (×2.0, Night Shift / VIP Treatment), Back of the
  Club + VIP Room, Velvet Rope + crowd modifiers, vice-sweep narc texture
  (heat-noisy, conviction-light; tips Hot+ only, one per tier). Ecstasy/
  Ice re-zoned Block→Strip; Housewife → Block ×1.5 first rung (Wolf ×2.8
  flag closed — pinned by test and drawn live).
- Measured, one tuning iteration: Corner fresh floor was regressing to
  Scorching-135 (root cause ~90% buyer reaction cards) → tuned to Cold-0;
  Strip stays hottest per session BY DESIGN (heat tax, zero busts in six
  blind sessions — feeds the burn-then-cool rotation); Block busts at
  Blazing with 5-run sentences (contrast intact).
- Systems insight recorded: self-referential heat sources (tips→heat→
  tiers→tips) compound — ration them per tier when authoring narcs.
- For Reed: art asks (club back-alley, VIP lounge backgrounds); judgment
  call on Strip session-3 heat (+85 at Blazing) after a human playtest.
- Process: fork committed to local main by mistake; branch rebuilt, main
  reset with Reed's approval, merge --no-ff as usual. Nothing lost.
- Next: SOW-029 City Map Screen (three real nodes to show).

### Iteration 6 — 2026-07-12

- SOW-027 merged (172 tests, ZERO warnings — all 41 pre-existing removed,
  −355 lines dead code): compositions with sparse inheritance, multipliers/⚖
  retired, Lay Low ($200/2 runs/−40) + Lawyer ($625/−25), Heat upgrade
  wired positive-only.
- Measured: fresh blind floor Warm-35/Hot-85, no busts (was Inferno-184 with
  fresh GAME OVERs); target-play unregressed; Wolf ×2.8 confirmed gated
  behind Coke — first-rung fix designed into SOW-028 zone coherence.
- Reed: creative freedom on zone three (+ adjust zones 1-2), tutorial arc
  onto the roadmap (optional, no skip benefit), three threads closed.
  Three-zone city designed (Corner / Strip / Block).
- Next: SOW-028 The Strip.

### Iteration 5 — 2026-07-12

- SOW-026 merged (174 tests): 8-card Weed-only start, full cash+cred ladder
  (Shrooms $100·1 → Fentanyl $12k·6), pacing measured (target-play reaches
  Shrooms session 2-3; Block 10-20 sessions), one tune (Shrooms $150→$100).
- Exposed the fresh-floor problem (Inferno-184 in 3 blind sessions) —
  carried as SOW-027's measured acceptance bar.

### Iteration 4 — 2026-07-12

- SOW-025 merged (170 tests): dealers stationed per area, +1 cred per Safe
  deal (never decays), moves at $250 + 1-run downtime via the sentence
  ticker, cred-gated shop items with "unlocked by <dealer>" credit line and
  "NEEDS CRED N (best: M)" locked states — all verified live on the hustler
  scenario, including a user-clicked double relocation during acceptance.
- Pilot gates: Storage Unit (3 Corner cred), Heroin (5 Block cred); Shrooms
  couldn't take the pilot (it's starting collection) — becomes real in the
  SOW-026 re-laddering.
- Tuning flags carried: move fee vs hire vs bail feel, cred thresholds,
  sentence constant, Wolf ×2.8.
- Next: SOW-026 Content Authoring Pass.

### Iteration 3 — 2026-07-12

- SOW-024 merged: The Block purchasable ($2,000), its ~$49k of authored stock
  revived, two-stage territory run selection (Wolf e2e-confirmed as Block
  clientele), shop_locations.ron promoted to validated source of truth,
  harness tab/overlay coordinates fixed.
- Design iterated with Reed mid-flight: areas are TERRITORIES; the difficulty
  gradient is authoring-first (lean start, shop ladder); dealers get
  stationing + per-area street cred (decisions locked in the studio repo).
  Roadmap resequenced above.
- Next: SOW-025 Street Cred & Stationing.

### Iteration 2 — 2026-07-12

- SOW-023 all phases complete on `sow-023-kingpin-dealers` (Review):
  dealer roster + turn-based jail + bail + kingpin-as-dealer (Phases 1-2),
  operations panel + arcade game-over board (Phase 3), forge/isolated-save/
  closed-loop playtest harness (Phase 4).
- Reed's design answers folded in mid-flight: turn-based sentences scaling
  with heat, proportional time-served heat reduction, prior-conviction scars,
  bail tradeoff, kingpin game-over feeding a fallen-empires leaderboard
  (stats shown, stories archived for SOW-026).
- e2e-verified live: dealer select, jail tick on run completion, bail cost
  decay, per-dealer heat transfer, global cash, HIRE cost doubling.
- SOW-023 accepted (user playtest + hot-scenario e2e of the GAME OVER board) and merged to main.

### Iteration 1 — 2026-07-12
- SOW-022 (Game Play v2) accepted + merged after playtest-directed iterations.
- Baseline scripted playtest: blind 3-slot play busts on hand 1
  (Evidence 64 > Cover 45) vs a career-heat-scaled narc; BUSTED offers only
  END RUN (permadeath). Confirms: difficulty-by-heat works, but permadeath
  makes it a wall, not a wager → P1 is correctly first.
- RFC-023 + SOW-023 authored (kingpin & dealers foundation).
- Open questions logged in SOW-023 Discussion.

## Backlog (post-core)

- **Character customization** (Reed, 2026-07-12): player-character
  portrait picker (and whatever else grows from it). The shipped
  silhouette placeholder is its hook — `normalize()`'s silhouette line
  is what a picker replaces.
- **Event log SOW** (from SOW-030 review): timestamps on stories would
  enable a true cross-empire timeline; epitaph naming rides the same
  schema change if Reed nods.
- **Stabilization pass candidates:** StoryHistoryOverlay focus-policy
  gap (pre-SOW-029); "ROUGHED UP" dealer status for the muscle bench
  (currently reads MOVING); scroll machinery if ledger caps+tails start
  to chafe.
