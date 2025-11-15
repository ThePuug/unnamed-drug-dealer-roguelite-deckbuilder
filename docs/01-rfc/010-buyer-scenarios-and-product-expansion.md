# RFC-010: Buyer Scenarios and Product/Location Expansion

## Status

**Approved** - 2025-11-15 (PLAYER and ARCHITECT approved, ready for SOW-010)

## Feature Request

### Player Need

From player perspective: **Buyer deals feel too generic - I want each encounter to tell a story with specific drug combinations and contextual locations that make thematic sense.**

**Current Problem:**
With current Buyer system (RFC-009):
- Buyers want generic "Pills" or "Weed" - not specific drug combinations
- No narrative context for WHY they want specific drugs (party? addiction? performance?)
- Location preferences are vague (just "public" vs "private")
- All scenarios with same Buyer feel identical
- Missing thematic drug variety (no psychedelics, no ecstasy, generic "Pills")
- Player deck locations include dealer-unfriendly spots (School Zone E:40 C:5 - terrible idea!)
- Only 5 products in game (limited variety)

**We need a system that:**
- Gives each Buyer 2 distinct **scenarios** (different drug combo goals)
- Makes scenario choice visible and sticky (chosen at Buyer selection, persists across hands)
- Expands product variety (9 products with specific identities)
- Makes locations thematically appropriate (dealer picks safe spots, Buyers override with context)
- Tags products and locations for future conditional logic
- Creates "which scenario am I facing?" strategy in deck building

### Desired Experience

**Scenario Clarity:**
- "I'm dealing with Frat Bro (Get Wild scenario) - he wants Weed or Coke for a crazy party"
- "Wall Street Wolf (Desperate Times) needs Ice or Codeine for his performance edge"
- Scenario tells a story, not just stat requirements
- Visible upfront so I can build my deck accordingly

**Drug Specificity:**
- Each product has identity and purpose
- "Codeine" feels different from "Fentanyl" (prescription vs deadly)
- "Ecstasy" creates party context, "Shrooms" creates psychedelic context
- More interesting than generic "Pills"

**Location Logic:**
- My deck has safe dealer spots (Safe House, Dead Drop, Storage Unit)
- Buyers override with their context (Frat House, In a Limo, At the Park)
- Locations tell story of the deal context
- No more "why would I deal at a School Zone?" confusion

**Strategic Depth:**
- Build deck knowing scenario requirements
- "Should I bring Weed (safe, meets demand) or Coke (risky, higher profit)?"
- Location override creates tension (will Frat Bro pick Frat House?)
- Different scenarios with same Buyer = different strategies

---

## Buyer Scenarios

Each Buyer persona has **2 scenarios** representing different motivations/contexts.

### Frat Bro (was College Party Host)

**Scenario A: "Get Wild"**
- **Products Required:** Weed OR Coke (at least one)
- **Preferred Locations:** Frat House, Locker Room, Park
- **Heat Threshold:** None (fearless - knows it's risky, willing to take it)
- **Multiplier:** ×2.5 (high volume party)
- **Reduced Multiplier:** ×1.0 (if demand not met)
- **Theme:** Chaotic party energy, maximum wildness, reckless fun

**Scenario B: "Get Laid"**
- **Products Required:** Weed OR Ecstasy (at least one)
- **Preferred Locations:** Frat House, Locker Room, Dorm
- **Heat Threshold:** 35 (cautious - "these should be legal anyway, not worth getting busted for romance")
- **Multiplier:** ×2.5
- **Reduced Multiplier:** ×1.0
- **Theme:** Social connection party, vibes over chaos, romance

**Persona Stats:**
- Display Name: "Frat Bro"
- Base Multiplier: ×2.5
- Overall Theme: High volume, public locations, youth culture

---

### Desperate Housewife (was Stay-at-Home Mom / Soccer Mom)

**Scenario A: "Rock Bottom"**
- **Products Required:** Codeine OR Fentanyl (at least one)
- **Preferred Locations:** Private Residence, By the Pool
- **Heat Threshold:** 40 (addicted - will take risks for her fix, desperate)
- **Multiplier:** ×1.2
- **Reduced Multiplier:** ×1.0
- **Theme:** Severe addiction, desperate for relief, hiding desperation

**Scenario B: "In Denial"**
- **Products Required:** Codeine OR Weed (at least one)
- **Preferred Locations:** Private Residence, By the Pool, At the Park
- **Heat Threshold:** 25 (panics quickly - "I'm not a drug user! Just managing anxiety!")
- **Multiplier:** ×1.2
- **Reduced Multiplier:** ×1.0
- **Theme:** Managing anxiety, denying the problem, "just this once"

**Persona Stats:**
- Display Name: "Desperate Housewife"
- Base Multiplier: ×1.2
- Overall Theme: Suburban, private locations, self-medication

---

### Wall Street Wolf (was Executive)

**Scenario A: "Desperate Times"**
- **Products Required:** Ice (Meth) OR Codeine (at least one)
- **Preferred Locations:** In a Limo, Office, Parking Lot
- **Heat Threshold:** 45 (desperate - needs performance edge badly, willing to risk everything)
- **Multiplier:** ×2.8
- **Reduced Multiplier:** ×1.0
- **Theme:** Performance under pressure, long hours, competitive edge, desperation

**Scenario B: "Adrenaline Junkie"**
- **Products Required:** Ice (Meth) OR Coke (at least one)
- **Preferred Locations:** Parking Lot, In a Limo
- **Heat Threshold:** 30 (moderately cautious - thrill-seeking but not stupid)
- **Multiplier:** ×2.8
- **Reduced Multiplier:** ×1.0
- **Theme:** Calculated risk-taking, chasing the rush, dangerous combinations

**Persona Stats:**
- Display Name: "Wall Street Wolf"
- Base Multiplier: ×2.8
- Overall Theme: High stakes, private locations, performance enhancement

---

## Heat Threshold Spectrum

Scenarios ranked by risk tolerance (low to high):

1. **In Denial** (Housewife B): 25 - Panics quickly, denial fragile
2. **Adrenaline Junkie** (Wolf B): 30 - Moderate caution
3. **Get Laid** (Frat A): 35 - Not worth the bust for romance
4. **Rock Bottom** (Housewife A): 40 - Addicted, takes risks
5. **Desperate Times** (Wolf A): 45 - Needs it badly, desperate
6. **Get Wild** (Frat B): None - Fearless, knows risk, doesn't care

---

## Product System Expansion

### Complete Product List (9 total)

| Product | Price | Heat | Schedule | Class | Context |
|---------|-------|------|----------|-------|---------|
| Weed | $30 | 5 | III | Cannabis | Party, Casual |
| Ice (was Meth) | $100 | 30 | II | Stimulant | Performance-Enhancing, Street |
| Heroin | $150 | 45 | I | Depressant | Street |
| Coke (was Cocaine) | $120 | 35 | II | Stimulant | Party, Street |
| Fentanyl | $200 | 50 | II | Depressant | Prescription (diverted) |
| **Codeine** | $50 | 10 | II | Depressant | Prescription, Medical |
| **Ecstasy** | $80 | 25 | I | Party | Party |
| **Shrooms** | $40 | 8 | I | Psychedelic | Party, Recreational |
| **Acid** | $60 | 12 | I | Psychedelic | Party, Recreational |

### Product Tags

Tags enable future conditional logic (Buyer preferences, special rules, events):

**Drug Class:**
- **Stimulant:** Ice, Coke
- **Depressant:** Heroin, Codeine, Fentanyl
- **Psychedelic:** Acid, Shrooms
- **Cannabis:** Weed
- **Party:** Ecstasy

**Use Context:**
- **Party:** Ecstasy, Weed, Shrooms, Acid
- **Medical/Prescription:** Codeine, Fentanyl
- **Street:** Coke, Heroin, Ice
- **Performance-Enhancing:** Ice, Codeine

**Legal Schedule (DEA):**
- **Schedule I:** Heroin, Ecstasy, Acid, Shrooms
- **Schedule II:** Coke, Ice, Fentanyl, Codeine
- **Schedule III:** Weed

**Risk Profile (Heat):**
- **HighHeat:** Fentanyl (50), Heroin (45), Coke (35), Ice (30)
- **ModerateHeat:** Ecstasy (25), Acid (12), Codeine (10)
- **LowHeat:** Shrooms (8), Weed (5)

**Market Tier (Price):**
- **Premium:** Fentanyl ($200), Heroin ($150), Coke ($120), Ice ($100)
- **MidTier:** Ecstasy ($80), Acid ($60), Codeine ($50)
- **Budget:** Weed ($30), Shrooms ($40)

---

## Location System Redesign

### Player Deck Locations (4 cards - Dealer's Safe Choices)

**Replaced:**
- ❌ School Zone (E:40 C:5 H:20) - Removed (terrible for dealer)
- ❌ Back Alley (E:25 C:20 H:0) - Removed (too risky)

**New Lineup:**
1. **Safe House** - E:10 C:30 H:-5 (controlled, private, bolt hole)
2. **Abandoned Warehouse** - E:15 C:25 H:-10 (isolated, industrial, no witnesses)
3. **Storage Unit** - E:12 C:28 H:-8 (private, locked, anonymous rental)
4. **Dead Drop** - E:8 C:20 H:-5 (no face-to-face, minimal exposure, tradecraft)

**Characteristics:**
- All have good Cover (20-30)
- All have low Evidence (8-15)
- All have negative Heat (safer choices)
- Mix of very safe (Dead Drop E:8) to moderately safe (Warehouse E:15)

### Buyer Location Cards (7 per persona, 2 are Locations)

**Frat Bro:**
- Locker Room (Safe: E:5 C:20 H:-5)
- Frat House (Risky: E:15 C:15 H:10)

**Desperate Housewife:**
- By the Pool (Safe: E:5 C:25 H:-10)
- At the Park (Risky: E:15 C:15 H:5)

**Wall Street Wolf:**
- In a Limo (Safe: E:5 C:30 H:-10)
- Parking Lot (Risky: E:15 C:20 H:5)

### Location Tags

**Privacy Level:**
- **Private:** Safe House, Storage Unit, Abandoned Warehouse, Private Office, In a Limo, By the Pool
- **SemiPrivate:** Dead Drop, Parking Lot, Locker Room
- **Public:** Frat House, At the Park, Dorm

**Location Type:**
- **Residential:** Safe House, By the Pool
- **Industrial:** Abandoned Warehouse, Storage Unit, Parking Lot
- **Commercial:** Office, Parking Lot, Storage Unit
- **Educational:** Locker Room, Dorm, Frat House
- **Recreational:** Frat House, At the Park, By the Pool

---

## Implementation Notes

### Scenario Selection
- **MVP:** Random scenario chosen at Buyer selection
- **Future:** Player choice UI ("Which scenario are you facing?")
- Scenario stored in `BuyerPersona.active_scenario` field

### Demand Validation
- Product requirement: **OR** logic (at least one matches)
- Location requirement: **OR** logic (at least one matches)
- Both must be satisfied for base_multiplier
- Override mechanic means only one Product/Location active at resolution

### Heat Threshold
- Stored per scenario (not per Buyer)
- Each scenario can have different threshold
- Creates variation within same Buyer ("Get Wild" = fearless, "Get Laid" = cautious)

### Tags
- Stored on Card struct as `tags: Vec<ProductTag>` and `tags: Vec<LocationTag>`
- Empty for non-Product/Location cards
- Enable future conditional logic without code changes

---

## PLAYER Validation

After implementing and playing 5 hands across different scenarios:

**Can I describe scenario personality?**
- "Get Wild Frat Bro is reckless, Rock Bottom Housewife is desperate"

**Do scenarios create different strategies?**
- Different deck builds for "needs Coke" vs "needs Ecstasy"

**Do product names feel meaningful?**
- "Ice" vs "Codeine" vs "Shrooms" - each has identity

**Do locations make thematic sense?**
- Dealer spots feel safe (Safe House, Dead Drop)
- Buyer spots feel contextual (Frat House for parties, Limo for executive)

**Does scenario variety improve replayability?**
- Same Buyer, different scenario = different challenge

**If 4+ YES → Feature is successful**

---

## Open Questions

1. Should player see both scenarios before selection, or just the chosen one?
2. Should scenario affect Buyer reaction deck cards (different 7 cards per scenario)?
3. Should products have visual indicators for tags (color-coded by Schedule, icons for class)?
4. Should Dead Drop have special mechanics (different stats if used multiple times)?

---

## Feasibility Analysis (ARCHITECT)

**Date:** 2025-11-15
**Reviewer:** ARCHITECT

### Technical Assessment

**Can we build this?** ✅ Yes

This RFC extends the existing Buyer system (RFC-009) without fundamental architectural changes. The core mechanics already support:
- BuyerPersona data structures (can extend with scenarios)
- Demand satisfaction logic (can update to check scenario preferences)
- Product/Location override system (just need more card variety)
- Heat threshold per Buyer (can move to per-scenario)

### System Integration

**Fits existing architecture:** ✅ Yes

**Integration points:**
1. **BuyerPersona struct** - Add scenarios array, active_scenario index
2. **Card creation** - Add 4 new Products, update 2 Location cards, add tags field
3. **Demand validation** - Update is_demand_satisfied() to check scenario.products/locations
4. **Buyer selection** - Add scenario randomization (1 line change)
5. **UI display** - Show active scenario in Buyer info panel

**No breaking changes:**
- Existing Buyer system remains functional
- Card override rules unchanged
- Resolution flow intact
- All tests should pass with scenario additions

### Scope Assessment

**Fits in one SOW (≤20 hours)?** ✅ Yes

**Estimated breakdown:**
- Phase 1: Add 4 new Products to player deck (2-3h)
- Phase 2: Update 2 player Locations, add tags to Card struct (2h)
- Phase 3: Implement BuyerScenario struct, update personas (3-4h)
- Phase 4: Update demand validation for scenarios (2h)
- Phase 5: UI updates (scenario display, better Buyer info) (2-3h)
- Phase 6: Testing and balance (2-3h)

**Total:** 13-17 hours (within ≤20 hour constraint)

### Risks and Unknowns

**Low Risk:**
- Product/Location additions (just data)
- Scenario structure (straightforward extension)
- Tag system (future-proofing, can start minimal)

**Medium Risk:**
- Balance (9 products might need tuning, price/heat relationships)
- Scenario variety (2 per Buyer enough? or feels repetitive?)
- Tag explosion (could over-tag early, then never use tags)

**Mitigation:**
- Start with MVP tags (just Drug Class, Privacy Level)
- Balance via playtesting (PLAYER feedback)
- Can add scenarios post-MVP if needed

### Technical Approach

**Recommended structure:**

```rust
struct BuyerScenario {
    id: String,
    display_name: String,
    products: Vec<String>,        // ["Weed", "Coke"]
    locations: Vec<String>,       // ["Frat House", "Locker Room"]
    heat_threshold: Option<u32>,
    description: String,
}

struct BuyerPersona {
    // ... existing fields
    scenarios: Vec<BuyerScenario>,  // 2 scenarios
    active_scenario_index: Option<usize>,  // 0 or 1
}

enum ProductTag {
    // Drug Class
    Stimulant, Depressant, Psychedelic, Cannabis, Party,
    // Use Context
    PartyDrug, MedicalPrescription, StreetDrug, PerformanceEnhancing,
    // Legal Schedule
    Schedule1, Schedule2, Schedule3,
    // Risk Profile
    HighHeat, ModerateHeat, LowHeat,
    // Market Tier
    Premium, MidTier, Budget,
}

enum LocationTag {
    // Privacy Level
    Private, SemiPrivate, Public,
    // Location Type
    Residential, Industrial, Commercial, Educational, Recreational,
}

struct Card {
    // ... existing fields
    tags: Vec<ProductTag>,  // Or Vec<LocationTag> - empty for non-Product/Location
}
```

**Validation logic:**
```rust
fn is_demand_satisfied(&self) -> bool {
    let scenario = &self.buyer_persona.scenarios[active_scenario_index];

    let product_match = self.active_product(true)
        .map(|p| scenario.products.contains(&p.name))
        .unwrap_or(false);

    let location_match = self.active_location(true)
        .map(|l| scenario.locations.contains(&l.name))
        .unwrap_or(false);

    product_match && location_match
}
```

### Architectural Concerns

**1. Tag Maintenance**
- **Issue:** Tags on every card instance (memory overhead, maintenance burden)
- **Solution:** Start with minimal tags (Drug Class, Privacy Level only), add more if actually needed
- **Alternative:** Derive tags from card name via lookup table (centralized, less redundant)

**2. Scenario Complexity**
- **Issue:** 3 Buyers × 2 scenarios = 6 total combinations (will players remember?)
- **Solution:** Clear UI display, scenario persists across hands (no surprises)
- **Future:** Could add scenario icons/colors for quick recognition

**3. Product/Location Count**
- **Issue:** 9 products + many locations = large card pool (deck building complexity)
- **Solution:** Deck builder already handles 20-card pool, adding variety improves it
- **Balance:** May need to adjust deck building to show "Products: 9 available" guidance

### Recommendation

**✅ FEASIBLE** - Proceed to implementation planning

**Scope is appropriate:**
- Extends existing system cleanly
- Fits in one SOW (13-17h estimate)
- Low technical risk
- High player value (thematic coherence, variety)

**Suggested refinements:**
1. Start with minimal tags (Drug Class, Privacy Level) - add more in future SOW if needed
2. Keep scenario count at 2 per Buyer for MVP (can expand post-launch)
3. Consider scenario icon/color system for quick recognition
4. Playtest balance after adding products (price/heat relationships)

**Status:** Ready for PLAYER/ARCHITECT iteration in Discussion section

---

## Discussion

### ARCHITECT Notes - Initial Review

**Strong Points:**
- Clear player need (thematic coherence, variety)
- Natural extension of RFC-009 (scenarios fit Buyer system well)
- Low technical risk (mostly data additions)
- Good scope control (≤20 hours)

**Open Items for Discussion:**

**Q1: Tag Granularity**
Should we implement all 5 tag categories initially, or start minimal?
- **Proposal:** Start with Drug Class + Privacy Level only, defer others to future SOW
- **Rationale:** YAGNI - implement tags when actually needed for logic

**Q2: Scenario Selection UX**
How should scenario be presented to player?
- **Option A:** Just show chosen scenario (simpler)
- **Option B:** Show both scenarios, highlight chosen one (more context)
- **Recommendation:** Option A for MVP (reduces UI complexity)

**Q3: Location Type Tags**
Parking Lot is both Industrial + Commercial - allow multi-tagging?
- **Proposal:** Yes, locations can have multiple Type tags
- **Implementation:** `Vec<LocationTag>` supports this naturally

**Awaiting PLAYER feedback on open items before approval.**

---

### PLAYER Response - 2025-11-15

**Q1: Tag Granularity**
✅ Approved - Start minimal (Drug Class + Privacy Level only), YAGNI principle applies

**Q2: Scenario Selection UX**
✅ Approved - Show only chosen scenario for MVP (simpler, clearer)

**Q3: Location Type Tags**
✅ Approved - Allow multi-tagging (Parking Lot = Industrial + Commercial makes sense)

**All open items resolved** - Ready for approval.

---

## Approval

**PLAYER:** ✅ Approved - 2025-11-15
- Solves player need (thematic coherence, variety)
- Scenarios create meaningful narrative context
- Product expansion adds strategic depth
- Scope appropriate for one SOW

**ARCHITECT:** ✅ Approved - 2025-11-15
- Technically feasible (extends RFC-009 cleanly)
- Scope contained (13-17 hours estimated)
- Low risk, high value
- Integration points clear

**Status:** ✅ **APPROVED** - Ready for SOW creation

**Next Step:** ARCHITECT creates SOW-010 implementation plan
