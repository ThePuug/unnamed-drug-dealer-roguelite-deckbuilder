# Heat System - Feature Matrix

**Spec:** [heat-system.md](heat-system.md)
**Last Updated:** 2026-07-12 (SOW-027)
**Overall Status:** 16/16 features complete (100%)

---

## Summary

| Category | Complete | Total | % |
|----------|:--------:|:-----:|:-:|
| Heat Accumulation | 4 | 4 | 100% |
| Heat Decay | 2 | 2 | 100% |
| Heat Tiers | 4 | 4 | 100% |
| Narc Difficulty (SOW-027) | 3 | 3 | 100% |
| Active Cooling (SOW-027) | 3 | 3 | 100% |
| **Total** | **16** | **16** | **100%** |

---

## Heat Accumulation - 4/4 (100%)

| Feature | Status | Notes |
|---------|:------:|-------|
| Heat delta calculation | ✅ | Sum all Heat modifiers in calculate_totals() |
| Immediate application | ✅ | Heat added when cards played, not at resolution |
| Heat on fold | ✅ | Keeps heat from played cards |
| Heat persistence | ✅ | CharacterState.heat in SaveData |

---

## Heat Decay - 2/2 (100%)

| Feature | Status | Notes |
|---------|:------:|-------|
| Real-time decay | ✅ | -1 Heat/hour, apply_decay() |
| Decay cap | ✅ | Max 168 hours (7 days) |

---

## Heat Tiers - 4/4 (100%)

| Feature | Status | Notes |
|---------|:------:|-------|
| Six tiers | ✅ | Cold/Warm/Hot/Blazing/Scorching/Inferno |
| Tier boundaries | ✅ | 30 points each (0-29, 30-59, etc.) |
| Tier colors | ✅ | Green→Yellow→Orange→DeepOrange→Red→Purple |
| Tier display | ✅ | "[Tier]" text with tier color in deck builder |

---

## Narc Difficulty - 3/3 (100%) — SOW-027 (composition-based)

| Feature | Status | Notes |
|---------|:------:|-------|
| Per-area × per-tier compositions | ✅ | narc_deck.ron sparse format (default ladder + area overrides), inheritance resolved and validated at load |
| Tier selects composition at run start | ✅ | create_narc_deck(assets, station, heat_tier) |
| Authored numbers everywhere | ✅ | RFC-018 multipliers/⚖ badge RETIRED (SOW-027): card face == telegraph == totals == resolution |

---

## Active Cooling - 3/3 (100%) — SOW-027

| Feature | Status | Notes |
|---------|:------:|-------|
| Lay Low | ✅ | $200, 2 runs committed bench, −40 heat on resurfacing (DealerStatus::LayingLow) |
| Crooked Lawyer | ✅ | $625, −25 heat immediately, no downtime |
| RFC-019 Heat upgrade | ✅ | Wired in get_card_heat: cools positive-heat player cards only, never worsens negative-heat cards |

---

## Retired Features

| Feature | Reason |
|---------|--------|
| Heat→Narc tier mapping (narc_upgrade_tier) | SOW-027: difficulty is the deck composition, not scaled numbers |
| Evidence multiplier (1.0×→1.5×) | SOW-027: retired with RFC-018 scaling |
| ⚖ danger badge / scaled displays | SOW-027: no scaled number exists to explain |

## Scrapped Features

| Feature | Reason |
|---------|--------|
| Decay projection | Unnecessary ("In 24 hours: X") |
| Decay feedback UI | Not implemented, not needed |
| Tier transition warnings | Unnecessary polish |

---

## Implementation Notes

- HeatTier enum: `src/save/types.rs` (from_heat; tier still selects the composition)
- apply_decay(): `src/save/types.rs`
- Narc compositions: `assets/narc_deck.ron` (sparse/inheritable), loaded and
  validated in `src/assets/loader.rs` (effective tables logged in debug),
  consumed by `src/data/narc_deck.rs::create_narc_deck`
- Coolers: `src/save/types.rs` (LAY_LOW_*/LAWYER_* constants beside MOVE_FEE,
  SaveData::{lay_low, hire_lawyer}, DealerStatus::LayingLow ticking through
  complete_run_tick); roster UI in `src/systems/ui_update.rs`, clicks in
  `src/systems/input.rs::roster_button_system`
- Heat bar UI (SOW-022): YOUR STANDING panel gradient track on a fixed 0–100
  scale with conviction-threshold tick marks (`src/ui/theme.rs` STANDING_HEAT_*,
  `src/ui/systems.rs` update_standing_panel_system, ticks from
  `src/ui/view.rs` conviction_ticks). Session heat tier chip uses
  `HeatTier::from_heat(current_heat)` name + color. The buyer's scenario heat
  cap displays separately on the BAILS AT HEAT chip.
- Tier display (deck builder): `src/systems/save_integration.rs` (tier.color())
