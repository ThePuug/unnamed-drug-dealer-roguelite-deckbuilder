// SOW-013-B Phase 2: Buyer personas from assets
// Loads personas from GameAssets registry
// SOW-024: personas belong to areas (territories) - a run draws only from
// the run area's clientele

use crate::models::buyer::BuyerPersona;
use crate::assets::GameAssets;

/// SOW-013-B: Get all available Buyer personas from loaded assets (3 personas)
pub fn create_buyer_personas(assets: &GameAssets) -> Vec<BuyerPersona> {
    assets.buyers.clone()
}

/// SOW-024: The clientele of one area. Customers don't relocate when you
/// expand - unlocking an area buys ACCESS to the people already there.
pub fn personas_in_area<'a>(personas: &'a [BuyerPersona], area: &str) -> Vec<&'a BuyerPersona> {
    personas.iter().filter(|p| p.area == area).collect()
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::buyer::BuyerDemand;

    fn persona(name: &str, area: &str) -> BuyerPersona {
        BuyerPersona {
            area: area.to_string(),
            portrait: String::new(),
            display_name: name.to_string(),
            demand: BuyerDemand {
                products: vec![],
                locations: vec![],
                description: String::new(),
            },
            base_multiplier: 1.0,
            reduced_multiplier: 1.0,
            evidence_threshold: None,
            reaction_deck_ids: vec![],
            reaction_deck: vec![],
            scenarios: vec![],
            active_scenario_index: None,
        }
    }

    #[test]
    fn personas_filter_by_area() {
        let personas = vec![
            persona("Frat Bro", "trailer_park"),
            persona("Wall Street Wolf", "suburbia"),
            persona("Desperate Housewife", "trailer_park"),
        ];

        let corner: Vec<&str> = personas_in_area(&personas, "trailer_park")
            .iter()
            .map(|p| p.display_name.as_str())
            .collect();
        assert_eq!(corner, vec!["Frat Bro", "Desperate Housewife"]);

        let block: Vec<&str> = personas_in_area(&personas, "suburbia")
            .iter()
            .map(|p| p.display_name.as_str())
            .collect();
        assert_eq!(block, vec!["Wall Street Wolf"]);

        assert!(personas_in_area(&personas, "downtown").is_empty());
    }
}
