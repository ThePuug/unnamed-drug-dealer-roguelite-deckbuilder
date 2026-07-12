// UI View-Model - Pure presentation logic for the Game Play v2 screen
// SOW-022: All screen-derivation logic lives here as pure, unit-testable
// functions. Systems in ui_update.rs/systems.rs only orchestrate.

use crate::models::card::{Card, CardType};
use crate::models::hand_state::{HandPhase, HandState};
use crate::Owner;

// ============================================================================
// Hand fan geometry
// ============================================================================

/// Placement of one hand card in the bottom fan arc (design-space pixels/degrees)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FanSlot {
    /// Horizontal offset of the card center from screen center
    pub offset_x: f32,
    /// Distance of the card bottom from the screen bottom
    pub lift: f32,
    /// Clockwise rotation in degrees
    pub angle_deg: f32,
    /// Stacking order (center cards above edge cards)
    pub z: i32,
}

const FAN_ANGLE_STEP: f32 = 8.5;
const FAN_SPREAD_X: f32 = 138.0;
const FAN_BASE_LIFT: f32 = 10.0;
const FAN_ARC_LIFT: f32 = 13.0;

/// Compute fan placement for `n` hand slots (mockup math from Game Play v2)
pub fn fan_layout(n: usize) -> Vec<FanSlot> {
    if n == 0 {
        return Vec::new();
    }
    let mid = (n as f32 - 1.0) / 2.0;
    (0..n)
        .map(|i| {
            let d = i as f32 - mid;
            FanSlot {
                offset_x: d * FAN_SPREAD_X,
                lift: FAN_BASE_LIFT + (mid - d.abs()) * FAN_ARC_LIFT,
                angle_deg: d * FAN_ANGLE_STEP,
                z: 10 - d.abs().round() as i32,
            }
        })
        .collect()
}

// ============================================================================
// Evidence vs Cover balance bar
// ============================================================================

/// Evidence's share of the balance bar as a percentage (0..=100).
/// With nothing played the bar rests at 50/50.
pub fn balance_split(evidence: u32, cover: u32) -> f32 {
    let total = evidence + cover;
    if total == 0 {
        50.0
    } else {
        evidence as f32 / total as f32 * 100.0
    }
}

// ============================================================================
// Discard stack derivation
// ============================================================================

/// What the discard stack shows: cards that have been "resolved into the past"
/// this hand — Evidence/Cover/Modifier plays plus slot cards that were
/// overridden by a later play of the same type. Derived chronologically from
/// `cards_played` so the top card is the most recent discard event.
pub fn discard_view(cards_played: &[Card]) -> (usize, Option<Card>) {
    let mut pile: Vec<&Card> = Vec::new();
    let mut active_slots: [Option<&Card>; 4] = [None, None, None, None]; // P/L/C/I

    for card in cards_played {
        match card.card_type {
            CardType::Evidence { .. } | CardType::Cover { .. } | CardType::DealModifier { .. } => {
                pile.push(card);
            }
            _ => {
                let slot = match card.card_type {
                    CardType::Product { .. } => 0,
                    CardType::Location { .. } => 1,
                    CardType::Conviction { .. } => 2,
                    _ => 3, // Insurance
                };
                if let Some(replaced) = active_slots[slot].replace(card) {
                    pile.push(replaced);
                }
            }
        }
    }

    (pile.len(), pile.last().map(|c| (*c).clone()))
}

// ============================================================================
// Narc intent telegraph
// ============================================================================

/// One stat row in the narc intent bubble: (emoji, signed value text)
pub type IntentRow = (&'static str, String);

/// Narc intent bubble contents
#[derive(Debug, Clone, PartialEq)]
pub struct IntentView {
    /// "INTENT" while the Narc is about to act, "PLAYED" afterwards
    pub verb: &'static str,
    pub card_name: String,
    pub rows: Vec<IntentRow>,
}

/// Stat rows for a narc card with the narc upgrade tier applied.
/// Mirrors the engine exactly: evidence TRUNCATES (`calculate_totals` does
/// `as u32`), heat ROUNDS (`get_card_heat` does `.round()`), and conviction
/// thresholds are checked raw at resolution - the telegraph must promise the
/// numbers that will actually materialize.
fn narc_card_rows(card: &Card, tier_multiplier: f32) -> Vec<IntentRow> {
    match card.card_type {
        CardType::Evidence { evidence, heat } => vec![
            ("🔍", format!("{:+}", (evidence as f32 * tier_multiplier) as i32)),
            ("🔥", format!("{:+}", (heat as f32 * tier_multiplier).round() as i32)),
        ],
        CardType::Conviction { heat_threshold } => {
            vec![("⚠", format!("busts at {heat_threshold}"))]
        }
        // Narc decks only contain Evidence/Conviction today; degrade gracefully
        _ => Vec::new(),
    }
}

/// What the narc intent bubble should show, if anything.
/// - Narc's pending turn: telegraph the card it is about to play (`hand[0]`)
/// - After the narc acted (rest of the round incl. buyer reaction): show the
///   card it actually played - the last narc-type card in `cards_played`
///   (the narc always acts first each round, so that is this round's play)
/// - Otherwise (dealing / hand over): nothing
pub fn narc_intent(hand_state: &HandState) -> Option<IntentView> {
    let tier_mult = hand_state.narc_upgrade_tier.multiplier();

    let narc_pending = hand_state.current_state == HandPhase::PlayerPhase
        && !hand_state.all_players_acted()
        && hand_state.current_player() == Owner::Narc;

    if narc_pending {
        let next = hand_state.cards(Owner::Narc).hand.iter().flatten().next()?;
        return Some(IntentView {
            verb: "INTENT",
            card_name: next.name.to_uppercase(),
            rows: narc_card_rows(next, tier_mult),
        });
    }

    if matches!(hand_state.current_state, HandPhase::PlayerPhase | HandPhase::DealerReveal) {
        let played = hand_state
            .cards_played
            .iter()
            .rev()
            .find(|c| matches!(c.card_type, CardType::Evidence { .. } | CardType::Conviction { .. }))?;
        return Some(IntentView {
            verb: "PLAYED",
            card_name: played.name.to_uppercase(),
            rows: narc_card_rows(played, tier_mult),
        });
    }

    None
}

// ============================================================================
// Buyer reaction bubble
// ============================================================================

/// Stat rows for a buyer reaction card (printed values - buyer cards carry no
/// tier scaling; `get_card_heat` applies them verbatim)
fn buyer_card_rows(card: &Card) -> Vec<IntentRow> {
    match card.card_type {
        CardType::Location { evidence, cover, heat } => vec![
            ("🔍", format!("{:+}", evidence as i32)),
            ("🛡", format!("{:+}", cover as i32)),
            ("🔥", format!("{heat:+}")),
        ],
        CardType::Cover { cover, heat } => vec![
            ("🛡", format!("{:+}", cover as i32)),
            ("🔥", format!("{heat:+}")),
        ],
        CardType::DealModifier { price_multiplier, evidence, cover, heat } => {
            let mut rows = Vec::new();
            if price_multiplier != 1.0 {
                rows.push(("💰", format!("{:+}%", ((price_multiplier - 1.0) * 100.0).round() as i32)));
            }
            if evidence != 0 {
                rows.push(("🔍", format!("{evidence:+}")));
            }
            if cover != 0 {
                rows.push(("🛡", format!("{cover:+}")));
            }
            rows.push(("🔥", format!("{heat:+}")));
            rows
        }
        _ => Vec::new(),
    }
}

/// What the buyer reaction bubble shows: the buyer's most recent reaction this
/// hand, visible from the moment it's played until the hand resolves. Buyer
/// plays previously had no on-screen callout at all (stdout only), which made
/// buyer heat swings look like bugs during playtesting.
pub fn buyer_played(hand_state: &HandState) -> Option<IntentView> {
    if hand_state.current_state == HandPhase::Bust {
        return None;
    }
    let card = hand_state.cards(Owner::Buyer).played.last()?;
    Some(IntentView {
        verb: "PLAYED",
        card_name: card.name.to_uppercase(),
        rows: buyer_card_rows(card),
    })
}

// ============================================================================
// Buyer confidence
// ============================================================================

/// How close the buyer is to bailing, as a face + label.
/// Proximity is the worst of the two bail axes (`should_buyer_bail`):
/// session heat vs the scenario's heat threshold, and evidence total vs the
/// persona's evidence threshold. Bail is checked only at resolution, so the
/// bands mirror that: SCARED means the buyer is over a line RIGHT NOW and
/// walks if the hand resolves as-is (still recoverable); NERVOUS means past
/// two-thirds of the way there. A buyer with no thresholds never bails and
/// is always confident.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BuyerConfidence {
    Confident,
    Nervous,
    Scared,
}

impl BuyerConfidence {
    pub fn emoji(self) -> &'static str {
        match self {
            BuyerConfidence::Confident => "🙂",
            BuyerConfidence::Nervous => "😟",
            BuyerConfidence::Scared => "😨",
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            BuyerConfidence::Confident => "CONFIDENT",
            BuyerConfidence::Nervous => "NERVOUS",
            BuyerConfidence::Scared => "SCARED",
        }
    }
}

pub fn buyer_confidence(hand_state: &HandState) -> Option<BuyerConfidence> {
    let persona = hand_state.buyer_persona.as_ref()?;

    let heat_proximity = persona
        .active_scenario_index
        .and_then(|idx| persona.scenarios.get(idx))
        .and_then(|s| s.heat_threshold)
        .filter(|t| *t > 0)
        .map(|t| hand_state.current_heat.max(0) as f32 / t as f32);

    let evidence_proximity = persona
        .evidence_threshold
        .filter(|t| *t > 0)
        .map(|t| hand_state.calculate_totals(true).evidence as f32 / t as f32);

    let proximity = match (heat_proximity, evidence_proximity) {
        (Some(h), Some(e)) => h.max(e),
        (Some(h), None) => h,
        (None, Some(e)) => e,
        (None, None) => 0.0, // fearless - never bails
    };

    // > 1.0 matches the engine's strict `>` check: exactly ON the threshold
    // still survives resolution, one past it does not
    Some(if proximity > 1.0 {
        BuyerConfidence::Scared
    } else if proximity > 2.0 / 3.0 {
        BuyerConfidence::Nervous
    } else {
        BuyerConfidence::Confident
    })
}

// ============================================================================
// Turn pill
// ============================================================================

/// Which actor the turn pill highlights (drives its color scheme)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PillActor {
    Narc,
    Player,
    Buyer,
    Neutral,
}

/// Turn pill label + actor for the current phase
pub fn turn_pill(hand_state: &HandState) -> (&'static str, PillActor) {
    match hand_state.current_state {
        HandPhase::Draw => ("DEALING...", PillActor::Neutral),
        HandPhase::PlayerPhase => {
            if hand_state.all_players_acted() {
                ("RESOLVING...", PillActor::Neutral)
            } else if hand_state.current_player() == Owner::Player {
                ("YOUR MOVE", PillActor::Player)
            } else {
                ("NARC'S MOVE", PillActor::Narc)
            }
        }
        HandPhase::DealerReveal => ("BUYER REACTING", PillActor::Buyer),
        HandPhase::Resolve | HandPhase::Bust => ("DEAL COMPLETE", PillActor::Neutral),
    }
}

/// Header line above the pill
pub fn round_header(hand_state: &HandState) -> String {
    let status = if hand_state.current_state == HandPhase::Bust {
        "DEAL COMPLETE"
    } else {
        "DEAL IN PROGRESS"
    };
    format!("ROUND {} / 3  ·  {}", hand_state.current_round, status)
}

// ============================================================================
// Standing panel (heat bar ticks + cash)
// ============================================================================

/// The standing panel heat bar runs on a fixed 0..=100 scale
pub const HEAT_BAR_MAX: u32 = 100;

/// Conviction thresholds present in the Narc's card set (deck + hand + played),
/// deduplicated and sorted, each labeled with its card name. These become the
/// tick marks on the standing panel heat bar. Content-driven: reflects whatever
/// convictions are authored in the narc deck RON.
pub fn conviction_ticks(hand_state: &HandState) -> Vec<(u32, String)> {
    let narc = hand_state.cards(Owner::Narc);
    let mut ticks: Vec<(u32, String)> = narc
        .deck
        .iter()
        .chain(narc.hand.iter().flatten())
        .chain(hand_state.cards_played.iter())
        .filter_map(|c| match c.card_type {
            CardType::Conviction { heat_threshold } => Some((heat_threshold, c.name.clone())),
            _ => None,
        })
        .collect();
    ticks.sort_by_key(|(t, _)| *t);
    ticks.dedup_by_key(|(t, _)| *t);
    ticks.retain(|(t, _)| *t <= HEAT_BAR_MAX);
    ticks
}

// ============================================================================
// Game-over arcade board (SOW-023)
// ============================================================================

/// Top-3 fallen empires by lifetime revenue for the GAME OVER overlay,
/// marking the empire that just fell (always the latest epitaph)
pub fn game_over_board(fallen: &[crate::save::EmpireEpitaph]) -> String {
    if fallen.is_empty() {
        return String::new();
    }
    let latest = fallen.len() - 1;
    crate::save::leaderboard_top(fallen, 3)
        .into_iter()
        .enumerate()
        .map(|(rank, idx)| {
            let epitaph = &fallen[idx];
            let marker = if idx == latest { "  ← THIS RUN" } else { "" };
            format!(
                "{}. {} · {} decks{}",
                rank + 1,
                format_cash(epitaph.lifetime_revenue),
                epitaph.decks_played,
                marker
            )
        })
        .collect::<Vec<_>>()
        .join("\n")
}

/// Format a cash amount with thousands separators: 2400 → "$2,400"
pub fn format_cash(n: u64) -> String {
    let s = n.to_string();
    let mut out = String::new();
    for (i, c) in s.chars().rev().enumerate() {
        if i > 0 && i % 3 == 0 {
            out.push(',');
        }
        out.push(c);
    }
    format!("${}", out.chars().rev().collect::<String>())
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::test_helpers::*;

    // ---- fan_layout ----

    #[test]
    fn fan_is_symmetric_and_center_raised() {
        let fan = fan_layout(3);
        assert_eq!(fan.len(), 3);
        assert_eq!(fan[0].offset_x, -fan[2].offset_x);
        assert_eq!(fan[0].angle_deg, -fan[2].angle_deg);
        assert_eq!(fan[1].offset_x, 0.0);
        assert_eq!(fan[1].angle_deg, 0.0);
        assert!(fan[1].lift > fan[0].lift);
        assert!(fan[1].z > fan[0].z);
        assert_eq!(fan[0].lift, fan[2].lift);
    }

    #[test]
    fn fan_handles_degenerate_counts() {
        assert!(fan_layout(0).is_empty());
        let single = fan_layout(1);
        assert_eq!(single[0].offset_x, 0.0);
        assert_eq!(single[0].angle_deg, 0.0);
    }

    #[test]
    fn fan_edge_cards_rotate_most() {
        let fan = fan_layout(5);
        assert!(fan[0].angle_deg < fan[1].angle_deg);
        assert!(fan[4].angle_deg > fan[3].angle_deg);
        assert_eq!(fan[0].angle_deg, -2.0 * FAN_ANGLE_STEP);
    }

    // ---- balance_split ----

    #[test]
    fn balance_split_rests_at_center_when_empty() {
        assert_eq!(balance_split(0, 0), 50.0);
    }

    #[test]
    fn balance_split_tracks_evidence_share() {
        assert_eq!(balance_split(45, 55), 45.0);
        assert_eq!(balance_split(10, 0), 100.0);
        assert_eq!(balance_split(0, 10), 0.0);
    }

    // ---- discard_view ----

    #[test]
    fn discard_view_empty_hand() {
        let (count, top) = discard_view(&[]);
        assert_eq!(count, 0);
        assert!(top.is_none());
    }

    #[test]
    fn discard_view_collects_pool_cards_chronologically() {
        let played = vec![
            create_evidence("Patrol", 5, 5),
            create_cover("Fake Receipts", 20, 5),
        ];
        let (count, top) = discard_view(&played);
        assert_eq!(count, 2);
        assert_eq!(top.unwrap().name, "Fake Receipts");
    }

    #[test]
    fn discard_view_includes_overridden_slot_cards() {
        let played = vec![
            create_product("Weed", 30, 5),
            create_cover("Alibi", 30, -5),
            create_product("Coke", 120, 35), // overrides Weed -> Weed discarded now
        ];
        let (count, top) = discard_view(&played);
        assert_eq!(count, 2); // Alibi + overridden Weed
        assert_eq!(top.unwrap().name, "Weed"); // override is the latest event
    }

    #[test]
    fn discard_view_active_slot_cards_are_not_discards() {
        let played = vec![
            create_product("Weed", 30, 5),
            create_location("Frat House", 15, 15, 10),
        ];
        let (count, top) = discard_view(&played);
        assert_eq!(count, 0);
        assert!(top.is_none());
    }

    // ---- narc_intent ----

    fn hand_state_with_narc_card(card: Card) -> HandState {
        let mut hs = HandState::default();
        hs.current_state = HandPhase::PlayerPhase;
        hs.current_player_index = 0; // Narc first in turn order
        hs.cards_mut(Owner::Narc).hand = [Some(card), None, None];
        hs
    }

    #[test]
    fn intent_telegraphs_narc_hand_card_during_narc_turn() {
        let hs = hand_state_with_narc_card(create_evidence("Surveillance", 20, 5));
        let intent = narc_intent(&hs).expect("intent should show during narc's move");
        assert_eq!(intent.verb, "INTENT");
        assert_eq!(intent.card_name, "SURVEILLANCE");
        assert_eq!(intent.rows[0], ("🔍", "+20".to_string()));
        assert_eq!(intent.rows[1], ("🔥", "+5".to_string()));
    }

    #[test]
    fn intent_applies_narc_tier_multiplier() {
        let mut hs = hand_state_with_narc_card(create_evidence("Surveillance", 20, 10));
        hs.narc_upgrade_tier = crate::save::UpgradeTier::Tier1; // +10%
        let intent = narc_intent(&hs).unwrap();
        assert_eq!(intent.rows[0], ("🔍", "+22".to_string()));
        assert_eq!(intent.rows[1], ("🔥", "+11".to_string()));
    }

    #[test]
    fn intent_evidence_truncates_like_the_engine() {
        // calculate_totals does `(evidence * mult) as u32` (truncation), so a
        // 5-evidence card at Tier1 contributes +5, not the rounded +6 - the
        // telegraph must not promise evidence that never materializes
        let mut hs = hand_state_with_narc_card(create_evidence("Patrol", 5, 5));
        hs.narc_upgrade_tier = crate::save::UpgradeTier::Tier1; // ×1.1 → 5.5
        let intent = narc_intent(&hs).unwrap();
        assert_eq!(intent.rows[0], ("🔍", "+5".to_string()));
        // heat rounds, matching get_card_heat: 5.5 → 6
        assert_eq!(intent.rows[1], ("🔥", "+6".to_string()));
    }

    #[test]
    fn intent_shows_played_card_after_narc_acts() {
        let mut hs = hand_state_with_narc_card(create_evidence("Surveillance", 20, 5));
        hs.current_player_index = 1; // Narc already acted
        // play_card pushes to cards_played (the production write path)
        hs.cards_played.push(create_evidence("Anonymous Tip", 5, 20));
        let intent = narc_intent(&hs).unwrap();
        assert_eq!(intent.verb, "PLAYED");
        assert_eq!(intent.card_name, "ANONYMOUS TIP");
    }

    #[test]
    fn intent_played_ignores_non_narc_cards_on_top() {
        // Player/buyer cards played after the narc's must not displace the
        // PLAYED display - only Evidence/Conviction are narc plays
        let mut hs = hand_state_with_narc_card(create_evidence("Surveillance", 20, 5));
        hs.current_player_index = 2; // everyone acted
        hs.cards_played.push(create_evidence("Anonymous Tip", 5, 20));
        hs.cards_played.push(create_product("Weed", 30, 5));
        hs.cards_played.push(create_cover("Alibi", 30, -5));
        let intent = narc_intent(&hs).unwrap();
        assert_eq!(intent.verb, "PLAYED");
        assert_eq!(intent.card_name, "ANONYMOUS TIP");
    }

    #[test]
    fn intent_hidden_when_hand_over() {
        let mut hs = hand_state_with_narc_card(create_evidence("Surveillance", 20, 5));
        hs.current_state = HandPhase::Bust;
        assert!(narc_intent(&hs).is_none());
    }

    #[test]
    fn conviction_intent_shows_raw_threshold() {
        // Resolution checks conviction thresholds unmultiplied - the telegraph
        // must show the number that will actually be used
        let mut hs = hand_state_with_narc_card(create_conviction("Warrant", 30));
        hs.narc_upgrade_tier = crate::save::UpgradeTier::Tier2;
        let intent = narc_intent(&hs).unwrap();
        assert_eq!(intent.rows[0], ("⚠", "busts at 30".to_string()));
    }

    // ---- buyer_played ----

    #[test]
    fn buyer_bubble_shows_last_reaction() {
        let mut hs = HandState::default();
        hs.current_state = HandPhase::DealerReveal;
        hs.cards_mut(Owner::Buyer).played.push(create_buyer_location("By the Pool", 5, 25, -10));
        let view = buyer_played(&hs).expect("bubble should show after buyer reacts");
        assert_eq!(view.card_name, "BY THE POOL");
        assert_eq!(
            view.rows,
            vec![
                ("🔍", "+5".to_string()),
                ("🛡", "+25".to_string()),
                ("🔥", "-10".to_string())
            ]
        );
    }

    #[test]
    fn buyer_bubble_hidden_before_first_reaction_and_after_hand() {
        let mut hs = HandState::default();
        hs.current_state = HandPhase::PlayerPhase;
        assert!(buyer_played(&hs).is_none());
        hs.cards_mut(Owner::Buyer).played.push(create_buyer_modifier("Secrecy", 1.0, 0, 20, -10));
        assert!(buyer_played(&hs).is_some());
        hs.current_state = HandPhase::Bust;
        assert!(buyer_played(&hs).is_none());
    }

    #[test]
    fn buyer_modifier_rows_skip_zero_stats() {
        let mut hs = HandState::default();
        hs.current_state = HandPhase::DealerReveal;
        // mult 1.0 and evidence 0 are omitted; cover and heat always shown
        hs.cards_mut(Owner::Buyer).played.push(create_buyer_modifier("Secrecy", 1.0, 0, 20, -10));
        let view = buyer_played(&hs).unwrap();
        assert_eq!(view.rows, vec![("🛡", "+20".to_string()), ("🔥", "-10".to_string())]);
    }

    // ---- buyer_confidence ----

    fn hand_state_with_scenario_threshold(heat_threshold: Option<u32>) -> HandState {
        use crate::models::buyer::{BuyerDemand, BuyerPersona, BuyerScenario};
        let mut hs = HandState::default();
        hs.buyer_persona = Some(BuyerPersona {
            display_name: "Test Buyer".to_string(),
            demand: BuyerDemand {
                products: vec![],
                locations: vec![],
                description: String::new(),
            },
            base_multiplier: 1.0,
            reduced_multiplier: 1.0,
            evidence_threshold: None,
            reaction_deck_ids: vec![],
            reaction_deck: vec![],
            scenarios: vec![BuyerScenario {
                display_name: "Test".to_string(),
                products: vec![],
                locations: vec![],
                heat_threshold,
                description: String::new(),
                narrative_fragments: None,
            }],
            active_scenario_index: Some(0),
        });
        hs
    }

    #[test]
    fn confidence_tracks_heat_proximity() {
        let mut hs = hand_state_with_scenario_threshold(Some(100));
        hs.current_heat = 66; // 0.66 <= 2/3 -> still confident
        assert_eq!(buyer_confidence(&hs), Some(BuyerConfidence::Confident));
        hs.current_heat = 67; // past two-thirds -> nervous
        assert_eq!(buyer_confidence(&hs), Some(BuyerConfidence::Nervous));
        hs.current_heat = 100; // exactly ON the line survives resolution -> nervous
        assert_eq!(buyer_confidence(&hs), Some(BuyerConfidence::Nervous));
        hs.current_heat = 101; // over the line: bails if resolved now -> scared
        assert_eq!(buyer_confidence(&hs), Some(BuyerConfidence::Scared));
        hs.current_heat = -10; // cooling run clamps to 0
        assert_eq!(buyer_confidence(&hs), Some(BuyerConfidence::Confident));
    }

    #[test]
    fn confidence_uses_worst_axis() {
        // Heat is comfortable but evidence is past the persona's bail line
        let mut hs = hand_state_with_scenario_threshold(Some(100));
        hs.buyer_persona.as_mut().unwrap().evidence_threshold = Some(10);
        hs.current_heat = 10;
        hs.cards_played.push(create_evidence("Stakeout", 11, 0)); // 1.1 -> scared
        assert_eq!(buyer_confidence(&hs), Some(BuyerConfidence::Scared));
    }

    #[test]
    fn fearless_buyer_is_always_confident() {
        let mut hs = hand_state_with_scenario_threshold(None);
        hs.current_heat = 500;
        assert_eq!(buyer_confidence(&hs), Some(BuyerConfidence::Confident));
        assert!(buyer_confidence(&HandState::default()).is_none()); // no persona
    }

    // ---- turn_pill / round_header ----

    #[test]
    fn pill_tracks_actor() {
        let mut hs = HandState::default();
        hs.current_state = HandPhase::PlayerPhase;
        hs.current_player_index = 0;
        assert_eq!(turn_pill(&hs), ("NARC'S MOVE", PillActor::Narc));
        hs.current_player_index = 1;
        assert_eq!(turn_pill(&hs), ("YOUR MOVE", PillActor::Player));
        hs.current_state = HandPhase::DealerReveal;
        assert_eq!(turn_pill(&hs), ("BUYER REACTING", PillActor::Buyer));
        hs.current_state = HandPhase::Bust;
        assert_eq!(turn_pill(&hs), ("DEAL COMPLETE", PillActor::Neutral));
    }

    #[test]
    fn round_header_shows_round_and_status() {
        let mut hs = HandState::default();
        hs.current_round = 2;
        hs.current_state = HandPhase::PlayerPhase;
        assert_eq!(round_header(&hs), "ROUND 2 / 3  ·  DEAL IN PROGRESS");
        hs.current_state = HandPhase::Bust;
        assert_eq!(round_header(&hs), "ROUND 2 / 3  ·  DEAL COMPLETE");
    }

    // ---- conviction_ticks ----

    #[test]
    fn ticks_derive_from_narc_conviction_cards() {
        let mut hs = HandState::default();
        hs.cards_mut(Owner::Narc).deck = vec![
            create_conviction("Random Search", 90),
            create_evidence("Patrol", 5, 5),
            create_conviction("Warrant", 30),
        ];
        hs.cards_mut(Owner::Narc).hand = [Some(create_conviction("Caught Red-Handed", 60)), None, None];
        let ticks = conviction_ticks(&hs);
        assert_eq!(
            ticks,
            vec![
                (30, "Warrant".to_string()),
                (60, "Caught Red-Handed".to_string()),
                (90, "Random Search".to_string())
            ]
        );
    }

    #[test]
    fn ticks_above_bar_max_are_dropped() {
        let mut hs = HandState::default();
        hs.cards_mut(Owner::Narc).deck = vec![create_conviction("Federal Case", 250)];
        assert!(conviction_ticks(&hs).is_empty());
    }

    // ---- format_cash ----

    #[test]
    fn cash_formats_with_separators() {
        assert_eq!(format_cash(0), "$0");
        assert_eq!(format_cash(950), "$950");
        assert_eq!(format_cash(2400), "$2,400");
        assert_eq!(format_cash(1234567), "$1,234,567");
    }

    // ---- game_over_board ----

    #[test]
    fn game_over_board_ranks_and_marks_this_run() {
        use crate::save::EmpireEpitaph;
        let epitaph = |revenue: u64, decks: u32| EmpireEpitaph {
            ended_at: 0,
            lifetime_revenue: revenue,
            cash_at_fall: 0,
            dealers_hired: 0,
            total_prior_convictions: 0,
            decks_played: decks,
            stories: vec![],
        };
        // Latest fall (600) places 2nd on the board and gets the marker
        let fallen = vec![epitaph(900, 3), epitaph(100, 1), epitaph(600, 9)];
        let board = game_over_board(&fallen);
        let lines: Vec<&str> = board.lines().collect();
        assert_eq!(lines[0], "1. $900 · 3 decks");
        assert_eq!(lines[1], "2. $600 · 9 decks  ← THIS RUN");
        assert_eq!(lines[2], "3. $100 · 1 decks");
        assert!(game_over_board(&[]).is_empty());
    }
}
