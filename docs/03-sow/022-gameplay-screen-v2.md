# SOW-022: Game Play Screen v2

## Status

**Review** - 2026-07-12 (implementation + adversarial review + e2e verification complete; awaiting user playtest sign-off)

## References

- **Design:** claude.ai/design project "Game views mockup" → `Game Play v2.dc.html` (1920×1080 mockup; supersedes the v1 layout implemented by SOW-011)
- **RFC:** none (direct design-to-implementation order, per SOW-021 precedent for single-SOW work)
- **Related:** SOW-011-A/B (current layout), RFC-017/018 (upgrade badges preserved), SOW-021 (turn indicator, exhaustion messaging)
- **Branch:** sow-022-gameplay-v2
- **Implementation Time:** 1 day

---

## Design Summary (what changes on screen)

The v2 mockup replaces the v1 "three columns + four rows" gameplay layout with a
character-anchored composition over the location background:

| Region | v1 | v2 |
|---|---|---|
| Player resources | none in-run (cash only on overlay); vertical heat bar next to scenario card | **YOUR STANDING** panel (left-mid): session cash + heat with tier chip (Warm/Hot/…), horizontal 0–100 bar with conviction-threshold tick marks |
| Turn state | text in counters row | Top-center **ROUND n / 3 · DEAL IN PROGRESS** + actor pill (**NARC'S MOVE** / YOUR MOVE / BUYER REACTING) |
| Narc | left column of face-down card backs + portrait in hand row | Top-left portrait with turn spotlight, name plate, face-down count chip, and an **intent bubble** (next card telegraph while the Narc is acting; last played card after) |
| Buyer | scenario text card + right column of face-up buyer cards + portrait | Top-right portrait with spotlight, name plate, count chip, **wants bubble** (scenario / demand / payout, hover for detail), and a **BAILS AT HEAT n** chip. Buyer hand is now hidden (count only) |
| Played cards | active slot row + wrapping played pool | **THE DEAL ON THE TABLE**: active slots (Location/Product/Conviction) + ghost **+ INSURANCE** slot. Evidence/Cover/Modifier plays aggregate into the balance bar and discard stack |
| Totals | three text counters | **EVIDENCE vs COVER balance bar** with SAFE/AT RISK + PAYOUT ×n chips |
| Deck/discard | "Deck: n" text; discard name list | Bottom-corner card **stacks**: deck (face-down, count) and discard (top card face-up, count) |
| Hand | flat 3-slot row | **Fanned arc** (rotation + lift), hover raises card |
| Actions | Pass/Bail Out gray boxes | Styled PASS / BAIL OUT buttons bottom-right of hand |

Resolution overlay, deck builder, shop, and upgrade screens are unchanged.

## Implementation Plan

### Phase 1: View-model foundation

**Goal:** All new screen logic exists as pure, unit-tested functions before any UI wiring.

**Deliverables:**
- `src/ui/view.rs` — pure view-model functions + tests

**Architectural Constraints:**
- Fan geometry (offsets/rotation/lift/z per hand slot), evidence-vs-cover split,
  discard-stack derivation (chronological walk of `cards_played`: Evidence/Cover/Modifier
  plays + overridden slot cards), narc intent preview (tier-scaled stat rows),
  turn-pill selection, conviction tick derivation, and cash formatting are pure
  functions of `HandState` data — no ECS types beyond model structs
- Conviction ticks derive from the Narc's actual Conviction cards (content-driven,
  not hardcoded — demand-string lesson from SOW-021)
- Narc intent must apply the same `narc_upgrade_tier` scaling the engine applies

**Success Criteria:**
- Fan layout: symmetric offsets, middle card highest, edge cards most rotated
- Discard view: override + E/C/M ordering matches play chronology; empty hand → count 0
- Balance split: 0/0 → 50%; monotone in evidence share
- Intent preview: Tier1 narc shows +10% evidence values
- All tests pass alongside existing 107

### Phase 2: Screen restructure

**Goal:** `create_ui` builds the v2 composition; all v1-only regions removed.

**Deliverables:**
- `src/ui/theme.rs` — SOW-022 palette + card size constants
- `src/ui/components.rs` — new markers; dead v1 markers removed
- `src/ui/setup.rs` — rewritten gameplay tree (background layer + resolution overlay preserved)
- `src/ui/helpers.rs` — `CardSize::{Table, Hand, Compact}` variants

**Architectural Constraints:**
- All colors as named `theme.rs` constants (SOW-011 rule); Bevy 0.18
  `BackgroundGradient`/`BoxShadow`/`BorderRadius`/`UiTransform` for gradients, glows,
  rounded chips, and the card fan — no new image assets
- Root stays a fixed 1920×1080 `UiRoot` node compatible with `scale_ui_to_fit_system`
  and `toggle_game_state_ui_system` (markers unchanged)
- Card rendering stays on the `helpers.rs` single path (template + overlays + upgrade
  badges + foil); emoji glyphs keep the separate-`EmojiFont`-entity pattern
- Buttons keep `CheckButton`/`FoldButton`/`RestartButton`/`GoHomeButton` markers
  (input systems `.expect()` on them)

**Success Criteria:**
- `cargo build` clean, no new warnings
- Every v2 mockup region present at mockup coordinates; no v1 remnants (scenario card,
  buyer visible hand, narc card column, played pool, discard list)

### Phase 3: System rebinding

**Goal:** Every v2 element live-updates from real game state.

**Deliverables:**
- `src/systems/ui_update.rs` + `src/ui/systems.rs` — new/rewritten update systems
- `src/systems/input.rs` — button state visuals for the new styling
- `src/main.rs` — registration updates

**Architectural Constraints:**
- `Changed<HandState>` gating for rebuild-style systems; change-guarded writes for
  per-frame text systems (SOW-021 rule)
- Narc intent: telegraph `hand[0]` only while the Narc is the pending actor; after the
  Narc acts show the card actually played this round (no new hidden-info leak beyond
  the 1s telegraph window)
- Buyer hand rendering removed ⇒ buyer cards become hidden information (intentional,
  supports fun-assessment root cause #2)
- Insurance ghost slot appears only while no Insurance is active; PASS/BAIL OUT keep
  existing enable/disable semantics
- Hand hover: `Interaction` on existing card `Button`s adjusts wrapper transform/z only

**Success Criteria:**
- Full hand loop playable: intent → narc plays → your move (hover + play/pass) →
  buyer reacts → resolution overlay → new deal
- Heat bar, tier chip, balance bar, payout chip, both count chips, deck/discard stacks
  all track state; buyer bail chip matches scenario threshold
- `cargo test` passes; no runtime panics from removed markers

### Phase 4: Documentation

**Deliverables:**
- Feature matrix updates (`core-gameplay-loop`, `heat-system`)
- SOW index + this SOW's Discussion/Acceptance sections

---

## Acceptance Criteria

**Functional:** all Phase 3 success criteria; resolution/permadeath/exhaustion flows unregressed.

**UX:** every datum visible in v1 remains discoverable in v2 (scenario detail moves to
hover; buyer hand intentionally hidden); player always knows round, actor, heat, and
safety margin at a glance.

**Performance:** no per-frame despawn/respawn beyond existing `Changed<HandState>` idiom.

**Code Quality:** view logic pure + unit tested; no hardcoded colors outside theme.rs;
no new warnings.

---

## Discussion

### Implementation Note: Narc intent semantics

The mockup shows an intent bubble but a mockup can't express timing. Decision:
while the Narc is the pending actor the bubble telegraphs `hand[0]` (the card
`ai_betting_system` will play) with tier-scaled values; after the Narc acts it
relabels to `PLAYED · <name>` for the rest of the round. The telegraph reveals
hidden info only during the ~1s AI-timer window before the card becomes public
anyway. Conviction thresholds display RAW because `resolve_hand` checks them
unmultiplied — the card face shows a multiplied threshold (pre-existing
display/engine mismatch in `helpers.rs`, out of scope here, worth its own fix).

### Implementation Note: Discard stack derivation

`HandState.discard_pile` only records overridden slot cards, but the mockup's
discard top card is a COVER card — so the v2 discard stack means "everything
resolved into the past this hand." `view::discard_view` derives it
chronologically from `cards_played` alone (E/C/M plays + slot overrides via a
replay walk), keeping `discard_pile` untouched as the engine-facing record.

### Implementation Note: Heat bar scale

v1's heat bar was denominated in the buyer's scenario threshold. v2 separates
the concerns: the YOUR STANDING track is a fixed 0–100 scale carrying
conviction-threshold ticks (derived from the Narc's actual Conviction cards,
content-driven per the SOW-021 authorability lesson), while the buyer's cap
lives on the BAILS AT HEAT chip. Heat above 100 clamps the fill; the numeric
readout stays truthful.

### Implementation Note: Legacy removals

Dead code orphaned or surfaced by the restructure was pruned rather than
suppressed: `HandPhase::FoldDecision` (never constructed since SOW-008),
`CardSize::Medium`, four legacy card-spawn helpers, and v1-only theme
constants/markers. Warning count is 40 vs 43 on main — no new warnings.

### Implementation Note: PASS button styling vs. enable/disable

The v2 PASS button uses a `BackgroundGradient` face, which the old
every-frame `BackgroundColor` overwrite in `update_betting_button_states`
would fight. The system now swaps gradient/border/text-color variants and is
gated by a `Local<Option<bool>>` so it writes only on state change.

---

### Implementation Note: e2e verification driver

Visual verification was done by driving the real game window:
`tools/e2e/game-drive.ps1` screenshots (occlusion-proof `PrintWindow`) and
clicks/hovers in 1920×1080 design coordinates, converting through the UiScale
letterbox and monitor DPI (auto-detected). Two hard-won lessons baked into the
script: posted `WM_LBUTTONDOWN` messages do NOT drive winit/bevy picking (real
input with a foreground-verification guard is required), and a DPI-unaware
capture process sees virtualized coordinates (screenshots silently become
top-left crops on a 150% monitor).

---

## Acceptance Review

### Scope Completion: 100%

- ✅ Phase 1: View-model foundation (`src/ui/view.rs`, 21 unit tests)
- ✅ Phase 2: Screen restructure (theme/components/setup/helpers)
- ✅ Phase 3: System rebinding (all v2 elements live-update)
- ✅ Phase 4: Documentation (feature matrices, SOW index)

### Architectural Compliance

- All colors are named theme constants; card rendering stays on the single
  helpers.rs path; emoji use the separate-EmojiFont-entity pattern; root/marker
  contracts for `scale_ui_to_fit_system`/`toggle_game_state_ui_system` intact.
- Build clean with **zero new warnings** (40 vs 43 on main — three
  pre-existing dead helpers were removed). `cargo test`: **128 passed**.
- Adversarial review (5 dimensions → 9 confirmed findings after verification,
  5 refuted): all confirmed defects fixed — dead intent-PLAYED branch, hand
  slot-index click validation (pre-existing, surfaced by the fan rewrite),
  telegraph rounding vs engine truncation, discard top-card badge info,
  disabled-PASS residual glow.

### Player Experience Validation

Verified live via the e2e driver (screenshots in session scratchpad):
deck builder → START RUN → full hand loop (narc intent bubble, card play from
the fan with hover lift, balance bar SAFE/AT RISK flips, discard chronology
incl. an overridden location, buyer bubble hover detail, location background +
vignette after a location play, disabled/enabled action buttons) → resolution
overlay → outcome messaging. **Subjective fun/readability judgment requires
the user's playtest — not claimed here** (DEVELOPER role rule).

### Performance

No new per-frame despawn/respawn paths; rebuild-style systems remain gated on
`Changed<HandState>`; per-frame text/pill/button systems write only on change.

---

## Sign-Off

**Reviewed By:** pending (user playtest)
**Date:** —
**Decision:** pending
**Status:** Branch `sow-022-gameplay-v2` ready for review
