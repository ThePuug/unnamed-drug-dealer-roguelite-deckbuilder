# RFC-014: Dynamic Narrative Construction

## Status

**Approved** - 2025-11-19

## Feature Request

### Player Need

**Problem:** The current narrative system (RFC-012) uses hardcoded sentence templates (e.g., `variant_a`, `variant_b`). While this provides some variety, it scales poorly. Adding new narrative elements (like a "Weather" card or a "Time of Day" modifier) requires manually authoring new sentence templates for every combination. Players will eventually recognize the patterns ("Oh, it's the 'Clause A + Clause B' template again").

**Desired Experience:**
- **Infinite Variety:** The system should dynamically construct sentences based on available narrative "ingredients" (Subject, Need, Product, Location, Complication, etc.).
- **Organic Flow:** Sentences should feel natural and varied in structure (e.g., sometimes starting with the location, sometimes ending with it).
- **Scalability:** Adding a new narrative role (e.g., "Witness") should automatically integrate into stories without writing new templates.

### Priority Justification

**High Priority** (Refactor). We are currently implementing the narrative system. It is better to switch to a dynamic architecture *now* before we author hundreds of static templates.

## Feasibility Analysis

### Technical Assessment

**Proposed Solution: Dynamic Sentence Builder**

Instead of selecting a pre-defined `SentenceStructure` tree, we will implement a `SentenceBuilder` that constructs the tree at runtime.

**Key Concepts:**
1.  **Core Kernel:** Every story has a mandatory core (usually Subject + Verb + Object/Need).
2.  **Satellites:** Optional elements (Location, Complication, Resolution, Time, etc.) that attach to the core.
3.  **Attachment Rules:** Each satellite type has rules for where it can attach (Start, End, Wrap, Conjunction).

**Algorithm:**
1.  **Identify Ingredients:** Collect all available narrative fragments from played cards.
2.  **Build Core:** Create the base sentence (e.g., "The junkie needed a fix").
3.  **Attach Satellites:** Iterate through available satellites (Location, Complication, etc.).
    - For each satellite, pick a valid attachment point (e.g., Location can go at Start "At the park,..." or End "...at the park").
    - Use a random seed to decide placement for variety.
4.  **Finalize:** Convert the dynamic tree into the final string.

**Example:**
- **Ingredients:** Buyer (Junkie), Product (Heroin), Location (Alley), Complication (Cops).
- **Core:** "The junkie needed heroin."
- **Step 1 (Location):** Randomly choose 'Start'. -> "In the alley, the junkie needed heroin."
- **Step 2 (Complication):** Randomly choose 'Conjunction(But)'. -> "In the alley, the junkie needed heroin, but the cops were watching."

### Technical Risks
- **Grammar Complexity:** Ensuring correct punctuation and capitalization when reordering clauses.
- **Coherence:** Preventing nonsensical combinations (though this is largely a content authoring issue).

## Discussion

### ARCHITECT Notes
This approach is superior to the static template approach in RFC-012. It moves the complexity from *content authoring* (writing templates) to *system logic* (the builder). This is a good trade-off for a roguelite where replayability is key.

**Refactoring Plan:**
- Replace `StoryPattern`'s hardcoded `sentence_structure` with a `DynamicPattern` strategy.
- Update `StoryComposer` to use the builder.
