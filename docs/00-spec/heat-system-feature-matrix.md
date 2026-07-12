# Heat System - Feature Matrix

**Spec:** [heat-system.md](heat-system.md)
**Last Updated:** 2026-07-12
**Overall Status:** 14/14 features complete (100%)

---

## Summary

| Category | Complete | Total | % |
|----------|:--------:|:-----:|:-:|
| Heat Accumulation | 4 | 4 | 100% |
| Heat Decay | 2 | 2 | 100% |
| Heat Tiers | 4 | 4 | 100% |
| Narc Scaling | 4 | 4 | 100% |
| **Total** | **14** | **14** | **100%** |

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

## Narc Scaling - 4/4 (100%)

| Feature | Status | Notes |
|---------|:------:|-------|
| Heat→Narc tier mapping | ✅ | HeatTier::narc_upgrade_tier() |
| Evidence multiplier | ✅ | 1.0×→1.1×→1.2×→1.3×→1.4×→1.5× |
| Tier locked at deck start | ✅ | Uses character heat at session start |
| Danger indicator | ✅ | ⚖ badge with tier name (Relaxed/Alert/etc.) |

---

## Scrapped Features

| Feature | Reason |
|---------|--------|
| Decay projection | Unnecessary ("In 24 hours: X") |
| Decay feedback UI | Not implemented, not needed |
| Tier transition warnings | Unnecessary polish |

---

## Implementation Notes

- HeatTier enum: `src/save/types.rs:104`
- apply_decay(): `src/save/types.rs:446`
- Narc scaling: `src/save/types.rs:150` (narc_upgrade_tier)
- Heat bar UI (SOW-022): YOUR STANDING panel gradient track on a fixed 0–100
  scale with conviction-threshold tick marks (`src/ui/theme.rs` STANDING_HEAT_*,
  `src/ui/systems.rs` update_standing_panel_system, ticks from
  `src/ui/view.rs` conviction_ticks). Session heat tier chip uses
  `HeatTier::from_heat(current_heat)` name + color. The buyer's scenario heat
  cap displays separately on the BAILS AT HEAT chip.
- Tier display (deck builder): `src/systems/save_integration.rs:214` (tier.color())
