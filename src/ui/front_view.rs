// SOW-031: Fronts view-model - pure presentation for supplier credit
// across its four surfaces (shop header, hub pressure indicator, map node
// supplier line, ledger debt figure). Same rule as view.rs/map_view.rs/
// ledger_view.rs: unit-testable without ECS; systems only orchestrate.

use crate::models::shop_location::ShopLocationDef;
use crate::save::{FrontState, SaveData, SupplierStanding, FRONT_WINDOW_RUNS};

fn plural(n: u32) -> &'static str {
    if n == 1 { "" } else { "S" }
}

/// "DUE IN 2 RUNS — $625"
pub fn due_line(front: &FrontState) -> String {
    format!(
        "DUE IN {} RUN{} — ${}",
        front.runs_remaining,
        plural(front.runs_remaining),
        front.owed
    )
}

/// Standing suffix for supplier lines; Good is the quiet default
pub fn standing_label(standing: SupplierStanding) -> Option<&'static str> {
    match standing {
        SupplierStanding::Good => None,
        SupplierStanding::CutOff => Some("CUT OFF"),
        SupplierStanding::Soured => Some("SOURED"),
    }
}

/// The FRONT button's face: full cost + window BEFORE commit. SOW-034: the
/// front is against a BATCH, so `batch_cost` is the batch's cash price.
pub fn front_button_label(batch_cost: u32) -> String {
    format!(
        "FRONT ${} · DUE {} RUNS",
        crate::save::front_owed(batch_cost),
        FRONT_WINDOW_RUNS
    )
}

/// The shop header under a zone's supplier: what the relationship looks
/// like right now. (name, voice, status) - status is None while Good with
/// no front (nothing to say).
pub struct SupplierHeader {
    pub name_line: String,
    pub voice_line: String,
    /// "DUE IN 2 RUNS — $625" / "CUT OFF — settle your debt" / "SOURED —
    /// cash only, no more fronts", with the urgent flag for red ink
    pub status_line: Option<String>,
    pub urgent: bool,
    /// PAY button amount when a front is live
    pub payable: Option<u64>,
}

pub fn supplier_header(area: &ShopLocationDef, save: &SaveData) -> Option<SupplierHeader> {
    let supplier = area.supplier.as_ref()?;
    let standing = save.standing_with(&area.id);
    let front = save.front_in(&area.id);

    let status_line = match (front, standing) {
        (Some(f), SupplierStanding::CutOff) => {
            Some(format!("CUT OFF — {}", due_line(f)))
        }
        (Some(f), _) => Some(due_line(f)),
        (None, SupplierStanding::Soured) => {
            Some("SOURED — cash only, no more fronts".to_string())
        }
        (None, SupplierStanding::CutOff) => {
            // Unreachable by construction (CutOff always carries a front),
            // but total: render the lock rather than hide it
            Some("CUT OFF — settle your debt".to_string())
        }
        (None, SupplierStanding::Good) => None,
    };

    Some(SupplierHeader {
        name_line: format!("SUPPLIER: {}", supplier.name.to_uppercase()),
        voice_line: format!("\u{201c}{}\u{201d}", supplier.voice),
        urgent: standing != SupplierStanding::Good
            || front.is_some_and(|f| f.runs_remaining <= 1),
        status_line,
        payable: front.map(|f| f.owed),
    })
}

/// One line on a map node: "SUPPLIER: LIL SMOKE · DUE IN 2 RUNS — $125".
/// Locked zones show the name alone (the aspiration); standings and due
/// counters only matter where you can already shop.
pub fn supplier_map_line(area: &ShopLocationDef, save: &SaveData) -> Option<String> {
    let supplier = area.supplier.as_ref()?;
    let name = format!("SUPPLIER: {}", supplier.name.to_uppercase());
    let unlocked =
        area.unlocked || save.account.unlocked_locations.contains(&area.id);
    if !unlocked {
        return Some(name);
    }
    let mut parts = vec![name];
    if let Some(label) = standing_label(save.standing_with(&area.id)) {
        parts.push(label.to_string());
    }
    if let Some(front) = save.front_in(&area.id) {
        parts.push(due_line(front));
    }
    Some(parts.join(" · "))
}

/// The hub's pressure line while any front is live: the most urgent front
/// (fewest runs left; ties to the biggest debt), named to its supplier.
/// None when the books are clean - the indicator disappears entirely.
pub fn pressure_line(save: &SaveData, areas: &[ShopLocationDef]) -> Option<String> {
    let most_urgent = save
        .fronts
        .iter()
        .min_by_key(|f| (f.runs_remaining, u64::MAX - f.owed))?;
    let supplier = areas
        .iter()
        .find(|a| a.id == most_urgent.area_id)
        .and_then(|a| a.supplier.as_ref())
        .map(|s| s.name.to_uppercase())
        .unwrap_or_else(|| most_urgent.area_id.to_uppercase());
    let mut line = format!(
        "FRONT DUE IN {} RUN{} — ${} TO {}",
        most_urgent.runs_remaining,
        plural(most_urgent.runs_remaining),
        most_urgent.owed,
        supplier
    );
    let others = save.fronts.len() - 1;
    if others > 0 {
        line.push_str(&format!(" (+{others} MORE)"));
    }
    Some(line)
}

/// Red ink when the clock is nearly out on any front
pub fn pressure_urgent(save: &SaveData) -> bool {
    save.fronts.iter().any(|f| f.runs_remaining <= 1)
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::shop_location::SupplierDef;

    fn area(id: &str, unlocked: bool, supplier: Option<&str>) -> ShopLocationDef {
        ShopLocationDef {
            id: id.to_string(),
            name: id.to_string(),
            description: String::new(),
            unlocked,
            price: 1000,
            identity: "CRAFT".to_string(),
            narc_hint: "eyes".to_string(),
            supplier: supplier.map(|name| SupplierDef {
                name: name.to_string(),
                voice: "Trust me.".to_string(),
            }),
            signature_dealer: None,
            narc_portrait: None,
            restock_margin: 0.5,
        }
    }

    fn save_with_front(area_id: &str, owed: u64, runs: u32) -> SaveData {
        let mut save = SaveData::new();
        save.fronts.push(FrontState {
            card_id: "shrooms".to_string(),
            area_id: area_id.to_string(),
            owed,
            runs_remaining: runs,
            charges: crate::save::BATCH_SIZE,
        });
        save
    }

    // -- due line / labels --

    #[test]
    fn due_line_pluralizes_runs() {
        let mut save = save_with_front("trailer_park", 625, 2);
        assert_eq!(due_line(&save.fronts[0]), "DUE IN 2 RUNS — $625");
        save.fronts[0].runs_remaining = 1;
        assert_eq!(due_line(&save.fronts[0]), "DUE IN 1 RUN — $625");
    }

    #[test]
    fn front_button_shows_vig_and_window() {
        assert_eq!(front_button_label(500), "FRONT $625 · DUE 4 RUNS");
    }

    // -- supplier header (shop) --

    #[test]
    fn header_quiet_while_good_with_no_front() {
        let save = SaveData::new();
        let h = supplier_header(&area("trailer_park", true, Some("Lil Smoke")), &save).unwrap();
        assert_eq!(h.name_line, "SUPPLIER: LIL SMOKE");
        assert_eq!(h.voice_line, "\u{201c}Trust me.\u{201d}");
        assert!(h.status_line.is_none());
        assert!(!h.urgent);
        assert!(h.payable.is_none());
    }

    #[test]
    fn header_shows_due_and_pay_while_fronted() {
        let save = save_with_front("trailer_park", 125, 3);
        let h = supplier_header(&area("trailer_park", true, Some("Lil Smoke")), &save).unwrap();
        assert_eq!(h.status_line.as_deref(), Some("DUE IN 3 RUNS — $125"));
        assert_eq!(h.payable, Some(125));
        assert!(!h.urgent);
    }

    #[test]
    fn header_urgent_at_one_run_or_bad_standing() {
        let save = save_with_front("trailer_park", 125, 1);
        let h = supplier_header(&area("trailer_park", true, Some("Lil Smoke")), &save).unwrap();
        assert!(h.urgent);

        let mut save = save_with_front("trailer_park", 125, 4);
        save.supplier_standing
            .insert("trailer_park".to_string(), SupplierStanding::CutOff);
        let h = supplier_header(&area("trailer_park", true, Some("Lil Smoke")), &save).unwrap();
        assert!(h.urgent);
        assert_eq!(
            h.status_line.as_deref(),
            Some("CUT OFF — DUE IN 4 RUNS — $125")
        );
    }

    #[test]
    fn header_soured_reads_cash_only() {
        let mut save = SaveData::new();
        save.supplier_standing
            .insert("trailer_park".to_string(), SupplierStanding::Soured);
        let h = supplier_header(&area("trailer_park", true, Some("Lil Smoke")), &save).unwrap();
        assert_eq!(
            h.status_line.as_deref(),
            Some("SOURED — cash only, no more fronts")
        );
        assert!(h.urgent);
        assert!(h.payable.is_none());
    }

    // -- map node line --

    #[test]
    fn map_line_name_only_while_locked() {
        let save = SaveData::new();
        assert_eq!(
            supplier_map_line(&area("red_light_district", false, Some("Miss Velvet")), &save).as_deref(),
            Some("SUPPLIER: MISS VELVET")
        );
    }

    #[test]
    fn map_line_appends_standing_and_due() {
        let mut save = save_with_front("trailer_park", 125, 2);
        save.supplier_standing
            .insert("trailer_park".to_string(), SupplierStanding::CutOff);
        assert_eq!(
            supplier_map_line(&area("trailer_park", true, Some("Lil Smoke")), &save).as_deref(),
            Some("SUPPLIER: LIL SMOKE · CUT OFF · DUE IN 2 RUNS — $125")
        );
    }

    #[test]
    fn map_line_none_without_supplier() {
        let save = SaveData::new();
        assert!(supplier_map_line(&area("trailer_park", true, None), &save).is_none());
    }

    // -- hub pressure --

    #[test]
    fn pressure_absent_with_clean_books() {
        let save = SaveData::new();
        assert!(pressure_line(&save, &[area("trailer_park", true, Some("Lil Smoke"))]).is_none());
        assert!(!pressure_urgent(&save));
    }

    #[test]
    fn pressure_names_the_most_urgent_supplier() {
        let mut save = save_with_front("trailer_park", 125, 3);
        save.fronts.push(FrontState {
            card_id: "ecstasy".to_string(),
            area_id: "red_light_district".to_string(),
            owed: 2000,
            runs_remaining: 1,
            charges: crate::save::BATCH_SIZE,
        });
        let areas = [
            area("trailer_park", true, Some("Lil Smoke")),
            area("red_light_district", true, Some("Miss Velvet")),
        ];
        assert_eq!(
            pressure_line(&save, &areas).as_deref(),
            Some("FRONT DUE IN 1 RUN — $2000 TO MISS VELVET (+1 MORE)")
        );
        assert!(pressure_urgent(&save));
    }

    #[test]
    fn pressure_falls_back_to_area_id_when_content_missing() {
        let save = save_with_front("the_docks", 500, 2);
        assert_eq!(
            pressure_line(&save, &[]).as_deref(),
            Some("FRONT DUE IN 2 RUNS — $500 TO THE_DOCKS")
        );
    }
}
