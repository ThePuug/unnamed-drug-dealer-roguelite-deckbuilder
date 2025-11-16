# RFC-013: Asset Externalization (RON Data Files)

## Status

**Draft** - 2025-11-16

## Feature Request

### Player Need

From player perspective: **The game content is hardcoded in Rust - I want to mod cards, create custom scenarios, translate text, and iterate on game balance without recompiling.**

**Current Problem:**
Without externalized assets:
- All game content hardcoded in `src/data/*.rs` files
- Can't modify cards, scenarios, or narrative fragments without Rust knowledge
- Can't translate to other languages
- Can't create community mods or content packs
- Designers can't iterate on balance without developer help
- Content changes require full recompilation (slow iteration)
- No clear separation between code (logic) and data (content)

**We need a system that:**
- Externalizes all game content to human-readable data files
- Uses RON format (Rusty Object Notation - readable, supports Rust types)
- Loads assets at runtime with error handling
- Supports hot-reloading during development (future)
- Makes modding straightforward (edit file, restart game)
- Enables translation/localization
- Separates content from code cleanly

### Desired Experience

**For Modders:**
- "I want to create a 'Cyberpunk Hacker' buyer persona → edit `assets/buyers/cyberpunk_hacker.ron`"
- "I want to add 'Bitcoin' as a product → edit `assets/products/bitcoin.ron`"
- "I want custom narrative fragments → edit the RON file, no code changes"
- All content visible, editable, understandable

**For Designers:**
- "Balance tuning: Change Weed price from $30 → $25 → edit `assets/products/weed.ron`"
- "New scenario: Add third scenario to Frat Bro → edit `assets/buyers/frat_bro.ron`"
- No compilation, no Rust knowledge required

**For Translators:**
- "Spanish translation → copy `assets/` to `assets_es/`, translate all strings"
- Clear file structure, easy to find all text

**For Developers:**
- Code focuses on logic/systems, not content
- Content changes don't trigger recompilation
- Asset validation catches errors (missing required fields, invalid values)

### Specification Requirements

**Asset Categories:**
- **Cards:** Products, Locations, Evidence, Cover, Insurance, Conviction, DealModifiers
- **Buyers:** Personas with scenarios and reaction decks
- **Narrative Fragments:** Subject/need/product/location/complication/action clauses
- **Game Config:** (Future) Balance constants, thresholds, starting values

**RON File Structure:**
```
assets/
├── cards/
│   ├── products/
│   │   ├── weed.ron
│   │   ├── codeine.ron
│   │   └── ... (9 total)
│   ├── locations/
│   │   ├── safe_house.ron
│   │   └── ... (10 total)
│   ├── evidence/
│   │   ├── patrol.ron
│   │   └── ... (17 total)
│   ├── cover/
│   ├── insurance/
│   └── modifiers/
├── buyers/
│   ├── frat_bro.ron
│   ├── desperate_housewife.ron
│   └── wall_street_wolf.ron
└── config/
    └── (future: balance.ron, thresholds.ron)
```

**RON Format Example:**
```ron
// assets/cards/products/weed.ron
ProductCard(
    id: 10,
    name: "Weed",
    price: 30,
    heat: 5,
    narrative_fragments: Some(NarrativeFragments(
        product_clauses: [
            "I had the weed",
            "I brought the green",
            "I was holding some bud",
        ],
    )),
)
```

**Loading System:**
- Asset loader on game startup
- Validate all assets (schema, required fields, value ranges)
- Error reporting (missing files, malformed RON, invalid values)
- Asset registry (HashMap of loaded content by ID/name)

**Migration Strategy:**
- Phase 1: Export current content to RON files
- Phase 2: Implement asset loader
- Phase 3: Replace hardcoded data with loaded assets
- Phase 4: Remove `src/data/*.rs` files (content now in assets/)

### MVP Scope

**Phase 1 includes:**
- RON file structure for all card types
- RON files for buyer personas and scenarios
- RON files for narrative fragments
- Asset loader with validation
- Error handling and reporting
- Migration of existing content to RON

**Phase 1 excludes:**
- Hot-reloading (requires file watching, complex)
- Multi-language support (requires localization system)
- Asset editor UI (use text editor for MVP)
- Config files for balance constants (can add later)
- Asset compression or binary formats

### Priority Justification

**HIGH PRIORITY** - Critical for market viability and community engagement

**Why high:**
- **Moddability** is a key selling point for roguelites
- **Community content** extends game life significantly
- **Translation** enables international markets
- **Balance iteration** becomes designer-friendly
- Foundational change - easier to do now than later

**Benefits:**
- Modding community can create content packs
- Designers iterate without developer bottleneck
- Translation becomes feasible
- Content versioning separated from code versioning
- Faster iteration (no recompilation for content changes)

---

## Feasibility Analysis

### Technical Assessment

**Proposed Solution: RON-Based Asset System**

#### Core Mechanism

**1. RON Serialization**
```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ProductCardAsset {
    pub id: u32,
    pub name: String,
    pub price: u32,
    pub heat: i32,
    pub narrative_fragments: Option<NarrativeFragmentsAsset>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct NarrativeFragmentsAsset {
    pub product_clauses: Vec<String>,
    // ... other fragment types
}
```

**2. Asset Loading**
```rust
pub struct AssetRegistry {
    pub products: HashMap<String, Card>,
    pub locations: HashMap<String, Card>,
    pub buyers: HashMap<String, BuyerPersona>,
    // ... other asset types
}

impl AssetRegistry {
    pub fn load() -> Result<Self, AssetError> {
        let products = load_all_products()?;
        let locations = load_all_locations()?;
        let buyers = load_all_buyers()?;

        Ok(Self { products, locations, buyers })
    }
}

fn load_all_products() -> Result<HashMap<String, Card>, AssetError> {
    let mut products = HashMap::new();

    for entry in std::fs::read_dir("assets/cards/products")? {
        let path = entry?.path();
        let content = std::fs::read_to_string(&path)?;
        let asset: ProductCardAsset = ron::from_str(&content)?;

        // Convert asset to Card
        let card = Card::from_asset(asset);
        products.insert(card.name.clone(), card);
    }

    Ok(products)
}
```

**3. Integration with Existing Systems**
```rust
// Replace hardcoded deck creation
pub fn create_player_deck() -> Vec<Card> {
    // OLD: Hardcoded cards
    // NEW: Load from registry
    let registry = AssetRegistry::get(); // Global singleton or resource

    vec![
        registry.products.get("Weed").unwrap().clone(),
        registry.products.get("Codeine").unwrap().clone(),
        // ... all 20 cards
    ]
}
```

#### Performance Projections

**Load Time:**
- ~50 RON files × ~1-2KB each = ~100KB total
- Parse time: ~10-50ms on startup (one-time cost)
- Negligible impact on game startup

**Memory Overhead:**
- Asset structs vs hardcoded: No difference (same data in memory)
- Asset registry HashMap: ~5-10KB overhead
- Total impact: < 1% memory increase

**Development Time:**
- Asset structure definitions: 4-6 hours
- RON file migration (export current content): 6-8 hours
- Asset loader implementation: 6-8 hours
- Error handling and validation: 4-6 hours
- Testing and integration: 4-6 hours
- **Total:** 24-34 hours (will need to split into 2 SOWs)

#### Technical Risks

**1. RON Parsing Errors**
- *Risk:* Malformed RON files crash game on startup
- *Mitigation:* Comprehensive error messages, validation, schema documentation
- *Impact:* Medium - bad UX if modders break game, but fixable with good errors

**2. Asset Schema Evolution**
- *Risk:* Changing Rust types breaks existing RON files
- *Mitigation:* Versioning, migration scripts, backward compatibility
- *Impact:* Medium - manageable with proper versioning

**3. Performance**
- *Risk:* Loading 100+ files at startup could be slow
- *Mitigation:* Profile on target hardware, combine files if needed
- *Impact:* Low - current content size is small

**4. Modding Validation**
- *Risk:* Invalid mod content breaks game balance or logic
- *Mitigation:* Value range validation, required field checks
- *Impact:* Low - can validate on load

### System Integration

**Affected Systems:**
- **Card creation:** `src/data/*.rs` → migrate to RON loading
- **Buyer personas:** `buyer_personas.rs` → `assets/buyers/*.ron`
- **Deck builders:** Load cards from registry instead of hardcoded
- **Narrative system:** Load fragments from RON
- **Game initialization:** Add asset loading phase

**Integration Points:**
- Bevy asset system integration (optional - could use std::fs for MVP)
- Global asset registry (Bevy Resource or lazy_static)
- Error handling during startup
- Asset hot-reloading (future)

**Compatibility:**
- ✅ No breaking changes to game logic
- ✅ Card/Buyer types remain unchanged
- ✅ Just changes where data comes from (code → files)
- ✅ All tests continue to work (use test_helpers, not assets)

**Dependencies:**
- `serde` crate (already used by Bevy)
- `ron` crate (Rusty Object Notation)

### Alternatives Considered

#### Alternative 1: JSON Assets

**Description:**
Use JSON instead of RON for asset files.

**Rejected because:**
- JSON doesn't support Rust enums elegantly
- JSON verbose for deeply nested structures
- RON designed specifically for Rust data
- RON supports comments (better for modding docs)
- Community expects RON for Rust games

#### Alternative 2: TOML Assets

**Description:**
Use TOML for configuration-style data.

**Rejected because:**
- TOML poor for nested structures (cards with fragments)
- TOML doesn't support complex Rust types well
- Less expressive than RON for game data
- Better for config, not content

#### Alternative 3: Keep Content Hardcoded

**Description:**
Don't externalize - keep all content in code.

**Rejected because:**
- Kills moddability (major market differentiator)
- Prevents translation/localization
- Slows content iteration
- Creates developer bottleneck for designers

#### Alternative 4: Custom Binary Format

**Description:**
Design custom binary asset format.

**Rejected because:**
- Not human-readable (kills modding)
- Requires custom tooling (editor, validator)
- Overkill for current content size
- Premature optimization

---

## Discussion

### ARCHITECT Notes - Initial Review

**Strong Points:**
- Clear player/market value (moddability, translation, iteration speed)
- Well-established pattern (many successful games use data files)
- RON is ideal choice for Rust games
- Natural evolution (content outgrowing code)

**Technical Insights:**
- Asset externalization is foundational - should be done before content expansion
- RON schema mirrors existing Rust types (smooth migration)
- Asset loading is one-time startup cost (negligible)
- Enables future features (DLC, content packs, workshop integration)

**Concerns:**
- Development time (24-34 hours) exceeds one SOW (≤20 hours)
- Need to split into 2 SOWs (Phase A + Phase B)
- Migration effort significant but necessary
- Asset validation critical for good mod UX

**Suggested Split:**
- **SOW-013-A** (Foundation): Asset structures, loader, validation, initial migration (12-16 hours)
- **SOW-013-B** (Complete Migration): Migrate all remaining content, remove hardcoded data (12-18 hours)

### Open Questions

**Q1: Asset Organization**
Should we use one RON file per card/buyer, or group related assets?
- **Option A:** One file per asset (`weed.ron`, `codeine.ron`) - granular, easy to find
- **Option B:** Group by category (`products.ron` with all products) - fewer files, bulk editing
- **Recommendation:** Option A (one file per asset) - better for modding, easier to add/remove

**Q2: Asset Loading Strategy**
When should assets be loaded?
- **Option A:** Game startup (all at once) - simple, ensures everything loads
- **Option B:** Lazy loading (on-demand) - faster startup, more complex
- **Recommendation:** Option A (startup) - current content size is small, simplicity preferred

**Q3: Bevy Asset System Integration**
Should we use Bevy's built-in asset system or custom loader?
- **Option A:** Bevy AssetServer (async, handles, change detection)
- **Option B:** Custom std::fs loader (simpler, synchronous)
- **Recommendation:** Option B for MVP (simpler), can upgrade to Bevy later if needed

**Q4: Validation Strictness**
How strict should asset validation be?
- **Option A:** Strict (reject invalid assets, refuse to start)
- **Option B:** Permissive (warn about issues, use defaults)
- **Recommendation:** Option A (strict) - prevents broken game states, clearer errors

---

## Approval

**Status:** Draft (awaiting PLAYER/ARCHITECT review)

**Next Steps:**
1. ARCHITECT/PLAYER discussion on open questions
2. Finalize split into SOW-013-A and SOW-013-B
3. Approval from both roles
4. Create SOW documents

**Date:** 2025-11-16
