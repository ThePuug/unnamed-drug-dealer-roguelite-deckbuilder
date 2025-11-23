// SOW-014 Phase 3: Story Composition Engine
// Assembles narratives from patterns and card fragments

use super::fragments::{SentenceStructure, FragmentSlot, NarrativeFragments, TaggedFragment, ClauseRelation, GrammaticalStructure};
use super::patterns::{DynamicPattern, NarrativeRole};
use crate::models::card::{Card, CardType};
use crate::models::buyer::BuyerScenario;
use crate::models::hand_state::HandOutcome;
use rand::seq::SliceRandom;
use bevy::prelude::Resource;

/// Main story composer - generates stories from hand state
#[derive(Resource)]
pub struct StoryComposer {
    patterns: Vec<DynamicPattern>,
    defaults: NarrativeFragments,
}

/// Context for filling fragments
struct FragmentContext<'a> {
    buyer_scenario: Option<&'a BuyerScenario>,
    played_cards: &'a [Card],
    outcome: HandOutcome,
    defaults: &'a NarrativeFragments,
    // Cached card lookups
    product_card: Option<&'a Card>,
    location_card: Option<&'a Card>,
    evidence_cards: Vec<&'a Card>,
}

impl<'a> FragmentContext<'a> {
    fn new(
        buyer_scenario: Option<&'a BuyerScenario>,
        played_cards: &'a [Card],
        outcome: HandOutcome,
        defaults: &'a NarrativeFragments
    ) -> Self {
        // Find relevant cards once
        // For override card types (Product, Location), find the LAST one played (active after overrides)
        let product_card = played_cards.iter().rev().find(|c| matches!(c.card_type, CardType::Product { .. }));
        let location_card = played_cards.iter().rev().find(|c| matches!(c.card_type, CardType::Location { .. }));
        // For additive card types (Evidence), all cards matter
        let evidence_cards: Vec<&Card> = played_cards.iter()
            .filter(|c| matches!(c.card_type, CardType::Evidence { .. }))
            .collect();

        Self {
            buyer_scenario,
            played_cards,
            outcome,
            defaults,
            product_card,
            location_card,
            evidence_cards,
        }
    }

    fn get_resolution_clause(&self) -> String {
        // Resolution clause WITHOUT conjunction - structure provides it
        self.defaults.resolution_clauses.get_random(self.outcome)
    }
}

impl StoryComposer {
    pub fn new(defaults: NarrativeFragments) -> Self {
        Self {
            patterns: DynamicPattern::create_all_patterns(),
            defaults,
        }
    }

    pub fn compose_story_from_hand(&self, hand_state: &crate::models::hand_state::HandState) -> String {
        let buyer_scenario = hand_state.buyer_persona.as_ref()
            .and_then(|persona| persona.active_scenario_index)
            .and_then(|index| hand_state.buyer_persona.as_ref()
                .and_then(|persona| persona.scenarios.get(index)));

        let outcome = hand_state.outcome.expect("HandState must have outcome set before composing story");

        self.compose_story(buyer_scenario, &hand_state.cards_played, outcome)
    }

    pub fn compose_story(&self, buyer_scenario: Option<&BuyerScenario>, played_cards: &[Card], outcome: HandOutcome) -> String {
        // 1. Find best matching pattern
        let pattern = self.match_pattern(buyer_scenario, played_cards, outcome);

        // 2. Build dynamic structure
        let structure = pattern.build_structure();

        // 3. Build fragment context
        let context = FragmentContext::new(buyer_scenario, played_cards, outcome, &self.defaults);

        // 4. Recursively assemble sentence
        let sentence = self.assemble_structure(&structure, &context);

        // 5. Finalize (unless it's a MultiSentence which handles its own finalization)
        // Check if structure is MultiSentence - if so, already finalized
        if matches!(structure, SentenceStructure::MultiSentence { .. }) {
            sentence
        } else {
            Self::finalize_sentence(sentence)
        }
    }

    fn match_pattern(&self, buyer_scenario: Option<&BuyerScenario>, played_cards: &[Card], outcome: HandOutcome) -> &DynamicPattern {
        let has_buyer = buyer_scenario.is_some();

        for pattern in &self.patterns {
            // Check if pattern matches this outcome
            if let Some(required_outcome) = pattern.required_outcome {
                if required_outcome != outcome {
                    continue;
                }
            }

            // Check if pattern matches cards
            if pattern.matches(has_buyer, played_cards) {
                return pattern;
            }
        }

        // Fallback to last pattern (should be generic catch-all)
        &self.patterns[self.patterns.len() - 1]
    }

    fn assemble_structure(&self, structure: &SentenceStructure, context: &FragmentContext) -> String {
        match structure {
            SentenceStructure::SubjectPredicate { subject, predicate } => {
                let s = self.fill_slot(subject, context);
                let p = self.fill_slot(predicate, context);
                format!("{} {}", s, p)
            },
            SentenceStructure::Phrasal { clause } => {
                self.fill_slot(clause, context)
            },
            SentenceStructure::Compound { clause1, conjunction, clause2 } => {
                let c1 = self.assemble_structure(clause1, context);
                let c2 = self.assemble_structure(clause2, context);
                // No comma for short, direct conjunctions (but, because, when)
                // These flow better without interruption
                format!("{} {} {}", c1, conjunction.as_str(), c2)
            },
            SentenceStructure::Complex { main_clause, subordinator, subordinate_clause } => {
                let main = self.assemble_structure(main_clause, context);
                let sub = self.assemble_structure(subordinate_clause, context);
                format!("{} {} {}", main, subordinator.as_str(), sub)
            },
            SentenceStructure::ReversedComplex { subordinator, subordinate_clause, main_clause } => {
                let sub = self.assemble_structure(subordinate_clause, context);
                let main = self.assemble_structure(main_clause, context);
                // Lowercase the main clause since it comes after comma
                // EXCEPT for "I" (pronoun) which must stay capitalized
                // "Although [sub], [main]" where main should start lowercase
                let main_formatted = if main.starts_with("I ") {
                    main
                } else {
                    Self::lowercase_first(main)
                };
                format!("{} {}, {}", subordinator.as_str(), sub, main_formatted)
            },
            SentenceStructure::Concatenated { clause1, clause2 } => {
                let c1 = self.assemble_structure(clause1, context);
                let c2 = self.assemble_structure(clause2, context);
                // Concatenated used for prepositional phrases
                // If clause2 starts with uppercase, it's a clause needing comma
                // Special case: "I" stays capitalized but still gets comma
                let first_char = c2.chars().next();
                if first_char.map_or(false, |c| c.is_uppercase()) {
                    if c2.starts_with("I ") {
                        // Keep "I" capitalized but add comma
                        format!("{}, {}", c1, c2)
                    } else {
                        // Lowercase other uppercase starts
                        let c2_lower = Self::lowercase_first(c2);
                        format!("{}, {}", c1, c2_lower)
                    }
                } else {
                    // Lowercase start - just append with space (true prepositional)
                    format!("{} {}", c1, c2)
                }
            },
            SentenceStructure::Parenthetical { clause1, subordinator, parenthetical, clause3 } => {
                let c1 = self.assemble_structure(clause1, context);
                let p = self.assemble_structure(parenthetical, context);
                let c3 = self.assemble_structure(clause3, context);
                format!("{}, {} {}, {}", c1, subordinator.as_str(), p, c3)
            },
            SentenceStructure::CompoundComplex { clause1, subordinator, subordinate, conjunction, clause2 } => {
                let c1 = self.assemble_structure(clause1, context);
                let sub = self.assemble_structure(subordinate, context);
                let c2 = self.assemble_structure(clause2, context);
                format!("{} {} {}, {} {}", c1, subordinator.as_str(), sub, conjunction.as_str(), c2)
            },
            SentenceStructure::MultiSentence { sentences } => {
                // Assemble each sentence, capitalize and add period
                let assembled: Vec<String> = sentences.iter()
                    .map(|s| {
                        let sentence = self.assemble_structure(s, context);
                        Self::finalize_sentence(sentence)
                    })
                    .collect();
                // Join with space (each already has period)
                assembled.join(" ")
            },
        }
    }

    fn fill_slot(&self, slot: &FragmentSlot, context: &FragmentContext) -> String {
        let role = slot.role;
        let relation_filter = slot.relation_filter;
        let structure_filter = slot.structure_filter;

        match role {
            NarrativeRole::BuyerSubject => {
                // Try buyer-specific subject fragments first, then display name, then default
                context.buyer_scenario
                    .and_then(|scenario| scenario.narrative_fragments.as_ref())
                    .and_then(|frags| Self::random_from_tagged_list(&frags.subject_clauses, relation_filter, structure_filter))
                    .or_else(|| context.buyer_scenario.map(|b| b.display_name.clone()))
                    .or_else(|| Self::random_from_tagged_list(&context.defaults.subject_clauses, relation_filter, structure_filter))
                    .unwrap_or_else(|| "A mysterious buyer".to_string())
            },
            NarrativeRole::BuyerNeed => {
                // Try buyer-specific need fragments first, then defaults
                context.buyer_scenario
                    .and_then(|scenario| scenario.narrative_fragments.as_ref())
                    .and_then(|frags| Self::random_from_tagged_list(&frags.need_clauses, relation_filter, structure_filter))
                    .or_else(|| Self::random_from_tagged_list(&context.defaults.need_clauses, relation_filter, structure_filter))
                    .unwrap_or_else(|| "needed something".to_string())
            },
            NarrativeRole::Product => {
                // Try product card fragments first, then defaults
                context.product_card
                    .and_then(|c| c.narrative_fragments.as_ref())
                    .and_then(|f| Self::random_from_tagged_list(&f.product_clauses, relation_filter, structure_filter))
                    .or_else(|| Self::random_from_tagged_list(&context.defaults.product_clauses, relation_filter, structure_filter))
                    .unwrap_or_else(|| "I had the goods".to_string())
            },
            NarrativeRole::Location => {
                // Try location card fragments first, then defaults
                context.location_card
                    .and_then(|c| c.narrative_fragments.as_ref())
                    .and_then(|f| Self::random_from_tagged_list(&f.location_clauses, relation_filter, structure_filter))
                    .or_else(|| Self::random_from_tagged_list(&context.defaults.location_clauses, relation_filter, structure_filter))
                    .unwrap_or_else(|| "at the spot".to_string())
            },
            NarrativeRole::Evidence => {
                // Try evidence card fragments first, then defaults
                context.evidence_cards.choose(&mut rand::thread_rng())
                    .and_then(|c| c.narrative_fragments.as_ref())
                    .and_then(|f| Self::random_from_tagged_list(&f.evidence_clauses, relation_filter, structure_filter))
                    .or_else(|| Self::random_from_tagged_list(&context.defaults.evidence_clauses, relation_filter, structure_filter))
                    .unwrap_or_else(|| "things got heated".to_string())
            },
            NarrativeRole::Resolution => {
                context.get_resolution_clause()
            },
        }
    }

    fn random_from_tagged_list(
        list: &[TaggedFragment],
        relation_filter: Option<ClauseRelation>,
        structure_filter: Option<GrammaticalStructure>
    ) -> Option<String> {
        if list.is_empty() {
            return None;
        }

        // Try with both filters
        let candidates: Vec<&TaggedFragment> = list.iter()
            .filter(|f| {
                (relation_filter.is_none() || f.relation.is_none() || f.relation == relation_filter) &&
                (structure_filter.is_none() || f.structure.is_none() || f.structure == structure_filter)
            })
            .collect();

        if !candidates.is_empty() {
            return candidates.choose(&mut rand::thread_rng()).map(|f| f.text.clone());
        }

        // Fallback: ignore structure filter, keep relation filter
        if structure_filter.is_some() {
            let candidates: Vec<&TaggedFragment> = list.iter()
                .filter(|f| relation_filter.is_none() || f.relation.is_none() || f.relation == relation_filter)
                .collect();
            if !candidates.is_empty() {
                return candidates.choose(&mut rand::thread_rng()).map(|f| f.text.clone());
            }
        }

        // Final fallback: any fragment from list
        list.choose(&mut rand::thread_rng()).map(|f| f.text.clone())
    }

    fn finalize_sentence(mut sentence: String) -> String {
        if let Some(first_char) = sentence.chars().next() {
            let capitalized = first_char.to_uppercase().to_string();
            sentence.replace_range(..1, &capitalized);
        }
        if !sentence.ends_with('.') {
            sentence.push('.');
        }
        sentence
    }

    fn lowercase_first(s: String) -> String {
        if let Some(first_char) = s.chars().next() {
            let mut result = first_char.to_lowercase().to_string();
            result.push_str(&s[first_char.len_utf8()..]);
            result
        } else {
            s
        }
    }
}
