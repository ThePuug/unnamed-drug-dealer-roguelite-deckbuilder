# RFC-005: Deck Balance and Card Distribution

## Status

**Approved** - 2025-11-09

## Feature Request

### Player Need

From player perspective: **Current decks are poorly balanced - Player gets cards that hurt them, AI runs out of cards too fast, and Customer barely participates strategically.**

**Current Problems:**

**Player Deck Issues:**
- Has Evidence cards (Informant +25 Evidence) that actively help opponents bust the player
- Evidence cards are dead weight - want to fold immediately just to discard them
- No strategic reason to ever play an Evidence card
- Creates frustration ("Why do I have cards that hurt me?")

**AI Deck Depletion:**
- Narc: 15 cards, plays ~9 cards per hand → runs out by hand 2
- Customer: 10 cards, plays ~9 cards per hand → runs out by hand 1-2
- AI frequently all-in or folding (boring, predictable)
- Reduces strategic variety (AI can't make interesting plays when out of cards)

**Customer Deck Boring:**
- Only has placeholder Evidence cards (Regular Order, Haggling)
- Can't play Products (no deals to make)
- Can't play Locations (no venues to suggest)
- Feels passive and uninteresting (just adds Evidence, no strategy)

**We need a system that:**
- Removes self-harming cards from Player deck (no Evidence)
- Gives all players larger decks (20-30 cards) to prevent early exhaustion
- Makes Customer strategic (can play Products, Locations, Deal Modifiers)
- Creates thematic, balanced decks that make sense for each role
- Sets up foundation for deck building (need good base decks first)

### Desired Experience

Players should experience:
- **Thematic Decks:** "Customer plays Bulk Order (deal modifier) - makes sense!"
- **Strategic Variety:** "Narc played Surveillance, Customer countered with Private Sale"
- **Balanced Competition:** All players have interesting plays throughout 4-6 hands
- **No Dead Cards:** Every card in deck has strategic purpose
- **AI Stays Engaged:** Narc/Customer don't run out of cards by hand 2

### Specification Requirements

**Player Deck (20 cards - Dealer Theme):**
- Products: 5 cards (Weed, Meth, Heroin, Cocaine, + 1 new)
- Locations: 4 cards (Safe House, School Zone, Warehouse, + 1 new)
- Cover: 4 cards (Alibi, Bribe, + 2 new)
- Insurance: 2 cards (Plea Bargain, Fake ID)
- Deal Modifiers: 5 cards (Disguise, + 4 new defensive modifiers)
- **NO Evidence cards** (Narc's job, not player's)
- **NO Conviction cards** (should be in Narc deck, currently in player for testing)

**Narc Deck (25 cards - Law Enforcement Theme):**
- Evidence: 17 cards (variety of threat levels)
  - 8× Low threat (Donut Break, Patrol)
  - 6× Medium threat (Surveillance, Stakeout, Informant)
  - 3× High threat (Raid, Wiretap, Undercover Op)
- Conviction: 8 cards (Make It Stick - prevent insurance)
  - 4× Warrant (threshold 40)
  - 3× DA Approval (threshold 60)
  - 1× RICO Case (threshold 80 - rare)
- **NO Insurance** (player defensive mechanic only)
- **NO Products** (Narc doesn't sell drugs)

**Customer Deck (25 cards - Deal Dynamics Theme):**
- Products: 5 cards (Customer requests - specific products they want)
- Locations: 5 cards (Customer suggests venues for deals)
- Deal Modifiers: 15 cards (haggling, bulk orders, rush jobs)
  - Price modifiers (×0.8 to ×1.5)
  - Evidence/Cover modifiers (±10 to ±20)
  - Heat modifiers (-10 to +15)
- **NO Evidence** (Customer isn't law enforcement)
- **NO Insurance/Conviction** (those are player vs narc mechanics)

**Card Distribution Philosophy:**
- **Player:** Dealer perspective (products to sell, locations to deal, cover tracks, insurance to survive)
- **Narc:** Law enforcement perspective (evidence to catch you, conviction to block insurance)
- **Customer:** Buyer perspective (products they want, locations they prefer, deal terms they negotiate)
- **Balance:** Each player 20-25 cards (lasts 6-8 hands before exhaustion)
- **Thematic Consistency:** Cards match the role (no Evidence for player, no Insurance for narc)

### MVP Scope

**Phase 1 includes:**
- Remove Evidence cards from Player deck
- Expand all decks to 20-25 cards
- Give Customer thematic cards (Products, Locations, Modifiers)
- Balance Narc deck (more variety, less Donut Break spam)
- Update card instantiation tests

**Phase 1 excludes:**
- New card mechanics (just use existing card types)
- Card unlocks / progression (defer to deck building RFC)
- Visual card design (text-based is fine for MVP)
- Advanced AI strategy (current AI decision-making sufficient)

### Priority Justification

**HIGH PRIORITY** - Fixes fundamental gameplay issues and sets up deck building

**Why High:**
- Evidence in player deck is actively anti-fun (hurts yourself)
- AI running out of cards reduces strategic depth
- Customer deck boring (no interesting decisions)
- Prerequisite for deck building (need good base decks first)
- Quick win (~4-6 hours) with major impact

**Benefits:**
- Better strategic variety (interesting cards throughout run)
- AI stays engaged longer (more cards to play)
- Customer becomes interesting (can make deals, suggest venues)
- Removes frustration (no self-harming cards)
- Foundation for deck building system

---

## Feasibility Analysis

### Technical Assessment

**Proposed Solution: Expand and Rebalance All Three Decks**

#### Core Mechanism

**Current State:**
- Player: 15 cards (4 Products, 3 Locations, 2 Cover, 2 Insurance, 2 Conviction, 1 Modifier, 1 Evidence)
- Narc: 15 cards (10 Donut Break, 3 Patrol, 2 Surveillance - all Evidence)
- Customer: 10 cards (5 Regular Order, 5 Haggling - placeholder Evidence cards)

**Target State:**
- Player: 20 cards (thematic dealer deck, no Evidence/Conviction)
- Narc: 25 cards (Evidence + Conviction variety)
- Customer: 25 cards (Products + Locations + Deal Modifiers)

**Changes Required:**
1. Move Conviction cards (Warrant, DA Approval) from player_deck to narc_deck
2. Remove Evidence cards (Informant) from player_deck
3. Add 5-6 new player cards (Cover, Deal Modifiers)
4. Add 10 new narc cards (Evidence variety, more Conviction)
5. Completely rebuild customer_deck (25 thematic cards)

**Data Structure Changes:**
- No struct changes needed (decks already `Vec<Card>`)
- Only changes to `create_*_deck()` functions
- Card types already exist (Product, Location, Evidence, Cover, Conviction, Insurance, DealModifier)

#### Performance Projections

**Overhead:**
- Larger decks (70 total cards vs 40) - negligible memory impact
- Shuffle operations still O(n), n=20-25 per deck
- No performance concerns

**Development Time:**
- Card design: 2-3 hours (define 30+ new cards with balanced values)
- Implementation: 1-2 hours (update create_*_deck functions)
- Testing: 1 hour (update card count tests, verify balance)
- **Total: 4-6 hours**

#### Technical Risks

**1. Card Balance**
- *Risk:* New card values might be too strong/weak
- *Mitigation:* Use existing card values as baseline, iterate based on playtesting
- *Impact:* Low - easy to adjust numbers post-merge

**2. Deck Exhaustion Timing**
- *Risk:* Larger decks might make runs too long
- *Mitigation:* 20-25 cards = 6-8 hands (reasonable run length)
- *Impact:* Low - target is good

**3. Customer Strategic Depth**
- *Risk:* Customer playing Products/Locations might feel weird thematically
- *Mitigation:* Frame as "Customer wants X product" or "Customer prefers Y location"
- *Impact:* Low - makes Customer more interesting

### System Integration

**Affected Systems:**
- Deck creation functions (`create_narc_deck`, `create_customer_deck`, `create_player_deck`)
- Card instantiation tests
- No changes to game logic (card types already support this)

**Compatibility:**
- ✅ No breaking changes (just different card distribution)
- ✅ Works with SOW-002 betting (Customer can now raise with Products)
- ✅ Works with SOW-003 insurance (Conviction moves to Narc)
- ✅ Works with SOW-004 card retention (larger decks last longer)

**Integration Points:**
- Override rules: Customer Products/Locations override player's (same as current)
- Totals calculation: No changes needed (already handles all card types)
- AI decision making: No changes needed (AI already plays first card)

### Alternatives Considered

#### Alternative 1: Keep Small Decks, Add Deck Refill Mechanic

**Approach:** Keep 15-card decks, add "reshuffle discard pile" mechanic

**Rejected because:**
- Adds complexity (discard pile tracking, reshuffle trigger)
- Doesn't solve thematic issues (player still has Evidence)
- Doesn't make Customer more interesting
- More work than just adding cards

#### Alternative 2: Asymmetric Deck Sizes (Player 20, AI 30+)

**Approach:** Give AI much larger decks to prevent exhaustion

**Rejected because:**
- Feels unfair (why do they get more cards?)
- Doesn't solve thematic issues
- Symmetric 20-25 cards is cleaner

#### Alternative 3: Keep Evidence in Player Deck, Make it Optional

**Approach:** Evidence cards give bonus cash if played

**Rejected because:**
- Still fundamentally weird (why would dealer help narc?)
- Band-aid on bad design
- Simpler to just remove them

---

## Discussion

### ARCHITECT Notes

**Implementation is Straightforward:**
- No new card types needed (all exist)
- No logic changes (just card data)
- Main work is card design (values, names, balance)

**Card Design Priorities:**
1. **Variety in Evidence** (not all Donut Break)
2. **Customer becomes strategic** (can make interesting plays)
3. **Remove anti-fun** (no self-harming player cards)
4. **Balance deck sizes** (prevent early exhaustion)

**Recommended Card Additions:**

**Player (5 new cards):**
- Cover: Fake Receipts (+20 Cover, +5 Heat)
- Cover: Bribed Witness (+15 Cover, -10 Heat)
- Modifier: Burner Phone (Cover +15, Heat -10)
- Modifier: Lookout (Cover +20, Heat 0)
- Product: Fentanyl ($200, Heat +50 - highest risk/reward)

**Narc (10 new cards):**
- Evidence: Stakeout (+10 Evidence, +3 Heat) ×3
- Evidence: Undercover Op (+30 Evidence, +10 Heat) ×2
- Evidence: Raid (+40 Evidence, +20 Heat) ×1
- Conviction: Warrant ×2 more (total 4)
- Conviction: DA Approval ×1 more (total 3)
- Conviction: RICO Case (threshold 80) ×1

**Customer (15 new cards to replace placeholders):**
- Products: Weed Request, Meth Request, Heroin Request, Cocaine Request, Pills Request
- Locations: Park, Nightclub, Apartment, Office, Alley
- Modifiers: Bulk Order (×1.5 price, +20 Evidence)
- Modifiers: Quick Sale (×0.8 price, -10 Evidence)
- Modifiers: Cash Upfront (×1.2 price, +10 Cover)
- Modifiers: Credit (×0.9 price, +15 Evidence, -5 Heat)
- Plus 6 more variety modifiers

**Thematic Clarity:**
- Customer Products: "Customer wants Cocaine" (overrides your product choice)
- Customer Locations: "Customer suggests Nightclub" (overrides your location)
- This makes sense narratively (customer has preferences)

### PLAYER Validation

**This fixes the major pain points:**
- ✅ No more self-harm (Evidence removed from player)
- ✅ AI stays in game longer (bigger decks)
- ✅ Customer becomes interesting (strategic plays, not just fodder)
- ✅ All decks feel thematic (cards match roles)

**Expected Feel:**
- "Customer wants Cocaine - that's risky but profitable"
- "Customer suggests Nightclub - high evidence location!"
- "Narc played Warrant - I'm at 35 heat, still safe... for now"

---

## Approval

**Status:** Approved

**Approvers:**
- ARCHITECT: ✅ Straightforward implementation, no technical risks
- PLAYER: ✅ Fixes anti-fun mechanics, makes all players interesting

**Scope Constraint:** ✅ Fits in one SOW (4-6 hours)

**Dependencies:**
- Requires SOW-001/002/003/004 (all card types exist)
- No blocking dependencies

**Next Steps:**
1. ARCHITECT creates SOW-005
2. DEVELOPER implements deck rebalance
3. Playtest to verify improved balance and variety

**Date:** 2025-11-09

---

## Discussion

*To be populated during RFC iteration*

---

## Approval

**Status:** Draft

**Approvers:**
- ARCHITECT: [Pending]
- PLAYER: [Pending]

**Date:** 2025-11-09
