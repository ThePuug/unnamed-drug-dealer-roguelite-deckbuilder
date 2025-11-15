// SOW-AAA Phase 1: Buyer persona data creators
// Extracted from main.rs (originally line 3859-4145)

use crate::models::card::{Card, CardType};
use crate::models::buyer::{BuyerDemand, BuyerPersona, BuyerScenario};

/// SOW-009 Phase 2: Create all available Buyer personas (MVP: 3 personas)
pub fn create_buyer_personas() -> Vec<BuyerPersona> {
    vec![
        create_college_party_host(),
        create_stay_at_home_mom(),
        create_executive(),
    ]
}

/// Buyer Persona 1: Frat Bro
/// High profit (×2.5), no threshold (won't bail), high Evidence risk
fn create_college_party_host() -> BuyerPersona {
    let mut id = 2000; // Start Buyer cards at 2000

    BuyerPersona {
        id: "frat_bro".to_string(),
        display_name: "Frat Bro".to_string(),
        demand: BuyerDemand {
            products: vec!["Weed".to_string(), "Pills".to_string()],
            locations: vec!["Dorm".to_string(), "Party".to_string(), "Park".to_string()],
            description: "Wants Weed or Pills, high volume, public locations".to_string(),
        },
        base_multiplier: 2.5,
        reduced_multiplier: 1.0,
        heat_threshold: None,  // Not paranoid, won't bail
        evidence_threshold: None,
        special_rules: vec![],  // TODO: Add "+10 Evidence if public Location" in future phase
        reaction_deck: vec![
            // 1. Evidence - Prior Conviction
            Card {
                id,
                name: "Prior Conviction".to_string(),
    
                card_type: CardType::DealModifier { price_multiplier: 1.0, evidence: 20, cover: 0, heat: 5 },
            },
            // 2. Cover - Invite Only
            { id += 1; Card {
                id,
                name: "Invite Only".to_string(),
    
                card_type: CardType::DealModifier { price_multiplier: 1.0, evidence: 0, cover: 15, heat: -5 },
            }},
            // 3. Location - Safe (Locker Room)
            { id += 1; Card {
                id,
                name: "Locker Room".to_string(),
    
                card_type: CardType::Location { evidence: 5, cover: 20, heat: -5 },
            }},
            // 4. Location - Risky (Frat House)
            { id += 1; Card {
                id,
                name: "Frat House".to_string(),
    
                card_type: CardType::Location { evidence: 15, cover: 15, heat: 10 },
            }},
            // 5. Price Up - Invite More People
            { id += 1; Card {
                id,
                name: "Invite More People".to_string(),
    
                card_type: CardType::DealModifier { price_multiplier: 1.5, evidence: 15, cover: 0, heat: 10 },
            }},
            // 6. Price Down - Second Supplier
            { id += 1; Card {
                id,
                name: "Second Supplier".to_string(),
    
                card_type: CardType::DealModifier { price_multiplier: 0.7, evidence: 5, cover: 0, heat: -5 },
            }},
            // 7. Heat - Noise Complaint
            { id += 1; Card {
                id,
                name: "Noise Complaint".to_string(),
    
                card_type: CardType::DealModifier { price_multiplier: 1.0, evidence: 5, cover: 0, heat: 20 },
            }},
        ],
        scenarios: vec![
            // Scenario A: Get Wild
            BuyerScenario {
                id: "get_wild".to_string(),
                display_name: "Get Wild".to_string(),
                products: vec!["Weed".to_string(), "Coke".to_string()],
                locations: vec!["Frat House".to_string(), "Locker Room".to_string(), "Park".to_string()],
                heat_threshold: None, // Fearless - knows it's risky, willing to take it
                description: "Chaotic party energy, maximum wildness".to_string(),
                special_rules: vec![],
            },
            // Scenario B: Get Laid
            BuyerScenario {
                id: "get_laid".to_string(),
                display_name: "Get Laid".to_string(),
                products: vec!["Weed".to_string(), "Ecstasy".to_string()],
                locations: vec!["Frat House".to_string(), "Locker Room".to_string(), "Dorm".to_string()],
                heat_threshold: Some(35), // Cautious - not worth getting busted for romance
                description: "Social connection party, vibes over chaos".to_string(),
                special_rules: vec![],
            },
        ],
        active_scenario_index: None, // Set during Buyer selection
    }
}

/// Buyer Persona 2: Desperate Housewife
/// Low profit (×1.2), paranoid (Heat < 30), private only
fn create_stay_at_home_mom() -> BuyerPersona {
    let mut id = 2100; // Mom cards start at 2100

    BuyerPersona {
        id: "desperate_housewife".to_string(),
        display_name: "Desperate Housewife".to_string(),
        demand: BuyerDemand {
            products: vec!["Pills".to_string()],
            locations: vec!["Private Residence".to_string(), "Warehouse".to_string()],
            description: "Wants Pills only, private locations only".to_string(),
        },
        base_multiplier: 1.2,
        reduced_multiplier: 1.0,
        heat_threshold: Some(30),  // Paranoid, bails if Heat > 30
        evidence_threshold: None,
        special_rules: vec![],
        reaction_deck: vec![
            // 1. Evidence - Neighborhood Watch
            Card {
                id,
                name: "Neighborhood Watch".to_string(),
    
                card_type: CardType::DealModifier { price_multiplier: 1.0, evidence: 15, cover: 0, heat: 5 },
            },
            // 2. Cover - Good Reputation
            { id += 1; Card {
                id,
                name: "Good Reputation".to_string(),
    
                card_type: CardType::DealModifier { price_multiplier: 1.0, evidence: 0, cover: 20, heat: -5 },
            }},
            // 3. Location - Safe (By the Pool)
            { id += 1; Card {
                id,
                name: "By the Pool".to_string(),
    
                card_type: CardType::Location { evidence: 5, cover: 25, heat: -10 },
            }},
            // 4. Location - Risky (At the Park)
            { id += 1; Card {
                id,
                name: "At the Park".to_string(),
    
                card_type: CardType::Location { evidence: 15, cover: 15, heat: 5 },
            }},
            // 5. Price Up - Obvious Wealth
            { id += 1; Card {
                id,
                name: "Obvious Wealth".to_string(),
    
                card_type: CardType::DealModifier { price_multiplier: 1.3, evidence: 10, cover: 5, heat: 5 },
            }},
            // 6. Price Down - "Alternative" Payment
            { id += 1; Card {
                id,
                name: "\"Alternative\" Payment".to_string(),
    
                card_type: CardType::DealModifier { price_multiplier: 0.5, evidence: 5, cover: 5, heat: 0 },
            }},
            // 7. Heat - Gossip Girl
            { id += 1; Card {
                id,
                name: "Gossip Girl".to_string(),
    
                card_type: CardType::DealModifier { price_multiplier: 1.0, evidence: 5, cover: 0, heat: 15 },
            }},
        ],
        scenarios: vec![
            // Scenario A: Rock Bottom
            BuyerScenario {
                id: "rock_bottom".to_string(),
                display_name: "Rock Bottom".to_string(),
                products: vec!["Codeine".to_string(), "Fentanyl".to_string()],
                locations: vec!["Private Residence".to_string(), "By the Pool".to_string()],
                heat_threshold: Some(40), // Addicted - will take risks for her fix
                description: "Severe addiction, desperate for relief".to_string(),
                special_rules: vec![],
            },
            // Scenario B: In Denial
            BuyerScenario {
                id: "in_denial".to_string(),
                display_name: "In Denial".to_string(),
                products: vec!["Codeine".to_string(), "Weed".to_string()],
                locations: vec!["Private Residence".to_string(), "By the Pool".to_string(), "At the Park".to_string()],
                heat_threshold: Some(25), // Panics quickly - "I'm not a drug user!"
                description: "Managing anxiety, denying the problem".to_string(),
                special_rules: vec![],
            },
        ],
        active_scenario_index: None,
    }
}

/// Buyer Persona 3: Wall Street Wolf
/// Highest profit (×2.8), very paranoid (Heat < 25), private only
fn create_executive() -> BuyerPersona {
    let mut id = 2200; // Executive cards start at 2200

    BuyerPersona {
        id: "wall_street_wolf".to_string(),
        display_name: "Wall Street Wolf".to_string(),
        demand: BuyerDemand {
            products: vec!["Pills".to_string()],
            locations: vec!["Private Residence".to_string(), "Office".to_string()],
            description: "Wants premium Pills, private only, very paranoid".to_string(),
        },
        base_multiplier: 2.8,  // Highest profit in game
        reduced_multiplier: 1.0,
        heat_threshold: Some(25),  // Very paranoid, bails easily
        evidence_threshold: None,
        special_rules: vec![],
        reaction_deck: vec![
            // 1. Evidence - Invincibility Complex
            Card {
                id,
                name: "Invincibility Complex".to_string(),
    
                card_type: CardType::DealModifier { price_multiplier: 1.0, evidence: 15, cover: 0, heat: 10 },
            },
            // 2. Cover - Friends in High Places
            { id += 1; Card {
                id,
                name: "Friends in High Places".to_string(),
    
                card_type: CardType::DealModifier { price_multiplier: 1.0, evidence: 0, cover: 25, heat: -10 },
            }},
            // 3. Location - Safe (In a Limo)
            { id += 1; Card {
                id,
                name: "In a Limo".to_string(),
    
                card_type: CardType::Location { evidence: 5, cover: 30, heat: -10 },
            }},
            // 4. Location - Risky (Parking Lot)
            { id += 1; Card {
                id,
                name: "Parking Lot".to_string(),
    
                card_type: CardType::Location { evidence: 15, cover: 20, heat: 5 },
            }},
            // 5. Price Up - Stressful Day
            { id += 1; Card {
                id,
                name: "Stressful Day".to_string(),
    
                card_type: CardType::DealModifier { price_multiplier: 1.8, evidence: 5, cover: 10, heat: 5 },
            }},
            // 6. Price Down - Negotiation
            { id += 1; Card {
                id,
                name: "Negotiation".to_string(),
    
                card_type: CardType::DealModifier { price_multiplier: 0.8, evidence: 0, cover: 15, heat: -5 },
            }},
            // 7. Heat - Sketchy Business
            { id += 1; Card {
                id,
                name: "Sketchy Business".to_string(),
    
                card_type: CardType::DealModifier { price_multiplier: 1.0, evidence: 10, cover: 5, heat: 20 },
            }},
        ],
        scenarios: vec![
            // Scenario A: Desperate Times
            BuyerScenario {
                id: "desperate_times".to_string(),
                display_name: "Desperate Times".to_string(),
                products: vec!["Ice".to_string(), "Codeine".to_string()],
                locations: vec!["In a Limo".to_string(), "Office".to_string(), "Parking Lot".to_string()],
                heat_threshold: Some(45), // Desperate - will risk everything for the edge
                description: "Performance under pressure, needs the edge".to_string(),
                special_rules: vec![],
            },
            // Scenario B: Adrenaline Junkie
            BuyerScenario {
                id: "adrenaline_junkie".to_string(),
                display_name: "Adrenaline Junkie".to_string(),
                products: vec!["Ice".to_string(), "Coke".to_string()],
                locations: vec!["Parking Lot".to_string(), "In a Limo".to_string()],
                heat_threshold: Some(30), // Moderately cautious - thrill-seeking but not stupid
                description: "Calculated risk-taking, chasing the rush".to_string(),
                special_rules: vec![],
            },
        ],
        active_scenario_index: None,
    }
}
