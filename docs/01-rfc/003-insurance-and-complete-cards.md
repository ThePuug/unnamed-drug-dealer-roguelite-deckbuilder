# RFC-003: Insurance and Complete Cards

## Status

**Approved** - 2025-11-09 (Ready for SOW-003 Creation)

**Depends On:** ✅ RFC-002 (SOW-002 completed and merged)

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

**Analyst:** ARCHITECT Role
**Date:** 2025-11-09
**Based On:** SOW-001 and SOW-002 completion (foundation validated)

### Technical Assessment

**Verdict:** ✅ **FEASIBLE** (12-14 hours estimated)

**Foundation Ready from SOW-001/002:**
- ✅ Card type system extensible (add Insurance and Conviction card types)
- ✅ Bust resolution logic exists (`resolve_hand()` - extend for insurance check)
- ✅ Totals calculation handles additive Cover (insurance adds Cover like other Cover cards)
- ✅ Override rules implemented (last insurance/conviction = active)
- ✅ UI framework established (can add insurance status, warnings)

**New Systems Required:**

**1. Insurance Card Type & Logic (~3-4 hours)**
- New card type: `Insurance { cover: u32, cost: u32, heat_penalty: i32 }`
- Track active insurance (last Insurance card played)
- Extend `calculate_totals`: Insurance adds to Cover total
- Track cash/profit for affordability check
- Single-use tracking (burn card after activation)

**Implementation Approach:**
- Extend `CardType` enum with `Insurance` variant
- Add `active_insurance()` method (similar to `active_product()`)
- Add `cash: u32` field to HandState (tracks cumulative profit)
- Insurance acts as Cover card during totals calculation
- At bust resolution, if bust → check insurance

**2. Conviction Card Type & Logic (~2-3 hours)**
- New card type: `Conviction { heat_threshold: u32 }`
- Track active conviction (last Conviction card played)
- Heat tracking (cumulative across hands - for MVP, use hand Heat only)
- Threshold check in bust resolution (current_heat >= threshold?)

**Implementation Approach:**
- Extend `CardType` enum with `Conviction` variant
- Add `active_conviction()` method
- Conviction doesn't affect totals (only bust resolution)
- At bust resolution, check conviction before insurance

**3. Extended Bust Resolution (~2-3 hours)**
- Update `resolve_hand()` with insurance + conviction logic
- Cost affordability check (cash >= insurance.cost?)
- Conviction threshold check (heat >= threshold?)
- Card burn logic (remove insurance from deck after use)
- Heat penalty application (add heat_penalty to cumulative Heat)

**Implementation Approach:**
```rust
fn resolve_hand(&mut self) -> HandOutcome {
    let totals = self.calculate_totals();

    if totals.evidence > totals.cover {
        // Check conviction first
        if let Some(conviction) = self.active_conviction() {
            if self.current_heat >= conviction.heat_threshold {
                return HandOutcome::Busted; // Conviction overrides insurance
            }
        }

        // Check insurance
        if let Some(insurance) = self.active_insurance() {
            if self.cash >= insurance.cost {
                // Activate insurance
                self.cash -= insurance.cost;
                self.current_heat += insurance.heat_penalty;
                // Burn insurance card (remove from deck)
                return HandOutcome::Safe; // Insurance saves you
            } else {
                return HandOutcome::Busted; // Can't afford insurance
            }
        }

        return HandOutcome::Busted; // No insurance
    }

    HandOutcome::Safe // Evidence ≤ Cover
}
```

**4. Card Collection Completion (~2-3 hours)**
- Create 5 new cards (1 Product, 1 Location, 1 Evidence, 1 Cover, 1 Deal Modifier)
- Add to appropriate decks (distribute across Narc/Customer/Player)
- Balance values (price, Evidence, Cover, Heat)

**Implementation Approach:**
- Extend `create_*_deck()` functions
- Hardcode for MVP (RON extraction still deferred)

**5. Insurance/Conviction UI (~3-4 hours)**
- Insurance status display (active, cost, Heat penalty)
- Conviction warning (threshold status, big red warning if active)
- Bust resolution messages (insurance activated, conviction override, failure)
- Heat display (cumulative Heat tracking)

**Implementation Approach:**
- Add UI components: `InsuranceStatusDisplay`, `ConvictionWarningDisplay`
- Update `ui_update_system` to show insurance/conviction status
- Add bust resolution message overlay (modal with large text)
- Color-code warnings (green = safe, red = conviction active)

**Total Estimate:** 12-14 hours (fits within ≤20 hour SOW constraint)

---

### System Integration

**Integration Points:**

**From SOW-001/002 (Reuse):**
1. **Card Type System** → Extend with Insurance and Conviction types (trivial enum additions)
2. **Override Rules** → Reuse for active insurance/conviction (same pattern as Product/Location)
3. **Bust Resolution** → Extend `resolve_hand()` with insurance + conviction checks
4. **Totals Calculation** → Insurance adds to Cover (additive like other Cover cards)
5. **UI Framework** → Add insurance/conviction status components

**New Dependencies:**
- None (all logic uses existing Bevy/Rust patterns)

**Data Flow:**
```
Bust Check (Evidence > Cover):
  ↓
Check Conviction (Heat >= threshold?) → YES: Busted (override)
  ↓ NO
Check Insurance (afford cost?) → YES: Pay, gain Heat, continue
  ↓ NO                        → NO: Busted (can't afford)
Safe (Evidence ≤ Cover)
```

**No Breaking Changes:**
- SOW-001/002 code untouched (all additions)
- Tests remain valid
- Backward compatible (insurance/conviction optional)

---

### Alternatives Considered

**Alternative 1: Complex Insurance Economy**
- **Pros:** More strategic depth (multiple uses, varying costs)
- **Cons:** 10-15 extra hours, not needed for MVP validation
- **Verdict:** ❌ Deferred - Single-use sufficient for stakes validation

**Alternative 2: Progressive Heat System**
- **Pros:** Heat accumulates across hands (long-term consequences)
- **Cons:** Requires persistence, Heat decay, multiple hands implementation
- **Verdict:** ⏸️ Deferred to Phase 2 - For MVP, use hand-level Heat only

**Alternative 3: No Conviction Cards**
- **Pros:** Simpler (just insurance)
- **Cons:** Insurance always reliable (no threat, no Heat management)
- **Verdict:** ❌ Rejected - Conviction creates Heat importance

**Alternative 4: More Insurance Types (8-10 cards)**
- **Pros:** More variety, deck building options
- **Cons:** 5-8 extra hours to design/balance
- **Verdict:** ⏸️ Deferred to Phase 2 - 2 types sufficient for MVP validation

**Chosen Approach:**
- 2 insurance types (cost vs Heat trade-off)
- 2 conviction types (different Heat thresholds)
- Single-use insurance (simplest for MVP)
- Hand-level Heat only (no persistence yet)
- **Rationale:** Fastest path to "does insurance feel clutch?" answer

---

### Risks & Mitigation

**Technical Risks:**

**Risk 1: Insurance Balance**
- **Concern:** Cost too high (never use) or too low (always use)
- **Mitigation:** Tune based on SOW-002 average profits, playtest iteratively
- **Likelihood:** Medium
- **Impact:** Medium (but easy to retune)

**Risk 2: Make It Stick Too Common**
- **Concern:** Every hand has conviction → insurance useless
- **Mitigation:** For MVP, Narc deck has 0 conviction cards (rare occurrence)
- **Likelihood:** Low (controlled via deck composition)
- **Impact:** High if present (breaks insurance value)

**Risk 3: Heat Tracking Without Persistence**
- **Concern:** Hand-level Heat doesn't create long-term consequences
- **Mitigation:** Acceptable for MVP (validates mechanic, persistence in Phase 2)
- **Likelihood:** Low (design decision, not bug)
- **Impact:** Low (MVP scope acceptable)

**Risk 4: Insurance UX Clarity**
- **Concern:** Player doesn't understand why insurance failed (conviction override unclear)
- **Mitigation:** BIG visual warnings (red text, modal messages, threshold display)
- **Likelihood:** Medium
- **Impact:** High (confusing = not fun)

**Process Risks:**

**Risk 5: Scope Creep**
- **Concern:** "Just add one more insurance type" expands past 20 hours
- **Mitigation:** Strict 2 insurance, 2 conviction limit for MVP
- **Likelihood:** Medium
- **Impact:** High (delays complete MVP)

**Risk 6: Not Fun After All**
- **Concern:** Insurance/conviction don't add enjoyment (just complexity)
- **Mitigation:** Immediate playtest after implementation
- **Likelihood:** Low (insurance is core to game concept)
- **Impact:** Critical (might need redesign)

---

### Recommendations

**Approve RFC-003 with conditions:**

✅ **Approve IF:**
1. Scope stays within 12-14 hours (2 insurance, 2 conviction, 5 cards only)
2. MVP uses hand-level Heat (no persistence yet)
3. Narc deck has 0 conviction cards (prevents insurance from being useless)
4. Immediate playtest after implementation (validate stakes feel)

❌ **Reject IF:**
- Scope expands beyond 20 hours (must stay tight for fast validation)
- Player adds multiple insurance types before validating 2 types work

**Post-Implementation:**
- If insurance feels clutch → MVP complete, success!
- If insurance feels mandatory → Retune Heat/cost balance
- If conviction creates dread → Success!
- If too complex → Simplify or reconsider mechanic

---

### Conclusion

**RFC-003 is technically feasible** and completes the MVP validation loop (mechanics → fun → stakes). Estimated 12-14 hours fits within ≤20 hour constraint. Foundation from SOW-001/002 provides all needed extension points.

**Key Success Metric:** After implementation, answer "Does insurance create clutch moments and does Make It Stick create dread?"

**This RFC completes MVP.** After RFC-003, we have full playable prototype for external validation.

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

**Status:** ✅ **APPROVED** - Ready for SOW-003 Creation

**Approvers:**
- ARCHITECT: ✅ Approved (12-14 hours estimated, technically feasible, completes MVP loop)
- PLAYER: ✅ Approved (player need defined, stakes validation goals clear, completes MVP loop)

**Scope Constraint:** 12-14 hours (fits within ≤20 hour SOW limit)

**Dependencies:**
- RFC-002: ✅ SOW-002 completed and merged (foundation validated)

**Conditions:**
1. Scope must stay within 12-14 hours (2 insurance, 2 conviction only)
2. MVP uses hand-level Heat (no persistence system)
3. Narc deck has 0 conviction cards (keeps insurance valuable)
4. Immediate playtest after implementation (validate stakes feel)

**Next Steps:**
1. ✅ **SOW-002 Complete** (foundation validated)
2. ✅ **ARCHITECT:** Feasibility analysis complete (12-14 hours estimated)
3. ➡️ **ARCHITECT:** Create SOW-003 (define phases, deliverables, acceptance criteria)
4. **DEVELOPER:** Implement per SOW-003
5. **Critical:** After RFC-003, have COMPLETE playable MVP for external validation

**Date:** 2025-11-09
