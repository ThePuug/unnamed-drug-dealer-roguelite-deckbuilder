# Card System - Feature Matrix

**Spec:** [card-system.md](card-system.md)
**Last Updated:** 2025-11-27
**Overall Status:** 24/24 features complete (100%)

---

## Summary

| Category | Complete | Total | % |
|----------|:--------:|:-----:|:-:|
| Card Types | 7 | 7 | 100% |
| Override System | 4 | 4 | 100% |
| Stacking Rules | 3 | 3 | 100% |
| Totals Calculation | 4 | 4 | 100% |
| Card Data | 3 | 3 | 100% |
| Card Upgrades | 3 | 3 | 100% |
| **Total** | **24** | **24** | **100%** |

---

## Card Types - 7/7 (100%)

| Feature | Status | Notes |
|---------|:------:|-------|
| Product | ✅ | price, heat |
| Location | ✅ | evidence, cover, heat |
| Evidence | ✅ | evidence, heat |
| Cover | ✅ | cover, heat |
| DealModifier | ✅ | price_multiplier, evidence, cover, heat |
| Insurance | ✅ | cover, cost, heat_penalty |
| Conviction | ✅ | heat_threshold |

---

## Override System - 4/4 (100%)

| Feature | Status | Notes |
|---------|:------:|-------|
| Product override | ✅ | `active_product()` - last played |
| Location override | ✅ | `active_location()` - last played |
| Insurance override | ✅ | `active_insurance()` - last played |
| Conviction override | ✅ | `active_conviction()` - last played |

---

## Stacking Rules - 3/3 (100%)

| Feature | Status | Notes |
|---------|:------:|-------|
| Additive Evidence | ✅ | Location base + Evidence cards |
| Additive Cover | ✅ | Location base + Cover cards + Insurance |
| Multiplicative Price | ✅ | Product base × DealModifier multipliers |

---

## Totals Calculation - 4/4 (100%)

| Feature | Status | Notes |
|---------|:------:|-------|
| Evidence total | ✅ | `calculate_totals()` |
| Cover total | ✅ | `calculate_totals()` |
| Profit calculation | ✅ | Base × multipliers |
| Heat accumulation | ✅ | Sum all Heat, applied on play |

---

## Card Data - 3/3 (100%)

| Feature | Status | Notes |
|---------|:------:|-------|
| RON file loading | ✅ | `assets/loader.rs` |
| CardRegistry resource | ✅ | Stores all loaded cards |
| Validation on load | ✅ | Type checks, required fields |

---

## Card Upgrades - 3/3 (100%)

| Feature | Status | Notes |
|---------|:------:|-------|
| Play count tracking | ✅ | Per-card, per-character |
| Tier calculation | ✅ | 6 tiers (Base → Foil) |
| Stat multipliers | ✅ | +10% per tier to beneficial stats |

---

## Implementation Notes

- CardType enum: `src/models/card.rs`
- Active card queries: `src/models/hand_state/card_engine.rs`
- Totals calculation: `src/models/hand_state/card_engine.rs`
- Card loading: `src/assets/loader.rs`
- Upgrade system: `src/save/types.rs` (UpgradeTier, UpgradeableStat)

---

## Out of Scope

This matrix tracks **system mechanics only**, not card content.

Card content (specific products, locations, etc.) is defined in:
- `assets/data/*.ron` files
- Not tracked here as "features"
