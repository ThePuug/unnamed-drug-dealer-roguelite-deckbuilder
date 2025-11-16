// HandState module - Core game state machine for hand progression

use bevy::prelude::*;
use std::collections::HashMap;

use crate::models::card::*;
use crate::models::buyer::*;
use crate::models::cards::Cards;
use crate::data::*;

// Re-export implementations
pub mod state_machine;
pub mod resolution;
pub mod card_engine;

// ============================================================================
// CORE DEFINITIONS
// ============================================================================

/// States the hand can be in
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HandPhase {
    Draw,
    PlayerPhase,
    DealerReveal,
    FoldDecision,
    Resolve,
    Bust,
}

/// Outcome of hand resolution
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HandOutcome {
    Safe,
    Busted,
    Folded,
    InvalidDeal,
    BuyerBailed,
}

/// Hand state tracking
#[derive(Component)]
pub struct HandState {
    pub current_state: HandPhase,
    pub current_round: u8,
    pub owner_cards: HashMap<Owner, Cards>,
    pub cards_played: Vec<Card>,
    pub cards_played_this_round: Vec<Card>,
    pub discard_pile: Vec<Card>,
    pub outcome: Option<HandOutcome>,
    pub cash: u32,
    pub current_heat: u32,
    pub current_player_index: usize,
    pub checks_this_hand: Vec<(Owner, u8)>,
    pub buyer_persona: Option<BuyerPersona>,
    pub hand_story: Option<String>, // SOW-012: Generated narrative for this hand
}

impl Default for HandState {
    fn default() -> Self {
        let mut owner_cards = HashMap::new();
        owner_cards.insert(Owner::Narc, Cards::new(create_narc_deck()));
        owner_cards.insert(Owner::Player, Cards::new(create_player_deck()));
        owner_cards.insert(Owner::Buyer, Cards::empty());

        Self {
            current_state: HandPhase::Draw,
            current_round: 1,
            owner_cards,
            cards_played: Vec::new(),
            cards_played_this_round: Vec::new(),
            discard_pile: Vec::new(),
            outcome: None,
            cash: 0,
            current_heat: 0,
            current_player_index: 0,
            checks_this_hand: Vec::new(),
            buyer_persona: None,
            hand_story: None, // SOW-012: No story initially
        }
    }
}

impl HandState {
    /// Get cards for an owner
    pub fn cards(&self, owner: Owner) -> &Cards {
        &self.owner_cards[&owner]
    }

    /// Get mutable cards for an owner
    pub fn cards_mut(&mut self, owner: Owner) -> &mut Cards {
        self.owner_cards.get_mut(&owner).unwrap()
    }
}
