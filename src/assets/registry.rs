// SOW-013-A Phase 3: Asset registry as Bevy Resource

use bevy::prelude::*;
use crate::models::card::Card;
use crate::models::buyer::BuyerPersona;
use crate::models::narrative::NarrativeFragments;
use std::collections::HashMap;

/// Game assets loaded from RON files
#[derive(Resource, Default)]
pub struct GameAssets {
    pub products: HashMap<String, Card>,
    pub locations: HashMap<String, Card>,
    pub evidence: Vec<Card>,          // Narc deck (Evidence + Conviction)
    pub cover: Vec<Card>,              // Player Cover cards
    pub insurance: Vec<Card>,          // Player Insurance cards
    pub modifiers: Vec<Card>,          // Player Deal Modifiers
    pub buyers: Vec<BuyerPersona>,
    pub narrative_defaults: NarrativeFragments, // Default fragments for cards without custom ones (includes resolution_clauses)
    pub assets_loaded: bool,
}

impl GameAssets {
    pub fn new() -> Self {
        Self {
            products: HashMap::new(),
            locations: HashMap::new(),
            evidence: Vec::new(),
            cover: Vec::new(),
            insurance: Vec::new(),
            modifiers: Vec::new(),
            buyers: Vec::new(),
            narrative_defaults: NarrativeFragments::default(),
            assets_loaded: false,
        }
    }
}
