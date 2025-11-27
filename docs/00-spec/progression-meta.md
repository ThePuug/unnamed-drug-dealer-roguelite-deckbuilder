# Progression & Meta-Game Specification

**Last Updated:** 2025-11-27

## Overview

Progression systems create long-term goals beyond individual runs. Cash is the universal currency for unlocking cards. Locations gate card pools and unlock via achievements. Card upgrades provide per-run power growth. Character permadeath creates meaningful stakes.

---

## Cash System

### Account-Wide Currency

- **Cash on hand:** Spendable currency, earned from deals
- **Lifetime revenue:** Total ever earned (never decreases)
- **Account-wide:** Shared across all characters, survives permadeath
- **Spending:** Used to purchase card unlocks at locations

```
Play deals, earn $10k    → Cash: $10k | Revenue: $10k
Buy cards for $3k        → Cash: $7k  | Revenue: $10k
Character dies           → Cash: $7k  | Revenue: $10k (preserved)
```

---

## Location System

Locations are card shops that gate card pools and unlock via achievements.

### Structure

| Location | Unlock Achievement | Card Pool Theme |
|----------|-------------------|-----------------|
| The Corner | Default | Basic products, simple cover |
| The Block | Survive 5 decks | Better products, more variety |
| Downtown | Earn $10k lifetime | Premium products, strong cover |
| The Docks | Survive 15 decks | Import products, specialized cards |
| The Tower | Earn $50k lifetime | Elite products, powerful modifiers |

### Rules

- Locations unlock permanently via achievements
- Can shop at any unlocked location
- Each location has unique card pool
- Cards purchased once, owned forever (account-wide)

---

## Card Unlock System

### Starting Collection

~15-20 cards unlocked by default:
- Basic Products (3-4)
- Basic Locations (2-3)
- Basic Cover (3-4)
- Basic Modifiers (2-3)

### Purchasing Cards

- Location-specific: Each card available at one location
- Cash purchase: Spend cash on hand to unlock
- Permanent: Never lost, account-wide
- Pricing tiers: $500-$1.5k (basic) to $20k+ (elite)

---

## Per-Run Card Upgrades

Cards improve through play count (per-character, lost on permadeath).

### Implementation (RFC-017/019)

**Tiers:** Base → Tier 1 → Tier 2 → Tier 3 → Tier 4 → Tier 5 (Foil)

**Play Thresholds:** 0, 3, 8, 15, 25, 40

**Stat Bonuses:** +10% to beneficial stat per tier

**Upgrade Choice:** When threshold reached, player chooses which stat to upgrade via dedicated UI (GameState::UpgradeChoice)

**Upgradeable Stats by Type:**
| Type | Options |
|------|---------|
| Product | Price (+), Heat (-) |
| Location | Evidence (-), Cover (+), Heat (-) |
| Cover | Cover (+), Heat (-) |
| Insurance | Cover (+), Heat Penalty (-) |
| DealModifier | Price Mult (+), Evidence (-), Cover (+), Heat (-) |

### Display

- Cards show ★ badge with tier color
- Grey → Bronze → Silver → Gold progression
- Foil shader effect at max tier

---

## Character System

### Basic Character (Implemented)

- Single character with Heat and card upgrades
- Permadeath on bust (character = None)
- Heat, upgrades lost; cash, unlocks preserved

### Character Slots (Not Implemented)

- Start with 1 slot, unlock more via achievements
- Each slot = independent character with own Heat/upgrades
- Shared cash pool across all characters

### Character Profiles (Not Implemented)

Narrative framing only (no mechanical differences):
- College Student
- Widow
- Cancer Patient
- Mafia Member

### Permadeath Consequences

**Lost:**
- Character itself
- Heat (new character starts at 0)
- All card upgrades (reset to base)

**Preserved:**
- Cash on hand
- Unlocked cards
- Unlocked locations
- Character slots

---

## Achievements

Achievements unlock locations and character slots.

### Location Unlocks

| Achievement | Requirement | Unlocks |
|-------------|-------------|---------|
| Street Cred | Survive 5 decks | The Block |
| Money Talks | $10k lifetime revenue | Downtown |
| Veteran | Survive 15 decks | The Docks |
| Kingpin | $50k lifetime revenue | The Tower |

### Character Slot Unlocks

| Achievement | Requirement | Unlocks |
|-------------|-------------|---------|
| Back in Business | Complete a run | Slot 2 |
| Parallel Operations | 10 decks total | Slot 3 |
| Empire Builder | Unlock 3 locations | Slot 4 |

---

## Narc Variety (Deferred)

Single Narc profile for MVP. Heat controls difficulty via card upgrades (RFC-018). Narc variety from locations deferred to post-launch.

---

## Meta-Game Loop

```
Per-Deck:  Build Deck → Play Hands → Earn Cash / Gain Heat → Cards Upgrade → Repeat

Per-Run:   Create Character → Play Decks → Cards Upgrade → Heat Accumulates → Bust

Meta:      Play Runs → Earn Cash → Buy Cards → Chase Achievements → Unlock Locations
```

---

## Implementation Status

| System | Status |
|--------|--------|
| Cash (earn, persist) | ✅ Implemented |
| Cash (spending) | ❌ Not Started |
| Location shops | ❌ Not Started |
| Card unlocks | ❌ Not Started |
| Per-run upgrades | ✅ Implemented (RFC-017/019) |
| Basic character | ✅ Implemented |
| Character slots | ❌ Not Started |
| Character profiles | ❌ Not Started |
| Achievements | ❌ Not Started |
