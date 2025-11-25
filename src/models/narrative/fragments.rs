// SOW-012 Phase 1: Narrative Generation - Core Data Structures

use rand::prelude::*;
use serde::{Deserialize, Serialize}; // SOW-013-A: Asset externalization
use crate::models::hand_state::HandOutcome;

/// Grammatical structure of the fragment
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum GrammaticalStructure {
    FullClause,      // Complete clause: "we made the deal at the park"
    Prepositional,   // Prepositional phrase: "at the park"
    Passive,         // Passive voice: "the deal was made"
    Gerund,          // -ing form: "meeting at the park"
}

/// Clause relation - semantic relationship (embedded in text)
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum ClauseRelation {
    // Coordinating (connect equal clauses)
    And,          // Addition/sequence
    But,          // Contrast/opposition
    So,           // Consequence/result
    Still,        // Concession with continuation

    // Subordinating (create dependent clauses)
    Although,     // Concessive: "although they were watching"
    Because,      // Causal: "because they caught me"
    When,         // Temporal: "when the cops showed up"
    While,        // Temporal: "while under surveillance"
    Since,        // Causal/Temporal: "since they tapped my phone"
    After,        // Temporal: "after they identified me"
    Before,       // Temporal: "before the deal went down"
    If,           // Conditional: "if they paid upfront"
}

impl ClauseRelation {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::And => "and",
            Self::But => "but",
            Self::So => "so",
            Self::Still => "still",
            Self::Although => ["although", "even though"].choose(&mut rand::rng()).unwrap(),
            Self::Because => "because",
            Self::When => "when",
            Self::While => "while",
            Self::Since => "since",
            Self::After => "after",
            Self::Before => "before",
            Self::If => "if",
        }
    }
}

/// Tagged narrative fragment with relation and structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaggedFragment {
    pub text: String,
    pub relation: Option<ClauseRelation>,           // Semantic relationship (None = works with any)
    pub structure: Option<GrammaticalStructure>,    // Grammatical structure (None = default/unspecified)
}

impl TaggedFragment {
    pub fn new(text: &str, relation: Option<ClauseRelation>, structure: Option<GrammaticalStructure>) -> Self {
        Self {
            text: text.to_string(),
            relation,
            structure,
        }
    }

    pub fn any(text: &str) -> Self {
        Self::new(text, None, None)
    }

    pub fn full_clause(text: &str, relation: Option<ClauseRelation>) -> Self {
        Self::new(text, relation, Some(GrammaticalStructure::FullClause))
    }

    pub fn prep(text: &str) -> Self {
        Self::new(text, None, Some(GrammaticalStructure::Prepositional))
    }
}

/// Narrative fragments for story generation (tagged with conjunction preferences)
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct NarrativeFragments {
    // PHRASAL FRAGMENTS (clause level - complete phrases with conjunction tags)
    #[serde(default)]
    pub subject_clauses: Vec<TaggedFragment>,      // "A desperate housewife", "The soccer mom"
    #[serde(default)]
    pub need_clauses: Vec<TaggedFragment>,         // "needed her fix", "was in denial"
    #[serde(default)]
    pub product_clauses: Vec<TaggedFragment>,      // "I had the stuff", "I was holding codeine"
    #[serde(default)]
    pub location_clauses: Vec<TaggedFragment>,     // "at the park", "in my safe house"
    #[serde(default)]
    pub evidence_clauses: Vec<TaggedFragment>,     // "the cops tapped my lines", "someone dropped a dime" (from Evidence cards)

    // RESOLUTION CLAUSES (outcome-specific endings)
    #[serde(default)]
    pub resolution_clauses: ResolutionClauses,
}


/// Resolution clauses for each outcome type (externalized from hard-coded strings)
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ResolutionClauses {
    #[serde(default)]
    pub safe: Vec<String>,
    #[serde(default)]
    pub busted: Vec<String>,
    #[serde(default)]
    pub folded: Vec<String>,
    #[serde(default)]
    pub buyer_bailed: Vec<String>,
    #[serde(default)]
    pub invalid_deal: Vec<String>,
}

impl ResolutionClauses {
    /// Get random resolution clause for the given outcome
    pub fn get_random(&self, outcome: HandOutcome) -> String {
        let list = match outcome {
            HandOutcome::Safe => &self.safe,
            HandOutcome::Busted => &self.busted,
            HandOutcome::Folded => &self.folded,
            HandOutcome::BuyerBailed => &self.buyer_bailed,
            HandOutcome::InvalidDeal => &self.invalid_deal,
        };

        list.choose(&mut rand::rng())
            .map(|s| s.clone())
            .unwrap_or_else(|| "something happened".to_string())
    }
}

/// Sentence structure types for grammatical composition
#[derive(Debug, Clone)]
pub enum SentenceStructure {
    /// Simple phrasal clause
    Phrasal {
        clause: FragmentSlot,
    },

    /// Subject + Predicate: "Subject predicate" (no conjunction, just space)
    SubjectPredicate {
        subject: FragmentSlot,
        predicate: FragmentSlot,
    },

    /// Concatenated: "Clause1 clause2" (no punctuation, just space - for prepositional phrases)
    Concatenated {
        clause1: Box<SentenceStructure>,
        clause2: Box<SentenceStructure>,
    },

    /// Parenthetical: "Clause1, subordinator clause2, clause3" (middle insertion)
    /// Example: "Subject need, although complication, product and resolution"
    Parenthetical {
        clause1: Box<SentenceStructure>,
        subordinator: ClauseRelation,
        parenthetical: Box<SentenceStructure>,
        clause3: Box<SentenceStructure>,
    },

    /// Compound: "Clause1, conjunction clause2"
    Compound {
        clause1: Box<SentenceStructure>,
        conjunction: ClauseRelation,
        clause2: Box<SentenceStructure>,
    },

    /// Complex: "Main clause subordinator subordinate"
    Complex {
        main_clause: Box<SentenceStructure>,
        subordinator: ClauseRelation,
        subordinate_clause: Box<SentenceStructure>,
    },

    /// Reversed Complex: "Subordinator subordinate, main clause"
    /// Example: "Although things got risky, still we made the deal"
    ReversedComplex {
        subordinator: ClauseRelation,
        subordinate_clause: Box<SentenceStructure>,
        main_clause: Box<SentenceStructure>,
    },

    /// Compound-Complex: "Clause1 subordinator subordinate, conjunction clause2"
    CompoundComplex {
        clause1: Box<SentenceStructure>,
        subordinator: ClauseRelation,
        subordinate: Box<SentenceStructure>,
        conjunction: ClauseRelation,
        clause2: Box<SentenceStructure>,
    },

    /// Multi-sentence: Multiple complete sentences joined with periods
    /// Example: "Subject need. Product and resolution."
    MultiSentence {
        sentences: Vec<Box<SentenceStructure>>,
    },
}

/// Fragment slot with role, relation filter, and structure filter
/// Fallback logic is handled by StoryComposer using defaults
#[derive(Debug, Clone)]
pub struct FragmentSlot {
    pub role: super::patterns::NarrativeRole,
    pub relation_filter: Option<ClauseRelation>,           // Only use fragments with this relation
    pub structure_filter: Option<GrammaticalStructure>,    // Only use fragments with this structure
}

impl FragmentSlot {
    pub fn new(role: super::patterns::NarrativeRole) -> Self {
        Self {
            role,
            relation_filter: None,
            structure_filter: None,
        }
    }

    pub fn with_relation(role: super::patterns::NarrativeRole, relation: ClauseRelation) -> Self {
        Self {
            role,
            relation_filter: Some(relation),
            structure_filter: None,
        }
    }

    pub fn with_structure(role: super::patterns::NarrativeRole, structure: GrammaticalStructure) -> Self {
        Self {
            role,
            relation_filter: None,
            structure_filter: Some(structure),
        }
    }

    pub fn with_relation_and_structure(
        role: super::patterns::NarrativeRole,
        relation: ClauseRelation,
        structure: GrammaticalStructure,
    ) -> Self {
        Self {
            role,
            relation_filter: Some(relation),
            structure_filter: Some(structure),
        }
    }
}
