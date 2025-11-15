// SOW-AAA Phase 1: Narc deck data creator
// Extracted from main.rs (originally line 3587-3691)

use crate::models::card::{Card, CardType};
use rand::seq::SliceRandom;

/// SOW-005: Create Narc deck (25 cards - Law Enforcement Theme)
pub fn create_narc_deck() -> Vec<Card> {
    let mut deck = vec![];
    let mut id = 1;

    // 17 Evidence cards (varied threat levels)
    // 8× Low threat
    for _ in 0..8 {
        deck.push(Card {
            id,
            name: "Donut Break".to_string(),
            card_type: CardType::Evidence { evidence: 0, heat: 0 },
        });
        id += 1;
    }

    // 6× Medium threat
    for _ in 0..2 {
        deck.push(Card {
            id,
            name: "Patrol".to_string(),
            card_type: CardType::Evidence { evidence: 5, heat: 2 },
        });
        id += 1;
    }

    for _ in 0..2 {
        deck.push(Card {
            id,
            name: "Surveillance".to_string(),
            card_type: CardType::Evidence { evidence: 20, heat: 5 },
        });
        id += 1;
    }

    for _ in 0..2 {
        deck.push(Card {
            id,
            name: "Stakeout".to_string(),
            card_type: CardType::Evidence { evidence: 10, heat: 3 },
        });
        id += 1;
    }

    // 3× High threat
    deck.push(Card {
        id,
        name: "Undercover Op".to_string(),
        card_type: CardType::Evidence { evidence: 30, heat: 10 },
    });
    id += 1;

    deck.push(Card {
        id,
        name: "Raid".to_string(),
        card_type: CardType::Evidence { evidence: 40, heat: 20 },
    });
    id += 1;

    deck.push(Card {
        id,
        name: "Wiretap".to_string(),
        card_type: CardType::Evidence { evidence: 35, heat: 15 },
    });
    id += 1;

    // 8 Conviction cards (SOW-005: Moved from player deck)
    for _ in 0..4 {
        deck.push(Card {
            id,
            name: "Warrant".to_string(),
            card_type: CardType::Conviction { heat_threshold: 40 },
        });
        id += 1;
    }

    for _ in 0..3 {
        deck.push(Card {
            id,
            name: "DA Approval".to_string(),
            card_type: CardType::Conviction { heat_threshold: 60 },
        });
        id += 1;
    }

    deck.push(Card {
        id,
        name: "RICO Case".to_string(),
        card_type: CardType::Conviction { heat_threshold: 80 },
    });

    // Shuffle deck for variety
    deck.shuffle(&mut rand::thread_rng());
    deck
}
