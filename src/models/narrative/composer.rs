// SOW-012 Phase 3: Story Composition Engine
// Assembles narratives from patterns and card fragments

use super::fragments::{SentenceStructure, FragmentSlot, NarrativeFragments};
use super::patterns::{StoryPattern, NarrativeRole};
use crate::models::card::{Card, CardType};
use crate::models::buyer::BuyerScenario;
use crate::models::hand_state::HandOutcome;
use crate::models::narrative::fragments::TaggedFragment;
use rand::seq::SliceRandom;
use bevy::prelude::Resource;

/// Main story composer - generates stories from hand state
/// Created once at startup as a Bevy Resource
/// Holds narrative defaults and does fallback logic at composition time
#[derive(Resource)]
pub struct StoryComposer {
    patterns: Vec<StoryPattern>,
    defaults: NarrativeFragments,
}

impl StoryComposer {
    pub fn new(defaults: NarrativeFragments) -> Self {
        Self {
            patterns: StoryPattern::create_all_patterns(),
            defaults,
        }
    }

    /// Compose story from hand state (reads outcome from hand_state.outcome)
    pub fn compose_story_from_hand(&self, hand_state: &crate::models::hand_state::HandState) -> String {
        // Get active buyer scenario from hand state
        let buyer_scenario = hand_state.buyer_persona.as_ref()
            .and_then(|persona| persona.active_scenario_index)
            .and_then(|index| hand_state.buyer_persona.as_ref()
                .and_then(|persona| persona.scenarios.get(index)));

        // Get outcome from hand state
        let outcome = hand_state.outcome.expect("HandState must have outcome set before composing story");

        // Compose story from hand state data
        self.compose_story(buyer_scenario, &hand_state.cards_played, outcome)
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
        let context = FragmentContext::new(buyer_scenario, played_cards, outcome, &self.defaults);

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

            SentenceStructure::Concatenated { clause1, clause2 } => {
                format!("{} {}",
                    self.assemble_structure(clause1, context),
                    self.assemble_structure(clause2, context)
                )
            },

            SentenceStructure::Parenthetical { clause1, subordinator, parenthetical, clause3 } => {
                format!("{}, {} {}, {}",
                    self.assemble_structure(clause1, context),
                    subordinator.as_str(),
                    self.assemble_structure(parenthetical, context),
                    self.assemble_structure(clause3, context)
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

    /// Pick fragment from card or use defaults (handled in get_fragment)
    fn pick_fragment(&self, slot: &FragmentSlot, context: &FragmentContext) -> String {
        context.get_fragment(slot.role, slot.relation_filter, slot.structure_filter)
            .expect("Fragment should always be available from card or defaults")
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

/// Context for fragment extraction from cards
struct FragmentContext<'a> {
    buyer_scenario: Option<&'a BuyerScenario>,
    product_card: Option<&'a Card>,
    location_card: Option<&'a Card>,
    evidence_cards: Vec<&'a Card>,
    outcome: HandOutcome,
    defaults: &'a NarrativeFragments,
}

impl<'a> FragmentContext<'a> {
    fn new(buyer_scenario: Option<&'a BuyerScenario>, played_cards: &'a [Card], outcome: HandOutcome, defaults: &'a NarrativeFragments) -> Self {
        let mut product_card = None;
        let mut location_card = None;
        let mut evidence_cards = Vec::new();

        eprintln!("DEBUG FragmentContext::new - Processing {} played cards", played_cards.len());

        for card in played_cards {
            eprintln!("  Card: {} (type: {:?})", card.name, card.card_type);
            match &card.card_type {
                CardType::Product { .. } => {
                    if product_card.is_none() {
                        eprintln!("    -> Set as product_card");
                        product_card = Some(card);
                    }
                },
                CardType::Location { .. } => {
                    if location_card.is_none() {
                        eprintln!("    -> Set as location_card");
                        location_card = Some(card);
                    }
                },
                CardType::Evidence { .. } | CardType::Conviction { .. } => {
                    eprintln!("    -> Added to evidence_cards");
                    evidence_cards.push(card);
                },
                CardType::DealModifier { .. } | CardType::Insurance { .. } | CardType::Cover { .. } => {
                    eprintln!("    -> Skipped (modifier/insurance/cover)");
                    // These cards don't have narrative fragments, so we skip them
                },
            }
        }

        eprintln!("DEBUG: Final context - product_card: {:?}, location_card: {:?}, evidence: {}",
            product_card.map(|c| &c.name), location_card.map(|c| &c.name), evidence_cards.len());

        Self {
            buyer_scenario,
            product_card,
            location_card,
            evidence_cards,
            outcome,
            defaults,
        }
    }

    /// Get outcome-specific resolution clause (no leading conjunction - pattern provides it)
    fn get_resolution_clause(&self) -> String {
        self.defaults.resolution_clauses.get_random(self.outcome)
    }

    /// Get fragment for a narrative role with optional relation/structure filtering
    /// Tries card-specific fragments first, then falls back to defaults
    fn get_fragment(
        &self,
        role: NarrativeRole,
        relation_filter: Option<super::fragments::ClauseRelation>,
        structure_filter: Option<super::fragments::GrammaticalStructure>
    ) -> Option<String> {
        match role {
            NarrativeRole::BuyerSubject => {
                // Try buyer scenario fragments first
                self.buyer_scenario
                    .and_then(|scenario| scenario.narrative_fragments.as_ref())
                    .and_then(|frags| Self::random_from_tagged_list(&frags.subject_clauses, relation_filter, structure_filter))
                    // Fall back to defaults
                    .or_else(|| Self::random_from_tagged_list(&self.defaults.subject_clauses, relation_filter, structure_filter))
            },

            NarrativeRole::BuyerNeed => {
                // Try buyer scenario fragments first
                self.buyer_scenario
                    .and_then(|scenario| scenario.narrative_fragments.as_ref())
                    .and_then(|frags| Self::random_from_tagged_list(&frags.need_clauses, relation_filter, structure_filter))
                    // Fall back to defaults
                    .or_else(|| Self::random_from_tagged_list(&self.defaults.need_clauses, relation_filter, structure_filter))
            },

            NarrativeRole::Product => {
                // Try product card fragments first
                self.product_card
                    .and_then(|card| {
                        eprintln!("DEBUG: Using product card: {}", card.name);
                        if let Some(frags) = &card.narrative_fragments {
                            eprintln!("  -> Has {} product_clauses", frags.product_clauses.len());
                        } else {
                            eprintln!("  -> narrative_fragments is NONE!");
                        }
                        card.narrative_fragments.as_ref()
                    })
                    .and_then(|frags| Self::random_from_tagged_list(&frags.product_clauses, relation_filter, structure_filter))
                    // Fall back to defaults
                    .or_else(|| {
                        eprintln!("DEBUG: Using DEFAULT product fragments");
                        Self::random_from_tagged_list(&self.defaults.product_clauses, relation_filter, structure_filter)
                    })
            },

            NarrativeRole::Location => {
                // Try location card fragments first
                self.location_card
                    .and_then(|card| card.narrative_fragments.as_ref())
                    .and_then(|frags| Self::random_from_tagged_list(&frags.location_clauses, relation_filter, structure_filter))
                    // Fall back to defaults
                    .or_else(|| Self::random_from_tagged_list(&self.defaults.location_clauses, relation_filter, structure_filter))
            },

            NarrativeRole::Complication => {
                // Try evidence card fragments first (pick random evidence card)
                self.evidence_cards.choose(&mut rand::thread_rng())
                    .and_then(|card| card.narrative_fragments.as_ref())
                    .and_then(|frags| Self::random_from_tagged_list(&frags.complication_clauses, relation_filter, structure_filter))
                    // Fall back to defaults
                    .or_else(|| Self::random_from_tagged_list(&self.defaults.complication_clauses, relation_filter, structure_filter))
            },

            NarrativeRole::Resolution => {
                // Use outcome-specific resolution clause from defaults
                Some(self.get_resolution_clause())
            },
        }
    }

    /// Pick random element from tagged list with filtering
    fn random_from_tagged_list(
        list: &[TaggedFragment],
        relation_filter: Option<super::fragments::ClauseRelation>,
        structure_filter: Option<super::fragments::GrammaticalStructure>
    ) -> Option<String> {
        if list.is_empty() {
            eprintln!("DEBUG random_from_tagged_list: list is EMPTY");
            return None;
        }

        eprintln!("DEBUG random_from_tagged_list: list has {} items, relation_filter={:?}, structure_filter={:?}",
            list.len(), relation_filter, structure_filter);

        // Filter by relation AND structure
        let candidates: Vec<&TaggedFragment> = list.iter()
            .filter(|frag| {
                let relation_match = relation_filter.is_none()
                    || frag.relation.is_none()
                    || frag.relation == relation_filter;

                let structure_match = structure_filter.is_none()
                    || frag.structure.is_none()
                    || frag.structure == structure_filter;

                eprintln!("  Fragment '{}': relation={:?} (match={}), structure={:?} (match={})",
                    frag.text, frag.relation, relation_match, frag.structure, structure_match);

                relation_match && structure_match
            })
            .collect();

        eprintln!("DEBUG: After filtering, {} candidates remain", candidates.len());

        if candidates.is_empty() {
            // Fallback to any fragment
            eprintln!("DEBUG: No candidates, falling back to any fragment");
            list.choose(&mut rand::thread_rng()).map(|f| f.text.clone())
        } else {
            let chosen = candidates.choose(&mut rand::thread_rng()).map(|f| f.text.clone());
            eprintln!("DEBUG: Chose: {:?}", chosen);
            chosen
        }
    }
}
