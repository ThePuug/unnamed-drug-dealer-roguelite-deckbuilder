// SOW-014: Dynamic Narrative Construction
// Defines dynamic patterns that build sentences at runtime

use super::builder::{SentenceBuilder, Satellite, Placement};
use super::fragments::{SentenceStructure, ClauseRelation};
use crate::models::{card::{Card, CardType}, hand_state::HandOutcome};

/// Dynamic story pattern that builds structure at runtime
pub struct DynamicPattern {
    pub pattern_id: &'static str,
    pub priority: u32,
    pub required_outcome: Option<HandOutcome>,
    pub required_cards: Vec<CardRequirement>,
    // Closure that takes a builder and configures it
    pub builder_factory: Box<dyn Fn() -> SentenceBuilder + Send + Sync>,
}

impl std::fmt::Debug for DynamicPattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DynamicPattern")
            .field("pattern_id", &self.pattern_id)
            .field("priority", &self.priority)
            .field("required_outcome", &self.required_outcome)
            .finish()
    }
}

/// Card requirement for pattern matching
#[derive(Debug, Clone)]
pub struct CardRequirement {
    pub role: NarrativeRole,
    pub card_type_filter: Option<CardTypeFilter>,
}

impl CardRequirement {
    pub fn buyer() -> Self {
        Self {
            role: NarrativeRole::BuyerSubject,
            card_type_filter: None,
        }
    }

    pub fn product() -> Self {
        Self {
            role: NarrativeRole::Product,
            card_type_filter: Some(CardTypeFilter::Product),
        }
    }

    pub fn location() -> Self {
        Self {
            role: NarrativeRole::Location,
            card_type_filter: Some(CardTypeFilter::Location),
        }
    }

    pub fn evidence() -> Self {
        Self {
            role: NarrativeRole::Evidence,
            card_type_filter: Some(CardTypeFilter::Evidence),
        }
    }
}

/// Filter for card types
#[derive(Debug, Clone, Copy)]
pub enum CardTypeFilter {
    Product,
    Location,
    Evidence,
}

impl CardTypeFilter {
    pub fn matches(&self, card_type: &CardType) -> bool {
        match (self, card_type) {
            (Self::Product, CardType::Product { .. }) => true,
            (Self::Location, CardType::Location { .. }) => true,
            (Self::Evidence, CardType::Evidence { .. }) | (Self::Evidence, CardType::Conviction { .. }) => true,
            _ => false,
        }
    }
}

/// Narrative role that a card plays in the story
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NarrativeRole {
    BuyerSubject,   // "A desperate housewife"
    BuyerNeed,      // "needed her fix"
    Product,        // "I had the stuff"
    Location,       // "at the park"
    Evidence,       // "the cops tapped my lines" (from Evidence cards)
    Resolution,     // "and we made the deal" / "but I got pinched"
}

impl DynamicPattern {
    /// Check if this pattern matches the given cards
    pub fn matches(&self, has_buyer: bool, played_cards: &[Card]) -> bool {
        for requirement in &self.required_cards {
            let has_match = match requirement.role {
                NarrativeRole::BuyerSubject | NarrativeRole::BuyerNeed => {
                    has_buyer
                },
                _ => {
                    if let Some(filter) = &requirement.card_type_filter {
                        played_cards.iter().any(|card| filter.matches(&card.card_type))
                    } else {
                        false
                    }
                }
            };

            if !has_match {
                return false;
            }
        }
        true
    }

    /// Build a sentence structure using this pattern's factory
    pub fn build_structure(&self) -> SentenceStructure {
        let builder = (self.builder_factory)();
        builder.build()
    }

    /// Create all dynamic patterns
    pub fn create_all_patterns() -> Vec<DynamicPattern> {
        vec![
            Self::pattern_complicated_deal(),
            Self::pattern_simple_bust(),
            Self::pattern_simple_buyer_bail(),
            Self::pattern_simple_dealer_bail(),
            Self::pattern_simple_invalid_deal(),
            Self::pattern_simple_deal(),
        ]
    }

    // --- Pattern Definitions ---

    fn pattern_simple_deal() -> DynamicPattern {
        DynamicPattern {
            pattern_id: "simple_deal",
            priority: 50,
            required_outcome: Some(HandOutcome::Safe),
            required_cards: vec![
                CardRequirement::buyer(),
                CardRequirement::product(),
            ],
            builder_factory: Box::new(|| {
                // Core: Subject + Need + Product + Resolution (Safe outcome)
                // Optional: Location (no conjunction - prepositional phrase)
                SentenceBuilder::new()
                    .with_satellite(Satellite::new(NarrativeRole::Location)
                        .with_placement(Placement::Start, 0.3)
                        .with_placement(Placement::End, 0.7)
                        .optional(0.4))
            }),
        }
    }

    fn pattern_complicated_deal() -> DynamicPattern {
        DynamicPattern {
            pattern_id: "complicated_deal",
            priority: 90,
            required_outcome: Some(HandOutcome::Safe),
            required_cards: vec![
                CardRequirement::buyer(),
                CardRequirement::product(),
                CardRequirement::evidence(), // Requires evidence/complication
            ],
            builder_factory: Box::new(|| {
                // Core: Subject + Need + Product + Resolution (Safe outcome)
                // Required: Evidence (complications from evidence cards)
                // Optional: Location
                SentenceBuilder::new()
                    .with_satellite(Satellite::new(NarrativeRole::Evidence)
                        .with_placement(Placement::Start, 0.4)
                        .with_placement(Placement::BeforeResolution, 0.6)
                        .with_conjunction(ClauseRelation::Although))  // Use "although" conjunction
                    .with_satellite(Satellite::new(NarrativeRole::Location)
                        .with_placement(Placement::Start, 1.0)
                        .optional(0.2))  // Only at start to avoid collision with complication
            }),
        }
    }

    fn pattern_simple_bust() -> DynamicPattern {
        DynamicPattern {
            pattern_id: "simple_bust",
            priority: 60,
            required_outcome: Some(HandOutcome::Busted),
            required_cards: vec![CardRequirement::buyer()],
            builder_factory: Box::new(|| {
                // Core: Subject + Need + Product + Resolution (Busted)
                // Optional: Evidence (what led to bust), Location
                SentenceBuilder::new()
                    .with_product_conjunction(ClauseRelation::But)  // Negative outcome
                    .with_satellite(Satellite::new(NarrativeRole::Evidence)
                        .with_placement(Placement::BeforeResolution, 1.0)
                        .with_conjunction(ClauseRelation::Because)  // "because" for bust explanations
                        .optional(0.6))
                    .with_satellite(Satellite::new(NarrativeRole::Location)
                        .with_placement(Placement::Start, 1.0)
                        .optional(0.2))  // Only at start to avoid collision
            }),
        }
    }

    fn pattern_simple_buyer_bail() -> DynamicPattern {
        DynamicPattern {
            pattern_id: "simple_buyer_bail",
            priority: 60,
            required_outcome: Some(HandOutcome::BuyerBailed),
            required_cards: vec![CardRequirement::buyer()],
            builder_factory: Box::new(|| {
                // Core: Subject + Need + Product + Resolution (Buyer bailed)
                // Optional: Location, Evidence (why they bailed)
                SentenceBuilder::new()
                    .with_product_conjunction(ClauseRelation::But)  // Negative outcome
                    .with_satellite(Satellite::new(NarrativeRole::Location)
                        .with_placement(Placement::Start, 1.0)
                        .optional(0.2))
                    .with_satellite(Satellite::new(NarrativeRole::Evidence)
                        .with_placement(Placement::BeforeResolution, 1.0)
                        .with_conjunction(ClauseRelation::Because)  // "because they bailed"
                        .optional(0.5))
            }),
        }
    }

    fn pattern_simple_dealer_bail() -> DynamicPattern {
        DynamicPattern {
            pattern_id: "simple_dealer_bail",
            priority: 60,
            required_outcome: Some(HandOutcome::Folded),
            required_cards: vec![CardRequirement::buyer()],
            builder_factory: Box::new(|| {
                // Core: Subject + Need + Product + Resolution (I walked away)
                // Optional: Location, Evidence (why I walked)
                SentenceBuilder::new()
                    .with_product_conjunction(ClauseRelation::But)  // Negative outcome
                    .with_satellite(Satellite::new(NarrativeRole::Location)
                        .with_placement(Placement::Start, 1.0)
                        .optional(0.2))
                    .with_satellite(Satellite::new(NarrativeRole::Evidence)
                        .with_placement(Placement::BeforeResolution, 1.0)
                        .with_conjunction(ClauseRelation::Because)  // "because I walked"
                        .optional(0.6))
            }),
        }
    }

    fn pattern_simple_invalid_deal() -> DynamicPattern {
        DynamicPattern {
            pattern_id: "simple_invalid_deal",
            priority: 60,
            required_outcome: Some(HandOutcome::InvalidDeal),
            required_cards: vec![CardRequirement::buyer()],
            builder_factory: Box::new(|| {
                // Core: Subject + Need + Product + Resolution (Deal fell through)
                // Optional: Location
                SentenceBuilder::new()
                    .with_product_conjunction(ClauseRelation::But)  // Negative outcome
                    .with_satellite(Satellite::new(NarrativeRole::Location)
                        .with_placement(Placement::Start, 0.4)
                        .with_placement(Placement::End, 0.6)
                        .optional(0.3))
            }),
        }
    }
}
