# Core Gameplay Loop Specification

**Last Updated:** 2025-11-27

## Overview

A roguelite deckbuilder where players deal drugs while evading law enforcement. Build a 10-20 card deck, play hands against the Narc, and manage Heat across a permadeath run.

**Core Loop:** Build deck → Play hands → Bank profit → Manage Heat → Repeat until bust or retire

**Session Length:** ~15 minutes per deck (3-5 hands)

---

## Game Structure

### Run (Character Lifecycle)

A run is one character's career from creation to permadeath.

**Persists Across Sessions:**
- Heat level (with real-time decay: -1/hour, max 168 hours)
- Card upgrade progress (play counts per card)
- Account cash (survives permadeath)

**Ends When:**
- Player gets busted (Evidence > Cover with no valid insurance)
- Deck exhausted (fewer than 3 cards remaining)

**Permadeath:** Character deleted on bust. Account cash and card unlocks persist.

---

### Deck (Session)

A session is one deck played to exhaustion or voluntary exit.

**Setup:**
1. Select Buyer persona (determines demand, thresholds, reaction deck)
2. Build deck: Choose 10-20 cards from 20-card pool
3. Heat decay applied from time since last play

**During Session:**
- Play 3-5 hands until deck exhausted
- Cards persist between hands (unplayed cards carry over)
- Can "Go Home" between hands to bank profit early

**Session End:**
- Profit added to account
- Heat persists on character
- Card play counts updated (for upgrades)

---

### Hand (3 Rounds)

Each hand follows a fixed structure with the Narc as opponent.

**Participants:**
- **Player:** Builds deck, plays cards, can fold
- **Narc:** AI opponent playing Evidence/Conviction cards
- **Buyer:** Persona with reaction deck (not a direct opponent)

**Round Structure (×3):**
1. **Narc Phase:** Narc plays one card face-up
2. **Player Phase:** Player plays card, checks, or folds
3. **Buyer Reveal:** Random card from Buyer's 3 visible cards

**Player Actions:**
- **Play Card:** Select card from hand, played face-up
- **Check:** Skip playing a card (conserve hand)
- **Fold:** Exit hand immediately (keep unplayed cards, lose profit)

**Running Totals:** Updated after each card played
- Evidence (Narc accumulates)
- Cover (Player accumulates)
- Heat (both accumulate)
- Profit (based on Product × modifiers)

---

### Resolution

After Round 3, the hand resolves:

**1. Validity Check**
- Must have active Product AND Location
- Invalid deal = no profit, no bust

**2. Buyer Bail Check**
- Buyer has Heat and Evidence thresholds
- If exceeded, Buyer bails = no profit, no bust

**3. Bust Check**
- If Evidence > Cover → Check for insurance
- Active insurance with met requirements → Pay cost, gain Heat penalty, survive
- Active conviction with Heat ≥ threshold → Insurance fails, busted
- No valid insurance → Busted (permadeath)

**4. Safe Outcome**
- Evidence ≤ Cover → Bank profit, apply Heat
- Demand satisfaction affects profit multiplier

---

## Buyer System

Buyers are personas that determine deal context and difficulty.

**3 Personas (2 scenarios each):**
- **Frat Bro:** Party drugs, high tolerance, moderate profit
- **Desperate Housewife:** Prescription/escape drugs, nervous, variable profit
- **Wall Street Wolf:** Premium products, high stakes, high profit

**Buyer Attributes:**
- **Demands:** Preferred Product and Location (bonus multiplier if matched)
- **Thresholds:** Heat and Evidence limits (bail if exceeded)
- **Reaction Deck:** 7 cards (2 Locations, 5 Deal Modifiers), 3 visible

**Buyer Cards:**
- Locations can override player Locations
- Modifiers stack additively with other cards
- One random card revealed per round from visible 3

---

## Card Types

| Type | Effect | Override Rule |
|------|--------|---------------|
| Product | Sets base price, adds Heat | Last played = active |
| Location | Sets base Evidence/Cover | Last played = active |
| Evidence | Adds to Evidence total | Additive (stacks) |
| Cover | Adds to Cover total | Additive (stacks) |
| Deal Modifier | Multiplies price, adjusts stats | Additive (stacks) |
| Insurance | Cover + bust protection | Last played = active |
| Conviction | Heat threshold, blocks insurance | Last played = active |

---

## Heat System

Heat represents law enforcement attention.

**Accumulation:**
- Cards played add Heat (Products especially)
- Heat applied when cards are played (not at resolution)
- Persists on character across sessions

**Decay:**
- -1 Heat per real-world hour
- Calculated at session start
- Capped at 168 hours (1 week)

**Tiers (30 points each):**
| Tier | Range | Narc Strength |
|------|-------|---------------|
| Cold | 0-29 | Base |
| Warm | 30-59 | +10% Evidence |
| Hot | 60-89 | +20% Evidence |
| Blazing | 90-119 | +30% Evidence |
| Scorching | 120-149 | +40% Evidence |
| Inferno | 150+ | +50% Evidence + Foil |

---

## Card Upgrades

Cards improve through play (per-character, lost on permadeath).

**Progression:**
- Track play count per card
- Threshold reached → Choose stat upgrade (+10% per tier)
- 5 tiers available (Base → Tier 5/Foil)

**Upgradeable Stats (by card type):**
- Product: Price, Heat
- Location: Evidence, Cover, Heat
- Cover: Cover, Heat
- Insurance: Cover, Heat Penalty
- Deal Modifier: Price Multiplier, Evidence, Cover, Heat

---

## Special Conditions

**Fold:**
- Available during Player Phase (any round)
- Keeps unplayed cards in hand
- Loses all profit from current hand
- Keeps Heat accumulated so far

**Go Home:**
- Available between hands
- Banks all profit earned
- Ends session, Heat persists
- Returns to deck builder

**Deck Exhaustion:**
- Triggered when < 3 cards remain
- Cannot start new hand
- Must Go Home (session ends)

---

## Win/Loss Conditions

**Win (per hand):** Evidence ≤ Cover at resolution
**Win (per session):** Go Home with profit banked
**Win (per run):** Survive as long as possible, maximize lifetime revenue

**Loss (per hand):** Busted (Evidence > Cover, no valid insurance)
**Loss (per run):** Permadeath on bust

---

## Integration Points

**Requires:**
- Card System (7 card types)
- Heat System (accumulation, decay, tiers)
- Save System (character persistence, account state)

**Feeds Into:**
- Progression System (cash → unlocks)
- Card Upgrades (play counts → stat bonuses)
