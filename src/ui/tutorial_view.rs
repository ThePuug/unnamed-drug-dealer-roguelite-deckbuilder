// SOW-032: Tutorial Arc "Road to Your First Dealer" - PURE beat detection +
// presentation. Same rule as view.rs / map_view.rs / ledger_view.rs:
// everything here is unit-testable without ECS; systems/tutorial.rs only
// orchestrates.
//
// HARD RULE (SOW-032, echoing SOW-030): derive, don't record. Beat completion
// is DETECTED from existing SaveData fields - no beat records its own flag, no
// beat grants anything. The only new save state is the cursor (how far the
// guided player has walked) + the status (offered/accepted/declined/graduated),
// and that state is purely presentational: declining or dismissing confers NO
// gameplay benefit (the hard invariant).
//
// RECONCILED to the loop as it exists on this base (SOW-037/038/039 merged):
//   - Hiring is MAP-ONLY (the roster HIRE button was retired in SOW-039). The
//     first hire is the zone's SIGNATURE dealer (Bubba @ Trailer Park, $500, no
//     cred gate) on the CITY MAP, so beat 6 teaches MAP hiring.
//   - Products are consumable STOCK (SOW-034): a buy_batch grows unlocked_cards
//     and adds charges; utility cards are NOT consumable (SOW-040 reversed), so
//     the restock beat is products-only.

use crate::save::{SaveData, TutorialState, TutorialStatus};

/// The six guided-play beats, in play order. Each completes by DOING the
/// action in ordinary play; the predicate reads existing save fields only.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Beat {
    /// 1. First banked deal (teaches product + spot + burning a charge).
    FirstDeal,
    /// 2. First session taken home (teaches session -> career heat transfer).
    GoHomeHot,
    /// 3. First front taken (teaches the debt clock).
    FirstFront,
    /// 4. First front resolved - paid OR defaulted-then-soured (both teach).
    FirstPayback,
    /// 5. First restock / new batch bought (teaches consumable stock + the ladder).
    Restock,
    /// 6. Graduation: the first dealer hire on the city map.
    Graduation,
}

impl Beat {
    /// The beats in play order. The cursor is an index into this.
    pub const ORDER: [Beat; 6] = [
        Beat::FirstDeal,
        Beat::GoHomeHot,
        Beat::FirstFront,
        Beat::FirstPayback,
        Beat::Restock,
        Beat::Graduation,
    ];

    /// The beat sitting at a cursor position (0..=5). `ORDER.len()` (6) means
    /// every beat has been walked - the arc has graduated.
    pub fn at_cursor(cursor: u8) -> Option<Beat> {
        Self::ORDER.get(cursor as usize).copied()
    }

    /// The goal-strip headline in the game's house voice - the inner monologue
    /// of someone starting at the bottom.
    pub fn line(self) -> &'static str {
        match self {
            Beat::FirstDeal => "Sell something. Anything.",
            Beat::GoHomeHot => "Take the night's heat home.",
            Beat::FirstFront => "Short on cash? Put it on the tab.",
            Beat::FirstPayback => "Square up before the muscle does.",
            Beat::Restock => "Never run dry - buy the next batch.",
            Beat::Graduation => "The Kingpin needs hands. $500 says someone's hungry.",
        }
    }

    /// The one-line hint under the headline.
    pub fn hint(self) -> &'static str {
        match self {
            Beat::FirstDeal => {
                "Play your Weed on a spot they want - each play burns one of your charges."
            }
            Beat::GoHomeHot => "Finish a run and GO HOME; the session's heat follows you.",
            Beat::FirstFront => {
                "Take a FRONT batch from the zone's supplier - the debt clock is the lesson."
            }
            Beat::FirstPayback => "PAY the front before the window closes (or learn the hard way).",
            Beat::Restock => "Buy a product batch in the SHOP - stock is spent, not permanent.",
            Beat::Graduation => "Open the CITY MAP and hire the zone's dealer.",
        }
    }
}

/// The send-off shown as the arc retires at graduation (the closing beat).
pub const GRADUATION_LINE: &str = "You're not a dealer anymore. You're a kingpin.";
/// No hint on the closing line - it stands alone.
pub const GRADUATION_HINT: &str = "";

/// One-time offer copy (house voice). ACCEPT takes the guided start; DECLINE
/// skips - and skipping is always free, because the arc never held anything back.
pub const OFFER_TITLE: &str = "ROAD TO YOUR FIRST DEALER";
pub const OFFER_BODY: &str = "New to these streets? The Kingpin can point you toward your first \
crew - one goal at a time, same risks, same money. Nothing here is handed to you that ordinary \
hustle wouldn't earn. Skip it whenever you like; it never holds anything back.";
pub const OFFER_ACCEPT: &str = "TAKE THE GUIDED START";
pub const OFFER_DECLINE: &str = "I KNOW THE STREETS";
/// The always-free mid-arc dismiss on the goal strip.
pub const DISMISS_LABEL: &str = "SKIP THE LESSONS";

/// Whether a beat's completion condition holds in the given save. PURE over
/// EXISTING fields only - no cursor, no tutorial state. The cursor's ordering
/// job (which makes beat 4 mean "PAID", not "never fronted") lives in
/// `TutorialState::advance`, not here.
pub fn beat_satisfied(save: &SaveData, beat: Beat) -> bool {
    match beat {
        // 1. FIRST DEAL - a hand banked. add_profit bumps hands_completed.
        Beat::FirstDeal => save.account.hands_completed >= 1,
        // 2. GO HOME HOT - a session banked somewhere in the roster
        // (mark_deck_completed bumps decks_played on the running dealer).
        Beat::GoHomeHot => {
            save.dealers
                .iter()
                .map(|d| d.character.decks_played)
                .sum::<u32>()
                >= 1
        }
        // 3. FIRST FRONT - a batch is on the books.
        Beat::FirstFront => !save.fronts.is_empty(),
        // 4. FIRST PAYBACK - the books are clean AGAIN. The cursor guarantees a
        //    front was taken first (advance only reaches this beat past beat 3),
        //    so an empty ledger here means PAID or SOURED - not "never fronted".
        Beat::FirstPayback => save.fronts.is_empty(),
        // 5. RESTOCK - a product batch was bought in the shop. Counts the exact
        //    event the beat teaches: buy_batch (a cash product purchase) bumps
        //    this, INCLUDING a same-product restock. The seeded starter uses
        //    add_stock (counter stays 0 on a fresh save), and non-product
        //    one-time unlocks (spots/cover/insurance/modifiers) never route
        //    through buy_batch, so they never satisfy this beat.
        Beat::Restock => save.account.product_batches_bought >= 1,
        // 6. GRADUATION - the first hire lands (roster past the kingpin).
        Beat::Graduation => save.dealers.len() >= 2,
    }
}

/// The goal strip's rendered content for the current tutorial state. `retired`
/// means the arc is over (declined or graduated) - the strip stops advancing.
/// An empty `line` means "hide the strip entirely".
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GoalStripView {
    pub line: String,
    pub hint: String,
    pub retired: bool,
}

impl GoalStripView {
    fn hidden(retired: bool) -> Self {
        Self {
            line: String::new(),
            hint: String::new(),
            retired,
        }
    }

    fn graduated() -> Self {
        Self {
            line: GRADUATION_LINE.to_string(),
            hint: GRADUATION_HINT.to_string(),
            retired: true,
        }
    }
}

/// Derive what the goal strip should show. Reads the save (for the graduation
/// short-circuit) and the tutorial state (for status + cursor).
///
/// - Offered:   hidden (the offer overlay owns the screen).
/// - Declined:  hidden, retired (a skipped run looks identical to pre-arc play).
/// - Graduated: the closing send-off, retired.
/// - Accepted:  the beat at the cursor - UNLESS the first hire already landed
///              (hire-first fast-forward), which retires the arc immediately
///              even if the progress system hasn't latched Graduated yet.
pub fn derive_view(save: &SaveData, state: &TutorialState) -> GoalStripView {
    match state.status {
        TutorialStatus::Offered => GoalStripView::hidden(false),
        TutorialStatus::Declined => GoalStripView::hidden(true),
        TutorialStatus::Graduated => GoalStripView::graduated(),
        TutorialStatus::Accepted => {
            if beat_satisfied(save, Beat::Graduation) {
                return GoalStripView::graduated();
            }
            match Beat::at_cursor(state.cursor) {
                Some(beat) => GoalStripView {
                    line: beat.line().to_string(),
                    hint: beat.hint().to_string(),
                    retired: false,
                },
                // Cursor past the last beat with status still Accepted should
                // not happen (advance flips to Graduated), but retire cleanly.
                None => GoalStripView::graduated(),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::save::{
        AccountState, DealerState, TutorialState, TutorialStatus, BATCH_SIZE, DEFAULT_STATION,
    };

    fn accepted(cursor: u8) -> TutorialState {
        TutorialState {
            status: TutorialStatus::Accepted,
            cursor,
        }
    }

    #[test]
    fn beat_first_deal_true_false() {
        let mut save = SaveData::new();
        assert!(!beat_satisfied(&save, Beat::FirstDeal));
        save.account.hands_completed = 1;
        assert!(beat_satisfied(&save, Beat::FirstDeal));
    }

    #[test]
    fn beat_go_home_hot_true_false() {
        let mut save = SaveData::new();
        assert!(!beat_satisfied(&save, Beat::GoHomeHot));
        save.dealers[0].character.decks_played = 1;
        assert!(beat_satisfied(&save, Beat::GoHomeHot));
    }

    #[test]
    fn beat_first_front_true_false() {
        let mut save = SaveData::new();
        assert!(!beat_satisfied(&save, Beat::FirstFront));
        save.account.unlocked_cards.insert("shrooms".to_string());
        save.take_front("shrooms", DEFAULT_STATION, 100).unwrap();
        assert!(beat_satisfied(&save, Beat::FirstFront));
    }

    #[test]
    fn beat_first_payback_reads_empty_ledger() {
        // Raw predicate is "books clean" - trivially true on a fresh save; the
        // cursor gate (advance) is what gives it the "PAID" meaning in play.
        let mut save = SaveData::new();
        assert!(beat_satisfied(&save, Beat::FirstPayback));
        save.account.unlocked_cards.insert("shrooms".to_string());
        save.take_front("shrooms", DEFAULT_STATION, 100).unwrap();
        assert!(!beat_satisfied(&save, Beat::FirstPayback)); // front live
        save.account.cash_on_hand = 1000;
        save.pay_front(DEFAULT_STATION);
        assert!(beat_satisfied(&save, Beat::FirstPayback)); // paid -> clean again
    }

    #[test]
    fn beat_restock_true_false() {
        let mut save = SaveData::new();
        assert!(!beat_satisfied(&save, Beat::Restock));
        save.account.cash_on_hand = 1000;
        save.account.buy_batch("shrooms", 50, BATCH_SIZE);
        assert!(beat_satisfied(&save, Beat::Restock));
    }

    #[test]
    fn beat_restock_fires_on_same_product_restock() {
        // Regression: the seeded starter is weed (added via add_stock, so the
        // counter is 0). Restocking that same product via buy_batch - exactly
        // what the goal strip tells the player to do - must satisfy the beat,
        // even though unlocked_cards never changes (weed is already present).
        let mut save = SaveData::new();
        assert!(!beat_satisfied(&save, Beat::Restock));
        assert_eq!(save.account.unlocked_cards, AccountState::starting_collection());
        save.account.cash_on_hand = 1000;
        save.account.buy_batch("weed", 25, BATCH_SIZE);
        // Collection is unchanged (a no-op re-insert), but the beat still fires.
        assert_eq!(save.account.unlocked_cards, AccountState::starting_collection());
        assert!(beat_satisfied(&save, Beat::Restock));
    }

    #[test]
    fn beat_restock_ignores_non_product_unlock() {
        // Regression: a non-product one-time unlock (a spot/cover/insurance/
        // modifier) grows unlocked_cards but never routes through buy_batch, so
        // it must NOT satisfy the restock beat (the old collection-diff proxy
        // false-fired here).
        let mut save = SaveData::new();
        // Mirror shop.rs's None branch for a one-time unlock: insert access,
        // no buy_batch.
        save.account.unlocked_cards.insert("at_the_park".to_string());
        assert_ne!(save.account.unlocked_cards, AccountState::starting_collection());
        assert!(!beat_satisfied(&save, Beat::Restock));
    }

    #[test]
    fn beat_graduation_true_false() {
        let mut save = SaveData::new();
        assert!(!beat_satisfied(&save, Beat::Graduation));
        save.dealers
            .push(DealerState::zone_dealer(DEFAULT_STATION, "Bubba", "Bubba"));
        assert!(beat_satisfied(&save, Beat::Graduation));
    }

    #[test]
    fn advance_walks_beats_in_order() {
        let mut save = SaveData::new();
        let mut tut = accepted(0);

        // Nothing done yet.
        tut.advance(&save);
        assert_eq!(tut.cursor, 0);

        // Beat 1.
        save.account.hands_completed = 1;
        tut.advance(&save);
        assert_eq!(tut.cursor, 1);

        // Beat 2.
        save.dealers[0].character.decks_played = 1;
        tut.advance(&save);
        assert_eq!(tut.cursor, 2);

        // Beat 3: a live front.
        save.dealers[0].add_cred(DEFAULT_STATION);
        save.account.unlocked_cards.insert("shrooms".to_string());
        save.account.cash_on_hand = 500;
        save.take_front("shrooms", DEFAULT_STATION, 100).unwrap();
        tut.advance(&save);
        assert_eq!(tut.cursor, 3);

        // Beat 4: pay it back. The books are clean, so the cursor walks to beat
        // 5 - but STOPS there: fronting shrooms was not a bought batch, so the
        // Restock beat is still pending (this is the false-positive the fix
        // closes - a front is not a shop purchase).
        save.pay_front(DEFAULT_STATION);
        tut.advance(&save);
        assert_eq!(tut.cursor, 4);
        assert_eq!(tut.status, TutorialStatus::Accepted);

        // Beat 5: buy a product batch in the shop (the real restock event).
        save.account.cash_on_hand = 1000;
        assert!(save.account.buy_batch("shrooms", 50, BATCH_SIZE));
        tut.advance(&save);
        assert_eq!(tut.cursor, 5);
        assert_eq!(tut.status, TutorialStatus::Accepted);

        // Beat 6: the first hire graduates the arc.
        save.dealers
            .push(DealerState::zone_dealer(DEFAULT_STATION, "Bubba", "Bubba"));
        tut.advance(&save);
        assert_eq!(tut.status, TutorialStatus::Graduated);
        assert_eq!(tut.cursor, Beat::ORDER.len() as u8);
    }

    #[test]
    fn advance_latches_past_paid_front_without_regressing_beat_three() {
        // A guided player who took a front (cursor 3) and paid it off: the
        // cursor moves forward, and beat 3's raw predicate going false again
        // (books clean) must NOT drag the cursor back.
        let mut save = SaveData::new();
        save.account.hands_completed = 1;
        save.dealers[0].character.decks_played = 1;
        save.dealers[0].add_cred(DEFAULT_STATION);
        save.account.unlocked_cards.insert("shrooms".to_string());
        save.account.cash_on_hand = 1000;
        save.take_front("shrooms", DEFAULT_STATION, 100).unwrap();
        let mut tut = accepted(3);

        // Front live: beat 4 (books clean) unsatisfied -> stays at 3.
        tut.advance(&save);
        assert_eq!(tut.cursor, 3);

        save.pay_front(DEFAULT_STATION);
        tut.advance(&save);
        assert!(tut.cursor >= 4, "cursor should walk past the paid front");
        // Beat 3's raw predicate is now false...
        assert!(!beat_satisfied(&save, Beat::FirstFront));
        // ...but the cursor never regressed below it.
        assert!(tut.cursor >= 3);
    }

    #[test]
    fn advance_never_decrements() {
        // Cursor high, but earlier beats' raw predicates unsatisfied: never drop.
        let save = SaveData::new(); // no beats satisfied
        let mut tut = accepted(4);
        tut.advance(&save);
        assert!(tut.cursor >= 4);
    }

    #[test]
    fn hire_first_fast_forwards_to_graduated() {
        // A guided player who hires before ever fronting: dealers == 2, cursor
        // still 0. advance short-circuits straight to Graduated, and the view
        // retires with the closing line.
        let mut save = SaveData::new();
        save.dealers
            .push(DealerState::zone_dealer(DEFAULT_STATION, "Bubba", "Bubba"));
        let mut tut = accepted(0);
        tut.advance(&save);
        assert_eq!(tut.status, TutorialStatus::Graduated);
        assert_eq!(tut.cursor, Beat::ORDER.len() as u8);

        let view = derive_view(&save, &tut);
        assert!(view.retired);
        assert_eq!(view.line, GRADUATION_LINE);
    }

    #[test]
    fn derive_hire_first_retires_even_before_advance() {
        // derive_view itself retires an Accepted save whose first hire already
        // landed, so the strip never flashes a stale beat for a frame.
        let mut save = SaveData::new();
        save.dealers
            .push(DealerState::zone_dealer(DEFAULT_STATION, "Bubba", "Bubba"));
        let view = derive_view(&save, &accepted(0));
        assert!(view.retired);
        assert_eq!(view.line, GRADUATION_LINE);
    }

    #[test]
    fn declined_stays_hidden_and_never_graduates() {
        // A skipped run must look identical to pre-arc play - even a hire must
        // not resurrect the strip with a graduation line.
        let mut save = SaveData::new();
        save.dealers
            .push(DealerState::zone_dealer(DEFAULT_STATION, "Bubba", "Bubba"));
        let mut tut = TutorialState {
            status: TutorialStatus::Declined,
            cursor: 0,
        };
        tut.advance(&save);
        assert_eq!(tut.status, TutorialStatus::Declined);

        let view = derive_view(&save, &tut);
        assert!(view.line.is_empty());
        assert!(view.retired);
    }

    #[test]
    fn derive_offered_is_hidden_not_retired() {
        let save = SaveData::new();
        let view = derive_view(
            &save,
            &TutorialState {
                status: TutorialStatus::Offered,
                cursor: 0,
            },
        );
        assert!(view.line.is_empty());
        assert!(!view.retired);
    }

    #[test]
    fn derive_accepted_shows_current_beat() {
        let save = SaveData::new();
        let view = derive_view(&save, &accepted(0));
        assert_eq!(view.line, Beat::FirstDeal.line());
        assert_eq!(view.hint, Beat::FirstDeal.hint());
        assert!(!view.retired);

        let view = derive_view(&save, &accepted(1));
        assert_eq!(view.line, Beat::GoHomeHot.line());
    }
}
