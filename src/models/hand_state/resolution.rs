// Hand resolution implementation for HandState

use super::*;

impl HandState {
    /// Resolve hand outcome (bust check with insurance/conviction)
    ///
    /// Resolution Order:
    /// 1. Evidence ≤ Cover → Safe (no insurance/conviction checks needed)
    /// 2. Evidence > Cover → Check Conviction:
    ///    - Conviction active AND current_heat >= threshold → Busted (override insurance)
    /// 3. Check Insurance:
    ///    - No insurance → Busted
    ///    - Has insurance, can't afford → Busted
    ///    - Has insurance, can afford → Pay cost, gain heat_penalty, burn insurance → Safe
    ///
    /// Post-resolution:
    /// - Safe outcome: Bank profit to cash (for future insurance affordability)
    /// - All outcomes: Accumulate totals.heat to current_heat (for conviction thresholds)
    pub fn resolve_hand(&mut self) -> HandOutcome {
        // Check 1: Validity (must have Product AND Location)
        if !self.is_valid_deal() {
            println!("Invalid deal: Must play at least 1 Product AND 1 Location");
            self.outcome = Some(HandOutcome::InvalidDeal);
            self.current_state = HandPhase::Bust;
            return HandOutcome::InvalidDeal;
        }

        // Check 2: Buyer bail (threshold exceeded)
        if self.should_buyer_bail() {
            if let Some(persona) = &self.buyer_persona {
                println!("Buyer ({}) bailed! Threshold exceeded", persona.display_name);
            }
            self.outcome = Some(HandOutcome::BuyerBailed);
            self.current_state = HandPhase::Bust;
            return HandOutcome::BuyerBailed;
        }

        let totals = self.calculate_totals(true); // Always include all cards at resolution

        // Calculate projected heat (current heat + this hand's heat)
        // This is what heat will be AFTER this hand, used for conviction checks
        let projected_heat = self.current_heat.saturating_add(totals.heat as u32);

        // Step 3: Evidence ≤ Cover → Safe (tie goes to player)
        let outcome = if totals.evidence <= totals.cover {
            HandOutcome::Safe
        } else {
            // Evidence > Cover → Potential bust, check insurance/conviction

            // Step 2: Check Conviction override (using PROJECTED heat after this hand)
            if let Some(conviction) = self.active_conviction(true) {
                if let CardType::Conviction { heat_threshold } = conviction.card_type {
                    if projected_heat >= heat_threshold {
                        // Conviction overrides insurance - run ends
                        HandOutcome::Busted
                    } else {
                        // Heat below threshold, conviction doesn't activate
                        self.try_insurance_activation()
                    }
                } else {
                    self.try_insurance_activation()
                }
            } else {
                // No conviction active
                self.try_insurance_activation()
            }
        };

        // Post-resolution: Accumulate cash and heat
        match outcome {
            HandOutcome::Safe => {
                // Bank profit to cash (for future insurance purchases)
                self.cash += totals.profit;
                // RFC-016: Track profit for account-wide cash accumulation
                self.last_profit = totals.profit;
            }
            HandOutcome::Busted => {
                // No cash gained on bust
                self.last_profit = 0;
            }
            HandOutcome::Folded => {
                self.last_profit = 0;
            }
            HandOutcome::InvalidDeal | HandOutcome::BuyerBailed => {
                // No cash gained on invalid deal or buyer bail (deal didn't complete)
                self.last_profit = 0;
            }
        }

        // Always accumulate heat (can't go below 0)
        if totals.heat >= 0 {
            self.current_heat = self.current_heat.saturating_add(totals.heat as u32);
        } else {
            self.current_heat = self.current_heat.saturating_sub((-totals.heat) as u32);
        }

        self.outcome = Some(outcome);
        self.current_state = HandPhase::Bust; // Transition to terminal state
        outcome
    }

    /// Try to activate insurance (Step 3 of resolution order)
    ///
    /// Returns:
    /// - Safe: Insurance activated (cost paid, heat gained, card burned)
    /// - Busted: No insurance OR can't afford
    fn try_insurance_activation(&mut self) -> HandOutcome {
        // Extract insurance values first to avoid borrow issues
        let insurance_info = self.active_insurance(true).and_then(|insurance| {
            if let CardType::Insurance { cost, heat_penalty, .. } = insurance.card_type {
                Some((insurance.name.clone(), cost, heat_penalty))
            } else {
                None
            }
        });

        if let Some((insurance_name, cost, heat_penalty)) = insurance_info {
            // Check affordability
            if self.cash >= cost {
                // Activate insurance: pay cost, gain heat penalty
                self.cash -= cost;
                self.current_heat = self.current_heat.saturating_add(heat_penalty as u32);

                // Burn insurance card (remove from deck permanently)
                self.cards_mut(Owner::Player).deck.retain(|card| card.name != insurance_name);

                HandOutcome::Safe
            } else {
                // Can't afford insurance
                HandOutcome::Busted
            }
        } else {
            // No insurance active
            HandOutcome::Busted
        }
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
    // Bust Check & Resolution Tests
    // ========================================================================

    #[test]
    fn test_bust_evidence_greater_than_cover() {
        let mut hand_state = HandState::default();

        hand_state.cards_played.push(create_product("Weed", 30, 5));
        hand_state.cards_played.push(create_location("School Zone", 40, 5, 20));
        hand_state.cards_played.push(create_evidence("Surveillance", 20, 5));

        // Totals: Evidence 60, Cover 5 → Busted
        let outcome = hand_state.resolve_hand();
        assert_eq!(outcome, HandOutcome::Busted);
        assert_eq!(hand_state.outcome, Some(HandOutcome::Busted));
        assert_eq!(hand_state.current_state, HandPhase::Bust);
    }

    #[test]
    fn test_safe_evidence_less_than_cover() {
        let mut hand_state = HandState::default();

        hand_state.cards_played.push(create_product("Weed", 30, 5));
        hand_state.cards_played.push(create_location("Safe House", 10, 30, -5));
        hand_state.cards_played.push(create_cover("Alibi", 30, -5));

        // Totals: Evidence 10, Cover 60 → Safe
        let outcome = hand_state.resolve_hand();
        assert_eq!(outcome, HandOutcome::Safe);
        assert_eq!(hand_state.outcome, Some(HandOutcome::Safe));
        assert_eq!(hand_state.current_state, HandPhase::Bust);
    }

    #[test]
    fn test_tie_goes_to_player() {
        let mut hand_state = HandState::default();

        hand_state.cards_played.push(create_product("Weed", 30, 5));
        hand_state.cards_played.push(create_location("Location", 30, 30, 0));

        // Totals: Evidence 30, Cover 30 → Safe (tie goes to player)
        let outcome = hand_state.resolve_hand();
        assert_eq!(outcome, HandOutcome::Safe);
        assert_eq!(hand_state.outcome, Some(HandOutcome::Safe));
    }

    #[test]
    fn test_insurance_activation_affordable() {
        let mut hand_state = HandState::default();
        let initial_cash = 1500;
        hand_state.cash = initial_cash;

        let product = create_product("Weed", 30, 5);
        let insurance = create_insurance("Plea Bargain", 5, 1000, 20);

        hand_state.cards_played.push(product.clone());
        hand_state.cards_played.push(create_location("Location", 30, 20, 0));
        hand_state.cards_played.push(insurance.clone());

        // Evidence > Cover, but insurance should save us
        let outcome = hand_state.resolve_hand();
        assert_eq!(outcome, HandOutcome::Safe);

        // Calculate expected values from card data
        let product_price = if let CardType::Product { price, .. } = product.card_type { price } else { 0 };
        let insurance_cost = if let CardType::Insurance { cost, .. } = insurance.card_type { cost } else { 0 };
        let expected_cash = initial_cash - insurance_cost + product_price;
        assert_eq!(hand_state.cash, expected_cash);

        let product_heat = if let CardType::Product { heat, .. } = product.card_type { heat } else { 0 };
        let insurance_heat = if let CardType::Insurance { heat_penalty, .. } = insurance.card_type { heat_penalty as i32 } else { 0 };
        let expected_heat = (product_heat + insurance_heat) as u32;
        assert_eq!(hand_state.current_heat, expected_heat);
    }

    #[test]
    fn test_conviction_overrides_insurance() {
        let mut hand_state = HandState::default();
        hand_state.cash = 2000;
        hand_state.current_heat = 50;

        hand_state.cards_played.push(create_product("Weed", 30, 5));
        hand_state.cards_played.push(create_location("Location", 30, 20, 0));
        hand_state.cards_played.push(create_insurance("Plea Bargain", 5, 1000, 20));
        hand_state.cards_played.push(create_conviction("Warrant", 40));

        // Evidence > Cover, conviction overrides insurance → Busted
        let outcome = hand_state.resolve_hand();
        assert_eq!(outcome, HandOutcome::Busted);
        assert_eq!(hand_state.cash, 2000); // Cash unchanged (conviction blocked insurance)
    }

    #[test]
    fn test_cash_accumulation_safe_hands() {
        let mut hand_state = HandState::default();
        hand_state.cash = 100;

        hand_state.cards_played.push(create_location("Location", 20, 30, 0));
        hand_state.cards_played.push(create_product("Weed", 50, 5));

        // Safe outcome, profit should be banked
        let outcome = hand_state.resolve_hand();
        assert_eq!(outcome, HandOutcome::Safe);
        assert_eq!(hand_state.cash, 150); // 100 + 50 profit
    }

    #[test]
    fn test_heat_accumulation_across_hands() {
        let mut hand_state = HandState::default();
        hand_state.current_heat = 10;

        hand_state.cards_played.push(create_location("Location", 20, 30, 15));
        hand_state.cards_played.push(create_product("Weed", 50, 5));

        hand_state.resolve_hand();

        // Heat should accumulate: 10 + 15 + 5 = 30
        assert_eq!(hand_state.current_heat, 30);
    }

    #[test]
    fn test_conviction_uses_projected_heat() {
        let mut hand_state = HandState::default();
        let initial_cash = 2000;
        let initial_heat = 40;
        hand_state.cash = initial_cash;
        hand_state.current_heat = initial_heat;

        let product = create_product("Weed", 30, 5);
        let location = create_location("Location", 30, 20, 30);

        hand_state.cards_played.push(product.clone());
        hand_state.cards_played.push(location.clone());
        hand_state.cards_played.push(create_insurance("Plea Bargain", 5, 1000, 20));
        hand_state.cards_played.push(create_conviction("DA Approval", 60));

        // Test that conviction uses projected heat to determine if it activates
        let outcome = hand_state.resolve_hand();
        assert_eq!(outcome, HandOutcome::Busted); // Conviction blocks insurance
        assert_eq!(hand_state.cash, initial_cash); // Cash unchanged (conviction blocked insurance)

        // Calculate expected heat from card data
        let product_heat = if let CardType::Product { heat, .. } = product.card_type { heat } else { 0 };
        let location_heat = if let CardType::Location { heat, .. } = location.card_type { heat } else { 0 };
        let expected_heat = ((initial_heat as i32) + product_heat + location_heat) as u32;
        assert_eq!(hand_state.current_heat, expected_heat);
    }
}
