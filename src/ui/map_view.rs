// SOW-029: City Map view-model - pure presentation logic for the map
// overlay. Same rule as view.rs: everything here is unit-testable without
// ECS; systems/city_map.rs only orchestrates spawning from these values.

use crate::models::buyer::BuyerPersona;
use crate::models::card::Card;
use crate::models::shop_location::ShopLocationDef;
use crate::save::{DealerState, SaveData};

// ============================================================================
// Zone node card
// ============================================================================

/// Everything one map node renders, derived from save + content
#[derive(Debug, Clone, PartialEq)]
pub struct ZoneNodeView {
    pub area_id: String,
    pub name: String,
    /// Zone identity line ("STREET CRAFT — ...")
    pub identity: &'static str,
    /// Narc texture in fiction voice - stays visible on locked zones
    /// (risk is part of the pitch)
    pub narc_hint: &'static str,
    pub status: ZoneStatus,
    /// "Frat Bro ×2.5" per persona whose home is this area
    pub clientele: Vec<String>,
    /// "×1.5–×2.8" across this area's personas (None: no clientele)
    pub payout_band: Option<String>,
    /// Native shop product names, cheapest first
    pub products: Vec<String>,
    /// Dealers stationed here (empty on locked zones by construction -
    /// stations can only be unlocked areas)
    pub dealers: Vec<DealerChip>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ZoneStatus {
    Unlocked,
    Locked { price: u32, affordable: bool },
}

/// One stationed-dealer chip on a node
#[derive(Debug, Clone, PartialEq)]
pub struct DealerChip {
    pub dealer_index: usize,
    pub name: String,
    pub heat: u32,
    pub tier_name: &'static str,
    pub tier_color: (f32, f32, f32),
    pub cred: u32,
    /// This dealer's rep is the roster's best for the zone (mirrors the
    /// shop's "unlocked by <name>" credit line)
    pub is_best_cred: bool,
    /// Only available dealers can be picked up and sent elsewhere
    pub selectable: bool,
    /// Status suffix for the chip ("JAILED · 2 RUNS"), None when ready
    pub status_note: Option<String>,
}

// ============================================================================
// Zone flavor (code-side until Reed next touches shop_locations.ron -
// authoring these as content fields is flagged in the SOW Discussion)
// ============================================================================

pub fn zone_identity(area_id: &str) -> &'static str {
    match area_id {
        "the_corner" => "STREET CRAFT — WHERE IT ALL STARTS",
        "the_strip" => "CROWD CRAFT — CLUBS, CASH, NOISE",
        "the_block" => "MONEY CRAFT — PRIVATE & PREMIUM",
        _ => "NEW TERRITORY",
    }
}

pub fn narc_hint(area_id: &str) -> &'static str {
    match area_id {
        "the_corner" => "patrols & donut breaks",
        "the_strip" => "vice sweeps — loud but forgiving",
        "the_block" => "task force — they build cases",
        _ => "watchful eyes",
    }
}

// ============================================================================
// Derivations
// ============================================================================

fn fmt_mult(m: f32) -> String {
    format!("×{m:.1}")
}

/// "Frat Bro ×2.5" for each persona homed in `area_id`
pub fn clientele_lines(personas: &[BuyerPersona], area_id: &str) -> Vec<String> {
    personas
        .iter()
        .filter(|p| p.area == area_id)
        .map(|p| format!("{} {}", p.display_name, fmt_mult(p.base_multiplier)))
        .collect()
}

/// Payout band across an area's personas: "×1.5–×2.8", collapsing to a
/// single value when the band is flat. None when the area has no clientele.
pub fn payout_band(personas: &[BuyerPersona], area_id: &str) -> Option<String> {
    let mults: Vec<f32> = personas
        .iter()
        .filter(|p| p.area == area_id)
        .map(|p| p.base_multiplier)
        .collect();
    let min = mults.iter().copied().reduce(f32::min)?;
    let max = mults.iter().copied().reduce(f32::max)?;
    if (max - min).abs() < f32::EPSILON {
        Some(fmt_mult(max))
    } else {
        Some(format!("{}–{}", fmt_mult(min), fmt_mult(max)))
    }
}

/// Names of an area's native shop products, cheapest first (the family the
/// node advertises: "Weed · Shrooms · Codeine · Acid")
pub fn native_products<'a>(
    products: impl Iterator<Item = &'a Card>,
    area_id: &str,
) -> Vec<String> {
    let mut stocked: Vec<&Card> = products
        .filter(|c| c.shop_location.as_deref() == Some(area_id))
        .collect();
    // Starting-collection stock carries price 0 and sorts first (Weed leads
    // the Corner's family) - ties break alphabetically for determinism
    stocked.sort_by(|a, b| {
        a.shop_price
            .unwrap_or(0)
            .cmp(&b.shop_price.unwrap_or(0))
            .then_with(|| a.name.cmp(&b.name))
    });
    stocked.into_iter().map(|c| c.name.clone()).collect()
}

pub fn zone_status(area: &ShopLocationDef, save: &SaveData) -> ZoneStatus {
    if area.unlocked || save.account.unlocked_locations.contains(&area.id) {
        ZoneStatus::Unlocked
    } else {
        ZoneStatus::Locked {
            price: area.price,
            affordable: save.account.cash_on_hand >= area.price as u64,
        }
    }
}

fn chip_status_note(dealer: &DealerState) -> Option<String> {
    let plural = |n: u32| if n == 1 { "" } else { "S" };
    if let Some(runs) = dealer.jail_remaining() {
        Some(format!("JAILED · {} RUN{}", runs, plural(runs)))
    } else if let Some(runs) = dealer.relocating_remaining() {
        Some(format!("MOVING · {} RUN{}", runs, plural(runs)))
    } else if let Some(runs) = dealer.laying_low_remaining() {
        Some(format!("LAYING LOW · {} RUN{}", runs, plural(runs)))
    } else {
        None
    }
}

/// Chips for the dealers stationed in `area_id`
pub fn dealer_chips(save: &SaveData, area_id: &str) -> Vec<DealerChip> {
    let best = save.best_cred(area_id);
    save.dealers
        .iter()
        .enumerate()
        .filter(|(_, d)| d.station == area_id)
        .map(|(index, d)| {
            let tier = d.character.heat_tier();
            DealerChip {
                dealer_index: index,
                name: d.name.clone(),
                heat: d.character.heat,
                tier_name: tier.name(),
                tier_color: tier.color(),
                cred: d.cred_in(area_id),
                is_best_cred: best.is_some_and(|(i, _)| i == index),
                selectable: d.is_available(),
                status_note: chip_status_note(d),
            }
        })
        .collect()
}

/// Assemble one node card
pub fn zone_node_view<'a>(
    area: &ShopLocationDef,
    save: &SaveData,
    personas: &[BuyerPersona],
    products: impl Iterator<Item = &'a Card>,
) -> ZoneNodeView {
    ZoneNodeView {
        area_id: area.id.clone(),
        name: area.name.clone(),
        identity: zone_identity(&area.id),
        narc_hint: narc_hint(&area.id),
        status: zone_status(area, save),
        clientele: clientele_lines(personas, &area.id),
        payout_band: payout_band(personas, &area.id),
        products: native_products(products, &area.id),
        dealers: dealer_chips(save, &area.id),
    }
}

// ============================================================================
// Move flow
// ============================================================================

/// Whether the selected dealer can be sent to a destination node right now.
/// Mirrors `SaveData::move_dealer`'s guards so the button never promises a
/// move the model would refuse.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MoveEligibility {
    Eligible { fee: u64 },
    /// Destination is the dealer's current station
    StationedHere,
    /// Dealer is jailed/relocating/laying low (chips shouldn't offer this,
    /// but state can shift between spawn and click)
    DealerUnavailable,
    CantAfford { fee: u64 },
}

pub fn move_eligibility(save: &SaveData, dealer_index: usize, to_area: &str) -> MoveEligibility {
    let fee = save.move_fee();
    let Some(dealer) = save.dealers.get(dealer_index) else {
        return MoveEligibility::DealerUnavailable;
    };
    if dealer.station == to_area {
        return MoveEligibility::StationedHere;
    }
    if !dealer.is_available() {
        return MoveEligibility::DealerUnavailable;
    }
    if save.account.cash_on_hand < fee {
        return MoveEligibility::CantAfford { fee };
    }
    MoveEligibility::Eligible { fee }
}

/// Header hint above the nodes: idle instructions, or the armed move with
/// its full cost (fee + downtime) BEFORE anything commits
pub fn map_hint(save: &SaveData, selected_dealer: Option<usize>) -> String {
    match selected_dealer.and_then(|i| save.dealers.get(i)) {
        Some(dealer) => format!(
            "SENDING {} — pick a destination · ${} + 1 RUN OUT",
            dealer.name.to_uppercase(),
            save.move_fee()
        ),
        None => "Click a dealer, then a destination, to relocate".to_string(),
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::buyer::BuyerDemand;
    use crate::models::card::CardType;
    use crate::save::DealerStatus;

    fn persona(name: &str, area: &str, mult: f32) -> BuyerPersona {
        BuyerPersona {
            area: area.to_string(),
            display_name: name.to_string(),
            demand: BuyerDemand {
                products: vec![],
                locations: vec![],
                description: String::new(),
            },
            base_multiplier: mult,
            reduced_multiplier: 1.0,
            evidence_threshold: None,
            reaction_deck_ids: vec![],
            reaction_deck: vec![],
            scenarios: vec![],
            active_scenario_index: None,
        }
    }

    fn product(name: &str, area: &str, price: u32) -> Card {
        Card {
            id: name.to_lowercase(),
            name: name.to_string(),
            card_type: CardType::Product { price: 100, heat: 5 },
            narrative_fragments: None,
            shop_location: Some(area.to_string()),
            shop_price: Some(price),
            shop_cred_required: None,
        }
    }

    fn area(id: &str, unlocked: bool, price: u32) -> ShopLocationDef {
        ShopLocationDef {
            id: id.to_string(),
            name: id.to_string(),
            description: String::new(),
            unlocked,
            price,
        }
    }

    /// SaveData::new() starts with the kingpin at the default station
    /// (the_corner); tests extend from there.
    fn save_with_cash(cash: u64) -> SaveData {
        let mut save = SaveData::new();
        save.account.cash_on_hand = cash;
        save
    }

    // -- zone status / affordability --

    #[test]
    fn unlocked_area_reports_unlocked() {
        let save = save_with_cash(0);
        assert_eq!(zone_status(&area("the_corner", true, 0), &save), ZoneStatus::Unlocked);
    }

    #[test]
    fn purchased_area_reports_unlocked() {
        let mut save = save_with_cash(5000);
        save.account
            .purchase_location("the_strip", 1200)
            .expect("purchase");
        assert_eq!(
            zone_status(&area("the_strip", false, 1200), &save),
            ZoneStatus::Unlocked
        );
    }

    #[test]
    fn locked_area_affordability_tracks_cash() {
        let strip = area("the_strip", false, 1200);
        assert_eq!(
            zone_status(&strip, &save_with_cash(1199)),
            ZoneStatus::Locked { price: 1200, affordable: false }
        );
        assert_eq!(
            zone_status(&strip, &save_with_cash(1200)),
            ZoneStatus::Locked { price: 1200, affordable: true }
        );
    }

    // -- clientele + payout band --

    #[test]
    fn clientele_lines_filter_by_area_and_format_multiplier() {
        let personas = vec![
            persona("Frat Bro", "the_corner", 2.5),
            persona("Wolf", "the_block", 2.8),
        ];
        assert_eq!(clientele_lines(&personas, "the_corner"), vec!["Frat Bro ×2.5"]);
        assert_eq!(clientele_lines(&personas, "the_block"), vec!["Wolf ×2.8"]);
    }

    #[test]
    fn payout_band_spans_min_to_max() {
        let personas = vec![
            persona("Housewife", "the_block", 1.5),
            persona("Wolf", "the_block", 2.8),
        ];
        assert_eq!(payout_band(&personas, "the_block"), Some("×1.5–×2.8".to_string()));
    }

    #[test]
    fn payout_band_collapses_when_flat_and_none_when_empty() {
        let personas = vec![persona("Pimp", "the_strip", 2.0)];
        assert_eq!(payout_band(&personas, "the_strip"), Some("×2.0".to_string()));
        assert_eq!(payout_band(&personas, "nowhere"), None);
    }

    // -- native products --

    #[test]
    fn native_products_sorted_cheapest_first() {
        let cards = vec![
            product("Acid", "the_corner", 400),
            product("Weed", "the_corner", 0),
            product("Shrooms", "the_corner", 100),
            product("Coke", "the_block", 5000),
        ];
        assert_eq!(
            native_products(cards.iter(), "the_corner"),
            vec!["Weed", "Shrooms", "Acid"]
        );
    }

    #[test]
    fn native_products_ignores_unstocked_cards() {
        let mut unstocked = product("Mystery", "the_corner", 0);
        unstocked.shop_location = None;
        assert!(native_products([unstocked].iter(), "the_corner").is_empty());
    }

    // -- dealer chips --

    #[test]
    fn chips_only_for_dealers_stationed_in_area() {
        let mut save = save_with_cash(10_000);
        save.hire_dealer();
        save.dealers[1].station = "the_block".to_string();
        let corner = dealer_chips(&save, "the_corner");
        assert_eq!(corner.len(), 1);
        assert_eq!(corner[0].name, "The Kingpin");
        assert_eq!(dealer_chips(&save, "the_block").len(), 1);
    }

    #[test]
    fn best_cred_marker_matches_roster_best() {
        let mut save = save_with_cash(10_000);
        save.hire_dealer();
        save.dealers[0].add_cred("the_corner");
        save.dealers[1].add_cred("the_corner");
        save.dealers[1].add_cred("the_corner");
        let chips = dealer_chips(&save, "the_corner");
        assert_eq!(chips.len(), 2);
        assert!(!chips[0].is_best_cred, "kingpin has 1 cred, not the best");
        assert!(chips[1].is_best_cred, "hire has 2 cred - the credit line");
        assert_eq!(chips[1].cred, 2);
    }

    #[test]
    fn no_best_marker_when_area_has_no_cred() {
        let save = save_with_cash(0);
        let chips = dealer_chips(&save, "the_corner");
        assert!(!chips[0].is_best_cred, "0 cred earns no credit line");
    }

    #[test]
    fn jailed_dealer_chip_unselectable_with_note() {
        let mut save = save_with_cash(0);
        save.dealers[0].status = DealerStatus::Jailed {
            runs_remaining: 2,
            sentence_total: 3,
            heat_at_bust: 50,
        };
        let chips = dealer_chips(&save, "the_corner");
        assert!(!chips[0].selectable);
        assert_eq!(chips[0].status_note.as_deref(), Some("JAILED · 2 RUNS"));
    }

    #[test]
    fn chip_carries_heat_tier() {
        let mut save = save_with_cash(0);
        save.dealers[0].character.heat = 95;
        let chips = dealer_chips(&save, "the_corner");
        assert_eq!(chips[0].tier_name, "Blazing");
        assert_eq!(chips[0].heat, 95);
    }

    // -- move eligibility (mirrors SaveData::move_dealer's guards) --

    #[test]
    fn move_eligible_when_available_elsewhere_and_funded() {
        let save = save_with_cash(1000);
        assert_eq!(
            move_eligibility(&save, 0, "the_block"),
            MoveEligibility::Eligible { fee: save.move_fee() }
        );
    }

    #[test]
    fn move_to_own_station_is_stationed_here() {
        let save = save_with_cash(1000);
        assert_eq!(move_eligibility(&save, 0, "the_corner"), MoveEligibility::StationedHere);
    }

    #[test]
    fn broke_empire_cant_afford_the_move() {
        let save = save_with_cash(0);
        assert_eq!(
            move_eligibility(&save, 0, "the_block"),
            MoveEligibility::CantAfford { fee: save.move_fee() }
        );
    }

    #[test]
    fn jailed_relocating_or_missing_dealer_unavailable() {
        let mut save = save_with_cash(1000);
        save.dealers[0].status = DealerStatus::Jailed {
            runs_remaining: 1,
            sentence_total: 1,
            heat_at_bust: 10,
        };
        assert_eq!(move_eligibility(&save, 0, "the_block"), MoveEligibility::DealerUnavailable);

        save.dealers[0].status = DealerStatus::Relocating { runs_remaining: 1 };
        assert_eq!(move_eligibility(&save, 0, "the_block"), MoveEligibility::DealerUnavailable);

        assert_eq!(move_eligibility(&save, 99, "the_block"), MoveEligibility::DealerUnavailable);
    }

    // -- hint line --

    #[test]
    fn hint_shows_full_cost_before_commit() {
        let save = save_with_cash(1000);
        let hint = map_hint(&save, Some(0));
        assert!(hint.contains("THE KINGPIN"), "{hint}");
        assert!(hint.contains(&format!("${}", save.move_fee())), "{hint}");
        assert!(hint.contains("1 RUN OUT"), "{hint}");
    }

    #[test]
    fn hint_idle_and_stale_selection_fall_back_to_instructions() {
        let save = save_with_cash(0);
        assert!(map_hint(&save, None).contains("Click a dealer"));
        assert!(map_hint(&save, Some(99)).contains("Click a dealer"));
    }

    // -- full node assembly --

    #[test]
    fn locked_node_still_sells_the_aspiration() {
        let save = save_with_cash(500);
        let personas = vec![persona("Pimp", "the_strip", 2.0)];
        let cards = vec![product("Ecstasy", "the_strip", 1600)];
        let node = zone_node_view(&area("the_strip", false, 1200), &save, &personas, cards.iter());

        assert_eq!(node.status, ZoneStatus::Locked { price: 1200, affordable: false });
        assert_eq!(node.clientele, vec!["Pimp ×2.0"]);
        assert_eq!(node.products, vec!["Ecstasy"]);
        assert_eq!(node.narc_hint, narc_hint("the_strip"));
        assert!(node.dealers.is_empty(), "nobody can be stationed in a locked zone");
    }

    #[test]
    fn unknown_area_gets_fallback_flavor() {
        assert_eq!(zone_identity("the_docks"), "NEW TERRITORY");
        assert_eq!(narc_hint("the_docks"), "watchful eyes");
    }
}
