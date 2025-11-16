// SOW-012 Phase 1: Narrative Generation - Core Data Structures

use rand::{seq::SliceRandom, thread_rng};
use serde::{Deserialize, Serialize}; // SOW-013-A: Asset externalization

/// Narrative fragments for story generation (phrasal-only MVP)
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct NarrativeFragments {
    // PHRASAL FRAGMENTS (clause level - complete phrases)
    pub subject_clauses: Vec<String>,      // "A desperate housewife", "The soccer mom"
    pub need_clauses: Vec<String>,         // "needed her fix", "was in denial"
    pub product_clauses: Vec<String>,      // "I had the stuff", "I was holding codeine"
    pub location_clauses: Vec<String>,     // "at the park", "in my safe house"
    pub complication_clauses: Vec<String>, // "the cops tapped my lines", "with heat closing in"
    pub action_clauses: Vec<String>,       // "I had no choice but to bail", "I made the deal"
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

    /// Compound: "Clause1, conjunction clause2"
    Compound {
        clause1: Box<SentenceStructure>,
        conjunction: ConjunctionType,
        clause2: Box<SentenceStructure>,
    },

    /// Complex: "Main clause subordinator subordinate"
    Complex {
        main_clause: Box<SentenceStructure>,
        subordinator: SubordinatorType,
        subordinate_clause: Box<SentenceStructure>,
    },

    /// Reversed Complex: "Subordinator subordinate, main clause"
    /// Example: "Although things got risky, still we made the deal"
    ReversedComplex {
        subordinator: SubordinatorType,
        subordinate_clause: Box<SentenceStructure>,
        main_clause: Box<SentenceStructure>,
    },

    /// Compound-Complex: "Clause1 subordinator subordinate, conjunction clause2"
    CompoundComplex {
        clause1: Box<SentenceStructure>,
        subordinator: SubordinatorType,
        subordinate: Box<SentenceStructure>,
        conjunction: ConjunctionType,
        clause2: Box<SentenceStructure>,
    },
}

/// Coordinating conjunctions
#[derive(Debug, Clone, Copy)]
pub enum ConjunctionType {
    And,  // Addition/sequence
    But,  // Contrast/complication
    So,   // Consequence/result
    Still, 
}

impl ConjunctionType {
    pub fn as_str(&self) -> &str {
        match self {
            Self::And => "and",
            Self::But => "but",
            Self::So => "so",
            Self::Still => "still",
        }
    }
}

/// Subordinating conjunctions
#[derive(Debug, Clone, Copy)]
pub enum SubordinatorType {
    When,      // Temporal: "when the cops showed up"
    Because,   // Causal: "because I needed cash"
    Although,  // Concessive: "although it was risky"
    If,        // Conditional: "if she paid upfront"
    While,     // Temporal: "while the narc watched"
    Since,     // Causal/Temporal: "since they tapped my phone"
    After,     // Temporal: "after she called"
    Before,    // Temporal: "before the deal went down"
}

impl SubordinatorType {
    pub fn as_str(&self) -> &str {
        match self {
            Self::When => "when",
            Self::Because => "because",
            Self::Although => "although",
            Self::If => "if",
            Self::While => "while",
            Self::Since => "since",
            Self::After => "after",
            Self::Before => "before",
        }
    }
}

/// Fragment slot with role and fallback
#[derive(Debug, Clone)]
pub struct FragmentSlot {
    pub role: super::patterns::NarrativeRole,
    pub fallback: String,
}

impl FragmentSlot {
    pub fn new(role: super::patterns::NarrativeRole, fallback: &str) -> Self {
        Self {
            role,
            fallback: fallback.to_string(),
        }
    }
}
