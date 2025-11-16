// SOW-012 Phase 3: Story Composition Engine
// Assembles narratives from patterns and card fragments

use super::fragments::{SentenceStructure, ConjunctionType, SubordinatorType, FragmentSlot, NarrativeFragments};
use super::patterns::{StoryPattern, NarrativeRole};
use crate::models::card::{Card, CardType};
use crate::models::buyer::BuyerScenario;
use crate::models::hand_state::HandOutcome;
use std::collections::HashMap;
use rand::seq::SliceRandom;

/// Main story composer - generates stories from hand state
pub struct StoryComposer {
    patterns: Vec<StoryPattern>,
}

impl StoryComposer {
    pub fn new() -> Self {
        Self {
            patterns: StoryPattern::create_all_patterns(),
        }
    }

    /// Compose story from played cards, buyer scenario, and hand outcome
    pub fn compose_story(&self, buyer_scenario: Option<&BuyerScenario>, played_cards: &[Card], outcome: HandOutcome) -> String {
        // 1. Find best matching pattern (by priority)
        let pattern = self.match_pattern(buyer_scenario, played_cards, outcome);

        // 2. Randomly select a sentence structure variant from the pattern
        let structure = pattern.sentence_structures
            .choose(&mut rand::thread_rng())
            .expect("Pattern must have at least one sentence structure");

        // 3. Build fragment context from cards and outcome
        let context = FragmentContext::new(buyer_scenario, played_cards, outcome);

        // 4. Recursively assemble sentence structure
        let sentence = self.assemble_structure(structure, &context);

        // 5. Capitalize first letter, add period
        Self::finalize_sentence(sentence)
    }

    /// Find best matching pattern based on cards and outcome
    fn match_pattern(&self, buyer_scenario: Option<&BuyerScenario>, played_cards: &[Card], outcome: HandOutcome) -> &StoryPattern {
        let buyer_exists = buyer_scenario.is_some();

        // Patterns are already sorted by priority (highest first)
        for pattern in &self.patterns {
            // Check if pattern matches this outcome
            if let Some(required_outcome) = pattern.required_outcome {
                if required_outcome != outcome {
                    continue; // Skip patterns that don't match this outcome
                }
            }

            // Check card requirements
            let matches = pattern.required_cards.iter().all(|req| {
                match req.role {
                    NarrativeRole::BuyerSubject | NarrativeRole::BuyerNeed => buyer_exists,
                    _ => {
                        if let Some(filter) = &req.card_type_filter {
                            played_cards.iter().any(|card| filter.matches(&card.card_type))
                        } else {
                            false
                        }
                    }
                }
            });

            if matches {
                return pattern;
            }
        }

        // Fallback (should never happen since GenericTransaction always matches)
        &self.patterns[self.patterns.len() - 1]
    }

    /// Recursively assemble sentence from structure
    fn assemble_structure(&self, structure: &SentenceStructure, context: &FragmentContext) -> String {
        match structure {
            SentenceStructure::Phrasal { clause } => {
                self.pick_fragment(clause, context)
            },

            SentenceStructure::SubjectPredicate { subject, predicate } => {
                format!("{} {}",
                    self.pick_fragment(subject, context),
                    self.pick_fragment(predicate, context)
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

            SentenceStructure::ReversedComplex { subordinator, subordinate_clause, main_clause } => {
                format!("{} {}, {}",
                    subordinator.as_str(),
                    self.assemble_structure(subordinate_clause, context),
                    self.assemble_structure(main_clause, context)
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

    /// Pick fragment from card or use fallback
    fn pick_fragment(&self, slot: &FragmentSlot, context: &FragmentContext) -> String {
        context.get_fragment(slot.role)
            .unwrap_or_else(|| slot.fallback.clone())
    }

    /// Finalize sentence (capitalize, punctuate)
    fn finalize_sentence(mut sentence: String) -> String {
        // Capitalize first letter
        if let Some(first_char) = sentence.get_mut(0..1) {
            first_char.make_ascii_uppercase();
        }

        // Add period if missing
        if !sentence.ends_with('.') {
            sentence.push('.');
        }

        sentence
    }
}

impl Default for StoryComposer {
    fn default() -> Self {
        Self::new()
    }
}

/// Context for fragment extraction from cards
struct FragmentContext<'a> {
    buyer_scenario: Option<&'a BuyerScenario>,
    product_card: Option<&'a Card>,
    location_card: Option<&'a Card>,
    evidence_cards: Vec<&'a Card>,
    action_cards: Vec<&'a Card>,
    outcome: HandOutcome,
}

impl<'a> FragmentContext<'a> {
    fn new(buyer_scenario: Option<&'a BuyerScenario>, played_cards: &'a [Card], outcome: HandOutcome) -> Self {
        let mut product_card = None;
        let mut location_card = None;
        let mut evidence_cards = Vec::new();
        let mut action_cards = Vec::new();

        for card in played_cards {
            match &card.card_type {
                CardType::Product { .. } => {
                    if product_card.is_none() {
                        product_card = Some(card);
                    }
                },
                CardType::Location { .. } => {
                    if location_card.is_none() {
                        location_card = Some(card);
                    }
                },
                CardType::Evidence { .. } | CardType::Conviction { .. } => {
                    evidence_cards.push(card);
                },
                CardType::DealModifier { .. } | CardType::Insurance { .. } | CardType::Cover { .. } => {
                    action_cards.push(card);
                },
            }
        }

        Self {
            buyer_scenario,
            product_card,
            location_card,
            evidence_cards,
            action_cards,
            outcome,
        }
    }

    /// Get outcome-specific resolution clause (no leading conjunction - pattern provides it)
    fn get_resolution_clause(&self) -> String {
        match self.outcome {
            HandOutcome::Safe => {
                vec![
                    "we made the deal",
                    "we made it happen",
                    "it went down",
                ].choose(&mut rand::thread_rng()).unwrap().to_string()
            },
            HandOutcome::Busted => {
                vec![
                    "I got pinched",
                    "they got me",
                    "the cops busted me",
                ].choose(&mut rand::thread_rng()).unwrap().to_string()
            },
            HandOutcome::Folded => {
                vec![
                    "I walked away",
                    "I cut my losses",
                    "I backed out",
                ].choose(&mut rand::thread_rng()).unwrap().to_string()
            },
            HandOutcome::BuyerBailed => {
                vec![
                    "they got cold feet",
                    "they walked",
                    "they ghosted me",
                ].choose(&mut rand::thread_rng()).unwrap().to_string()
            },
            HandOutcome::InvalidDeal => {
                vec![
                    "the deal fell through",
                    "it went belly up"
                ].choose(&mut rand::thread_rng()).unwrap().to_string()
            },
        }
    }

    /// Get fragment for a narrative role
    fn get_fragment(&self, role: NarrativeRole) -> Option<String> {
        match role {
            NarrativeRole::BuyerSubject => {
                self.buyer_scenario
                    .and_then(|scenario| scenario.narrative_fragments.as_ref())
                    .and_then(|frags| Self::random_from_list(&frags.subject_clauses))
            },

            NarrativeRole::BuyerNeed => {
                self.buyer_scenario
                    .and_then(|scenario| scenario.narrative_fragments.as_ref())
                    .and_then(|frags| Self::random_from_list(&frags.need_clauses))
            },

            NarrativeRole::Product => {
                self.product_card
                    .and_then(|card| card.narrative_fragments.as_ref())
                    .and_then(|frags| Self::random_from_list(&frags.product_clauses))
            },

            NarrativeRole::Location => {
                self.location_card
                    .and_then(|card| card.narrative_fragments.as_ref())
                    .and_then(|frags| Self::random_from_list(&frags.location_clauses))
            },

            NarrativeRole::Complication => {
                // Pick random evidence card, then random fragment
                self.evidence_cards.choose(&mut rand::thread_rng())
                    .and_then(|card| card.narrative_fragments.as_ref())
                    .and_then(|frags| Self::random_from_list(&frags.complication_clauses))
            },

            NarrativeRole::Action => {
                // Pick random action card, then random fragment
                self.action_cards.choose(&mut rand::thread_rng())
                    .and_then(|card| card.narrative_fragments.as_ref())
                    .and_then(|frags| Self::random_from_list(&frags.action_clauses))
            },

            NarrativeRole::Resolution => {
                // Use outcome-specific resolution clause
                Some(self.get_resolution_clause())
            },
        }
    }

    /// Pick random element from list (if non-empty)
    fn random_from_list(list: &[String]) -> Option<String> {
        if list.is_empty() {
            None
        } else {
            Some(list.choose(&mut rand::thread_rng()).unwrap().clone())
        }
    }
}
