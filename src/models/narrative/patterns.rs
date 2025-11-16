// SOW-012 Phase 2: Story Pattern System
// Pattern matching for selecting appropriate story structures based on played cards

use super::fragments::{SentenceStructure, FragmentSlot};
use crate::models::{card::{Card, CardType}, hand_state::HandOutcome};

/// Story pattern types (outcome-specific patterns)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PatternType {
    SimpleDeal,           // Buyer + Product (Safe outcome)
    ComplicatedDeal,      // + Evidence/complications
    SimpleBust,           // Busted outcome
    SimpleBuyerBail,      // BuyerBailed outcome
    SimpleDealerBail,     // Folded outcome (dealer/player folded)
    SimpleInvalidDeal,    // InvalidDeal outcome
}

/// Story pattern with matching rules and sentence structure(s)
#[derive(Debug, Clone)]
pub struct StoryPattern {
    pub pattern_id: &'static str,
    pub pattern_type: PatternType,
    pub priority: u32,  // Higher = checked first
    pub required_cards: Vec<CardRequirement>,
    pub required_outcome: Option<super::super::hand_state::HandOutcome>, // Pattern only applies to specific outcome
    pub sentence_structures: Vec<SentenceStructure>, // Multiple variants - randomly selected
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
            card_type_filter: None, // Buyer cards identified by owner, not CardType
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
            role: NarrativeRole::Complication,
            card_type_filter: Some(CardTypeFilter::Evidence),
        }
    }

    pub fn action() -> Self {
        Self {
            role: NarrativeRole::Action,
            card_type_filter: Some(CardTypeFilter::ActionOrModifier),
        }
    }
}

/// Filter for card types
#[derive(Debug, Clone, Copy)]
pub enum CardTypeFilter {
    Product,
    Location,
    Evidence,
    ActionOrModifier, // DealModifier, Insurance, Cover
}

impl CardTypeFilter {
    pub fn matches(&self, card_type: &CardType) -> bool {
        match (self, card_type) {
            (Self::Product, CardType::Product { .. }) => true,
            (Self::Location, CardType::Location { .. }) => true,
            (Self::Evidence, CardType::Evidence { .. }) | (Self::Evidence, CardType::Conviction { .. }) => true,
            (Self::ActionOrModifier, CardType::DealModifier { .. }) |
            (Self::ActionOrModifier, CardType::Insurance { .. }) |
            (Self::ActionOrModifier, CardType::Cover { .. }) => true,
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
    Complication,   // "the cops tapped my lines"
    Action,         // "I had no choice but to bail"
    Resolution,     // "and we made the deal" / "but I got pinched"
}

impl StoryPattern {
    /// Check if this pattern matches the given cards
    pub fn matches(&self, buyer_card: Option<&Card>, played_cards: &[Card]) -> bool {
        for requirement in &self.required_cards {
            let has_match = match requirement.role {
                NarrativeRole::BuyerSubject | NarrativeRole::BuyerNeed => {
                    // Buyer requirements check for buyer_card existence
                    buyer_card.is_some()
                },
                _ => {
                    // Other requirements check played_cards
                    if let Some(filter) = &requirement.card_type_filter {
                        played_cards.iter().any(|card| filter.matches(&card.card_type))
                    } else {
                        false
                    }
                }
            };

            if !has_match {
                return false; // Missing required card
            }
        }

        true // All requirements met
    }

    /// Create all story patterns for MVP (sorted by priority: highest first)
    pub fn create_all_patterns() -> Vec<StoryPattern> {
        vec![
            // Priority 90
            Self::pattern_complicated_deal(), // Has 2 structure variants
            // Priority 60 (outcome-specific - higher than SimpleDeal)
            Self::pattern_simple_bust(),
            Self::pattern_simple_buyer_bail(),
            Self::pattern_simple_dealer_bail(),
            Self::pattern_simple_invalid_deal(),
            // Priority 50 (generic - catches all)
            Self::pattern_simple_deal(),
        ]
    }

    /// Pattern: Complicated Deal (2 structure variants - randomly selected)
    /// Variant A: "Although things got risky, still we made the deal"
    /// Variant B: "Things got risky, and yet we made the deal"
    fn pattern_complicated_deal() -> StoryPattern {
        use super::fragments::{ConjunctionType, SubordinatorType};

        let variant_a = SentenceStructure::ReversedComplex {
            subordinator: SubordinatorType::Although,
            // "although things got risky"
            subordinate_clause: Box::new(SentenceStructure::Phrasal {
                clause: FragmentSlot::new(NarrativeRole::Complication, "things got risky"),
            }),
            // "still someone wanted a deal"
            main_clause: Box::new(SentenceStructure::Compound {
                clause1: Box::new(SentenceStructure::SubjectPredicate {
                    subject: FragmentSlot::new(NarrativeRole::BuyerSubject, "someone"),
                    predicate: FragmentSlot::new(NarrativeRole::BuyerNeed, "wanted a deal"),
                }),
                conjunction: ConjunctionType::So, // "still" or "and yet"
                clause2: Box::new(SentenceStructure::Phrasal {
                    clause: FragmentSlot::new(NarrativeRole::Resolution, "it went down"),
                }),
            }),
        };

        let variant_b = SentenceStructure::Complex {
            // Main: "A desperate housewife needed her fix"
            main_clause: Box::new(SentenceStructure::SubjectPredicate {
                subject: FragmentSlot::new(NarrativeRole::BuyerSubject, "Someone"),
                predicate: FragmentSlot::new(NarrativeRole::BuyerNeed, "wanted a deal"),
            }),
            subordinator: SubordinatorType::Although,
            // Subordinate: "although things got risky, and yet we made the deal"
            subordinate_clause: Box::new(SentenceStructure::Compound {
                clause1: Box::new(SentenceStructure::Phrasal {
                    clause: FragmentSlot::new(NarrativeRole::Complication, "things got risky"),
                }),
                conjunction: ConjunctionType::Still,
                clause2: Box::new(SentenceStructure::Phrasal {
                    clause: FragmentSlot::new(NarrativeRole::Resolution, "it went down"),
                }),
            }),
        };

        StoryPattern {
            pattern_id: "complicated_deal",
            pattern_type: PatternType::ComplicatedDeal,
            priority: 90,
            required_outcome: Some(HandOutcome::Safe),
            required_cards: vec![
                CardRequirement::buyer(),
                CardRequirement::product(),
                CardRequirement::evidence(),
            ],
            sentence_structures: vec![variant_a, variant_b], // Both variants available
        }
    }

    /// Pattern: Simple Deal
    /// "A Wall Street wolf wanted ice, I had it and we made the deal"
    fn pattern_simple_deal() -> StoryPattern {
        use super::fragments::ConjunctionType;

        StoryPattern {
            pattern_id: "simple_deal",
            pattern_type: PatternType::SimpleDeal,
            priority: 50,
            required_outcome: None, // Matches any outcome
            required_cards: vec![
                CardRequirement::buyer(),
                CardRequirement::product(),
            ],
            sentence_structures: vec![SentenceStructure::Compound {
                // "A Wall Street wolf wanted ice"
                clause1: Box::new(SentenceStructure::SubjectPredicate {
                    subject: FragmentSlot::new(NarrativeRole::BuyerSubject, "Someone"),
                    predicate: FragmentSlot::new(NarrativeRole::BuyerNeed, "wanted product"),
                }),
                conjunction: ConjunctionType::And,
                // "and I had it and [resolution]"
                clause2: Box::new(SentenceStructure::Compound {
                    clause1: Box::new(SentenceStructure::Phrasal {
                        clause: FragmentSlot::new(NarrativeRole::Product, "I had product"),
                    }),
                    conjunction: ConjunctionType::And,
                    clause2: Box::new(SentenceStructure::Phrasal {
                        clause: FragmentSlot::new(NarrativeRole::Resolution, "we made the deal"),
                    }),
                }),
            }],
        }
    }

    /// Pattern: Simple Bust
    /// "A frat bro needed party supplies but I got pinched"
    fn pattern_simple_bust() -> StoryPattern {
        use super::fragments::ConjunctionType;
        use super::super::hand_state::HandOutcome;

        StoryPattern {
            pattern_id: "simple_bust",
            pattern_type: PatternType::SimpleBust,
            priority: 60, // Higher than SimpleDeal to match first when Busted
            required_outcome: Some(HandOutcome::Busted),
            required_cards: vec![
                CardRequirement::buyer(),
            ],
            sentence_structures: vec![SentenceStructure::Compound {
                clause1: Box::new(SentenceStructure::SubjectPredicate {
                    subject: FragmentSlot::new(NarrativeRole::BuyerSubject, "Someone"),
                    predicate: FragmentSlot::new(NarrativeRole::BuyerNeed, "wanted a deal"),
                }),
                conjunction: ConjunctionType::But,
                clause2: Box::new(SentenceStructure::Phrasal {
                    clause: FragmentSlot::new(NarrativeRole::Resolution, "I got busted"),
                }),
            }],
        }
    }

    /// Pattern: Simple Buyer Bail
    /// "A frat bro wanted to get wild but they got cold feet"
    fn pattern_simple_buyer_bail() -> StoryPattern {
        use super::fragments::ConjunctionType;
        use super::super::hand_state::HandOutcome;

        StoryPattern {
            pattern_id: "simple_buyer_bail",
            pattern_type: PatternType::SimpleBuyerBail,
            priority: 60, // Higher than SimpleDeal
            required_outcome: Some(HandOutcome::BuyerBailed),
            required_cards: vec![
                CardRequirement::buyer(),
            ],
            sentence_structures: vec![SentenceStructure::Compound {
                clause1: Box::new(SentenceStructure::SubjectPredicate {
                    subject: FragmentSlot::new(NarrativeRole::BuyerSubject, "Someone"),
                    predicate: FragmentSlot::new(NarrativeRole::BuyerNeed, "wanted a deal"),
                }),
                conjunction: ConjunctionType::But,
                clause2: Box::new(SentenceStructure::Phrasal {
                    clause: FragmentSlot::new(NarrativeRole::Resolution, "they bailed"),
                }),
            }],
        }
    }

    /// Pattern: Simple Dealer Bail (Player folded)
    /// "A frat bro wanted to get wild but I walked away"
    fn pattern_simple_dealer_bail() -> StoryPattern {
        use super::fragments::ConjunctionType;
        use super::super::hand_state::HandOutcome;

        StoryPattern {
            pattern_id: "simple_dealer_bail",
            pattern_type: PatternType::SimpleDealerBail,
            priority: 60, // Higher than SimpleDeal
            required_outcome: Some(HandOutcome::Folded),
            required_cards: vec![
                CardRequirement::buyer(),
            ],
            sentence_structures: vec![SentenceStructure::Compound {
                clause1: Box::new(SentenceStructure::SubjectPredicate {
                    subject: FragmentSlot::new(NarrativeRole::BuyerSubject, "Someone"),
                    predicate: FragmentSlot::new(NarrativeRole::BuyerNeed, "wanted a deal"),
                }),
                conjunction: ConjunctionType::But,
                clause2: Box::new(SentenceStructure::Phrasal {
                    clause: FragmentSlot::new(NarrativeRole::Resolution, "I folded"),
                }),
            }],
        }
    }

    /// Pattern: Simple Invalid Deal
    /// "A frat bro needed party supplies but the deal fell through"
    fn pattern_simple_invalid_deal() -> StoryPattern {
        use super::fragments::ConjunctionType;
        use super::super::hand_state::HandOutcome;

        StoryPattern {
            pattern_id: "simple_invalid_deal",
            pattern_type: PatternType::SimpleInvalidDeal,
            priority: 60, // Higher than SimpleDeal
            required_outcome: Some(HandOutcome::InvalidDeal),
            required_cards: vec![
                CardRequirement::buyer(),
            ],
            sentence_structures: vec![SentenceStructure::Compound {
                clause1: Box::new(SentenceStructure::SubjectPredicate {
                    subject: FragmentSlot::new(NarrativeRole::BuyerSubject, "Someone"),
                    predicate: FragmentSlot::new(NarrativeRole::BuyerNeed, "tried to make a deal"),
                }),
                conjunction: ConjunctionType::But,
                clause2: Box::new(SentenceStructure::Phrasal {
                    clause: FragmentSlot::new(NarrativeRole::Resolution, "it fell through"),
                }),
            }],
        }
    }
}
