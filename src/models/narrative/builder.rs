use super::fragments::{SentenceStructure, FragmentSlot, ClauseRelation, GrammaticalStructure};
use super::patterns::NarrativeRole;
use rand::prelude::*;

/// A builder for dynamically constructing sentence structures.
pub struct SentenceBuilder {
    satellites: Vec<Satellite>,
    product_conjunction: ClauseRelation,
}

#[derive(Clone)]
pub struct Satellite {
    pub role: NarrativeRole,
    pub placements: Vec<(Placement, f32)>, // Placement and weight
    pub inclusion_chance: f32,
    pub relation: Option<ClauseRelation>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Placement {
    Start,
    End,
    AfterSubject,
    BeforeResolution,
}

impl SentenceBuilder {
    pub fn new() -> Self {
        Self {
            satellites: Vec::new(),
            product_conjunction: ClauseRelation::And, // Default to "and"
        }
    }

    pub fn with_product_conjunction(mut self, conjunction: ClauseRelation) -> Self {
        self.product_conjunction = conjunction;
        self
    }

    pub fn with_satellite(mut self, satellite: Satellite) -> Self {
        self.satellites.push(satellite);
        self
    }

    pub fn build(self) -> SentenceStructure {
        let mut rng = rand::thread_rng();

        // Base Core: Subject + Need + Product + Resolution
        let subject_need = SentenceStructure::SubjectPredicate {
            subject: FragmentSlot::new(NarrativeRole::BuyerSubject),
            predicate: FragmentSlot::new(NarrativeRole::BuyerNeed),
        };

        let product = SentenceStructure::Phrasal {
            clause: FragmentSlot::new(NarrativeRole::Product),
        };

        let resolution = SentenceStructure::Phrasal {
            clause: FragmentSlot::new(NarrativeRole::Resolution),
        };

        let product_resolution = SentenceStructure::Compound {
            clause1: Box::new(product.clone()),
            conjunction: self.product_conjunction,
            clause2: Box::new(resolution.clone()),
        };

        // Choose narrative template with weighted randomness for variety
        let roll: f32 = rng.gen();

        let (mut sentence1, mut sentence2) = if roll < 0.70 {
            // Template A (70%): Subject need. Product resolution.
            // Traditional, buyer-focused opening
            (subject_need, product_resolution)
        } else {
            // Template B (30%): Product. Subject need, resolution.
            // Dealer-first, outcome emphasis
            let subject_need_resolution = SentenceStructure::Compound {
                clause1: Box::new(subject_need),
                conjunction: self.product_conjunction,
                clause2: Box::new(resolution),
            };
            (product, subject_need_resolution)
        };

        // Attach satellites to appropriate sentences
        let mut satellites = self.satellites;
        satellites.shuffle(&mut rng);

        for sat in satellites {
            if rng.gen::<f32>() > sat.inclusion_chance {
                continue;
            }

            let placement = sat.pick_placement(&mut rng);
            let sat_structure = sat.to_structure();

            match placement {
                Placement::Start => {
                    // Attach to sentence 1 as opener
                    sentence1 = if let Some(relation) = sat.relation {
                        // Subordinating relation: "Although X, subject need"
                        SentenceStructure::ReversedComplex {
                            subordinator: relation,
                            subordinate_clause: Box::new(SentenceStructure::Phrasal {
                                clause: FragmentSlot::new(sat.role),
                            }),
                            main_clause: Box::new(sentence1),
                        }
                    } else {
                        // Prepositional phrase: "At X, subject need"
                        // Request prepositional structure for clean concatenation
                        SentenceStructure::Concatenated {
                            clause1: Box::new(SentenceStructure::Phrasal {
                                clause: FragmentSlot::with_structure(sat.role, GrammaticalStructure::Prepositional),
                            }),
                            clause2: Box::new(sentence1),
                        }
                    };
                },
                Placement::End | Placement::BeforeResolution => {
                    // Attach to sentence 2 as closer
                    sentence2 = if let Some(relation) = sat.relation {
                        // With conjunction: "Product but resolution because X"
                        SentenceStructure::Compound {
                            clause1: Box::new(sentence2),
                            conjunction: relation,
                            clause2: Box::new(sat_structure),
                        }
                    } else {
                        // Prepositional phrase: "Product but resolution at X"
                        // Request prepositional structure for clean appending
                        SentenceStructure::Concatenated {
                            clause1: Box::new(sentence2),
                            clause2: Box::new(SentenceStructure::Phrasal {
                                clause: FragmentSlot::with_structure(sat.role, GrammaticalStructure::Prepositional),
                            }),
                        }
                    };
                },
                Placement::AfterSubject => {
                    // Attach to sentence 1 as closer
                    sentence1 = if let Some(relation) = sat.relation {
                        SentenceStructure::Compound {
                            clause1: Box::new(sentence1),
                            conjunction: relation,
                            clause2: Box::new(sat_structure),
                        }
                    } else {
                        // Prepositional phrase: "Subject need at X"
                        // Request prepositional structure
                        SentenceStructure::Concatenated {
                            clause1: Box::new(sentence1),
                            clause2: Box::new(SentenceStructure::Phrasal {
                                clause: FragmentSlot::with_structure(sat.role, GrammaticalStructure::Prepositional),
                            }),
                        }
                    };
                },
            }
        }

        // Return multi-sentence structure
        SentenceStructure::MultiSentence {
            sentences: vec![Box::new(sentence1), Box::new(sentence2)],
        }
    }
}

impl Satellite {
    pub fn new(role: NarrativeRole) -> Self {
        Self {
            role,
            placements: Vec::new(),
            inclusion_chance: 1.0,
            relation: None,
        }
    }
    
    pub fn with_placement(mut self, placement: Placement, weight: f32) -> Self {
        self.placements.push((placement, weight));
        self
    }
    
    pub fn optional(mut self, chance: f32) -> Self {
        self.inclusion_chance = chance;
        self
    }
    
    /// Set the conjunction for sentence structure (e.g., "because", "although")
    /// This does NOT filter fragments - all fragments of this role will be considered
    pub fn with_conjunction(mut self, relation: ClauseRelation) -> Self {
        self.relation = Some(relation);
        self
    }
    
    fn pick_placement(&self, rng: &mut impl Rng) -> Placement {
        if self.placements.is_empty() {
            return Placement::End;
        }
        
        let total: f32 = self.placements.iter().map(|(_, w)| w).sum();
        let mut r = rng.gen::<f32>() * total;
        
        for (p, w) in &self.placements {
            r -= w;
            if r <= 0.0 {
                return *p;
            }
        }
        self.placements[0].0
    }
    
    fn to_structure(&self) -> SentenceStructure {
        // Don't use relation as a filter - just create basic Phrasal
        // The relation is used in the sentence structure (Compound/ReversedComplex)
        SentenceStructure::Phrasal {
            clause: FragmentSlot::new(self.role),
        }
    }
}
