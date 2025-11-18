// Narrative story generation test - generates all possible variations for review
// Run with: cargo test test_all_story_variations -- --ignored --nocapture

#[cfg(test)]
mod tests {
    use crate::models::narrative::StoryComposer;
    use crate::models::hand_state::HandOutcome;
    use crate::models::card::Card;
    use crate::assets::GameAssets;
    use std::fs;

    #[test]
    #[ignore] // Only run explicitly with --ignored
    fn test_all_story_variations() {
        println!("\n{}", "=".repeat(80));
        println!("NARRATIVE STORY VARIATION TEST");
        println!("Generating all possible story combinations for review");
        println!("{}\n", "=".repeat(80));

        // Load actual game assets
        let assets = load_test_assets();

        let mut all_stories = Vec::new();
        let composer = StoryComposer::new(assets.narrative_defaults.clone());

        // Test each buyer scenario
        for buyer in &assets.buyers {
            for (scenario_idx, scenario) in buyer.scenarios.iter().enumerate() {
                println!("\n--- Buyer: {} - Scenario: {} ---", buyer.display_name, scenario.display_name);

                // Test each product
                for (product_name, product_card) in &assets.products {

                    // Test each location (player deck)
                    for (location_name, location_card) in &assets.locations {

                        // Test with 0 evidence, 1 evidence, and 2 evidence cards
                        // For 1+ evidence, test EACH evidence card individually
                        let evidence_configs: Vec<Vec<usize>> = {
                            let mut configs = vec![vec![]]; // No evidence

                            // Add configs for each individual evidence card
                            for i in 0..assets.evidence.len() {
                                configs.push(vec![i]); // Single evidence card
                            }

                            configs
                        };

                        for evidence_indices in evidence_configs {
                            let mut cards_played = vec![product_card.clone(), location_card.clone()];

                            // Add evidence cards
                            for &idx in &evidence_indices {
                                if let Some(evidence) = assets.evidence.get(idx) {
                                    cards_played.push(evidence.clone());
                                }
                            }

                            // Test each outcome
                            for outcome in [HandOutcome::Safe, HandOutcome::Busted, HandOutcome::Folded, HandOutcome::BuyerBailed] {
                                // Generate multiple iterations to capture all fragment variations
                                // Run 50 times to ensure we hit random fragment combinations
                                for _ in 0..50 {
                                    let story = composer.compose_story(Some(scenario), &cards_played, outcome);

                                    let evidence_desc = if evidence_indices.is_empty() {
                                        "None".to_string()
                                    } else {
                                        format!("{:?}", evidence_indices)
                                    };

                                    let context = format!(
                                        "Buyer={} Scenario={} Product={} Location={} Evidence={} Outcome={:?}",
                                        buyer.display_name, scenario.display_name, product_name, location_name, evidence_desc, outcome
                                    );

                                    all_stories.push((context, story.clone()));
                                }

                                // Print sample (every 10000th story to avoid spam)
                                if all_stories.len() % 10000 == 0 {
                                    println!("  [{}] stories generated...", all_stories.len());
                                }
                            }
                        }
                    }
                }
            }
        }

        println!("\n{}", "=".repeat(80));
        println!("Generated {} unique story variations", all_stories.len());
        println!("{}\n", "=".repeat(80));

        // Write all stories to file for review (just the stories, one per line)
        let mut unique_stories: Vec<String> = all_stories.iter()
            .map(|(_, story)| story.clone())
            .collect();

        // Sort and deduplicate
        unique_stories.sort();
        unique_stories.dedup();

        let output = unique_stories.join("\n");

        fs::write("story_variations.txt", output).expect("Failed to write story variations");
        println!("✅ {} unique stories written to story_variations.txt (sorted, one per line)", unique_stories.len());

        // Look for problems
        let problems = all_stories.iter()
            .filter(|(_, story)| {
                story.contains("product") ||  // Fallback text
                story.contains("Someone") ||  // Fallback subject
                story.contains("we met up") || // Fallback location
                story.contains("wanted a deal") // Fallback need
            })
            .collect::<Vec<_>>();

        println!("\n⚠️  Found {} stories with fallback text:", problems.len());
        for (ctx, story) in problems.iter().take(10) {
            println!("  {}", ctx);
            println!("    Story: {}\n", story);
        }

        if !problems.is_empty() {
            println!("❌ Test reveals {} stories using fallbacks - fragments missing!", problems.len());
        } else {
            println!("✅ All stories use actual fragments - no fallbacks!");
        }
    }

    fn load_test_assets() -> GameAssets {
        let mut assets = GameAssets::default();

        // Load narrative defaults first
        match load_narrative_defaults("assets/narrative_defaults.ron") {
            Ok(defaults) => {
                assets.narrative_defaults = defaults;
            }
            Err(e) => eprintln!("Warning: Failed to load defaults: {}", e),
        }

        // Load actual assets from files (no default merging - StoryComposer handles fallback)
        match load_cards("assets/cards/products.ron") {
            Ok(cards) => {
                for card in &cards {
                    assets.products.insert(card.name.clone(), card.clone());
                }
            }
            Err(e) => panic!("Failed to load products: {}", e),
        }

        match load_cards("assets/cards/locations.ron") {
            Ok(cards) => {
                for card in &cards {
                    assets.locations.insert(card.name.clone(), card.clone());
                }
            }
            Err(e) => panic!("Failed to load locations: {}", e),
        }

        match load_cards("assets/cards/evidence.ron") {
            Ok(cards) => {
                assets.evidence = cards;
            }
            Err(e) => panic!("Failed to load evidence: {}", e),
        }

        match load_buyers("assets/buyers.ron") {
            Ok(buyers) => {
                assets.buyers = buyers;
            }
            Err(e) => panic!("Failed to load buyers: {}", e),
        }

        assets
    }

    fn load_cards(path: &str) -> Result<Vec<Card>, String> {
        let content = std::fs::read_to_string(path)
            .map_err(|e| format!("Failed to read {}: {}", path, e))?;
        ron::from_str::<Vec<Card>>(&content)
            .map_err(|e| format!("Failed to parse {}: {}", path, e))
    }

    fn load_buyers(path: &str) -> Result<Vec<crate::models::buyer::BuyerPersona>, String> {
        let content = std::fs::read_to_string(path)
            .map_err(|e| format!("Failed to read {}: {}", path, e))?;
        ron::from_str::<Vec<crate::models::buyer::BuyerPersona>>(&content)
            .map_err(|e| format!("Failed to parse {}: {}", path, e))
    }

    fn load_narrative_defaults(path: &str) -> Result<crate::models::narrative::NarrativeFragments, String> {
        let content = std::fs::read_to_string(path)
            .map_err(|e| format!("Failed to read {}: {}", path, e))?;
        ron::from_str(&content)
            .map_err(|e| format!("Failed to parse {}: {}", path, e))
    }
}
