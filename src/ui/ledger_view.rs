// SOW-030: Kingpin Ledger view-model - pure presentation logic for the
// ledger overlay. Same rule as view.rs/map_view.rs: everything here is
// unit-testable without ECS; systems/kingpin_ledger.rs only orchestrates.
//
// HARD RULE (SOW-030): derive, don't record. Every number below comes from
// existing SaveData - this module takes &SaveData everywhere and never
// mutates. If a stat can't be derived, it doesn't ship this SOW.

use crate::models::shop_location::ShopLocationDef;
use crate::save::{DealerState, EmpireEpitaph, SaveData};

// ============================================================================
// Panel 1: THE EMPIRE - the tombstone being carved
// ============================================================================

/// The live empire's summary strip: the exact numbers
/// `EmpireEpitaph::from_save` will freeze when the empire falls.
#[derive(Debug, Clone, PartialEq)]
pub struct EmpireSummary {
    pub lifetime_revenue: u64,
    pub cash_on_hand: u64,
    /// Decks played across the whole roster
    pub decks_played: u32,
    /// Roster size beyond the kingpin
    pub dealers_hired: u32,
    pub zones_unlocked: usize,
    /// Times anyone in the roster went through the system
    pub convictions: u32,
}

pub fn empire_summary(save: &SaveData) -> EmpireSummary {
    EmpireSummary {
        lifetime_revenue: save.account.lifetime_revenue,
        cash_on_hand: save.account.cash_on_hand,
        decks_played: save.dealers.iter().map(|d| d.character.decks_played).sum(),
        dealers_hired: save.dealers.len().saturating_sub(1) as u32,
        zones_unlocked: save.account.unlocked_locations.len(),
        convictions: save.dealers.iter().map(|d| d.prior_convictions).sum(),
    }
}

// ============================================================================
// Panel 2: THE ROSTER - one dossier per dealer, stories one click deeper
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
pub struct DossierRow {
    pub dealer_index: usize,
    pub name: String,
    pub is_kingpin: bool,
    /// Station's display name ("The Corner"), falling back to the raw id
    /// for areas missing from content
    pub station: String,
    /// Σ street_cred across all zones - +1 per successful deal makes cred
    /// a deals-closed counter for free
    pub deals_closed: u32,
    /// "The Corner 4 · The Block 2" in canonical area order (zero-cred
    /// zones omitted); empty string when no cred anywhere
    pub cred_line: String,
    pub decks_played: u32,
    pub priors: u32,
    pub heat: u32,
    pub tier_name: &'static str,
    pub tier_color: (f32, f32, f32),
    pub story_count: usize,
    /// "JAILED · 2 RUNS" etc., None when available (shared with map chips)
    pub status_note: Option<String>,
}

fn area_display_name<'a>(areas: &'a [ShopLocationDef], id: &'a str) -> &'a str {
    areas
        .iter()
        .find(|a| a.id == id)
        .map(|a| a.name.as_str())
        .unwrap_or(id)
}

/// Total successful deals a dealer has closed anywhere
pub fn deals_closed(dealer: &DealerState) -> u32 {
    dealer.street_cred.values().sum()
}

/// Per-zone cred in canonical (shop_locations.ron) order; zones the content
/// no longer knows come last, alphabetically, so old saves stay readable
fn cred_line(dealer: &DealerState, areas: &[ShopLocationDef]) -> String {
    let mut parts: Vec<String> = areas
        .iter()
        .filter_map(|a| {
            let cred = dealer.cred_in(&a.id);
            (cred > 0).then(|| format!("{} {}", a.name, cred))
        })
        .collect();
    let mut orphans: Vec<(&String, &u32)> = dealer
        .street_cred
        .iter()
        .filter(|(id, cred)| **cred > 0 && !areas.iter().any(|a| &a.id == *id))
        .collect();
    orphans.sort_by_key(|(id, _)| id.as_str());
    parts.extend(orphans.into_iter().map(|(id, cred)| format!("{id} {cred}")));
    parts.join(" · ")
}

/// Dossier rows in roster order - the kingpin is dealers[0] by invariant,
/// so "kingpin first" comes for free
pub fn dossier_rows(save: &SaveData, areas: &[ShopLocationDef]) -> Vec<DossierRow> {
    save.dealers
        .iter()
        .enumerate()
        .map(|(dealer_index, d)| {
            let tier = d.character.heat_tier();
            DossierRow {
                dealer_index,
                name: d.name.clone(),
                is_kingpin: d.is_kingpin,
                station: area_display_name(areas, &d.station).to_string(),
                deals_closed: deals_closed(d),
                cred_line: cred_line(d, areas),
                decks_played: d.character.decks_played,
                priors: d.prior_convictions,
                heat: d.character.heat,
                tier_name: tier.name(),
                tier_color: tier.color(),
                story_count: d.character.story_history.len(),
                status_note: super::map_view::chip_status_note(d),
            }
        })
        .collect()
}

/// A dealer's story feed, newest first (story_history appends
/// chronologically). Missing dealer -> empty.
pub fn dealer_stories(save: &SaveData, dealer_index: usize) -> Vec<String> {
    save.dealers
        .get(dealer_index)
        .map(|d| d.character.story_history.iter().rev().cloned().collect())
        .unwrap_or_default()
}

// ============================================================================
// Panel 3: FALLEN EMPIRES - the arcade board, browsable while you play
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
pub struct BoardRow {
    /// 1-based arcade rank; None for the living empire's IN PROGRESS row
    /// (the dead hold their ranks - you pass one by BEATING it)
    pub rank: Option<usize>,
    pub is_current: bool,
    /// Index into save.fallen_empires for click-to-stories; None = living
    pub epitaph_index: Option<usize>,
    pub lifetime_revenue: u64,
    pub decks_played: u32,
    pub dealers_hired: u32,
    pub convictions: u32,
    pub ended_at: Option<u64>,
}

/// The full board: every epitaph ranked by lifetime revenue, with the
/// living empire slotted UNRANKED at its would-be position. Ties go to the
/// dead - the record stands until strictly beaten.
pub fn board_rows(save: &SaveData) -> Vec<BoardRow> {
    let fallen = &save.fallen_empires;
    let mut rows: Vec<BoardRow> = crate::save::leaderboard_top(fallen, fallen.len())
        .into_iter()
        .enumerate()
        .map(|(rank, idx)| {
            let e: &EmpireEpitaph = &fallen[idx];
            BoardRow {
                rank: Some(rank + 1),
                is_current: false,
                epitaph_index: Some(idx),
                lifetime_revenue: e.lifetime_revenue,
                decks_played: e.decks_played,
                dealers_hired: e.dealers_hired,
                convictions: e.total_prior_convictions,
                ended_at: Some(e.ended_at),
            }
        })
        .collect();

    let live = empire_summary(save);
    let position = rows
        .iter()
        .take_while(|r| r.lifetime_revenue >= live.lifetime_revenue)
        .count();
    rows.insert(
        position,
        BoardRow {
            rank: None,
            is_current: true,
            epitaph_index: None,
            lifetime_revenue: live.lifetime_revenue,
            decks_played: live.decks_played,
            dealers_hired: live.dealers_hired,
            convictions: live.convictions,
            ended_at: None,
        },
    );
    rows
}

/// A fallen empire's archived stories, newest first. Missing index -> empty.
pub fn epitaph_stories(save: &SaveData, epitaph_index: usize) -> Vec<String> {
    save.fallen_empires
        .get(epitaph_index)
        .map(|e| e.stories.iter().rev().cloned().collect())
        .unwrap_or_default()
}

// ============================================================================
// Map node history line (SOW-029 acceptance confirmed the placement) -
// lives here so the ledger and the map derive the SAME numbers
// ============================================================================

/// Σ roster street_cred in a zone = deals closed there
pub fn zone_deals_closed(save: &SaveData, area_id: &str) -> u32 {
    save.dealers.iter().map(|d| d.cred_in(area_id)).sum()
}

/// One line of zone history for the map node card: "12 deals closed ·
/// best: Ray (4)". None while the zone has no history to tell.
pub fn zone_history_line(save: &SaveData, area_id: &str) -> Option<String> {
    let deals = zone_deals_closed(save, area_id);
    if deals == 0 {
        return None;
    }
    let plural = if deals == 1 { "" } else { "s" };
    let best = save
        .best_cred(area_id)
        .and_then(|(i, cred)| save.dealers.get(i).map(|d| (d.name.clone(), cred)));
    Some(match best {
        Some((name, cred)) => format!("{deals} deal{plural} closed · best: {name} ({cred})"),
        None => format!("{deals} deal{plural} closed"),
    })
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::save::DealerStatus;

    fn area(id: &str, name: &str) -> ShopLocationDef {
        ShopLocationDef {
            id: id.to_string(),
            name: name.to_string(),
            description: String::new(),
            unlocked: id == "the_corner",
            price: 1000,
        }
    }

    fn city() -> Vec<ShopLocationDef> {
        vec![
            area("the_corner", "The Corner"),
            area("the_strip", "The Strip"),
            area("the_block", "The Block"),
        ]
    }

    fn epitaph(revenue: u64, decks: u32) -> EmpireEpitaph {
        EmpireEpitaph {
            ended_at: 1000,
            lifetime_revenue: revenue,
            cash_at_fall: 0,
            dealers_hired: 1,
            total_prior_convictions: 2,
            decks_played: decks,
            stories: vec!["first".to_string(), "latest".to_string()],
        }
    }

    fn roster_save() -> SaveData {
        let mut save = SaveData::new();
        save.account.cash_on_hand = 10_000;
        save.account.lifetime_revenue = 2_000;
        save.hire_dealer();
        save
    }

    // -- empire summary --

    #[test]
    fn empire_summary_matches_what_the_epitaph_would_freeze() {
        let mut save = roster_save();
        save.dealers[0].character.decks_played = 7;
        save.dealers[1].character.decks_played = 3;
        save.dealers[1].prior_convictions = 2;

        let live = empire_summary(&save);
        let frozen = EmpireEpitaph::from_save(&save, 42);
        assert_eq!(live.lifetime_revenue, frozen.lifetime_revenue);
        assert_eq!(live.cash_on_hand, frozen.cash_at_fall);
        assert_eq!(live.decks_played, frozen.decks_played);
        assert_eq!(live.dealers_hired, frozen.dealers_hired);
        assert_eq!(live.convictions, frozen.total_prior_convictions);
    }

    #[test]
    fn empire_summary_counts_unlocked_zones() {
        let mut save = roster_save();
        assert_eq!(empire_summary(&save).zones_unlocked, 1); // the_corner
        save.account.unlocked_locations.insert("the_strip".to_string());
        assert_eq!(empire_summary(&save).zones_unlocked, 2);
    }

    // -- dossiers --

    #[test]
    fn kingpin_dossier_is_first_and_marked() {
        let rows = dossier_rows(&roster_save(), &city());
        assert_eq!(rows.len(), 2);
        assert!(rows[0].is_kingpin);
        assert_eq!(rows[0].name, "The Kingpin");
        assert!(!rows[1].is_kingpin);
    }

    #[test]
    fn deals_closed_sums_cred_across_zones() {
        let mut save = roster_save();
        save.dealers[0].add_cred("the_corner");
        save.dealers[0].add_cred("the_corner");
        save.dealers[0].add_cred("the_block");
        let rows = dossier_rows(&save, &city());
        assert_eq!(rows[0].deals_closed, 3);
        assert_eq!(rows[1].deals_closed, 0);
    }

    #[test]
    fn cred_line_in_canonical_order_with_display_names() {
        let mut save = roster_save();
        save.dealers[0].add_cred("the_block"); // inserted first,
        save.dealers[0].add_cred("the_corner"); // renders second anyway
        let rows = dossier_rows(&save, &city());
        assert_eq!(rows[0].cred_line, "The Corner 1 · The Block 1");
        assert_eq!(rows[1].cred_line, "");
    }

    #[test]
    fn cred_in_unknown_zone_still_renders_after_known() {
        let mut save = roster_save();
        save.dealers[0].add_cred("the_corner");
        save.dealers[0].add_cred("the_docks"); // content no longer ships it
        let rows = dossier_rows(&save, &city());
        assert_eq!(rows[0].cred_line, "The Corner 1 · the_docks 1");
    }

    #[test]
    fn dossier_station_uses_display_name_with_id_fallback() {
        let mut save = roster_save();
        save.dealers[1].station = "atlantis".to_string();
        let rows = dossier_rows(&save, &city());
        assert_eq!(rows[0].station, "The Corner");
        assert_eq!(rows[1].station, "atlantis");
    }

    #[test]
    fn dossier_carries_status_note_and_tier() {
        let mut save = roster_save();
        save.dealers[1].status = DealerStatus::Jailed {
            runs_remaining: 2,
            sentence_total: 3,
            heat_at_bust: 50,
        };
        save.dealers[1].character.heat = 95;
        let rows = dossier_rows(&save, &city());
        assert_eq!(rows[1].status_note.as_deref(), Some("JAILED · 2 RUNS"));
        assert_eq!(rows[1].tier_name, "Blazing");
        assert!(rows[0].status_note.is_none());
    }

    #[test]
    fn dossier_counts_stories() {
        let mut save = roster_save();
        save.dealers[0]
            .character
            .story_history
            .extend(["a".to_string(), "b".to_string()]);
        assert_eq!(dossier_rows(&save, &city())[0].story_count, 2);
    }

    // -- story feeds --

    #[test]
    fn dealer_stories_newest_first_and_missing_index_empty() {
        let mut save = roster_save();
        save.dealers[0]
            .character
            .story_history
            .extend(["first".to_string(), "latest".to_string()]);
        assert_eq!(dealer_stories(&save, 0), vec!["latest", "first"]);
        assert!(dealer_stories(&save, 99).is_empty());
    }

    #[test]
    fn epitaph_stories_newest_first_and_missing_index_empty() {
        let mut save = roster_save();
        save.fallen_empires.push(epitaph(1000, 5));
        assert_eq!(epitaph_stories(&save, 0), vec!["latest", "first"]);
        assert!(epitaph_stories(&save, 7).is_empty());
    }

    // -- the board --

    #[test]
    fn empty_board_is_just_the_living_empire() {
        let rows = board_rows(&roster_save());
        assert_eq!(rows.len(), 1);
        assert!(rows[0].is_current);
        assert_eq!(rows[0].rank, None);
        assert_eq!(rows[0].epitaph_index, None);
    }

    #[test]
    fn epitaphs_ranked_by_revenue_with_living_slotted_between() {
        let mut save = roster_save(); // lifetime_revenue 2_000
        save.fallen_empires.push(epitaph(900, 3)); // index 0
        save.fallen_empires.push(epitaph(5000, 9)); // index 1
        let rows = board_rows(&save);

        assert_eq!(rows.len(), 3);
        // rank 1: the 5000 empire
        assert_eq!(rows[0].rank, Some(1));
        assert_eq!(rows[0].lifetime_revenue, 5000);
        assert_eq!(rows[0].epitaph_index, Some(1));
        // living empire at its would-be position, unranked
        assert!(rows[1].is_current);
        assert_eq!(rows[1].rank, None);
        assert_eq!(rows[1].lifetime_revenue, 2000);
        // rank 2: the 900 empire (dead ranks don't skip for the living)
        assert_eq!(rows[2].rank, Some(2));
        assert_eq!(rows[2].lifetime_revenue, 900);
    }

    #[test]
    fn living_empire_tops_the_board_when_winning() {
        let mut save = roster_save();
        save.account.lifetime_revenue = 9_999;
        save.fallen_empires.push(epitaph(5000, 9));
        let rows = board_rows(&save);
        assert!(rows[0].is_current);
        assert_eq!(rows[1].rank, Some(1));
    }

    #[test]
    fn tie_goes_to_the_dead_record() {
        let mut save = roster_save();
        save.account.lifetime_revenue = 5000;
        save.fallen_empires.push(epitaph(5000, 9));
        let rows = board_rows(&save);
        assert_eq!(rows[0].epitaph_index, Some(0), "the record stands until beaten");
        assert!(rows[1].is_current);
    }

    #[test]
    fn exactly_one_living_row() {
        let mut save = roster_save();
        save.fallen_empires.push(epitaph(900, 3));
        save.fallen_empires.push(epitaph(5000, 9));
        assert_eq!(board_rows(&save).iter().filter(|r| r.is_current).count(), 1);
    }

    // -- zone history (shared with the map) --

    #[test]
    fn zone_deals_closed_sums_roster_cred() {
        let mut save = roster_save();
        save.dealers[0].add_cred("the_corner");
        save.dealers[1].add_cred("the_corner");
        save.dealers[1].add_cred("the_corner");
        assert_eq!(zone_deals_closed(&save, "the_corner"), 3);
        assert_eq!(zone_deals_closed(&save, "the_block"), 0);
    }

    #[test]
    fn zone_history_line_names_the_best_dealer() {
        let mut save = roster_save();
        save.dealers[0].add_cred("the_corner");
        save.dealers[1].add_cred("the_corner");
        save.dealers[1].add_cred("the_corner");
        assert_eq!(
            zone_history_line(&save, "the_corner").as_deref(),
            Some("3 deals closed · best: Slim (2)")
        );
    }

    #[test]
    fn zone_history_singular_and_silent_when_empty() {
        let mut save = roster_save();
        assert_eq!(zone_history_line(&save, "the_corner"), None);
        save.dealers[0].add_cred("the_corner");
        assert_eq!(
            zone_history_line(&save, "the_corner").as_deref(),
            Some("1 deal closed · best: The Kingpin (1)")
        );
    }

    #[test]
    fn zone_history_best_matches_shop_credit_line() {
        // The map's history line and the shop's "unlocked by" must never
        // disagree - both come from SaveData::best_cred
        let mut save = roster_save();
        save.dealers[0].add_cred("the_corner");
        save.dealers[1].add_cred("the_corner");
        let (best_idx, _) = save.best_cred("the_corner").unwrap();
        let line = zone_history_line(&save, "the_corner").unwrap();
        assert!(line.contains(&save.dealers[best_idx].name), "{line}");
    }
}
