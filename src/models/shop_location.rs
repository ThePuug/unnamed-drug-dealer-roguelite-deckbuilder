// SOW-024: Meta-game AREAS (shop + buyer gating), loaded from
// assets/data/shop_locations.ron. Human-readable content validated at load
// (authorability rule) - these are NOT the Location card type.

use serde::{Deserialize, Serialize};

/// SOW-031: the zone's supplier - one named NPC fronting the shop stock.
/// Pure fiction fields; the front MECHANICS live in save state keyed by
/// area id (the supplier is the face, the zone is the account).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SupplierDef {
    pub name: String,
    /// One line in their voice, shown on the shop header
    pub voice: String,
}

/// SOW-036: the zone's SIGNATURE dealer - one themed named face you hire AT
/// this zone (you don't hire generically from anywhere; you hire at a
/// location, and the hire lands stationed there). Pure fiction: `portrait`
/// is a KEY into GameAssets.actor_portraits ("Bubba" -> "dealer-bubba.png").
/// The hire MECHANICS live in save state (SaveData::hire_signature_dealer +
/// DealerState.signature_of).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SignatureDealerDef {
    pub name: String,
    /// Portrait key into GameAssets.actor_portraits
    pub portrait: String,
}

/// SOW-038: an UNLOCKABLE dealer offered AT a zone, gated by street cred. Purely
/// additive over the signature model: a zone may list several of these, each a
/// named face you hire once the roster's best cred there reaches
/// `cred_required`. `portrait` is a KEY into GameAssets.actor_portraits, same
/// convention as SignatureDealerDef. The hire MECHANICS live in save state
/// (SaveData::hire_zone_dealer + DealerState.signature_of); NO new save field.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AreaDealerDef {
    pub name: String,
    /// Portrait key into GameAssets.actor_portraits
    pub portrait: String,
    /// Best street cred in the zone required before this face can be hired
    pub cred_required: u32,
}

/// An unlockable area: gates a card shop and (RFC-024) its buyer personas
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShopLocationDef {
    pub id: String,
    pub name: String,
    pub description: String,
    /// Unlocked from a fresh account (price is ignored when true)
    pub unlocked: bool,
    /// Purchase price in global cash (RFC-024)
    #[serde(default)]
    pub price: u32,
    /// SOW-031 (SOW-029 carry): zone identity line for the map node
    /// ("STREET CRAFT — WHERE IT ALL STARTS")
    #[serde(default)]
    pub identity: String,
    /// Narc texture in fiction voice ("patrols & donut breaks")
    #[serde(default)]
    pub narc_hint: String,
    /// SOW-031: who fronts this zone's stock
    #[serde(default)]
    pub supplier: Option<SupplierDef>,
    /// SOW-036: this zone's signature dealer - the themed named face you hire
    /// AT this zone. Required at load (validate_shop_locations); serde-default
    /// keeps old-format tests compiling, but authored content must carry one.
    #[serde(default)]
    pub signature_dealer: Option<SignatureDealerDef>,
    /// SOW-038: additional named dealers offered AT this zone, each unlocking at
    /// a street-cred threshold. Additive over `signature_dealer`; empty by
    /// default. Every dealer NAME in a zone (signature + all unlockables) must
    /// be unique - the (area, name) pair is the hire-once identity, enforced at
    /// load by validate_shop_locations.
    #[serde(default)]
    pub unlockable_dealers: Vec<AreaDealerDef>,
    /// SOW-033: per-area narc portrait filename under assets/art/actors/
    /// ("narc-<slug>.png"). None falls back to the "narc-<area>.png" template.
    #[serde(default)]
    pub narc_portrait: Option<String>,
    /// SOW-034: the zone's restock margin - the fraction of a product's BASE
    /// sale price that one restock charge costs here. Easy in the starter zone,
    /// tighter up the ladder. Authored in RON, validated in (0.0, 1.0) at load
    /// (a missing/out-of-range margin fails loud - the authorability rule).
    #[serde(default)]
    pub restock_margin: f32,
}

/// Load-time validation for the area list:
/// - ids unique
/// - at least one area starts unlocked (the fresh-empire home turf)
/// - locked areas carry a non-zero price (they must be purchasable)
/// - SOW-031: every area carries its map flavor (identity + narc_hint)
///   and a supplier with a name and a voice - the map and shop render
///   these unconditionally, so missing content fails loud here
/// - SOW-036: every area carries a signature dealer with a name and a
///   portrait - the map offers it as a hire, so missing content fails loud
pub fn validate_shop_locations(areas: &[ShopLocationDef]) -> Result<(), String> {
    let mut seen = std::collections::HashSet::new();
    for area in areas {
        if !seen.insert(area.id.as_str()) {
            return Err(format!("duplicate area id '{}'", area.id));
        }
        if !area.unlocked && area.price == 0 {
            return Err(format!(
                "area '{}' is locked but has no price - it could never be unlocked",
                area.id
            ));
        }
        if area.identity.trim().is_empty() {
            return Err(format!("area '{}' has no identity line", area.id));
        }
        if area.narc_hint.trim().is_empty() {
            return Err(format!("area '{}' has no narc_hint", area.id));
        }
        match &area.supplier {
            None => return Err(format!("area '{}' has no supplier", area.id)),
            Some(s) if s.name.trim().is_empty() || s.voice.trim().is_empty() => {
                return Err(format!("area '{}' supplier needs a name and a voice", area.id));
            }
            Some(_) => {}
        }
        // SOW-036: the signature dealer is offered as a hire on the map, so
        // a missing/empty name or portrait fails loud (authorability rule)
        match &area.signature_dealer {
            None => return Err(format!("area '{}' has no signature_dealer", area.id)),
            Some(s) if s.name.trim().is_empty() || s.portrait.trim().is_empty() => {
                return Err(format!(
                    "area '{}' signature_dealer needs a name and a portrait",
                    area.id
                ));
            }
            Some(_) => {}
        }
        // SOW-038: each unlockable dealer needs a non-empty name AND portrait
        // (same authorability rule as the signature), and every dealer NAME in a
        // zone (signature + all unlockables) must be UNIQUE - the (area, name)
        // pair is the hire-once identity, so a collision must fail loud at load.
        let mut names = std::collections::HashSet::new();
        if let Some(s) = &area.signature_dealer {
            names.insert(s.name.trim());
        }
        for dealer in &area.unlockable_dealers {
            if dealer.name.trim().is_empty() || dealer.portrait.trim().is_empty() {
                return Err(format!(
                    "area '{}' unlockable dealer needs a name and a portrait",
                    area.id
                ));
            }
            if !names.insert(dealer.name.trim()) {
                return Err(format!(
                    "area '{}' has a duplicate dealer name '{}' - names must be unique per zone",
                    area.id,
                    dealer.name.trim()
                ));
            }
        }
        // SOW-034: the restock margin must be a sensible fraction of a sale -
        // 0 would give product away free, >=1 would run every product
        // underwater (a restock costs more than the sale it enables)
        if !(area.restock_margin > 0.0 && area.restock_margin < 1.0) {
            return Err(format!(
                "area '{}' restock_margin {} must be in (0.0, 1.0)",
                area.id, area.restock_margin
            ));
        }
    }
    if !areas.iter().any(|a| a.unlocked) {
        return Err("no area starts unlocked - a fresh empire would have nowhere to operate".to_string());
    }
    Ok(())
}

/// SOW-024: The areas a run may take place in, in definition order.
/// (INTERIM: run area is picked randomly from these until dealer stationing
/// lands - see the stationing design update.)
pub fn unlocked_area_ids<'a>(
    areas: &'a [ShopLocationDef],
    unlocked: &std::collections::HashSet<String>,
) -> Vec<&'a str> {
    areas
        .iter()
        .filter(|a| unlocked.contains(&a.id))
        .map(|a| a.id.as_str())
        .collect()
}

/// SOW-034: per-charge restock cost of a product at a zone's margin.
/// `product_price` is the Product card's BASE SALE price (its CardType price),
/// NOT its one-time shop unlock price. The margin is the fraction of a base
/// sale that a restock charge costs, so restock_unit stays below the sale it
/// enables and no product runs underwater. Rounded to whole dollars.
pub fn restock_unit(product_price: u32, margin: f32) -> u32 {
    // SOW-034 review: floor at 1. A cheap product in a low-margin zone could
    // otherwise round to 0 (e.g. base 1 x 0.35 -> round(0.35) = 0), and a $0
    // batch_cost would hand out the batch AND permanent access for free.
    ((product_price as f32 * margin).round() as u32).max(1)
}

/// SOW-034: cash to buy or restock one full batch (BATCH_SIZE charges) of a
/// product at a zone's margin.
pub fn batch_cost(product_price: u32, margin: f32) -> u32 {
    restock_unit(product_price, margin) * crate::save::BATCH_SIZE
}

/// SOW-040: the cred discount ladder - as the roster's BEST street cred in a
/// zone (SOW-025's best_cred) climbs, restock gets progressively cheaper. This
/// is the earn-back reward for SOW-034's per-zone restock_margin ladder.
/// Ordered high->low; the first cleared threshold wins (see cred_margin_factor).
/// Kept as a code const - a game-wide progression curve, matching the
/// hire_cost/bail_cost/FRONT_VIG precedent (not RON). [TUNING] thresholds+factors.
const CRED_MARGIN_LADDER: [(u32, f32); 3] = [(10, 0.55), (6, 0.70), (3, 0.85)];

/// SOW-040: the factor in (0.0, 1.0] applied to a zone's authored restock margin
/// for the given best cred. Scans CRED_MARGIN_LADDER high->low and returns the
/// first cleared threshold's factor, defaulting to 1.0 (no discount) below the
/// lowest step. Monotonic non-increasing in cred (1.0 >= 0.85 >= 0.70 >= 0.55),
/// so restock cost never rises as a zone is built up. cred 0-2 -> 1.0, a strict
/// no-op that preserves SOW-034's shipped economy on a fresh zone.
pub fn cred_margin_factor(best_cred: u32) -> f32 {
    for (threshold, factor) in CRED_MARGIN_LADDER {
        if best_cred >= threshold {
            return factor;
        }
    }
    1.0
}

/// SOW-040: a zone's EFFECTIVE restock margin - the authored base margin scaled
/// by the cred discount factor. restock_unit / batch_cost take THIS effective
/// margin, so buy, restock, the shop label, and the front all discount from a
/// single derivation. At cred 0 the factor is 1.0, so this returns base_margin
/// unchanged (no economy regression on a fresh zone).
pub fn effective_restock_margin(base_margin: f32, best_cred: u32) -> f32 {
    base_margin * cred_margin_factor(best_cred)
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn area(id: &str, unlocked: bool, price: u32) -> ShopLocationDef {
        ShopLocationDef {
            id: id.to_string(),
            name: id.to_string(),
            description: String::new(),
            unlocked,
            price,
            identity: "SOME CRAFT".to_string(),
            narc_hint: "watchful eyes".to_string(),
            supplier: Some(SupplierDef {
                name: "Plug".to_string(),
                voice: "First one rides on trust.".to_string(),
            }),
            signature_dealer: Some(SignatureDealerDef {
                name: "Bubba".to_string(),
                portrait: "Bubba".to_string(),
            }),
            unlockable_dealers: Vec::new(),
            narc_portrait: None,
            restock_margin: 0.5,
        }
    }

    #[test]
    fn restock_margin_out_of_range_rejected() {
        let mut zero = area("trailer_park", true, 0);
        zero.restock_margin = 0.0;
        assert!(validate_shop_locations(&[zero]).unwrap_err().contains("restock_margin"));

        let mut too_high = area("trailer_park", true, 0);
        too_high.restock_margin = 1.0;
        assert!(validate_shop_locations(&[too_high]).unwrap_err().contains("restock_margin"));
    }

    #[test]
    fn valid_area_list_passes() {
        let areas = vec![area("trailer_park", true, 0), area("suburbia", false, 2000)];
        assert!(validate_shop_locations(&areas).is_ok());
    }

    #[test]
    fn missing_flavor_or_supplier_rejected() {
        let mut no_identity = area("trailer_park", true, 0);
        no_identity.identity = String::new();
        assert!(validate_shop_locations(&[no_identity]).unwrap_err().contains("identity"));

        let mut no_hint = area("trailer_park", true, 0);
        no_hint.narc_hint = "  ".to_string();
        assert!(validate_shop_locations(&[no_hint]).unwrap_err().contains("narc_hint"));

        let mut no_supplier = area("trailer_park", true, 0);
        no_supplier.supplier = None;
        assert!(validate_shop_locations(&[no_supplier]).unwrap_err().contains("supplier"));

        let mut mute_supplier = area("trailer_park", true, 0);
        mute_supplier.supplier = Some(SupplierDef { name: "Plug".to_string(), voice: String::new() });
        assert!(validate_shop_locations(&[mute_supplier]).unwrap_err().contains("voice"));
    }

    #[test]
    fn missing_or_faceless_signature_dealer_rejected() {
        let mut no_sig = area("trailer_park", true, 0);
        no_sig.signature_dealer = None;
        assert!(validate_shop_locations(&[no_sig]).unwrap_err().contains("signature_dealer"));

        let mut nameless = area("trailer_park", true, 0);
        nameless.signature_dealer = Some(SignatureDealerDef {
            name: "  ".to_string(),
            portrait: "Bubba".to_string(),
        });
        assert!(validate_shop_locations(&[nameless]).unwrap_err().contains("name"));

        let mut faceless = area("trailer_park", true, 0);
        faceless.signature_dealer = Some(SignatureDealerDef {
            name: "Bubba".to_string(),
            portrait: String::new(),
        });
        assert!(validate_shop_locations(&[faceless]).unwrap_err().contains("portrait"));
    }

    #[test]
    fn unlockable_dealer_missing_name_or_portrait_rejected() {
        // SOW-038: same authorability rule as the signature - a faceless or
        // nameless unlockable fails loud at load.
        let mut nameless = area("trailer_park", true, 0);
        nameless.unlockable_dealers = vec![AreaDealerDef {
            name: "  ".to_string(),
            portrait: "Gladys".to_string(),
            cred_required: 5,
        }];
        assert!(validate_shop_locations(&[nameless]).unwrap_err().contains("name"));

        let mut faceless = area("trailer_park", true, 0);
        faceless.unlockable_dealers = vec![AreaDealerDef {
            name: "Gladys".to_string(),
            portrait: String::new(),
            cred_required: 5,
        }];
        assert!(validate_shop_locations(&[faceless]).unwrap_err().contains("portrait"));
    }

    #[test]
    fn duplicate_dealer_name_within_zone_rejected() {
        // SOW-038: the (area, name) pair is the hire-once identity, so a name
        // that collides with the signature - or with another unlockable - in the
        // same zone must fail loud at load.
        let mut clash_with_signature = area("trailer_park", true, 0);
        clash_with_signature.unlockable_dealers = vec![AreaDealerDef {
            name: "Bubba".to_string(), // same as the signature's name
            portrait: "Gladys".to_string(),
            cred_required: 5,
        }];
        assert!(validate_shop_locations(&[clash_with_signature])
            .unwrap_err()
            .contains("duplicate dealer name"));

        let mut clash_between_unlockables = area("trailer_park", true, 0);
        clash_between_unlockables.unlockable_dealers = vec![
            AreaDealerDef { name: "Gladys".to_string(), portrait: "Gladys".to_string(), cred_required: 5 },
            AreaDealerDef { name: "Gladys".to_string(), portrait: "Marcus".to_string(), cred_required: 9 },
        ];
        assert!(validate_shop_locations(&[clash_between_unlockables])
            .unwrap_err()
            .contains("duplicate dealer name"));
    }

    #[test]
    fn zone_with_valid_unlockables_passes() {
        // SOW-038: the Gladys pilot shape - a unique-named, credited unlockable
        // alongside the signature - validates cleanly.
        let mut tp = area("trailer_park", true, 0);
        tp.unlockable_dealers = vec![AreaDealerDef {
            name: "Gladys".to_string(),
            portrait: "Gladys".to_string(),
            cred_required: 5,
        }];
        assert!(validate_shop_locations(&[tp]).is_ok());
    }

    #[test]
    fn duplicate_ids_rejected() {
        let areas = vec![area("trailer_park", true, 0), area("trailer_park", false, 100)];
        assert!(validate_shop_locations(&areas).unwrap_err().contains("duplicate"));
    }

    #[test]
    fn locked_area_without_price_rejected() {
        let areas = vec![area("trailer_park", true, 0), area("suburbia", false, 0)];
        assert!(validate_shop_locations(&areas).unwrap_err().contains("no price"));
    }

    #[test]
    fn all_locked_rejected() {
        let areas = vec![area("suburbia", false, 2000)];
        assert!(validate_shop_locations(&areas).unwrap_err().contains("nowhere to operate"));
    }

    #[test]
    fn restock_unit_rounds_and_stays_below_the_sale() {
        // Margin < 1 keeps the per-charge cost under the base sale price,
        // so a batch of 4 sales always clears a batch's cost (never underwater).
        assert_eq!(restock_unit(30, 0.35), 11); // round(10.5) -> 11
        assert_eq!(restock_unit(40, 0.35), 14); // round(14.0)
        assert_eq!(restock_unit(80, 0.65), 52); // round(52.0)
        // Every case: unit < price
        for (price, margin) in [(30, 0.35), (40, 0.35), (120, 0.65)] {
            assert!(restock_unit(price, margin) < price, "margin<1 must stay under sale");
        }
        // SOW-034 review: floored at 1 so a cheap product can never restock free
        assert_eq!(restock_unit(1, 0.35), 1, "round(0.35)=0 must floor to 1, never free");
    }

    #[test]
    fn batch_cost_is_unit_times_batch_size() {
        assert_eq!(batch_cost(30, 0.35), 11 * crate::save::BATCH_SIZE);
        assert_eq!(batch_cost(80, 0.65), 52 * crate::save::BATCH_SIZE);
    }

    #[test]
    fn shipped_zone_economy_is_positive_and_laddered() {
        // (base sale price, shipped zone margin, restock_unit, batch_cost) for
        // each of the 6 shipped products. Pins the economy and asserts a batch
        // always clears under four base-price sales (no product runs underwater).
        let cases = [
            ("weed", 30u32, 0.35f32, 11u32, 44u32),      // trailer_park
            ("shrooms", 40, 0.35, 14, 56),               // trailer_park
            ("codeine", 50, 0.50, 25, 100),              // suburbia
            ("xanax", 55, 0.50, 28, 112),                // suburbia
            ("ecstasy", 80, 0.65, 52, 208),              // red_light_district
            ("coke", 120, 0.65, 78, 312),                // red_light_district
        ];
        for (name, base, margin, unit, batch) in cases {
            assert_eq!(restock_unit(base, margin), unit, "{name} restock_unit");
            assert_eq!(batch_cost(base, margin), batch, "{name} batch_cost");
            assert!(batch < base * 4, "{name}: a batch must clear under four base sales");
        }
    }

    // SOW-040: the six shipped (name, base sale price, authored zone margin)
    // products, reused by the discount tests to pin the whole economy.
    const SHIPPED: [(&str, u32, f32); 6] = [
        ("weed", 30, 0.35),    // trailer_park
        ("shrooms", 40, 0.35), // trailer_park
        ("codeine", 50, 0.50), // suburbia
        ("xanax", 55, 0.50),   // suburbia
        ("ecstasy", 80, 0.65), // red_light_district
        ("coke", 120, 0.65),   // red_light_district
    ];

    #[test]
    fn cred_margin_factor_step_boundaries() {
        // SOW-040: highest cleared threshold wins; below the lowest step is 1.0.
        assert_eq!(cred_margin_factor(0), 1.0);
        assert_eq!(cred_margin_factor(2), 1.0);
        assert_eq!(cred_margin_factor(3), 0.85);
        assert_eq!(cred_margin_factor(5), 0.85);
        assert_eq!(cred_margin_factor(6), 0.70);
        assert_eq!(cred_margin_factor(9), 0.70);
        assert_eq!(cred_margin_factor(10), 0.55);
        assert_eq!(cred_margin_factor(100), 0.55); // top tier caps
    }

    #[test]
    fn cred_zero_is_a_strict_no_op() {
        // SOW-040: a fresh zone (cred 0) charges the authored margin EXACTLY -
        // the discount must never perturb SOW-034's shipped economy. Multiplying
        // by the literal 1.0 factor is exact, so this pins weed to 11/$44.
        assert_eq!(effective_restock_margin(0.35, 0), 0.35);
        let eff = effective_restock_margin(0.35, 0);
        assert_eq!(restock_unit(30, eff), 11);
        assert_eq!(batch_cost(30, eff), 44);
    }

    #[test]
    fn high_cred_zone_costs_less() {
        // SOW-040: at the deepest tier weed and coke both restock cheaper than
        // their cred-0 baselines (44 and 312).
        assert_eq!(batch_cost(30, effective_restock_margin(0.35, 10)), 24);
        assert!(24 < 44);
        assert_eq!(batch_cost(120, effective_restock_margin(0.65, 10)), 172);
        assert!(172 < 312);
    }

    #[test]
    fn discount_lands_exactly_at_thresholds() {
        // SOW-040: weed's batch cost steps down as cred crosses 3/6/10 - assert
        // the exact step values, not merely "less".
        let weed_batch = |cred| batch_cost(30, effective_restock_margin(0.35, cred));
        assert_eq!(weed_batch(2), 44); // still full price just below the first step
        assert_eq!(weed_batch(3), 36); // 0.85 factor
        assert_eq!(weed_batch(6), 28); // 0.70 factor
        assert_eq!(weed_batch(10), 24); // 0.55 factor
    }

    #[test]
    fn restock_unit_is_monotonic_non_increasing_in_cred() {
        // SOW-040: for every shipped product, restock_unit never INCREASES as
        // cred climbs - guards against a rounding-induced bump anywhere on the
        // ladder (0..=15 spans all thresholds and past the top tier).
        for (name, base, margin) in SHIPPED {
            let mut prev = restock_unit(base, effective_restock_margin(margin, 0));
            for cred in 1..=15u32 {
                let unit = restock_unit(base, effective_restock_margin(margin, cred));
                assert!(
                    unit <= prev,
                    "{name}: restock_unit rose from {prev} to {unit} at cred {cred}"
                );
                prev = unit;
            }
        }
    }

    #[test]
    fn never_free_and_never_underwater_at_deepest_discount() {
        // SOW-040: even at the deepest factor the max(1) floor keeps restock
        // non-free (a base-1 product still costs 1/charge), and the effective
        // margin stays in (0,1) for every shipped (margin, cred) combo, so a
        // 4-sale batch always clears its restock (SOW-034's invariant holds).
        assert_eq!(restock_unit(1, effective_restock_margin(0.35, 10)), 1);
        for (name, base, margin) in SHIPPED {
            for cred in [0u32, 3, 6, 10, 15] {
                let eff = effective_restock_margin(margin, cred);
                assert!(eff > 0.0 && eff < 1.0, "{name}: effective margin {eff} left (0,1)");
                assert!(
                    batch_cost(base, eff) < base * crate::save::BATCH_SIZE,
                    "{name}: a batch must clear under {} base sales at cred {cred}",
                    crate::save::BATCH_SIZE
                );
            }
        }
    }

    #[test]
    fn unlocked_area_ids_filters_and_preserves_order() {
        let areas = vec![
            area("trailer_park", true, 0),
            area("suburbia", false, 2000),
            area("downtown", false, 5000),
        ];
        let mut unlocked = std::collections::HashSet::new();
        unlocked.insert("trailer_park".to_string());
        assert_eq!(unlocked_area_ids(&areas, &unlocked), vec!["trailer_park"]);

        unlocked.insert("downtown".to_string());
        assert_eq!(unlocked_area_ids(&areas, &unlocked), vec!["trailer_park", "downtown"]);
    }
}
