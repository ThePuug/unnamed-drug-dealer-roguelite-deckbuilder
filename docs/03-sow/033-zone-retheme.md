# SOW-033: Zone Re-Theme — Three Neighborhoods

## Status

**Planned** - 2026-07-13 (jumps ahead of SOW-032; foundational content
that every future zone/buyer/narc inherits)

## References

- **Design:** studio repo `design-updates/2026-07-13-zone-retheme.md`
- **Mapping sweep:** 5-agent touchpoint map (2026-07-13) — its full
  checklist + silent-risk flags are reproduced in §5–§6 below.
- **Prior art:** SOW-028 (third zone), SOW-031 (suppliers & fronts),
  SOW-021 (SAVE_VERSION policy: version-mismatched saves fall back to
  fresh, no field migration).
- **Branch:** `sow-033-zone-retheme`
- **Implementation Time:** 3-4 days (large content refactor + actor-art
  system, §7)

---

## 1. Goal

Rename and re-theme the three meta-areas from tier-labels ("The Corner /
Strip / Block") to three **low-heat neighborhoods** so the early game
coheres — each zone themed across clientele, narcs, supplier, and
unlockables. Reduce shop stock to **exactly 2 products per zone**. Grow
clientele to **3 buyers per zone** (9 personas). Shelve (preserve,
unhook) the higher-heat premium tier for a future mid-game arc.

**Locked mapping** (design-fixed — do NOT re-derive):

| Old id | New id | Rung | 2 products | Buyers | Supplier |
|---|---|---|---|---|---|
| `the_corner` | `trailer_park` | 0 — START, unlocked, $0 | Weed, Shrooms | Biker, Tweaker, Deadbeat *(all new)* | Lil Smoke |
| `the_block` | `suburbia` | 1 — MIDDLE, $1,200 | Codeine, Xanax *(new card)* | Desperate Housewife *(re-homed)*, Widow *(new)*, Frat Bro *(re-homed)* | Deb *(re-theme of The Broker slot)* |
| `the_strip` | `red_light_district` | 2 — TOP, $2,500 | Ecstasy, Coke *(re-homed down)* | Pimp *(re-homed)*, Working Girl *(new)*, Club Kid *(new)* | Miss Velvet |

**⚠ Progression flips.** Today the RON order is corner(0) → strip(1) →
block(2). After this SOW the order is **[trailer_park, suburbia,
red_light_district]** and red_light is the TOP (hardest, priciest). The
RON array MUST be reordered and prices swapped (top > middle). Rung =
array position drives ladder attainability, first-unlocked fallback, MOVE
targets, ledger cred order, map node left→right order, and the e2e
node-X table. A find/replace that leaves order intact silently inverts
the ladder — see §6.

**Shelve set** (remove `shop_location`/price/cred; keep the definition):
products **Acid, Ice, Heroin, Fentanyl**; buyer **Wall Street Wolf**
(its 7 reaction cards orphan harmlessly). **Coke is NOT shelved** — it
re-homes to red_light_district (rebase its price down from $5,000).

---

## 2. Resolved content decisions (the sweep's 13 open items)

1. **Frat Bro → suburbia**, demand Weed/Ecstasy unchanged (Weed
   serviceable from start; Ecstasy is the red-light reach that motivates
   expansion). Trailer's ≥1-clientele requirement is met by its 3 new
   buyers.
2. **Suburbia 2nd product = Xanax** (new card). Stats target
   `Product(price: 55, heat: 12)`; pharma/benzo; on-theme with the
   Housewife's "managing her anxiety" and the Widow's grief.
3. **Acid → shelved.**
4. **Prostitute = new red-light BUYER, display_name "Working Girl."**
   Pimp stays alongside her; Club Kid is the third.
5. **Trailer buyers = 3 new** (Biker, Tweaker, Deadbeat); Frat Bro leaves.
6. **Housewife "Rock Bottom" Fentanyl → Xanax** (keeps her whole demand
   inside Suburbia's two products, Codeine + Xanax).
7. **Unlock prices:** Trailer $0 (keep), Suburbia $1,200, Red Light
   $2,500 (top > middle).
8. **Narc difficulty:** swap the two override blocks so red_light (top)
   is hardest and suburbia (middle) is middle; re-theme flavor —
   red_light = spiky vice sweeps, suburbia = nosy-neighbor/HOA pressure
   that builds evidence slowly. Trailer keeps the empty override (inherits
   the gentle default baseline).
9. **Storage Unit → suburbia.**
10. **Clean Money modifier → red_light_district** (laundering fits the
    cash-heavy night trade).
11. **Suburbia supplier = "Deb"** — re-theme The Broker slot into a
    suburban middlewoman (voice: warm, transactional, neighborly — e.g.
    "Sweetie, I keep the cul-de-sac comfortable. You're good for it.").
    The Broker persona returns with the Business District arc.
12. **Product economy re-ladder** (per-zone cred model kept). Targets —
    tune exact values to keep `test_shipped_demands_attainable_on_ladder`
    at **zero warnings**:
    - Trailer: Weed (cred 0 / $0), Shrooms (cred 1 / $100).
    - Suburbia: Codeine (cred 1 / ~$400), Xanax (cred 2 / ~$800).
    - Red Light: Ecstasy (cred 1 / ~$1,600), Coke (cred 3 / ~$2,800,
      rebased from $5,000).
13. **Wolf's 7 orphaned reaction cards → leave in place** (harmless).

**Also:** Plea Bargain insurance ($4,000, block→suburbia) — re-ladder to
suburbia's tier or shelve, implementation's call under the zero-warning
constraint. `at_the_park` / `abandoned_warehouse` carry a re-skin art
flag (playground/industrial art vs new zone) — log to art backlog, no
code change.

---

## 3. New buyer persona briefs

All 6 draw 7-card reaction decks **from the existing pool** (the ~24
reaction modifiers in `modifiers.ron` + zone-appropriate Location cards);
**no new reaction cards required.** Each persona needs: `display_name`,
`area`, `demand` (products/locations/description), base/reduced
multipliers, `reaction_deck_ids` (7, all must resolve to real card ids),
2 `scenarios` with `heat_threshold` + narrative_fragments (3 subject
clauses, 3 need clauses), `active_scenario_index: None`. **Every demand
and scenario product/location string must exactly match a live card name**
(`validate_buyer_demand_strings` panics otherwise).

**Trailer Park:**
- **Biker** — bulk Weed, loyal, low paranoia. Higher base (~1.8). Deck
  leans prior_conviction / second_supplier / negotiation / sketchy_business
  / secrecy + trailer locations (Dead Drop, Parking Lot). Scenarios:
  "Club Run" (weed for the whole MC), "Personal Stash." Voice: gruff,
  unbothered.
- **Tweaker** — erratic, takes whatever, pays poorly (base ~1.2, reduced
  0.8). Deck: struggling_addiction / gossip_girl / making_a_scene /
  noise_complaint / prior_conviction / alternative_payment. Scenarios:
  "Coming Down," "Chasing It." Voice: jittery, paranoid.
- **Deadbeat** — hard-luck regular, Weed/Shrooms, leans alternative_payment
  (base ~1.3). Deck: alternative_payment / secrecy / struggling_addiction
  / negotiation / prior_conviction + a trailer location. Scenarios:
  "Rent's Due," "Just Unwinding." Voice: apologetic, familiar.

**Suburbia:**
- **Widow** — grief, loneliness; Codeine/Xanax; overpays for company
  (base ~1.6, obvious_wealth + alternative_payment). Locations Safe
  House / By the Pool / At the Park. Deck: secrecy / gossip_girl /
  obvious_wealth / alternative_payment / struggling_addiction /
  negotiation / prior_conviction. Scenarios: "Empty House" (Xanax to
  sleep), "Keeping Up Appearances" (Codeine, hides it — gossip risk).
  Voice: dignified, quietly unraveling.

**Red Light District:**
- **Working Girl** (the Prostitute) — uppers for the shift; Coke/Ecstasy;
  alternative_payment heavy (base ~1.4). Locations Back of the Club /
  VIP Room / Parking Lot. Deck: alternative_payment / secrecy /
  sketchy_business / making_a_scene / crowd_cover / prior_conviction /
  negotiation. Scenarios: "Long Night" (Coke, stay up), "Regulars" (VIP,
  discretion, Ecstasy). Voice: weary, guarded, streetwise.
- **Club Kid** — party, reckless spender (base ~2.0), high heat.
  Ecstasy/Coke; Back of the Club / VIP Room. Deck: making_a_scene /
  invite_more_people / noise_complaint / crowd_cover / comped_bottles /
  invite_only / prior_conviction. Scenarios: "Peak Night" (Ecstasy),
  "Afterparty" (Coke, VIP). Voice: euphoric, oblivious to risk.

---

## 4. Implementation Plan (phased; keep the build green at each phase)

### Phase 1 — Mechanical rename + reorder (lights stay on)

Rename the three ids everywhere (§5), **reorder** the `shop_locations.ron`
array to [trailer_park, suburbia, red_light_district], swap the two locked
unlock prices (Suburbia $1,200 < Red Light $2,500), re-home every card
`shop_location`, shelve Acid/Ice/Heroin/Fentanyl, move Coke →
red_light_district. Re-home the existing 4 buyers (Housewife→suburbia,
Pimp→red_light, Frat Bro→suburbia, **shelve Wolf**) and fix the two
load-breakers (Pimp Ice→Coke ×2, Housewife Fentanyl→Xanax). Author the
**Xanax** product. Bump **SAVE_VERSION 6→7**. Fix all Rust literals/
defaults (§5B) — especially the three DANGEROUS silent fallbacks in §6.
**Exit criteria:** `cargo build` + `cargo test` green (existing tests
updated in lockstep), game loads, a fresh save starts in Trailer Park.

### Phase 2 — Content authoring + theme

Author the **6 new buyer personas** (§3) — 3/zone rosters complete.
Re-theme zone `name`/`description`/`identity`/`narc_hint`. Re-theme the
supplier (**Deb**). **Swap + re-theme the narc override blocks** so
red_light (top) is hardest, suburbia (middle) is middle, with themed
flavor. Re-ladder product prices/cred to **zero attainability warnings**.
**Exit criteria:** all validators green, `test_shipped_demands_attainable_
on_ladder` reports zero warnings, `cargo test` green, zero compiler
warnings on build AND test.

### Phase 3 — Actor art & naming (§7)

Establish the `<role>-<slug>.png` naming template, **move the hard-coded
portrait map out of `loader.rs` into RON** (pays down art-backlog E3),
rename the actor files to the template, wire the 6 new buyer portraits,
and add **per-area narc art** — all three zones ship distinct narc art
(`narc-trailer-park.png`, `narc-suburbia.png`, and the original narc as
`narc-default.png` for Red Light + future-zone fallback).
**Exit criteria:** portraits load from RON; a missing mapped file is a
**loud** load error (not a silent fallback); build + tests green.

### Phase 4 — Verification

Update `forge.rs` scenarios + `playtest.ps1` node-X map (§5D — rename
keys AND re-map X to the new order). Fresh-empire e2e walk: launch →
confirm Trailer Park start → unlock Suburbia → unlock Red Light,
screenshots to `out\sweep33\` (confirm the trailer-park narc portrait +
new buyer portraits render). Update feature matrices, write the SOW
Discussion (shipped numbers + any deviations). Roadmap Iteration 11 entry
is the coordinator's.

---

## 5. Touchpoint checklist (from the mapping sweep — exhaustive)

### 5A. RON content (`assets/`)

**`assets/data/shop_locations.ron`** (definition order = ladder rung):
rename the three `id`s; rewrite each `name`/`description`/`identity`/
`narc_hint`; KEEP `unlocked`/`price` flags per the ladder (Trailer
unlocked/$0; Suburbia $1,200; Red Light $2,500); **REORDER** to
[trailer_park, suburbia, red_light_district]; re-theme Broker→Deb
(L48-51); fix stale ladder comment L26-27.

**`assets/cards/products.ron`:** weed L17 / shrooms L31 → `trailer_park`;
codeine L46 → `suburbia`; **acid L61 → SHELVE**; ecstasy L77 →
`red_light_district`; **ice L93 → SHELVE**; coke L108 → `red_light_district`
(rebase price); **heroin L123, fentanyl L139 → SHELVE**; author **Xanax**
(suburbia); re-ladder per-zone price/cred.

**`assets/buyers.ron`:** header comment L1-2 → new roster; Frat Bro L8
area → `suburbia`; Housewife L79 → `suburbia`, scenario L105 Fentanyl→Xanax;
**Wolf L147-209 → SHELVE entire persona**; Pimp L214 → `red_light_district`,
L217 & L257 Ice→Coke; author Prostitute("Working Girl"), Club Kid, Biker,
Tweaker, Deadbeat.

**`assets/narc_deck.ron`:** L91-93 `the_corner`→`trailer_park` (keep
empty); **swap** L108 `the_strip` block ↔ L155 `the_block` block contents
so red_light (top) hardest / suburbia (middle) middle, then re-key to new
ids; re-theme comments.

**`assets/cards/locations.ron`** (shop_location only; card NAMES stay):
dead_drop L21, parking_lot L37, at_the_park L53 → `trailer_park`;
storage_unit L70 → `suburbia`; safe_house L90, abandoned_warehouse L107 →
`suburbia`; back_of_the_club L195 → `red_light_district`. Buyer-only cards
(no shop_location) unchanged.

**`assets/cards/modifiers.ron`:** burner_phone L12, lookout L20,
false_trail L28 → `trailer_park`; disguise L39 → `suburbia`; **clean_money
L48 → `red_light_district`**; velvet_rope L59 → `red_light_district`.

**`assets/cards/cover.ron`:** alibi L10, fake_receipts L18 → `trailer_park`;
bribe L26, bribed_witness L35 → `suburbia`.

**`assets/cards/insurance.ron`:** fake_id L10 → `trailer_park`; plea_bargain
L18 → `suburbia` (re-ladder or shelve, §2.13-adjacent).

### 5B. Rust production code

- `src/save/types.rs`: **L15 SAVE_VERSION 6→7**; L1060 `DEFAULT_STATION`
  `the_corner`→`trailer_park` (auto-propagates); L1352 fresh
  `unlocked_locations` `the_corner`→`trailer_park`; comment L1798 "(5→6)"
  → "(6→7)".
- `src/systems/shop.rs` L26 `selected_location` default → `trailer_park`
  (**silent**, §6).
- `src/models/buyer.rs` L28 `default_persona_area()` → `trailer_park`
  (**silent**, §6).
- `src/ui/setup.rs` L198 literal `["the_corner"]` fallback → `trailer_park`
  (**silent — bypasses DEFAULT_STATION**, §6).
- **Verify-only** (auto-follow DEFAULT_STATION): `input.rs` L453/457/462/465,
  `data/narc_deck.rs` L19/21, `hand_state/mod.rs` L113.
- **Order-sensitive, re-verify after reorder:** `ui_update.rs` L740-749,
  `input.rs` L462, `ledger_view.rs` L87-105, `map_view.rs`
  (zone_status/zone_node_view/native_products), `city_map.rs` L224,
  `loader.rs` L506-539 (ladder_attainability_warnings).

### 5C. Tests (update in lockstep — non-exhaustive high-value list)

`loader.rs`: L868 area-id vec (**note reorder**), L870 price, L879
supplier order → [Lil Smoke, Deb, Miss Velvet], L851-860 persona-areas
(Wolf shelved), L884/L890 Pimp/Housewife areas, L896-902 strip stock
(Ice shelved), L907, L1002 (zero-warning), L1013-1047, L1052, L1072,
L1120/L1136 → trailer_park, L761/L820/L900 builders, L831-848.
`save/types.rs`: L1616-1816 front/standing, L1796-1816, L1949-2007,
L2108-2168. `models/shop_location.rs` L117-173. `data/buyer_personas.rs`
L51-68. `ui/map_view.rs` L299-513. `ui/ledger_view.rs` L330-342
(hard-coded display names!), L387-427, L644-721. `ui/front_view.rs`
L180-292. `ui/view.rs` L610. `models/test_helpers.rs` L18/31/57/70/83/166/
210. `models/hand_state/state_machine.rs` L582-584.
`models/narrative/story_test.rs` L138/176.

### 5D. e2e / forge

- `src/save/forge.rs`: L61/69/72 (hustler `the_block`→`suburbia`),
  L80-85 (nightowl strip+block → red_light+suburbia), L94/108/113 (legacy
  strip→red_light), L50 comment, L237-248 shape asserts. Keep
  `every_scenario_validates_and_roundtrips` (L204) green at v7.
- `tools/e2e/playtest.ps1`: L85 node-X switch — **rename keys AND re-map
  X to new order**: trailer_park=450, suburbia=960, red_light_district=1470
  (renaming keys without swapping X lands clicks on the wrong node —
  silent e2e failure). L36 comment.

---

## 6. Silent-risk flags (NOT caught by the compiler — verify live)

1. **SAVE_VERSION bump is the reset mechanism.** Every area-keyed save
   field stores the raw id string; without the 6→7 bump a v6 save
   deserializes carrying dead `the_corner`/etc. ids. Bumping forces the
   clean wipe (io.rs:100 rejects version mismatch → fresh account).
2-4. **Three silent default fallbacks** — `shop.rs:26` selected_location,
   `buyer.rs:28` default_persona_area, `setup.rs:198` literal set (bypasses
   DEFAULT_STATION). A stale id here → empty shop tab / a persona that
   vanishes from every run, no crash.
5. `input.rs:458-467` run-area else → first-unlocked with only a `warn!`.
6. `data/narc_deck.rs:16-22` runtime `or_else` serves DEFAULT_STATION's
   deck (load-time loader.rs:238-241 panics first on an unknown key).
7-8. `ledger_view`/`front_view`/`map_view` render the **raw id verbatim**
   when an area isn't found (visible symptom of a missed rename).
9. `forge.rs` literal ids compile fine but build orphaned scenarios —
   the roundtrip test is the net.
10. **Order inversion** — reorder the RON array, don't just rename in place.
11. **Narc swapped-but-not-retuned** — keys resolve but suburbia stays
    harder than red_light (silent design bug).

**Loud safety net (WILL panic in debug on a bad id — your correctness
harness):** `loader.rs:182-186` (card shop_location known), `:238-241`
(narc override area exists), `:350-355` validate_persona_areas (area
exists AND ≥1 clientele), `shop_location.rs:47` validate_shop_locations,
`loader.rs:643-670` validate_buyer_demand_strings (every demand/scenario
string matches a live card name — turns a shelved-product reference into
a hard load failure).

---

## 7. Actor art & naming system (Reed, 2026-07-13)

**Goal:** consistent art filenames + portrait mappings authored in RON
(not hard-coded in `loader.rs`), and per-area narc art. Resolves art-
backlog **E3** (portrait maps hard-coded → new personas silently get no
art). Reed dropped `narc-trailer-park.png` (already renamed from the
Gemini export) and pre-positioned the buyer faces.

### Naming template — `assets/art/actors/<role>-<slug>.png`

- **`buyer-<slug>.png`** — one per buyer persona (slug = display_name).
- **`dealer-<slug>.png`** — hire-pool faces.
- **`narc-<slug>.png`** — per-area narc (slug = area, hyphenated) +
  **`narc-default.png`** fallback.
- **`silhouette.png`** — player placeholder, kept special (the
  unclaimed-slot fiction; already special-cased in `normalize()`).

### File renames (`git mv` in the assets submodule — stage by explicit
path only; NEVER touch `ui-concept.png` or `style-reference/`)

| Persona / role | New file | From (existing) |
|---|---|---|
| Frat Bro | `buyer-frat-bro.png` | `frat-bro.png` |
| Desperate Housewife | `buyer-desperate-housewife.png` | `desperate-housewife.png` |
| Widow | `buyer-widow.png` | `widow.png` |
| Pimp | `buyer-pimp.png` | `pimp.png` |
| Working Girl | `buyer-working-girl.png` | `street-walker.png` |
| Club Kid | `buyer-club-kid.png` | `pretty-woman.png` *(placeholder — shuffle later)* |
| Biker | `buyer-biker.png` | `hells-angel.png` |
| Tweaker | `buyer-tweaker.png` | `hippie.png` |
| Deadbeat | `buyer-deadbeat.png` | `flower-child.png` |
| Wall Street Wolf *(shelved)* | `buyer-wall-street-wolf.png` | `wall-street-wolf.png` |
| *(spare buyer art)* | `buyer-displaced-patriot.png` | `displaced-patriot.png` |
| Julie/Marcus/Gladys/Bubba/Roxanne/Barista | `dealer-<name>.png` | `<name>.png` |
| Narc (Trailer Park) | `narc-trailer-park.png` | *(done)* |
| Narc (Suburbia) | `narc-suburbia.png` | `Gemini_Generated_Image_kdfdxmkdfdxmkdfd_256w_nobg.png` |
| Narc (Red Light / default fallback) | `narc-default.png` | `narc.png` |
| Player placeholder | `silhouette.png` | *(unchanged)* |

*Art→persona assignments are Reed's aesthetic call and explicitly
**shuffleable** — the RON mapping makes re-assignment a one-line edit.*

### RON schema

- **`buyers.ron`:** add `portrait: "buyer-<slug>.png"` to every persona.
- **`shop_locations.ron`:** add `narc_portrait: Option<String>` per area
  — `trailer_park` → `"narc-trailer-park.png"`; `suburbia` →
  `"narc-suburbia.png"`; `red_light_district` → `"narc-default.png"` (the
  original narc art, kept as the fallback for any future zone). All three
  zones now ship explicit narc art; `None` still falls back to
  `narc-default.png` for zones authored later.
- **Dealer pool** (`DEALER_PORTRAIT_POOL`, `src/save/types.rs`): update
  the filenames to `dealer-*.png` (moving the pool wholesale to RON is a
  nice-to-have; at minimum the filenames must track the renames).

### Loader + render changes

- `loader.rs:717-740` `load_actor_portraits` — **delete the hard-coded
  `HashMap`**; build the portrait map from the RON `portrait` /
  `narc_portrait` fields instead.
- **Loud validation:** at load, `std::path::Path` — for every mapped
  portrait, assert the file exists under `assets/art/actors/`; a missing
  file is a hard error (Bevy's async `asset_server.load` would otherwise
  fail silently). This is the E3 "loud error" requirement.
- `ui_update.rs:1073-1077` narc portrait — currently "always Narc";
  change to look up the **current run area's** `narc_portrait` (fallback
  `narc-default.png`).
- New buyer personas resolve their portrait via the RON field — no
  hard-coded entries.

### Art backlog follow-ups (log at closeout, ship on placeholder)

- `buyer-club-kid.png` real art (using `pretty-woman.png` as placeholder).
- The `at_the_park` / `abandoned_warehouse` re-skin flags from §2.

---

## Acceptance Criteria

**Functional:** three neighborhoods (Trailer Park unlocked start →
Suburbia $1,200 → Red Light $2,500); exactly 2 products per zone; 3
buyers per zone (9 personas); Wolf + Acid/Ice/Heroin/Fentanyl shelved
(definitions preserved, unhooked); a fresh empire starts in Trailer Park;
save loads clean at v7.
**Content integrity:** all loud validators pass; `test_shipped_demands_
attainable_on_ladder` reports zero warnings; no demand/scenario references
a shelved product.
**Actor art (§7):** portrait mappings authored in RON, not hard-coded;
a missing mapped portrait file is a **loud** load error; the Trailer Park
narc + all 9 buyer portraits render; files follow the `<role>-<slug>`
template.
**Code Quality:** zero warnings on `cargo build` AND `cargo test`;
deletion over `#[allow]`; new buyer view logic (if any) unit-tested in a
`_view` module. e2e fresh-empire walk screenshots in `out\sweep33\`.

---

## Discussion

*Populated during implementation.*

---

## Acceptance Review

*Populated after implementation.*
