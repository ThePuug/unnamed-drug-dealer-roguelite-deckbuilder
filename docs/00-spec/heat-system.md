# Heat System Specification

**Last Updated:** 2025-11-27

## Overview

Heat is a persistent character stat representing law enforcement attention. Higher Heat means stronger Narc cards next deck. Heat accumulates from card play and decays over real-world time.

---

## Heat Accumulation

**Rule:** Sum all Heat modifiers from cards played during hand.

- Applied immediately when cards are played (not at resolution)
- On fold: Heat from played cards is kept
- On bust: Heat applied before permadeath
- Persists on character across sessions

---

## Heat Decay

**Rule:** -1 Heat per real-world hour (capped at 168 hours / 7 days).

- Calculated and applied when entering deck builder
- `apply_decay()` method on CharacterState
- Decay amount shown in UI ("Heat decayed by X while away")

---

## Heat Tiers

Six tiers determine Narc card scaling and UI colors.

| Tier | Heat Range | Narc Multiplier | Tier Color |
|------|------------|-----------------|------------|
| Cold | 0-29 | 1.0× (base) | Green |
| Warm | 30-59 | 1.1× (+10%) | Yellow |
| Hot | 60-89 | 1.2× (+20%) | Orange |
| Blazing | 90-119 | 1.3× (+30%) | Deep Orange |
| Scorching | 120-149 | 1.4× (+40%) | Red |
| Inferno | 150+ | 1.5× (+50%) | Purple |

**Tier Display:** Character tier shown in deck builder with tier-colored text (e.g., "[Hot]" in orange).

**Danger Names:** Cold=Relaxed, Warm=Alert, Hot=Dangerous, Blazing=Severe, Scorching=Intense, Inferno=Deadly

---

## Narc Scaling (RFC-018)

Heat determines Narc card upgrade tier for the NEXT deck (not current).

- Heat tier → UpgradeTier mapping
- Narc Evidence cards get stat multiplier based on tier
- Narc cards display ⚖ (scales) badge with tier color
- Tier locked at deck start from character's current heat

---

## Heat Bar UI

In-game heat bar uses percentage-based coloring:
- 0-50%: Green
- 50-80%: Yellow
- 80-100%: Red

This is separate from the 6-tier system colors (which appear on tier text in deck builder).

---

## Integration

**Input:**
- Card heat modifiers (from CardType)
- Real-world timestamps (for decay calculation)

**Output:**
- Character.heat (persisted)
- HeatTier (derived from heat value)
- Narc upgrade tier (for card scaling)

**Stored In:**
- CharacterState.heat
- CharacterState.last_played (for decay)
