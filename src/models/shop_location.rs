// SOW-024: Meta-game AREAS (shop + buyer gating), loaded from
// assets/data/shop_locations.ron. Human-readable content validated at load
// (authorability rule) - these are NOT the Location card type.

use serde::{Deserialize, Serialize};

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
}

/// Load-time validation for the area list:
/// - ids unique
/// - at least one area starts unlocked (the fresh-empire home turf)
/// - locked areas carry a non-zero price (they must be purchasable)
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
    }
    if !areas.iter().any(|a| a.unlocked) {
        return Err("no area starts unlocked - a fresh empire would have nowhere to operate".to_string());
    }
    Ok(())
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
        }
    }

    #[test]
    fn valid_area_list_passes() {
        let areas = vec![area("the_corner", true, 0), area("the_block", false, 2000)];
        assert!(validate_shop_locations(&areas).is_ok());
    }

    #[test]
    fn duplicate_ids_rejected() {
        let areas = vec![area("the_corner", true, 0), area("the_corner", false, 100)];
        assert!(validate_shop_locations(&areas).unwrap_err().contains("duplicate"));
    }

    #[test]
    fn locked_area_without_price_rejected() {
        let areas = vec![area("the_corner", true, 0), area("the_block", false, 0)];
        assert!(validate_shop_locations(&areas).unwrap_err().contains("no price"));
    }

    #[test]
    fn all_locked_rejected() {
        let areas = vec![area("the_block", false, 2000)];
        assert!(validate_shop_locations(&areas).unwrap_err().contains("nowhere to operate"));
    }
}
