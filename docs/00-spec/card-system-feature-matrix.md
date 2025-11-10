# Card System - Feature Matrix

Implementation tracking for Card System specification.

**Spec:** [card-system.md](card-system.md)

**Last Updated:** 2025-11-10 (Updated to reflect RFC-005, RFC-008 implementations)

---

## Major Changes from Spec to Implementation

**RFC-005 (Deck Balance and Card Distribution):**
- ✅ Player deck: 20 cards total, NO Evidence cards, NO Conviction cards
- ✅ Narc deck: 25 cards (17 Evidence, 8 Conviction)
- ✅ Customer deck: 25 cards (5 Products, 5 Locations, 15 Deal Modifiers)
- ✅ Conviction cards moved from player deck to Narc deck

**RFC-008 (Sequential Play - NEW DEALER CARDS):**
- ✅ **Dealer deck: 20 cards** (8 Locations, 8 Modifiers, 4 Wild)
- ✅ Dealer cards revealed progressively (one per round)
- ✅ Dealer Locations can be overridden by player Locations
- ✅ Dealer Modifiers cannot be overridden

---

## Legend

- ✅ **Complete** - Fully implemented per spec
- 🔄 **In Progress** - Currently being developed (SOW active)
- 🎯 **Planned** - RFC approved, SOW created, ready for implementation
- ❌ **Not Started** - Planned but not implemented
- ⏸️ **Deferred** - Intentionally postponed to post-MVP

---

## Summary

**Overall Completion:** Updated to reflect recent RFC implementations

| Category | Complete | Not Started | Deferred |
|----------|----------|-------------|----------|
| Product Cards | 5 | 0 | 0 |
| Location Cards | 6 | 0 | 0 |
| Deal Modifier Cards | 3 | 3 | 0 |
| Evidence Cards | 5 | 0 | 0 |
| Cover Cards | 5 | 0 | 0 |
| Get Out of Jail Cards | 2 | 4 | 0 |
| Make It Stick Cards | 3 | 1 | 0 |
| **Dealer Cards (NEW)** | **3** | **0** | **0** |
| Card Interactions | 6 | 2 | 0 |
| Edge Cases | 4 | 2 | 0 |
| **Total** | **42** | **12** | **0** |

---

## Product Cards: 5/5 complete (100%)

| Feature | Status | RFC/ADR | Notes |
|---------|:------:|---------|-------|
| Product override system | ✅ | SOW-001 | Last Product played = active |
| Base price calculation | ✅ | SOW-001 | Product defines base, modified by multipliers |
| Heat modifier application | ✅ | SOW-001 | Products add Heat to hand total |
| Weed/Meth/Heroin (MVP cards) | ✅ | SOW-001 | Weed: $30/+5, Meth: $100/+30, Heroin: $150/+45 |
| Pills + 1 variant (MVP cards) | ✅ | SOW-002 | Pills: $60/+15 + additional Products |

---

## Location Cards: 6/6 complete (100%)

| Feature | Status | RFC/ADR | Notes |
|---------|:------:|---------|-------|
| Location override system | ✅ | SOW-001 | Last Location played = active |
| Base Evidence value | ✅ | SOW-001 | Location defines base Evidence |
| Base Cover value | ✅ | SOW-001 | Location defines base Cover |
| Heat modifier application | ✅ | SOW-001 | Locations add/subtract Heat |
| Safe House/School Zone (MVP) | ✅ | SOW-001 | Safe House: 10 Ev, 30 Cov, -5 Heat | School Zone: 40 Ev, 5 Cov, +20 Heat |
| Parking Lot (MVP) | ✅ | SOW-002 | Parking Lot: 25 Ev, 15 Cov, 0 Heat |

---

## Deal Modifier Cards: 3/6 complete (50%)

| Feature | Status | RFC/ADR | Notes |
|---------|:------:|---------|-------|
| Additive modifiers (+/-) | ❌ | RFC-003 | Stack additively (e.g., +20 Cover) |
| Multiplicative modifiers (×) | ✅ | SOW-002 | Apply to base Price (e.g., ×1.5) |
| Player modifiers | ✅ | SOW-002 | Disguise, Lookout implemented |
| Narc modifiers | ❌ | RFC-003 | Heat Wave, Undercover Op |
| Customer modifiers | ✅ | SOW-002 | Bulk Order, Haggling, Premium Buyer |
| Modifier stacking calculation | ❌ | RFC-003 | Correct order: base × multipliers, then +/- |

---

## Evidence Cards: 4/5 complete (80%)

| Feature | Status | RFC/ADR | Notes |
|---------|:------:|---------|-------|
| Pure Evidence boost | ✅ | SOW-001 | Additive only (e.g., +20 Evidence) |
| Stack with Location base | ✅ | SOW-001 | Location base + Evidence cards |
| Patrol/Surveillance (MVP) | ✅ | SOW-001 | Patrol: +5 Ev, +2 Heat | Surveillance: +20 Ev, +5 Heat |
| Narc AI plays Evidence | ✅ | SOW-002 | AI deck generation with static decks |
| Wiretap + 1 more (MVP) | ❌ | RFC-003 | Additional Evidence cards |

---

## Cover Cards: 5/5 complete (100%)

| Feature | Status | RFC/ADR | Notes |
|---------|:------:|---------|-------|
| Pure Cover boost | ✅ | SOW-001 | Additive only (e.g., +30 Cover) |
| Stack with Location base | ✅ | SOW-001 | Location base + Cover cards |
| Heat modifier on Cover cards | ✅ | SOW-001 | Some Cover cards also affect Heat |
| Alibi (MVP) | ✅ | SOW-001 | Alibi: +30 Cover, -5 Heat |
| Lawyer Up/Lay Low (MVP) | ✅ | SOW-002 | Additional Cover cards implemented |

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

## Make It Stick Cards: 3/4 complete (75%) - **Updated per RFC-005**

| Feature | Status | RFC/ADR | Notes |
|---------|:------:|---------|-------|
| Conviction override system | ✅ | RFC-005, SOW-005 | Last Make It Stick played = active |
| Heat threshold check | ✅ | RFC-005, SOW-005 | Only applies if current_heat >= threshold |
| Override Get Out of Jail | ✅ | RFC-005, SOW-005 | Insurance fails if threshold met |
| Warrant/DA Approval/RICO Case | ✅ | RFC-005, SOW-005 | **Moved to Narc deck (was in player deck)** - 8 Conviction cards total |

**Note:** Per RFC-005, Conviction cards moved from player deck to Narc deck for thematic consistency

---

## Dealer Cards (NEW in RFC-008): 3/3 complete (100%)

| Feature | Status | RFC/ADR | Notes |
|---------|:------:|---------|-------|
| Dealer deck system (20 cards) | ✅ | RFC-008, SOW-008 | Separate deck, 3 cards drawn per hand |
| Dealer Location cards (8) | ✅ | RFC-008, SOW-008 | Base Evidence/Cover, can be overridden |
| Dealer Modifier cards (8) | ✅ | RFC-008, SOW-008 | Adjust totals additively, cannot be overridden |
| Dealer Wild cards (4) | ✅ | RFC-008, SOW-008 | High-impact swings (Lucky Break, Bad Intel) |
| Progressive reveal (one per round) | ✅ | RFC-008, SOW-008 | Dealer reveals after Player Phase |
| Integration with override system | ✅ | RFC-008, SOW-008 | Dealer Locations subject to override, Modifiers are not |

**Note:** Dealer cards are a completely new mechanic introduced in RFC-008 to create "river tension" and progressive information revelation

---

## Card Interactions: 5/8 complete (63%)

| Feature | Status | RFC/ADR | Notes |
|---------|:------:|---------|-------|
| Override rule (Products) | ✅ | SOW-001 | New Product discards old Product |
| Override rule (Locations) | ✅ | SOW-001 | New Location replaces old base Evidence/Cover |
| Additive stacking (Evidence/Cover) | ✅ | SOW-001 | Evidence/Cover cards stack on Location base |
| Heat accumulation | ✅ | SOW-001 | Sum all Heat modifiers from cards played |
| Multiplicative stacking (Price) | ✅ | SOW-002 | Apply multipliers to Product price |
| Override rule (Insurance) | ❌ | RFC-003, ADR-003 | New Get Out of Jail discards old |
| Override rule (Conviction) | ❌ | RFC-003, ADR-003 | New Make It Stick replaces old threshold |
| Card replacement feedback UI | ❌ | Phase 2 polish | "Weed → Meth (previous discarded)" |

---

## Edge Cases: 1/6 complete (17%)

| Feature | Status | RFC/ADR | Notes |
|---------|:------:|---------|-------|
| Multiple Products same round | ✅ | SOW-001 | Override applies, only last counts |
| Location override after Evidence added | ❌ | RFC-003 | Location base changes, Evidence cards remain |
| Insurance played but not needed | ❌ | RFC-003, ADR-003 | Acts as Cover only, not consumed |
| Multiple Get Out of Jail cards | ❌ | RFC-003, ADR-003 | Override applies, only one active |
| Make It Stick below threshold | ❌ | RFC-003, ADR-003 | Conviction inactive, insurance works |
| Make It Stick AND Insurance | ❌ | RFC-003, ADR-003 | Conviction overrides if threshold met |

---

## Implementation Deviations

_No deviations yet - SOW-001 in progress._

---

## Implementation Status by RFC/SOW

### SOW-001: Minimal Playable Hand (~4h actual) - ✅ Complete

**Status:** Approved - Ready to Merge

**Features Delivered:**
- ✅ Product override system (3 Products: Weed, Meth, Heroin)
- ✅ Location override system (2 Locations: Safe House, School Zone)
- ✅ Evidence additive stacking (2 Evidence: Patrol, Surveillance)
- ✅ Cover additive stacking (1 Cover: Alibi)
- ✅ Heat accumulation (sum all Heat modifiers)
- ✅ Base price/Evidence/Cover calculation
- ✅ Multiple Products edge case (override applies)
- ✅ 8 cards total implemented

**Completion:** 19 features complete (Product Cards 3/5, Location Cards 4/6, Evidence Cards 3/5, Cover Cards 1/5, Card Interactions 4/8, Edge Cases 1/6)

### SOW-002: Betting System and AI Opponents (~4h actual) - ✅ Complete

**Status:** Review - Implementation Complete, Awaiting Playtest

**Features Delivered:**
- ✅ Expanded to 15 cards total (from 8)
- ✅ Additional Products (Pills + variants)
- ✅ Additional Location (Parking Lot)
- ✅ Additional Cover cards (Lawyer Up, Lay Low)
- ✅ Deal Modifiers (multiplicative price modifiers)
- ✅ Player modifiers (Disguise, Lookout)
- ✅ Customer modifiers (Bulk Order, Haggling, Premium Buyer)
- ✅ Multiplicative stacking for Price calculation

**Completion:** +10 features (Product Cards 5/5, Location Cards 6/6, Cover Cards 5/5, Deal Modifiers 3/6, Evidence 4/5, Card Interactions 5/8)

**Total after SOW-002:** 29/51 features (57%)

---

## Notes

- **SOW-001 scope:** 8 cards only (3 Products, 2 Locations, 2 Evidence, 1 Cover)
- **RFC-002 scope:** Expand to 15 cards (add Deal Modifiers, more Products/Locations)
- **RFC-003 scope:** Complete 20 cards (add Insurance + Conviction cards)
- **Full spec:** 80-100 cards in Phase 3 (far future)
- Phase 2: Expand to 80-100 cards
- Card interaction rules critical for correct gameplay
- Override system is unique mechanic (not standard card game rules)
