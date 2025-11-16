# RFC-012: Narrative Generation System

## Status

**Approved** - 2025-11-16 (PLAYER and ARCHITECT approved, ready for SOW-012)

## Feature Request

### Player Need

From player perspective: **Hands feel like abstract stat calculations - I want each deal to tell a dynamic story based on the cards I played, creating memorable narratives that emerge from gameplay.**

**Current Problem:**
Without narrative generation:
- Hand resolution is just numbers (Evidence: 15, Cover: 20, Heat: 30)
- No flavor text or storytelling - feels mechanical and disconnected
- Can't remember individual hands ("Was that the one where I bailed on the housewife?")
- Missing immersion - where's the drug dealer fantasy?
- Cards are just stat containers, not narrative building blocks
- No sense of how cards interact to create situations
- Resolution feels transactional, not dramatic

**We need a system that:**
- Generates dynamic stories from the cards played in each hand
- Composes complex, grammatically correct sentences from card combinations
- Creates variety through randomized fragment selection
- Supports both atomic (word-level) and phrasal (clause-level) fragments
- Recognizes narrative patterns (simple deal, complicated deal, busted deal, etc.)
- Displays stories as part of the resolution overlay content
- Makes each hand feel unique and memorable
- Adds depth to the drug dealer roleplay fantasy

### Desired Experience

**Emergent Storytelling:**
- "A desperate housewife needed her fix, I had the stuff but when the cops tapped my lines I had no choice but to bail out."
- Story reflects actual cards played (Buyer scenario, Product, Evidence, Action)
- Same cards can tell different stories through randomized fragment selection
- Complex sentence structures feel natural, not template-y

**Memorable Moments:**
- "Remember that hand where the Wall Street Wolf was desperate for ice and I made the deal in a limo?"
- Can recall specific hands by their stories, not just outcomes
- Best/worst stories become conversation pieces
- Turn history shows story log for the entire run

**Immersion and Theme:**
- First-person perspective ("I had the stuff")
- Drug dealer voice and tone
- Cards contribute their personality to the narrative
- Stories reinforce the risk/reward tension of dealing

**Variety and Replayability:**
- Same card combination generates different stories across plays
- Multiple fragment variants per card role
- Pattern matching creates appropriate story structures
- Fallbacks prevent nonsensical or broken sentences

### Specification Requirements

**Fragment System:**
- **Hybrid fragments:** Cards contain both atomic (word-level) and phrasal (clause-level) fragments
- **Atomic fragments:** Pronouns, verbs, objects for maximum composability
  - Example: `pronouns: ["She", "that bitch", "the woman"]`
  - Example: `verbs: ["needed", "wanted", "was jonesing for"]`
  - Example: `objects: ["her fix", "a score", "relief"]`
- **Phrasal fragments:** Complete clauses for natural flow
  - Example: `subject_clauses: ["A desperate housewife", "The soccer mom"]`
  - Example: `need_clauses: ["needed her fix", "was in denial"]`
  - Example: `product_clauses: ["I had the stuff", "I was holding codeine"]`
- **Fragment lists:** Each fragment type supports multiple variants
- **Randomized selection:** Randomly pick from fragment list for variety
- **Fallbacks:** Default text when card lacks specific fragment type
  - Example: If Buyer has no subject fragments, use fallback "They"

**Story Patterns:**
- **Pattern types:** Simple deal, complicated deal, busted deal, insured deal, close call
- **Pattern matching:** Identify which pattern best fits played cards
- **Priority system:** Check specific patterns before generic fallbacks
- **Card requirements:** Each pattern specifies required cards (Buyer, Product, Evidence, etc.)

**Sentence Structures:**
- **Simple:** Subject + Verb + Object
  - "A desperate housewife needed her fix"
- **Compound:** Clause + Conjunction + Clause
  - "I had the stuff, but the heat was on"
- **Complex:** Main Clause + Subordinator + Subordinate Clause
  - "I had the stuff when she called"
- **Compound-Complex:** Multiple clauses with subordination and coordination
  - "She needed her fix when I called, but the cops tapped my lines so I bailed"

**Grammar Elements:**
- **Conjunctions:** and, but, so, yet
- **Subordinators:** when, because, although, if, while, since, after, before
- **Proper punctuation:** Commas before conjunctions, capitalization, periods
- **Natural flow:** Sentences read like natural English, not mad-libs

**Narrative Roles:**
- **Subject:** Buyer persona (provides character)
- **Need:** Buyer scenario motivation (why they want the deal)
- **Product:** What was sold
- **Location:** Where it happened
- **Complication:** Evidence/Heat cards that create tension
- **Action:** Player action cards (Insurance, DealModifier, Bail Out)
- **Resolution:** Outcome (bust, success, close call)

**Display Integration:**
- **Resolution overlay:** Story appears as prominent text in hand resolution overlay (RFC-011)
- **Typography:** Large, readable font with thematic styling
- **Positioning:** Above or below outcome stats, clearly separated
- **Turn history:** Stories logged for each completed hand
- **End-of-run summary:** Highlight best/worst/most memorable stories

**Fragment Authoring:**
- Each card defines its narrative fragments in card data
- 3-5 variants per fragment type for variety
- Empty lists allowed (fallback will be used)
- Fragments authored in first-person player perspective
- Buyer cards provide subject/need fragments
- Product cards provide product fragments
- Evidence cards provide complication fragments
- Action cards provide action/resolution fragments

### MVP Scope

**Phase 1 includes:**
- Core sentence structure system (Simple, Compound, Complex, Compound-Complex)
- Fragment system (atomic + phrasal with fallbacks)
- Pattern matching (3-5 basic patterns)
- Fragment authoring for existing cards (Buyers, Products, Evidence, Actions)
- Story composition engine
- Display integration in resolution overlay

**Phase 1 excludes:**
- Advanced contextual fragments (heat-based modifiers, conditional text)
- Voice/tone variation by buyer persona
- Story history UI with filtering/search
- Achievement/milestone story tracking ("Best story" awards)
- Bust-specific story variations (third-person narc perspective)
- Multi-language support
- Story export/sharing features

### Priority Justification

**MEDIUM PRIORITY** - Enhances immersion and replayability, but not core gameplay

**Why medium:**
- Adds significant player value (immersion, memorability, theme)
- Doesn't affect core mechanics or balance
- Can be implemented independently of other systems
- Complements resolution overlay work (RFC-011)
- Medium complexity - requires content authoring but system is straightforward

**Benefits:**
- **Immersion:** Transforms mechanical resolution into narrative experience
- **Memorability:** Individual hands become stories, not just stat outcomes
- **Replayability:** Same cards tell different stories across plays
- **Theme reinforcement:** Strengthens drug dealer roleplay fantasy
- **Content foundation:** Fragment system enables future narrative expansions
- **Player engagement:** Stories are shareable, discussable, memorable

---

## Feasibility Analysis

### Technical Assessment

**Proposed Solution: Compositional Grammar-Based Narrative Engine**

#### Core Mechanism

**1. Fragment Storage**
```rust
/// Narrative fragments attached to cards
#[derive(Debug, Clone, Default)]
pub struct NarrativeFragments {
    // ATOMIC FRAGMENTS (word/phrase level)
    pub pronouns: Vec<String>,
    pub verbs: Vec<String>,
    pub objects: Vec<String>,

    // PHRASAL FRAGMENTS (clause level)
    pub subject_clauses: Vec<String>,
    pub need_clauses: Vec<String>,
    pub product_clauses: Vec<String>,
    pub location_clauses: Vec<String>,
    pub complication_clauses: Vec<String>,
    pub action_clauses: Vec<String>,
}

/// Extended Card struct
pub struct Card {
    // ... existing fields
    pub narrative_fragments: Option<NarrativeFragments>,
}
```

**2. Sentence Structure Definition**
```rust
/// Linguistic sentence patterns
#[derive(Debug, Clone)]
pub enum SentenceStructure {
    Simple {
        subject: FragmentSlot,
        verb: FragmentSlot,
        object: FragmentSlot,
    },
    Compound {
        clause1: Box<SentenceStructure>,
        conjunction: ConjunctionType,
        clause2: Box<SentenceStructure>,
    },
    Complex {
        main_clause: Box<SentenceStructure>,
        subordinator: SubordinatorType,
        subordinate_clause: Box<SentenceStructure>,
    },
    CompoundComplex {
        clause1: Box<SentenceStructure>,
        subordinator: SubordinatorType,
        subordinate: Box<SentenceStructure>,
        conjunction: ConjunctionType,
        clause2: Box<SentenceStructure>,
    },
}

#[derive(Debug, Clone, Copy)]
pub enum ConjunctionType { And, But, So, Yet }

#[derive(Debug, Clone, Copy)]
pub enum SubordinatorType { When, Because, Although, If, While, Since, After, Before }
```

**3. Story Pattern Matching**
```rust
/// Narrative pattern with requirements
#[derive(Debug, Clone)]
pub struct StoryPattern {
    pub pattern_id: &'static str,
    pub pattern_type: PatternType,
    pub priority: u32,
    pub required_cards: Vec<CardRequirement>,
    pub sentence_structure: SentenceStructure,
}

#[derive(Debug, Clone, PartialEq)]
pub enum PatternType {
    SimpleDeal,           // Buyer + Product + Location
    ComplicatedDeal,      // + Evidence/Heat complications
    BustedDeal,           // + Conviction triggered
    InsuredDeal,          // + Insurance activated
    CloseCall,            // High heat, barely succeeded
    GenericTransaction,   // Fallback pattern
}
```

**4. Story Composition**
```rust
pub struct StoryComposer {
    patterns: Vec<StoryPattern>,
}

impl StoryComposer {
    pub fn compose_story(&self, hand_state: &HandState) -> String {
        // 1. Find best matching pattern (by priority)
        let pattern = self.match_pattern(hand_state);

        // 2. Build fragment context from played cards
        let context = FragmentContext::from_hand_state(hand_state);

        // 3. Recursively assemble sentence structure
        let sentence = self.assemble_structure(&pattern.sentence_structure, &context);

        // 4. Capitalize, punctuate, finalize
        Self::finalize_sentence(sentence)
    }

    fn assemble_structure(&self, structure: &SentenceStructure, context: &FragmentContext)
        -> String
    {
        match structure {
            SentenceStructure::Simple { subject, verb, object } => {
                format!("{} {} {}",
                    self.pick_fragment(subject, context),
                    self.pick_fragment(verb, context),
                    self.pick_fragment(object, context)
                )
            },

            SentenceStructure::Compound { clause1, conjunction, clause2 } => {
                format!("{}, {} {}",
                    self.assemble_structure(clause1, context),
                    conjunction.as_str(),
                    self.assemble_structure(clause2, context)
                )
            },

            SentenceStructure::Complex { main_clause, subordinator, subordinate_clause } => {
                format!("{} {} {}",
                    self.assemble_structure(main_clause, context),
                    subordinator.as_str(),
                    self.assemble_structure(subordinate_clause, context)
                )
            },

            SentenceStructure::CompoundComplex { clause1, subordinator, subordinate, conjunction, clause2 } => {
                format!("{} {} {}, {} {}",
                    self.assemble_structure(clause1, context),
                    subordinator.as_str(),
                    self.assemble_structure(subordinate, context),
                    conjunction.as_str(),
                    self.assemble_structure(clause2, context)
                )
            },
        }
    }
}
```

**5. Fragment Selection with Fallbacks**
```rust
/// Context extracted from played cards
pub struct FragmentContext {
    buyer_card: Option<Card>,
    product_card: Option<Card>,
    location_card: Option<Card>,
    evidence_cards: Vec<Card>,
    action_cards: Vec<Card>,
    // Cache for random selections (consistency within one story)
    fragment_cache: HashMap<(NarrativeRole, FragmentLevel), String>,
}

impl FragmentContext {
    pub fn get_phrasal(&self, role: NarrativeRole) -> Option<String> {
        // Get card for role
        let card = self.card_for_role(role)?;

        // Extract phrasal fragments
        let fragments = card.narrative_fragments.as_ref()?;
        let list = match role {
            NarrativeRole::BuyerSubject => &fragments.subject_clauses,
            NarrativeRole::BuyerNeed => &fragments.need_clauses,
            NarrativeRole::ProductHave => &fragments.product_clauses,
            NarrativeRole::Complication => &fragments.complication_clauses,
            NarrativeRole::Action => &fragments.action_clauses,
            _ => return None,
        };

        // Randomly pick from list (returns None if empty, triggering fallback)
        if list.is_empty() {
            None
        } else {
            Some(list.choose(&mut rand::thread_rng()).unwrap().clone())
        }
    }
}

/// Fragment slot with fallback
#[derive(Debug, Clone)]
pub struct FragmentSlot {
    pub role: NarrativeRole,
    pub level: FragmentLevel,  // Atomic, Phrasal, or Either
    pub fallback: String,       // Used if card lacks fragments
}
```

#### Example Pattern Definition

```rust
// Pattern: Complicated Deal with Bailout
// "A desperate housewife needed her fix, I had the stuff but when the cops tapped my lines I had no choice but to bail out"
StoryPattern {
    pattern_id: "complicated_deal_bailout",
    pattern_type: PatternType::ComplicatedDeal,
    priority: 90,
    required_cards: vec![
        CardRequirement::buyer(),
        CardRequirement::product(),
        CardRequirement::evidence(),
        CardRequirement::action_modifier(),
    ],
    sentence_structure: SentenceStructure::CompoundComplex {
        // "A desperate housewife needed her fix"
        clause1: Box::new(SentenceStructure::Simple {
            subject: FragmentSlot {
                role: NarrativeRole::BuyerSubject,
                level: FragmentLevel::Phrasal,
                fallback: "A buyer".to_string(),
            },
            verb: FragmentSlot {
                role: NarrativeRole::BuyerVerb,
                level: FragmentLevel::Atomic,
                fallback: "wanted".to_string(),
            },
            object: FragmentSlot {
                role: NarrativeRole::BuyerObject,
                level: FragmentLevel::Atomic,
                fallback: "a deal".to_string(),
            },
        }),
        subordinator: SubordinatorType::When,
        // "when the cops tapped my lines"
        subordinate: Box::new(SentenceStructure::Phrasal {
            clause: FragmentSlot {
                role: NarrativeRole::Complication,
                level: FragmentLevel::Phrasal,
                fallback: "things got risky".to_string(),
            },
        }),
        conjunction: ConjunctionType::So,
        // "I had no choice but to bail out"
        clause2: Box::new(SentenceStructure::Phrasal {
            clause: FragmentSlot {
                role: NarrativeRole::Action,
                level: FragmentLevel::Phrasal,
                fallback: "I made a choice".to_string(),
            },
        }),
    },
}
```

#### Example Fragment Authoring

```rust
// Desperate Housewife (Buyer)
Card {
    id: 101,
    name: "Desperate Housewife",
    narrative_fragments: Some(NarrativeFragments {
        // ATOMIC
        pronouns: vec!["She".into(), "That woman".into(), "The housewife".into()],
        verbs: vec!["needed".into(), "was desperate for".into(), "was jonesing for".into()],
        objects: vec!["her fix".into(), "relief".into(), "a score".into()],

        // PHRASAL
        subject_clauses: vec![
            "A desperate housewife".into(),
            "The soccer mom".into(),
            "A suburban mother at rock bottom".into(),
        ],
        need_clauses: vec![
            "needed her fix".into(),
            "was in denial about her problem".into(),
            "couldn't go another day without relief".into(),
        ],

        ..Default::default()
    }),
}

// Codeine (Product)
Card {
    id: 201,
    name: "Codeine",
    narrative_fragments: Some(NarrativeFragments {
        product_clauses: vec![
            "I had the stuff".into(),
            "I was holding codeine".into(),
            "I brought prescription pills".into(),
        ],
        ..Default::default()
    }),
}

// Wiretap (Evidence)
Card {
    id: 301,
    name: "Wiretap",
    narrative_fragments: Some(NarrativeFragments {
        complication_clauses: vec![
            "the cops tapped my lines".into(),
            "the feds were listening in".into(),
            "my phone was compromised".into(),
        ],
        ..Default::default()
    }),
}

// Bail Out (Action)
Card {
    id: 401,
    name: "Bail Out",
    narrative_fragments: Some(NarrativeFragments {
        action_clauses: vec![
            "I had no choice but to bail out".into(),
            "I called off the deal".into(),
            "I ghosted them".into(),
        ],
        ..Default::default()
    }),
}
```

#### Performance Projections

**Memory Overhead:**
- `NarrativeFragments` per card: ~500-1000 bytes (7 Vec<String> with 3-5 entries each)
- Total for 50 cards: ~25-50 KB
- Negligible impact on overall game memory

**Runtime Overhead:**
- Story composition: 1-2ms per hand resolution (pattern matching + string assembly)
- Occurs only at resolution (not per-frame)
- Negligible impact on frame rate

**Development Time:**
- System implementation: 8-12 hours
- Fragment authoring (50 cards × 5 fragments): 6-10 hours
- Testing and iteration: 4-6 hours
- **Total:** 18-28 hours (slightly over one SOW, may need split or scope reduction)

#### Technical Risks

**1. Fragment Authoring Burden**
- *Risk:* Writing 3-5 variants per fragment type per card is time-consuming
- *Mitigation:* Start with 2 variants minimum, expand over time; prioritize key cards (Buyers, top Products)
- *Impact:* Medium - affects content quality but not system functionality

**2. Grammar Complexity**
- *Risk:* Complex sentence structures may produce awkward or ungrammatical sentences
- *Mitigation:* Extensive testing with varied card combinations; fallback to simpler patterns
- *Impact:* Medium - worst case is slightly awkward stories, not broken gameplay

**3. Fragment Coherence**
- *Risk:* Random fragment selection may create nonsensical combinations
- *Mitigation:* Careful fragment authoring with compatible tone/tense; pattern constraints
- *Impact:* Low - fragments written for their role should naturally compose

**4. Pattern Coverage**
- *Risk:* Unusual card combinations may not match any pattern
- *Mitigation:* Generic fallback pattern handles all cases; expand patterns over time
- *Impact:* Low - system degrades gracefully to basic story

### System Integration

**Affected Systems:**
- **Card data model:** Add `narrative_fragments: Option<NarrativeFragments>` field
- **Hand state:** Add `hand_story: Option<String>` field to store generated story
- **Resolution overlay (RFC-011):** Display story text in overlay UI
- **Turn history:** Store stories for completed hands

**Integration Points:**
- `HandState::resolve()` - Generate story after calculating totals
- Resolution overlay rendering - Display `hand_state.hand_story`
- Card data files (buyer_personas.rs, player_deck.rs, narc_deck.rs) - Add fragment definitions

**Compatibility:**
- ✅ No breaking changes to existing card mechanics
- ✅ Fragment field is optional - cards without fragments still work
- ✅ Story generation isolated from game logic
- ✅ Display integration non-invasive (just additional UI element)

**Dependencies:**
- **Requires:** RFC-011 (Resolution Overlay) for optimal display integration
- **Complements:** RFC-010 (Buyer Scenarios) - scenario names/motivations perfect for fragments
- **Future:** Could integrate with achievement system (track memorable stories)

### Alternatives Considered

#### Alternative 1: Simple Template System

**Description:**
Use simple string templates with placeholder substitution.
```rust
template: "{buyer} wanted {product}, I had it but {evidence} so {action}"
```

**Rejected because:**
- Limited grammatical variety - all stories follow same structure
- No support for complex clauses or subordination
- Feels mechanical and repetitive
- Can't express conditional sentence structures
- Doesn't support atomic vs phrasal fragments

#### Alternative 2: Procedural Text Generation (Tracery/Grammar)

**Description:**
Use formal grammar system (like Tracery) with production rules.

**Rejected because:**
- Overkill for our needs - too much abstraction
- Requires learning/maintaining grammar DSL
- Harder to author content (grammar rules vs simple fragment lists)
- Less control over narrative voice/tone
- Unnecessary complexity for single-use case

#### Alternative 3: Neural/LLM-Based Generation

**Description:**
Use language model to generate stories from card data.

**Rejected because:**
- External dependency (API calls or model hosting)
- Unpredictable output (may generate inappropriate content)
- Performance concerns (latency, cost)
- Overkill - don't need AI for structured narrative composition
- Loses thematic consistency (harder to control tone)

#### Alternative 4: Hand-Authored Stories per Combination

**Description:**
Pre-write stories for each possible card combination.

**Rejected because:**
- Combinatorial explosion (50 cards = thousands of combinations)
- No variety (same combo = same story every time)
- Inflexible - adding new cards requires authoring all new combinations
- Massive content burden
- Doesn't support emergent gameplay

---

## Discussion

### ARCHITECT Notes - Initial Review

**Strong Points:**
- Clear player value (immersion, memorability, theme reinforcement)
- Well-scoped technical approach (compositional grammar)
- Natural extension of existing card system
- Complements RFC-011 (resolution overlay) perfectly
- Degrades gracefully (fallbacks for missing fragments)

**Technical Insights:**
- Recursive sentence assembly is elegant and extensible
- Hybrid atomic/phrasal fragments provide flexibility
- Pattern priority system allows sophisticated matching
- Fragment caching ensures consistency within one story
- First-person perspective creates player agency

**Concerns:**
- Development time estimate (18-28 hours) exceeds one SOW (≤20 hours)
- Fragment authoring is substantial content work
- Grammar complexity risk needs mitigation plan
- May need to reduce scope for MVP

**Suggested Scope Reduction (if needed):**
1. **Start with phrasal-only fragments** (defer atomic fragments to Phase 2)
   - Reduces fragment types per card (4-5 instead of 7+)
   - Simplifies composition logic
   - Still achieves core goal of dynamic stories
2. **Implement 3 core patterns** (SimpleDeal, ComplicatedDeal, GenericFallback)
   - Covers 80% of hands
   - Can expand pattern library post-MVP
3. **Author 2 variants per fragment** (not 3-5)
   - Reduces authoring time by ~40%
   - Still provides variety

**With scope reduction: 12-18 hours (fits one SOW)**

### PLAYER Validation

**How this meets player needs:**
- ✅ Transforms mechanical resolution into narrative experience
- ✅ Creates memorable, shareable moments
- ✅ Strengthens drug dealer roleplay theme
- ✅ Adds replayability through story variety
- ✅ Makes cards feel like narrative building blocks

**Success Criteria:**
After playing 20 hands with narrative system:
1. Can I describe a specific hand by its story? ("The one where I bailed on the housewife")
2. Do stories feel coherent and natural? (Not mad-libs or nonsensical)
3. Is there enough variety? (Same cards don't always tell same story)
4. Does it enhance immersion? (Feel more like a drug dealer)
5. Are stories memorable? (Can recall favorite/worst stories)

**If 4+ YES → Feature succeeds**

---

## Discussion

### Scope Resolution - 2025-11-16

**PLAYER Decision:**
Accept RFC as-is with full scope (18-28 hour estimate). Willing to split into two SOWs if needed.

**ARCHITECT Decision:**
Recommend **phrasal-only MVP** to fit one SOW:
- Defer atomic fragments to Phase 2 (future RFC)
- Implement 3 core patterns (SimpleDeal, ComplicatedDeal, GenericFallback)
- Author 2-3 variants per fragment type
- **Estimated time: 14-18 hours** (fits one SOW)

**Consensus:**
✅ Proceed with phrasal-only MVP for SOW-012
- Phase 1 excludes atomic fragments (deferred)
- 3 core patterns for MVP
- 2-3 fragment variants per card
- Atomic fragment system can be added in future RFC if needed

---

## Approval

**Status:** ✅ **Approved** - 2025-11-16

**PLAYER:** ✅ Approved
- Solves player need (immersion, memorability, storytelling)
- Phrasal-only MVP still achieves core goals
- Atomic fragments can wait for Phase 2 if needed
- Success criteria remain valid

**ARCHITECT:** ✅ Approved
- Technically feasible (compositional grammar system)
- Scope contained (14-18 hours with phrasal-only)
- Low technical risk with proper fallbacks
- Clean integration with RFC-011 (resolution overlay)
- Fragment authoring is manageable

**Scope Constraint:** ✅ Fits in one SOW (14-18 hours)

**Dependencies:**
- RFC-011 (Resolution Overlay) - provides display integration point
- RFC-010 (Buyer Scenarios) - provides rich buyer personality content

**Next Steps:**
1. ✅ ARCHITECT creates SOW-012
2. DEVELOPER implements per SOW-012
3. ARCHITECT reviews implementation
4. Merge to main and update feature matrices

**Date:** 2025-11-16
