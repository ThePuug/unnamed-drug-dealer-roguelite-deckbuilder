# Card System - Feature Matrix

Implementation tracking for Card System specification.

**Spec:** [card-system.md](card-system.md)

**Last Updated:** 2025-11-09

---

## Summary

**Overall Completion:** 0/48 features (0%)

| Category | Complete | Partial | Not Started | Deferred |
|----------|----------|---------|-------------|----------|
| Product Cards | 0 | 0 | 5 | 0 |
| Location Cards | 0 | 0 | 5 | 0 |
| Deal Modifier Cards | 0 | 0 | 6 | 0 |
| Evidence Cards | 0 | 0 | 4 | 0 |
| Cover Cards | 0 | 0 | 4 | 0 |
| Get Out of Jail Cards | 0 | 0 | 6 | 0 |
| Make It Stick Cards | 0 | 0 | 4 | 0 |
| Card Interactions | 0 | 0 | 8 | 0 |
| Edge Cases | 0 | 0 | 6 | 0 |
| **Total** | **0** | **0** | **48** | **0** |

---

## Product Cards: 0/5 complete (0%)

| Feature | Status | RFC/ADR | Notes |
|---------|:------:|---------|-------|
| Product override system | ❌ | - | Last Product played = active |
| Base price calculation | ❌ | - | Product defines base, modified by multipliers |
| Heat modifier application | ❌ | - | Products add Heat to hand total |
| Weed (MVP card) | ❌ | - | $30, +5 Heat |
| Pills/Meth/Heroin (MVP cards) | ❌ | - | Pills: $60/+15, Meth: $100/+30, Heroin: $150/+45 |

---

## Location Cards: 0/5 complete (0%)

| Feature | Status | RFC/ADR | Notes |
|---------|:------:|---------|-------|
| Location override system | ❌ | - | Last Location played = active |
| Base Evidence value | ❌ | - | Location defines base Evidence |
| Base Cover value | ❌ | - | Location defines base Cover |
| Heat modifier application | ❌ | - | Locations add/subtract Heat |
| Safe House/Parking Lot/School Zone (MVP) | ❌ | - | 3 core Location cards |

---

## Deal Modifier Cards: 0/6 complete (0%)

| Feature | Status | RFC/ADR | Notes |
|---------|:------:|---------|-------|
| Additive modifiers (+/-) | ❌ | - | Stack additively (e.g., +20 Cover) |
| Multiplicative modifiers (×) | ❌ | - | Apply to base Price (e.g., ×1.5) |
| Player modifiers | ❌ | - | Disguise, Lookout, Bulk Sale Pressure |
| Narc modifiers | ❌ | - | Heat Wave, Undercover Op |
| Customer modifiers | ❌ | - | Bulk Order, Haggling, Premium Buyer |
| Modifier stacking calculation | ❌ | - | Correct order: base × multipliers, then +/- |

---

## Evidence Cards: 0/4 complete (0%)

| Feature | Status | RFC/ADR | Notes |
|---------|:------:|---------|-------|
| Pure Evidence boost | ❌ | - | Additive only (e.g., +20 Evidence) |
| Stack with Location base | ❌ | - | Location base + Evidence cards |
| Narc AI plays Evidence | ❌ | - | AI deck generation |
| Patrol/Surveillance/Wiretap (MVP) | ❌ | - | 3 Evidence cards for MVP |

---

## Cover Cards: 0/4 complete (0%)

| Feature | Status | RFC/ADR | Notes |
|---------|:------:|---------|-------|
| Pure Cover boost | ❌ | - | Additive only (e.g., +30 Cover) |
| Stack with Location base | ❌ | - | Location base + Cover cards |
| Heat modifier on Cover cards | ❌ | - | Some Cover cards also affect Heat |
| Alibi/Lawyer Up/Lay Low (MVP) | ❌ | - | 3 Cover cards for MVP |

---

## Get Out of Jail Cards: 0/6 complete (0%)

| Feature | Status | RFC/ADR | Notes |
|---------|:------:|---------|-------|
| Insurance override system | ❌ | - | Last Get Out of Jail played = active |
| Dual function (Cover + Insurance) | ❌ | - | Acts as Cover while active, Insurance on bust |
| Single use per deck | ❌ | - | Burned after insurance triggered |
| Requirements check (cost payment) | ❌ | - | Verify can afford before activation |
| Heat penalty application | ❌ | - | Gain overage + card penalty Heat |
| Plea Bargain/Fake ID (MVP) | ❌ | - | 2 Insurance cards for MVP |

---

## Make It Stick Cards: 0/4 complete (0%)

| Feature | Status | RFC/ADR | Notes |
|---------|:------:|---------|-------|
| Conviction override system | ❌ | - | Last Make It Stick played = active |
| Heat threshold check | ❌ | - | Only applies if current_heat >= threshold |
| Override Get Out of Jail | ❌ | - | Insurance fails if threshold met |
| Warrant/DA Approval (MVP) | ❌ | - | 2 Conviction cards for MVP |

---

## Card Interactions: 0/8 complete (0%)

| Feature | Status | RFC/ADR | Notes |
|---------|:------:|---------|-------|
| Override rule (Products) | ❌ | - | New Product discards old Product |
| Override rule (Locations) | ❌ | - | New Location replaces old base Evidence/Cover |
| Override rule (Insurance) | ❌ | - | New Get Out of Jail discards old |
| Override rule (Conviction) | ❌ | - | New Make It Stick replaces old threshold |
| Additive stacking display | ❌ | - | Show breakdown: base + card1 + card2 = total |
| Multiplicative stacking display | ❌ | - | Show breakdown: base × mult1 × mult2 = total |
| Heat accumulation | ❌ | - | Sum all Heat modifiers from cards played |
| Card replacement feedback | ❌ | - | "Weed → Meth (previous discarded)" |

---

## Edge Cases: 0/6 complete (0%)

| Feature | Status | RFC/ADR | Notes |
|---------|:------:|---------|-------|
| Multiple Products same round | ❌ | - | Override applies, only last counts |
| Location override after Evidence added | ❌ | - | Location base changes, Evidence cards remain |
| Insurance played but not needed | ❌ | - | Acts as Cover only, not consumed |
| Multiple Get Out of Jail cards | ❌ | - | Override applies, only one active |
| Make It Stick below threshold | ❌ | - | Conviction inactive, insurance works |
| Make It Stick AND Insurance | ❌ | - | Conviction overrides if threshold met |

---

## Implementation Deviations

_No implementations yet._

---

## Notes

- MVP scope: 20 cards total (5 Products, 3 Locations, 8 support, 4 insurance)
- Phase 2: Expand to 80-100 cards
- Card interaction rules critical for correct gameplay
- Override system is unique mechanic (not standard card game rules)
