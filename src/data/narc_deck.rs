// SOW-AAA Phase 1: Narc deck data creator
// Extracted from main.rs (originally line 3587-3691)

use crate::models::card::{Card, CardType};
use rand::seq::SliceRandom;

/// SOW-005: Create Narc deck (25 cards - Law Enforcement Theme)
pub fn create_narc_deck() -> Vec<Card> {
    let mut deck = vec![];
    let mut id = 1;

    // 17 Evidence cards (balanced evidence/heat values)
    // 3× Donut Break (0 evidence, 0 heat)
    for _ in 0..3 {
        deck.push(Card {
            id,
            name: "Donut Break".to_string(),
            card_type: CardType::Evidence { evidence: 0, heat: 0 },
        });
        id += 1;
    }

    // 3× Patrol (5 evidence, 5 heat)
    for _ in 0..3 {
        deck.push(Card {
            id,
            name: "Patrol".to_string(),
            card_type: CardType::Evidence { evidence: 5, heat: 5 },
        });
        id += 1;
    }

    // 2× Anonymous Tip (5 evidence, 20 heat)
    for _ in 0..2 {
        deck.push(Card {
            id,
            name: "Anonymous Tip".to_string(),
            card_type: CardType::Evidence { evidence: 5, heat: 20 },
        });
        id += 1;
    }

    // 2× Suspect Identified (10 evidence, 10 heat)
    for _ in 0..2 {
        deck.push(Card {
            id,
            name: "Suspect Identified".to_string(),
            card_type: CardType::Evidence { evidence: 10, heat: 10 },
        });
        id += 1;
    }

    // 2× Probable Cause (15 evidence, 10 heat)
    for _ in 0..2 {
        deck.push(Card {
            id,
            name: "Probable Cause".to_string(),
            card_type: CardType::Evidence { evidence: 15, heat: 10 },
        });
        id += 1;
    }

    // 1× Surveillance (20 evidence, 5 heat)
    deck.push(Card {
        id,
        name: "Surveillance".to_string(),
        card_type: CardType::Evidence { evidence: 20, heat: 5 },
    });
    id += 1;

    // 1× Stakeout (30 evidence, 15 heat)
    deck.push(Card {
        id,
        name: "Stakeout".to_string(),
        card_type: CardType::Evidence { evidence: 30, heat: 15 },
    });
    id += 1;

    // 1× Undercover Op (30 evidence, 0 heat)
    deck.push(Card {
        id,
        name: "Undercover Op".to_string(),
        card_type: CardType::Evidence { evidence: 30, heat: 0 },
    });
    id += 1;

    // 1× Tapped Lines (35 evidence, 0 heat)
    deck.push(Card {
        id,
        name: "Tapped Lines".to_string(),
        card_type: CardType::Evidence { evidence: 35, heat: 0 },
    });
    id += 1;

    // 8 Conviction cards (revised thresholds: 10/20/40)
    // 2× Warrant (threshold: 10)
    for _ in 0..2 {
        deck.push(Card {
            id,
            name: "Warrant".to_string(),
            card_type: CardType::Conviction { heat_threshold: 10 },
        });
        id += 1;
    }

    // 2× Caught Red-Handed (threshold: 20)
    for _ in 0..2 {
        deck.push(Card {
            id,
            name: "Caught Red-Handed".to_string(),
            card_type: CardType::Conviction { heat_threshold: 20 },
        });
        id += 1;
    }

    // 4× Random Search (threshold: 40)
    for _ in 0..4 {
        deck.push(Card {
            id,
            name: "Random Search".to_string(),
            card_type: CardType::Conviction { heat_threshold: 40 },
        });
        id += 1;
    }

    // Shuffle deck for variety
    deck.shuffle(&mut rand::thread_rng());
    deck
}
