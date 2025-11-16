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
    pub fn calculate_totals(&self, include_current_round: bool) -> Totals {
        let mut totals = Totals::default();
        let mut price_multiplier: f32 = 1.0;

        // Get base Evidence/Cover from active Location (player or dealer)
        if let Some(location) = self.active_location(include_current_round) {
            match location.card_type {
                CardType::Location { evidence, cover, heat } => {
                    totals.evidence = evidence;
                    totals.cover = cover;
                    totals.heat += heat;
                }
                _ => {} // Shouldn't happen
            }
        }

        for card in self.get_cards_for_calculation(include_current_round) {
            match card.card_type {
                CardType::Evidence { evidence, heat } => {
                    totals.evidence += evidence;
                    totals.heat += heat;
                }
                CardType::Cover { cover, heat } => {
                    totals.cover += cover;
                    totals.heat += heat;
                }
                CardType::DealModifier { price_multiplier: multiplier, evidence, cover, heat } => {
                    price_multiplier *= multiplier;
                    totals.evidence = totals.evidence.saturating_add_signed(evidence);
                    totals.cover = totals.cover.saturating_add_signed(cover);
                    totals.heat += heat;
                }
                CardType::Insurance { cover, .. } => {
                    totals.cover += cover;
                }
                CardType::Conviction { .. } => {}
                _ => {}
            }
        }

        // Get profit from active Product (apply multipliers)
        if let Some(product) = self.active_product(include_current_round) {
            if let CardType::Product { price, heat } = product.card_type {
                let buyer_multiplier = self.get_profit_multiplier();
                totals.profit = (price as f32 * price_multiplier * buyer_multiplier) as u32;
                totals.heat += heat;
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
}
