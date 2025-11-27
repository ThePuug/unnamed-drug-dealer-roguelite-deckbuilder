// Card interaction engine implementation for HandState

use super::*;

impl HandState {
    /// Helper to get cards for calculation based on include_current_round flag
    fn get_cards_for_calculation(&self, include_current_round: bool) -> Vec<&Card> {
        if include_current_round {
            self.cards_played.iter().chain(self.cards_played_this_round.iter()).collect()
        } else {
            self.cards_played.iter().collect()
        }
    }

    /// Get active Product card (last Product played, if any)
    /// Override rule: Only last Product matters
    pub fn active_product(&self, include_current_round: bool) -> Option<&Card> {
        self.get_cards_for_calculation(include_current_round)
            .into_iter()
            .rev()
            .find(|card| matches!(card.card_type, CardType::Product { .. }))
    }

    /// Get active Location card (last Location played, required)
    /// Override rule: Only last Location matters
    pub fn active_location(&self, include_current_round: bool) -> Option<&Card> {
        self.get_cards_for_calculation(include_current_round)
            .into_iter()
            .rev()
            .find(|card| matches!(card.card_type, CardType::Location { .. }))
    }

    /// Get active Insurance card (last Insurance played, if any)
    /// Override rule: Only last Insurance matters
    pub fn active_insurance(&self, include_current_round: bool) -> Option<&Card> {
        self.get_cards_for_calculation(include_current_round)
            .into_iter()
            .rev()
            .find(|card| matches!(card.card_type, CardType::Insurance { .. }))
    }

    /// Get active Conviction card (last Conviction played, if any)
    /// Override rule: Only last Conviction matters
    pub fn active_conviction(&self, include_current_round: bool) -> Option<&Card> {
        self.get_cards_for_calculation(include_current_round)
            .into_iter()
            .rev()
            .find(|card| matches!(card.card_type, CardType::Conviction { .. }))
    }

    /// Calculate current totals from all played cards
    ///
    /// Override rules:
    /// - Last Product played becomes active (previous discarded)
    /// - Last Location played becomes active (Evidence/Cover base changes)
    /// - Last Insurance played becomes active
    /// - Last Conviction played becomes active
    ///
    /// Additive rules:
    /// - Evidence = Location base + sum(all Evidence cards + DealModifier evidence)
    /// - Cover = Location base + sum(all Cover cards + Insurance cover + DealModifier cover)
    /// - Heat = sum(all heat modifiers from all cards)
    /// - Profit = Active Product price × product(all DealModifier price_multiplier)
    ///
    /// Special rules:
    /// - Insurance acts as Cover card during totals calculation
    /// - Conviction has no effect on totals (only affects bust resolution)
    ///
    /// RFC-019 Upgrade Stat Choice:
    /// - Players choose which stat to upgrade at each tier
    /// - Each upgrade adds +10% to the chosen stat (additive stacking)
    /// - "Good" stats increase (Price, Cover, PriceMultiplier)
    /// - "Bad" stats decrease (Evidence, Heat, HeatPenalty)
    pub fn calculate_totals(&self, include_current_round: bool) -> Totals {
        use crate::save::UpgradeableStat;

        let mut totals = Totals::default();
        let mut price_multiplier: f32 = 1.0;

        // Get base Evidence/Cover from active Location (player or dealer)
        if let Some(location) = self.active_location(include_current_round) {
            match location.card_type {
                CardType::Location { evidence, cover, heat } => {
                    // RFC-019: Apply per-stat upgrade multipliers
                    let evidence_mult = 2.0 - self.get_stat_multiplier(&location.name, UpgradeableStat::Evidence); // Decrease
                    let cover_mult = self.get_stat_multiplier(&location.name, UpgradeableStat::Cover);
                    let heat_mult = 2.0 - self.get_stat_multiplier(&location.name, UpgradeableStat::Heat); // Decrease

                    totals.evidence = (evidence as f32 * evidence_mult).max(0.0) as u32;
                    totals.cover = (cover as f32 * cover_mult) as u32;
                    totals.heat += (heat as f32 * heat_mult) as i32;
                }
                _ => {} // Shouldn't happen
            }
        }

        for card in self.get_cards_for_calculation(include_current_round) {
            match card.card_type {
                CardType::Evidence { evidence, heat } => {
                    // RFC-018: Apply Narc upgrade tier to Evidence cards based on Heat
                    // (Evidence cards are Narc cards, not player-upgradeable)
                    let narc_mult = self.narc_upgrade_tier.multiplier();
                    let upgraded_evidence = (evidence as f32 * narc_mult) as u32;
                    let upgraded_heat = (heat as f32 * narc_mult) as i32;
                    totals.evidence += upgraded_evidence;
                    totals.heat += upgraded_heat;
                }
                CardType::Cover { cover, heat } => {
                    // RFC-019: Apply per-stat upgrade multipliers
                    let cover_mult = self.get_stat_multiplier(&card.name, UpgradeableStat::Cover);
                    let heat_mult = 2.0 - self.get_stat_multiplier(&card.name, UpgradeableStat::Heat); // Decrease

                    let upgraded_cover = (cover as f32 * cover_mult) as u32;
                    totals.cover += upgraded_cover;
                    totals.heat += (heat as f32 * heat_mult) as i32;
                }
                CardType::DealModifier { price_multiplier: multiplier, evidence, cover, heat } => {
                    // RFC-019: Apply per-stat upgrade multipliers
                    let price_mult = self.get_stat_multiplier(&card.name, UpgradeableStat::PriceMultiplier);
                    let evidence_mult = 2.0 - self.get_stat_multiplier(&card.name, UpgradeableStat::Evidence); // Decrease
                    let cover_mult = self.get_stat_multiplier(&card.name, UpgradeableStat::Cover);
                    let heat_mult = 2.0 - self.get_stat_multiplier(&card.name, UpgradeableStat::Heat); // Decrease

                    let upgraded_multiplier = multiplier * price_mult;
                    price_multiplier *= upgraded_multiplier;
                    totals.evidence = totals.evidence.saturating_add_signed((evidence as f32 * evidence_mult) as i32);
                    totals.cover = totals.cover.saturating_add_signed((cover as f32 * cover_mult) as i32);
                    totals.heat += (heat as f32 * heat_mult) as i32;
                }
                CardType::Insurance { cover, .. } => {
                    // RFC-019: Apply per-stat upgrade multipliers
                    let cover_mult = self.get_stat_multiplier(&card.name, UpgradeableStat::Cover);
                    let upgraded_cover = (cover as f32 * cover_mult) as u32;
                    totals.cover += upgraded_cover;
                    // Note: HeatPenalty is only applied when insurance activates, not in totals
                }
                CardType::Conviction { .. } => {}
                _ => {}
            }
        }

        // Get profit from active Product (apply multipliers)
        if let Some(product) = self.active_product(include_current_round) {
            if let CardType::Product { price, heat } = product.card_type {
                // RFC-019: Apply per-stat upgrade multipliers
                let price_mult = self.get_stat_multiplier(&product.name, UpgradeableStat::Price);
                let heat_mult = 2.0 - self.get_stat_multiplier(&product.name, UpgradeableStat::Heat); // Decrease

                let upgraded_price = (price as f32 * price_mult) as u32;

                let buyer_multiplier = self.get_profit_multiplier();
                totals.profit = (upgraded_price as f32 * price_multiplier * buyer_multiplier) as u32;
                totals.heat += (heat as f32 * heat_mult) as i32;
            }
        }

        totals
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::test_helpers::*; // SOW-012: Use shared test helpers

    // ========================================================================
    // Card Interaction Engine Tests
    // ========================================================================

    #[test]
    fn test_override_product() {
        let mut hand_state = HandState::default();

        let weed = create_product("Weed", 30, 5);
        let meth = create_product("Meth", 100, 30);

        hand_state.cards_played.push(weed);
        hand_state.cards_played.push(meth.clone());

        // Active product should be Meth (last played)
        let active = hand_state.active_product(true);
        assert!(active.is_some());
        assert_eq!(active.unwrap().name, "Meth");

        // Totals should reflect Meth price, not Weed price
        let totals = hand_state.calculate_totals(true);
        let expected_profit = if let CardType::Product { price, .. } = meth.card_type { price } else { 0 };
        assert_eq!(totals.profit, expected_profit);
    }

    #[test]
    fn test_override_location() {
        let mut hand_state = HandState::default();

        let school_zone = create_location("School Zone", 40, 5, 20);
        let safe_house = create_location("Safe House", 10, 30, -5);

        hand_state.cards_played.push(school_zone);
        hand_state.cards_played.push(safe_house.clone());

        // Active location should be Safe House (last played)
        let active = hand_state.active_location(true);
        assert!(active.is_some());
        assert_eq!(active.unwrap().name, "Safe House");

        // Totals should reflect Safe House base (Evidence 10, Cover 30)
        let totals = hand_state.calculate_totals(true);
        assert_eq!(totals.evidence, 10);
        assert_eq!(totals.cover, 30);
    }

    #[test]
    fn test_additive_evidence() {
        let mut hand_state = HandState::default();

        hand_state.cards_played.push(create_location("Safe House", 10, 30, -5));
        hand_state.cards_played.push(create_evidence("Patrol", 5, 2));
        hand_state.cards_played.push(create_evidence("Surveillance", 20, 5));

        // Evidence should stack: 10 (location) + 5 (patrol) + 20 (surveillance) = 35
        let totals = hand_state.calculate_totals(true);
        assert_eq!(totals.evidence, 35);
    }

    #[test]
    fn test_additive_cover() {
        let mut hand_state = HandState::default();

        hand_state.cards_played.push(create_location("Safe House", 10, 30, -5));
        hand_state.cards_played.push(create_cover("Alibi", 30, -5));

        // Cover should stack: 30 (location) + 30 (alibi) = 60
        let totals = hand_state.calculate_totals(true);
        assert_eq!(totals.cover, 60);
    }

    #[test]
    fn test_heat_accumulation() {
        let mut hand_state = HandState::default();

        hand_state.cards_played.push(create_product("Meth", 100, 30));
        hand_state.cards_played.push(create_location("School Zone", 40, 5, 20));
        hand_state.cards_played.push(create_evidence("Surveillance", 20, 5));

        // Heat should accumulate: 30 + 20 + 5 = 55
        let totals = hand_state.calculate_totals(true);
        assert_eq!(totals.heat, 55);
    }

    #[test]
    fn test_no_product_played() {
        let mut hand_state = HandState::default();

        hand_state.cards_played.push(create_location("Safe House", 10, 30, -5));

        // Profit should be 0 (no Product played)
        let totals = hand_state.calculate_totals(true);
        assert_eq!(totals.profit, 0);
    }

    #[test]
    fn test_complete_hand_scenario() {
        let mut hand_state = HandState::default();

        let location = create_location("Safe House", 10, 30, -5);
        let product = create_product("Meth", 100, 30);
        let cover = create_cover("Alibi", 30, -5);
        let evidence = create_evidence("Surveillance", 20, 5);

        hand_state.cards_played.push(location.clone());
        hand_state.cards_played.push(product.clone());
        hand_state.cards_played.push(cover.clone());
        hand_state.cards_played.push(evidence.clone());

        let totals = hand_state.calculate_totals(true);

        // Calculate expected values from card data
        let expected_evidence = if let CardType::Location { evidence: loc_ev, .. } = location.card_type { loc_ev } else { 0 }
            + if let CardType::Evidence { evidence: ev, .. } = evidence.card_type { ev } else { 0 };
        let expected_cover = if let CardType::Location { cover: loc_cov, .. } = location.card_type { loc_cov } else { 0 }
            + if let CardType::Cover { cover: cov, .. } = cover.card_type { cov } else { 0 };
        let expected_heat = if let CardType::Location { heat: h1, .. } = location.card_type { h1 } else { 0 }
            + if let CardType::Product { heat: h2, .. } = product.card_type { h2 } else { 0 }
            + if let CardType::Cover { heat: h3, .. } = cover.card_type { h3 } else { 0 }
            + if let CardType::Evidence { heat: h4, .. } = evidence.card_type { h4 } else { 0 };
        let expected_profit = if let CardType::Product { price, .. } = product.card_type { price } else { 0 };

        assert_eq!(totals.evidence, expected_evidence);
        assert_eq!(totals.cover, expected_cover);
        assert_eq!(totals.heat, expected_heat);
        assert_eq!(totals.profit, expected_profit);
    }

    #[test]
    fn test_insurance_acts_as_cover() {
        let mut hand_state = HandState::default();

        hand_state.cards_played.push(create_location("Location", 20, 20, 0));
        hand_state.cards_played.push(create_insurance("Fake ID", 15, 0, 40));

        // Totals: E:20 C:35 (20 + 15 from insurance)
        let totals = hand_state.calculate_totals(true);
        assert_eq!(totals.evidence, 20);
        assert_eq!(totals.cover, 35); // Insurance adds to cover
        assert_eq!(totals.heat, 0); // Insurance heat penalty only applies on activation, not in totals
    }

    #[test]
    fn test_conviction_no_effect_on_totals() {
        let mut hand_state = HandState::default();

        hand_state.cards_played.push(create_location("Location", 20, 20, 0));
        hand_state.cards_played.push(create_conviction("Warrant", 40));

        // Totals: E:20 C:20 (conviction doesn't change anything)
        let totals = hand_state.calculate_totals(true);
        assert_eq!(totals.evidence, 20);
        assert_eq!(totals.cover, 20);
        assert_eq!(totals.heat, 0);
    }

    // ========================================================================
    // RFC-019: Upgrade Stat Choice Tests
    // ========================================================================

    #[test]
    fn test_upgrade_stat_choice_price_on_product() {
        use crate::save::{CardUpgrades, UpgradeableStat};
        let mut hand_state = HandState::default();

        // Product with base price 100
        hand_state.cards_played.push(create_product("Test Product", 100, 0));
        hand_state.cards_played.push(create_location("Location", 10, 10, 0));

        // Without upgrades, profit = 100
        let totals_base = hand_state.calculate_totals(true);
        assert_eq!(totals_base.profit, 100);

        // RFC-019: Add one Price upgrade (+10%)
        let mut upgrades = CardUpgrades::new();
        upgrades.add_upgrade(UpgradeableStat::Price);
        hand_state.card_upgrades.insert("Test Product".to_string(), upgrades);

        // With 1 Price upgrade, profit = 110 (100 * 1.1)
        let totals_upgraded = hand_state.calculate_totals(true);
        assert_eq!(totals_upgraded.profit, 110);
    }

    #[test]
    fn test_upgrade_stat_choice_cover_on_cover_card() {
        use crate::save::{CardUpgrades, UpgradeableStat};
        let mut hand_state = HandState::default();

        // Cover card with base 30 cover
        hand_state.cards_played.push(create_location("Location", 10, 10, 0));
        hand_state.cards_played.push(create_cover("Test Cover", 30, 0));

        // Without upgrades, total cover = 10 (location) + 30 (cover) = 40
        let totals_base = hand_state.calculate_totals(true);
        assert_eq!(totals_base.cover, 40);

        // RFC-019: Add one Cover upgrade (+10%)
        let mut upgrades = CardUpgrades::new();
        upgrades.add_upgrade(UpgradeableStat::Cover);
        hand_state.card_upgrades.insert("Test Cover".to_string(), upgrades);

        // With 1 Cover upgrade, cover = 10 + 33 (30 * 1.1) = 43
        let totals_upgraded = hand_state.calculate_totals(true);
        assert_eq!(totals_upgraded.cover, 43);
    }

    #[test]
    fn test_upgrade_stat_choice_on_location() {
        use crate::save::{CardUpgrades, UpgradeableStat};
        let mut hand_state = HandState::default();

        // Location with base evidence 20, cover 20
        hand_state.cards_played.push(create_location("Test Location", 20, 20, 0));

        // Without upgrades, evidence = 20, cover = 20
        let totals_base = hand_state.calculate_totals(true);
        assert_eq!(totals_base.evidence, 20);
        assert_eq!(totals_base.cover, 20);

        // RFC-019: Player chooses to upgrade Cover (+10%)
        let mut upgrades = CardUpgrades::new();
        upgrades.add_upgrade(UpgradeableStat::Cover);
        hand_state.card_upgrades.insert("Test Location".to_string(), upgrades);

        // With 1 Cover upgrade: evidence = 20 (unchanged), cover = 22 (20 * 1.1)
        let totals_cover = hand_state.calculate_totals(true);
        assert_eq!(totals_cover.evidence, 20);
        assert_eq!(totals_cover.cover, 22);

        // Now also upgrade Evidence (-10%)
        hand_state.card_upgrades.get_mut("Test Location").unwrap()
            .add_upgrade(UpgradeableStat::Evidence);

        // With Cover+Evidence upgrades: evidence = 18 (20 * 0.9), cover = 22 (20 * 1.1)
        let totals_both = hand_state.calculate_totals(true);
        assert_eq!(totals_both.evidence, 18);
        assert_eq!(totals_both.cover, 22);
    }

    #[test]
    fn test_upgrade_stat_stacking() {
        use crate::save::{CardUpgrades, UpgradeableStat};
        let mut hand_state = HandState::default();

        // Product with base price 100
        hand_state.cards_played.push(create_product("Test Product", 100, 0));
        hand_state.cards_played.push(create_location("Location", 10, 10, 0));

        // RFC-019: Add 5 Price upgrades (+50% total, additive stacking)
        let mut upgrades = CardUpgrades::new();
        for _ in 0..5 {
            upgrades.add_upgrade(UpgradeableStat::Price);
        }
        hand_state.card_upgrades.insert("Test Product".to_string(), upgrades);

        // With 5 Price upgrades, profit = 150 (100 * 1.5)
        let totals = hand_state.calculate_totals(true);
        assert_eq!(totals.profit, 150);
    }

    #[test]
    fn test_upgrade_heat_reduction() {
        use crate::save::{CardUpgrades, UpgradeableStat};
        let mut hand_state = HandState::default();

        // Product with base price 100 and heat 20
        hand_state.cards_played.push(create_product("Test Product", 100, 20));
        hand_state.cards_played.push(create_location("Location", 10, 10, 0));

        // Without upgrades, heat = 20
        let totals_base = hand_state.calculate_totals(true);
        assert_eq!(totals_base.heat, 20);

        // RFC-019: Add one Heat upgrade (-10%)
        let mut upgrades = CardUpgrades::new();
        upgrades.add_upgrade(UpgradeableStat::Heat);
        hand_state.card_upgrades.insert("Test Product".to_string(), upgrades);

        // With 1 Heat upgrade, heat = 18 (20 * 0.9)
        let totals_upgraded = hand_state.calculate_totals(true);
        assert_eq!(totals_upgraded.heat, 18);
    }

    #[test]
    fn test_upgrade_no_effect_on_narc_cards() {
        use crate::save::{CardUpgrades, UpgradeableStat};
        let mut hand_state = HandState::default();

        // Evidence is a Narc card type (not player-upgradeable)
        hand_state.cards_played.push(create_location("Location", 10, 10, 0));
        hand_state.cards_played.push(create_evidence("Narc Evidence", 20, 0));

        // Player upgrades don't affect Narc cards (they use narc_upgrade_tier instead)
        let mut upgrades = CardUpgrades::new();
        upgrades.add_upgrade(UpgradeableStat::Evidence);
        hand_state.card_upgrades.insert("Narc Evidence".to_string(), upgrades);

        // With Base narc_upgrade_tier (default), evidence = 10 + 20 = 30
        // The player upgrade has no effect on Narc cards
        let totals = hand_state.calculate_totals(true);
        assert_eq!(totals.evidence, 30);
    }

    // ========================================================================
    // RFC-018: Narc Difficulty Scaling Tests
    // ========================================================================

    #[test]
    fn test_narc_tier_base_no_bonus() {
        let mut hand_state = HandState::default();
        hand_state.narc_upgrade_tier = crate::save::UpgradeTier::Base;

        hand_state.cards_played.push(create_location("Location", 10, 10, 0));
        hand_state.cards_played.push(create_evidence("Surveillance", 20, 5));

        let totals = hand_state.calculate_totals(true);
        // Evidence: 10 (location) + 20 (narc * 1.0) = 30
        // Heat: 0 (location) + 5 (narc * 1.0) = 5
        assert_eq!(totals.evidence, 30);
        assert_eq!(totals.heat, 5);
    }

    #[test]
    fn test_narc_tier1_bonus() {
        let mut hand_state = HandState::default();
        hand_state.narc_upgrade_tier = crate::save::UpgradeTier::Tier1; // +10%

        hand_state.cards_played.push(create_location("Location", 10, 10, 0));
        hand_state.cards_played.push(create_evidence("Surveillance", 20, 10));

        let totals = hand_state.calculate_totals(true);
        // Evidence: 10 (location) + 22 (20 * 1.1) = 32
        // Heat: 0 (location) + 11 (10 * 1.1) = 11
        assert_eq!(totals.evidence, 32);
        assert_eq!(totals.heat, 11);
    }

    #[test]
    fn test_narc_tier2_bonus() {
        let mut hand_state = HandState::default();
        hand_state.narc_upgrade_tier = crate::save::UpgradeTier::Tier2; // +20%

        hand_state.cards_played.push(create_location("Location", 10, 10, 0));
        hand_state.cards_played.push(create_evidence("Surveillance", 20, 10));

        let totals = hand_state.calculate_totals(true);
        // Evidence: 10 (location) + 24 (20 * 1.2) = 34
        // Heat: 0 (location) + 12 (10 * 1.2) = 12
        assert_eq!(totals.evidence, 34);
        assert_eq!(totals.heat, 12);
    }

    #[test]
    fn test_narc_tier4_max_bonus() {
        let mut hand_state = HandState::default();
        hand_state.narc_upgrade_tier = crate::save::UpgradeTier::Tier4; // +40%

        hand_state.cards_played.push(create_location("Location", 10, 10, 0));
        hand_state.cards_played.push(create_evidence("Surveillance", 20, 10));

        let totals = hand_state.calculate_totals(true);
        // Evidence: 10 (location) + 28 (20 * 1.4) = 38
        // Heat: 0 (location) + 14 (10 * 1.4) = 14
        assert_eq!(totals.evidence, 38);
        assert_eq!(totals.heat, 14);
    }

    #[test]
    fn test_narc_tier_multiple_evidence_cards() {
        let mut hand_state = HandState::default();
        hand_state.narc_upgrade_tier = crate::save::UpgradeTier::Tier2; // +20%

        hand_state.cards_played.push(create_location("Location", 10, 10, 0));
        hand_state.cards_played.push(create_evidence("Patrol", 10, 2));
        hand_state.cards_played.push(create_evidence("Surveillance", 20, 5));

        let totals = hand_state.calculate_totals(true);
        // Evidence: 10 (location) + 12 (10 * 1.2) + 24 (20 * 1.2) = 46
        // Heat: 0 (location) + 2 (2 * 1.2 = 2.4 -> 2) + 6 (5 * 1.2) = 8
        assert_eq!(totals.evidence, 46);
        assert_eq!(totals.heat, 8);
    }
}
