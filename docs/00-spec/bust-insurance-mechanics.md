# Bust & Insurance Mechanics Specification

**Last Updated:** 2025-11-27

## Overview

Bust mechanics determine when a run ends (permadeath). The core rule: **Evidence > Cover = Busted**. Insurance cards can prevent bust, but Conviction cards can override insurance at high Heat.

---

## Resolution Flow

```
End of Hand:
1. Validity Check
   - Must have Product AND Location
   - Invalid → No profit, no bust

2. Buyer Bail Check
   - Heat or Evidence exceeds Buyer threshold
   - Bail → No profit, no bust

3. Evidence vs Cover
   - Evidence ≤ Cover → Safe (bank profit)
   - Evidence > Cover → Continue to step 4

4. Conviction Check
   - If Conviction active AND current_heat ≥ threshold
   - → Busted (insurance overridden)

5. Insurance Check
   - If Insurance active AND can afford cost
   - → Pay cost, gain heat_penalty, burn card, Safe
   - Else → Busted

6. Permadeath
   - Character deleted from save
   - Account cash persists
```

---

## Core Bust Rule

| Condition | Result |
|-----------|--------|
| Evidence ≤ Cover | Safe (tie goes to player) |
| Evidence > Cover | Bust check (insurance/conviction) |
| Busted | Character deleted (permadeath) |

---

## Insurance Cards

Insurance cards (Get Out of Jail) provide Cover and bust protection.

**Card Properties:**
- `cover`: Adds to Cover total during hand
- `cost`: Cash required to activate on bust
- `heat_penalty`: Heat gained when activated

**Activation (when Evidence > Cover):**
1. Check affordability: `cash >= cost`
2. If affordable:
   - Deduct cost from cash
   - Add heat_penalty to current_heat
   - Remove card from deck (burned)
   - Outcome: Safe
3. If not affordable:
   - Outcome: Busted

**Override Rule:** Only last Insurance played is active.

---

## Conviction Cards

Conviction cards (Make It Stick) can override insurance at high Heat.

**Card Properties:**
- `heat_threshold`: Minimum Heat for activation

**Activation (when Evidence > Cover):**
1. Check threshold: `current_heat >= heat_threshold`
2. If threshold met:
   - Insurance is overridden
   - Outcome: Busted
3. If below threshold:
   - Conviction inactive
   - Fall through to insurance check

**Override Rule:** Only last Conviction played is active.

---

## Card Examples

### Insurance Cards

| Card | Cover | Cost | Heat Penalty |
|------|-------|------|--------------|
| Burner Phone | 15 | $500 | +15 |
| Plea Bargain | 20 | $1,000 | +20 |

### Conviction Cards

| Card | Heat Threshold |
|------|----------------|
| Warrant | 40 |
| DA Approval | 60 |
| Federal Case | 80 |

---

## Edge Cases

| Scenario | Resolution |
|----------|------------|
| Multiple Insurance cards | Last played = active (override rule) |
| Multiple Conviction cards | Last played = active (override rule) |
| Insurance provides enough Cover | Not consumed (bust not triggered) |
| Can't afford insurance cost | Busted |
| Conviction threshold not met | Insurance can activate |
| Fold during hand | No bust check, insurance unused |

---

## Player Feedback

| State | UI Indicator |
|-------|--------------|
| During hand | Heat bar color (green/yellow/red) |
| Insurance active | Insurance slot shows card |
| Conviction active | Conviction slot shows card |
| Safe outcome | "DEAL COMPLETE!" (green) |
| Busted outcome | "BUSTED!" (red) |

---

## Integration

**Input:**
- HandState (cards_played, current_heat, cash)
- Active Insurance card (if any)
- Active Conviction card (if any)

**Output:**
- HandOutcome (Safe or Busted)
- Updated cash (if insurance used)
- Updated heat (if insurance used)
- Burned insurance (removed from deck)

**Permadeath:**
- On Busted: `save_data.character = None`
- Account cash survives
- Card unlocks survive
