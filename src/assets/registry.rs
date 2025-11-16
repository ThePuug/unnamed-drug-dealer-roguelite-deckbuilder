// SOW-013-A Phase 3: Asset registry as Bevy Resource

use bevy::prelude::*;
use crate::models::card::Card;
use crate::models::buyer::BuyerPersona;
use std::collections::HashMap;

/// Game assets loaded from RON files
#[derive(Resource, Default)]
pub struct GameAssets {
    pub products: HashMap<String, Card>,
    pub locations: HashMap<String, Card>,
    pub buyers: Vec<BuyerPersona>,
    pub assets_loaded: bool,
}

impl GameAssets {
    pub fn new() -> Self {
        Self {
            products: HashMap::new(),
            locations: HashMap::new(),
            buyers: Vec::new(),
            assets_loaded: false,
        }
    }
}
