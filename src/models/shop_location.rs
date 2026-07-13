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
    /// SOW-033: per-area narc portrait filename under assets/art/actors/
    /// ("narc-<slug>.png"). None falls back to "narc-default.png" at load.
    #[serde(default)]
    pub narc_portrait: Option<String>,
}

/// Load-time validation for the area list:
/// - ids unique
/// - at least one area starts unlocked (the fresh-empire home turf)
/// - locked areas carry a non-zero price (they must be purchasable)
/// - SOW-031: every area carries its map flavor (identity + narc_hint)
///   and a supplier with a name and a voice - the map and shop render
///   these unconditionally, so missing content fails loud here
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
    (product_price as f32 * margin).round() as u32
}

/// SOW-034: cash to buy or restock one full batch (BATCH_SIZE charges) of a
/// product at a zone's margin.
pub fn batch_cost(product_price: u32, margin: f32) -> u32 {
    restock_unit(product_price, margin) * crate::save::BATCH_SIZE
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
            narc_portrait: None,
        }
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
    }

    #[test]
    fn batch_cost_is_unit_times_batch_size() {
        assert_eq!(batch_cost(30, 0.35), 11 * crate::save::BATCH_SIZE);
        assert_eq!(batch_cost(80, 0.65), 52 * crate::save::BATCH_SIZE);
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
