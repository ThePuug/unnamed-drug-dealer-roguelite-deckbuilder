# RFC-003: Insurance and Complete Cards

## Status

**Draft** - 2025-11-09

**Depends On:** RFC-002 (must complete SOW-002 first)

## Feature Request

### Player Need

From player perspective: **I need to feel the high-stakes consequences of getting busted and the clutch moment of insurance saving my run** - Does insurance feel valuable? Does the threat of Make It Stick create dread at high Heat? Does the complete 20-card collection provide enough strategic variety?

**Current Problem:**
After RFC-002 (fun validation with betting + AI):
- No consequences for getting busted (just game over, restart immediately)
- No insurance escape (can't save yourself from bust)
- No Make It Stick threat (no reason to manage Heat carefully)
- Limited card variety (15 cards, no insurance/conviction options)
- No high-stakes moments (every hand feels same importance)

**We need a system that:**
- Implements Get Out of Jail cards (insurance with cost)
- Implements Make It Stick cards (conviction that overrides insurance)
- Creates insurance activation moment (pay cost, gain Heat, survive bust)
- Creates conviction override moment (insurance fails, run ends despite having it)
- Completes 20-card collection (full strategic variety for deck building)
- Validates insurance value (worth including 2-3 in every deck?)

### Desired Experience

**This is STAKES VALIDATION - the emotional peak.**

Players should experience:
- **Insurance clutch moment** - Evidence 85 > Cover 60, Plea Bargain activates, "Whew! Survived!"
- **Make It Stick dread** - Narc plays Warrant, Heat is 65, "Oh no, insurance won't work"
- **Cost decision** - Pay $1k to survive or let run end? (have insurance, can't afford it)
- **Heat management matters** - High Heat = conviction threat, must include insurance in deck
- **Strategic deck building** - Need 2-3 insurance cards at high Heat (mandatory survival)
- **Permadeath weight** - Getting busted MEANS something (not just "restart")

**If insurance doesn't feel clutch or Make It Stick doesn't create dread, the risk/reward loop is broken.**

### Specification Requirements

**Get Out of Jail Cards (Insurance):**

**2 Insurance Types (MVP):**
```
Plea Bargain
  Cover: +20
  Cost: $1,000
  Heat Penalty: +20
  (Balanced: moderate cost, moderate Heat)

Fake ID
  Cover: +15
  Cost: $0
  Heat Penalty: +40
  (Free but expensive Heat - desperate play)
```

**Insurance Mechanics:**
- Acts as Cover card during hand (adds to Cover total)
- Acts as insurance at resolution (if Evidence > Cover)
- Override rule: Last Get Out of Jail played = active insurance (only one)
- Single use per deck: Burned after activation (can't reuse)
- Requirements check: Must have cash ≥ cost to activate
- If activated: Pay cost, gain Heat (overage + penalty), run continues
- If can't afford: Insurance fails, run ends (busted)
- If not needed (Evidence ≤ Cover): Not consumed, still available

**Make It Stick Cards (Conviction):**

**2 Conviction Types (MVP):**
```
Warrant
  Heat Threshold: 40

DA Approval
  Heat Threshold: 60
```

**Conviction Mechanics:**
- Override rule: Last Make It Stick played = active conviction
- Heat threshold check: Only applies if current_heat >= threshold
- Insurance override: If threshold met, Get Out of Jail fails (run ends)
- If threshold not met: Conviction inactive, insurance works normally

**Bust Resolution Flow (Complete):**
```
1. Calculate totals: Evidence, Cover
2. IF Evidence > Cover:
   a. Check Make It Stick: current_heat >= threshold?
      - YES: Insurance overridden → RUN ENDS
      - NO: Fall through to insurance check
   b. Check Get Out of Jail: can afford cost?
      - YES: Pay cost, gain Heat, burn card, continue
      - NO: Insurance fails → RUN ENDS
   c. No insurance: RUN ENDS
3. ELSE (Evidence ≤ Cover):
   - Safe, bank profit, continue
```

**Card Collection Completion:**
- Add 5 more cards to reach 20 total:
  - 1 Product variant (e.g., Cocaine: $120, +35 Heat)
  - 1 Location variant (e.g., Warehouse: 15 Evidence, 25 Cover, -10 Heat)
  - 1 Evidence variant (e.g., Informant: +25 Evidence, +15 Heat)
  - 1 Cover variant (e.g., Bribe: +25 Cover, +10 Heat)
  - 1 Deal Modifier (e.g., Lookout already planned in RFC-002)

**Player Feedback Additions:**
- Insurance status: "Insurance Active: Plea Bargain (cost $1k, Heat +20)"
- Conviction warning: "⚠️ WARRANT ACTIVE - Threshold: 40 (your Heat: 65) - INSURANCE WILL NOT WORK"
- Bust prevention: "INSURANCE ACTIVATED - Plea Bargain - Paid $1k, gained +45 Heat, run continues"
- Insurance failure: "INSURANCE FAILED - Insufficient funds ($800 / $1,000) - Run Ends"
- Conviction override: "⚠️ DA APPROVAL OVERRIDES INSURANCE - Run Ends"

**Heat Threshold Warnings:**
- Before hand start: If Heat near threshold, warn "Heat 38 - close to Warrant threshold (40)"
- During hand: If Make It Stick played, show threshold status clearly
- Color-coded: Green (below threshold), Red (above threshold)

### MVP Scope

**Phase 1 includes:**
- Get Out of Jail cards (2 types: Plea Bargain, Fake ID)
- Make It Stick cards (2 types: Warrant, DA Approval)
- Insurance activation (cost payment, Heat gain, card burn)
- Conviction override (threshold check, insurance override)
- Complete bust resolution flow (Insurance check → Conviction check)
- 20-card collection (5 additional cards)
- UI improvements (insurance status, conviction warnings, bust prevention messaging)
- Cost affordability check (can't activate insurance if insufficient cash)

**Phase 1 excludes:**
- Additional insurance types (8-10 total) (Phase 2)
- Additional conviction types (Caught Red-Handed, Federal Case) (Phase 2)
- Heat decay (persistence system) (Phase 2)
- Trust system (Phase 2)
- Character permadeath (Phase 2)
- Deck building UI (can hardcode for now)
- Session continuity (currently: bust = game over, no restart)

### Priority Justification

**HIGH PRIORITY** - This completes the core loop validation (stakes + consequences)

**Why HIGH:**
- Insurance is THE unique mechanic (push-your-luck with escape)
- Make It Stick creates meaningful Heat management (high Heat = danger)
- Without stakes, game is just math (no emotional weight)
- Completes MVP validation loop (mechanics → fun → stakes)

**Benefits:**
- **Stakes validation** - Does insurance feel clutch when it saves you?
- **Threat validation** - Does Make It Stick create dread?
- **Heat validation** - Does Heat management matter? (conviction at high Heat)
- **Collection validation** - 20 cards enough for deck variety?
- **Complete MVP** - After RFC-003, have full playable prototype

---

## Feasibility Analysis

*[ARCHITECT to complete]*

### Technical Assessment

*To be added by ARCHITECT*

### System Integration

*To be added by ARCHITECT*

### Alternatives Considered

*To be added by ARCHITECT*

---

## Discussion

### PLAYER Notes

**Insurance Emotional Moments:**
- **Relief:** Evidence 90, Cover 65, Plea Bargain saves you → "YES! Survived!"
- **Panic:** Evidence 85, Cover 60, have Plea Bargain but only $800 → "NO! Can't afford it!"
- **Strategic:** Have insurance in deck, should I play it early (Cover) or save (insurance)?

**If insurance doesn't create these moments, it's not working.**

**Make It Stick Emotional Moments:**
- **Dread:** Narc plays Warrant, Heat 65, "I'm screwed if I get busted"
- **Calculated risk:** Heat 35, Warrant played, "I'm safe from conviction, insurance will work"
- **Fold decision:** DA Approval played, Heat 62, "Even if I have insurance, it won't work - FOLD NOW"

**If Make It Stick doesn't change decisions, it's not working.**

**20-Card Collection:**
- Enough variety for:
  - Aggressive deck (high-value Products, risky Locations, light Cover)
  - Balanced deck (mix of Products/Locations/Cover)
  - Defensive deck (low-value Products, safe Locations, heavy Cover)
- Missing: Full variety (80-100 cards in Phase 3)
- Acceptable: MVP demonstrates variety, not completeness

### PLAYER Validation

**What makes this fun?**
1. **Insurance saves run** - Bust doesn't mean instant death (if you planned ahead)
2. **Cost decision** - Worth paying $1k to survive? (depends on Heat, run length, profit)
3. **Make It Stick escalates** - High Heat makes insurance unreliable (dread)
4. **Deck building matters** - Must include insurance at high Heat (strategic choice)
5. **Heat management matters** - Keep Heat low, insurance reliable (long-term thinking)

**What could go wrong?**
- **Insurance feels mandatory** - Every deck needs 3-4 insurance cards (reduces variety)?
  - *Mitigation:* Balance Heat accumulation, insurance should be needed at high Heat only
- **Make It Stick too common** - Every hand has conviction card (insurance useless)?
  - *Mitigation:* Narc static deck has 0× Make It Stick for RFC-003 MVP (add in Phase 2 with Heat scaling)
- **Cost too high** - $1k for Plea Bargain eats all profit?
  - *Mitigation:* Tune based on average hand profit (~$200-400), insurance should cost 2-3 hands of profit
- **Bust feels cheap** - "I had insurance, why didn't it work?" (Make It Stick not clear)?
  - *Mitigation:* BIG warnings when Make It Stick active + threshold met

---

## Approval

**Status:** Awaiting ARCHITECT feasibility analysis

**Approvers:**
- ARCHITECT: [Pending]
- PLAYER: ✅ Approved (player need defined, stakes validation goals clear, completes MVP loop)

**Scope Constraint:** [ARCHITECT to estimate - target ≤20 hours]

**Dependencies:**
- RFC-002: Must complete SOW-002 first (betting, AI, 3-round structure)

**Next Steps:**
1. **Wait for RFC-002 completion** (SOW-002 implementation, 15-18 hours)
2. **ARCHITECT:** Add feasibility analysis
3. **If Approved:** ARCHITECT creates SOW-003
4. **DEVELOPER:** Implements per SOW-003
5. **Critical:** After RFC-003, have COMPLETE playable MVP for validation

**Date:** 2025-11-09
