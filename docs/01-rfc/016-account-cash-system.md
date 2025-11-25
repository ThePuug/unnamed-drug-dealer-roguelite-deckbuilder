# RFC-016: Account Cash System

## Status

**Draft** - 2025-11-25

## Feature Request

### Player Need

From player perspective: **I want to keep the money I earn, even when my character dies, so I always make progress.**

**Current Problem:**
Without account-wide cash:
- Character death means losing all accumulated wealth
- No persistent progression currency
- Can't save up for expensive unlocks across runs
- Permadeath feels too punishing (lose everything)

**We need a system that:**
- Tracks cash earned from completing deals
- Persists cash across character deaths (account-wide)
- Shares cash between multiple characters (when slots unlock)
- Separates "spendable cash" from "revenue metric" (for leaderboards)

### Desired Experience

Players should experience:
- **Permanent progress:** Every deal earns cash that's never lost
- **Strategic spending:** Choose what to unlock with limited resources
- **Reduced death sting:** Lose character, keep wealth
- **Long-term goals:** Save up for expensive unlocks over many runs

### Specification Requirements

**Cash Pool:**
- Single account-wide value (not per-character)
- Persists forever (survives permadeath)
- Shared across all character slots
- Can be spent on card unlocks at locations (future RFC)

**Earning Cash:**
- Add profit to cash pool when hand completes successfully
- Profit = Product base price × multipliers from Deal Modifiers
- Only earned if hand resolves Safe (Evidence ≤ Cover)
- Folded hands earn nothing

**Revenue Metric (Separate):**
- Tracks total cash ever earned (lifetime)
- NOT reduced by spending
- Used for leaderboards (future) and achievements
- Display: "Lifetime Revenue: $X"

**Cash Display:**
- Show current cash on hand (spendable)
- Show cash earned this hand/deck/run
- Show lifetime revenue (achievement tracking)

### MVP Scope

**Phase 1 includes:**
- Account-wide cash pool
- Cash earning from successful hands
- Cash persistence (save/load)
- Basic cash display in UI
- Revenue metric tracking

**Phase 1 excludes:**
- Cash spending (needs location shops - future RFC)
- Leaderboard integration (future)
- Cash-based achievements (future)

### Priority Justification

**HIGH PRIORITY** - Core progression currency

**Why HIGH:**
- Cash is the universal progression currency
- Required before location unlocks can work
- Reduces permadeath frustration (always progress)
- Simple to implement, high impact

**Benefits:**
- Every play session feels productive
- Creates spending decisions (save vs spend)
- Enables future unlock systems
- Softens permadeath without removing stakes

---

## Feasibility Analysis

### Technical Assessment

**Proposed Solution: Account State with Dual Tracking**

#### Core Mechanism

**Account State (persisted):**
```rust
struct AccountState {
    cash_on_hand: u32,      // Spendable, reduced by purchases
    lifetime_revenue: u32,  // Total earned, never reduced
    // Future: unlocked_cards, unlocked_locations, achievements
}
```

**Cash Earning (at hand resolution):**
```rust
fn resolve_hand_profit(hand: &HandResult, account: &mut AccountState) {
    if hand.outcome == HandOutcome::Safe {
        let profit = calculate_profit(hand);
        account.cash_on_hand += profit;
        account.lifetime_revenue += profit;
    }
    // Folded or Busted = no cash earned
}

fn calculate_profit(hand: &HandResult) -> u32 {
    let base_price = hand.active_product.base_price;
    let multiplier = hand.price_multipliers.iter().product();
    let additive = hand.price_modifiers.iter().sum();
    (base_price * multiplier + additive) as u32
}
```

**Cash Spending (future, stub for now):**
```rust
fn purchase_card(card: &Card, account: &mut AccountState) -> Result<(), PurchaseError> {
    if account.cash_on_hand >= card.price {
        account.cash_on_hand -= card.price;
        // Note: lifetime_revenue unchanged
        Ok(())
    } else {
        Err(PurchaseError::InsufficientFunds)
    }
}
```

#### Performance Projections

- **Storage:** ~16 bytes for account state (minimal)
- **Computation:** O(1) for all operations
- **Development time:** ~4-6 hours

#### Technical Risks

**1. Integer Overflow**
- *Risk:* Cash exceeds u32 max (~4 billion)
- *Mitigation:* Use u64 or cap at reasonable max (e.g., 999,999,999)
- *Impact:* Low (would take extreme play time)

**2. Save Corruption**
- *Risk:* Account state lost
- *Mitigation:* Separate account save from character save, backups
- *Impact:* High (lose all progression)

**3. Desync Between Cash and Revenue**
- *Risk:* Bug causes cash > revenue or other inconsistencies
- *Mitigation:* Validate on load, revenue always ≥ cash on hand minus purchases
- *Impact:* Low (cosmetic issue)

### System Integration

**Affected Systems:**
- Hand resolution (cash earning)
- Save/load system (account persistence)
- UI (cash display)
- Future: Location shops (cash spending)

**Compatibility:**
- ✅ Hand resolution calculates profit already
- ✅ Save system exists (add account state)
- ✅ UI has space for cash display
- ✅ Character permadeath doesn't touch account state

### Alternatives Considered

#### Alternative 1: Per-Character Cash

Each character has own cash pool, lost on permadeath.

**Rejected because:**
- Too punishing (lose everything on death)
- No long-term progression
- Conflicts with spec design

#### Alternative 2: Partial Cash Survival

Percentage of cash survives permadeath (e.g., 50%).

**Rejected because:**
- Adds complexity without clear benefit
- Still feels punishing
- Spec says 100% survival

---

## Discussion

### ARCHITECT Notes

- Account state is separate from character state (different lifecycle)
- Cash earning hooks into existing profit calculation
- Revenue metric enables future leaderboard without affecting gameplay
- Consider atomic saves (account + character together)

### PLAYER Validation

Success criteria from spec:
- ✅ Cash never lost (survives permadeath)
- ✅ Spending is real tradeoff (buy now vs save)
- ✅ Always making progress (every hand matters)
- ✅ Clear display of earnings

---

## Approval

**Status:** Draft

**Approvers:**
- ARCHITECT: [ ] Pending review
- PLAYER: [ ] Pending review

**Scope Constraint:** ~4-6 hours (fits in one SOW)

**Dependencies:**
- RFC-015: Heat & Character Persistence (permadeath context)
- Hand resolution (profit calculation exists)
- Save system (exists)

**Next Steps:**
1. Review and approve RFC
2. Create SOW-016
3. Add AccountState to save system
4. Hook cash earning into hand resolution
5. Add cash display to UI
6. Test cash persistence across deaths

**Date:** 2025-11-25
