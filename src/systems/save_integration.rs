// Character state and save integration systems

use bevy::prelude::*;
use crate::save::{SaveManager, SaveData};
use crate::models::hand_state::{HandState, HandPhase, HandOutcome};
use crate::ui::components::{DecayInfoDisplay, AccountCashText, LifetimeRevenueText, StoryHistoryText, StoryHistoryButton, StoryHistoryOverlay, StoryHistoryCloseButton};

/// Resource tracking if character data has been loaded this session
#[derive(Resource, Default)]
pub struct CharacterLoaded(pub bool);

/// Resource tracking decay info to display to player
#[derive(Resource, Default)]
pub struct DecayInfo {
    pub decay_amount: u32,
    pub displayed: bool,
}

/// System to load character state on game startup (entering DeckBuilding)
pub fn load_character_system(
    mut commands: Commands,
    save_manager: Res<SaveManager>,
    mut character_loaded: ResMut<CharacterLoaded>,
) {
    // Only load once per session
    if character_loaded.0 {
        return;
    }

    let save_data = save_manager.load_or_create();

    // RFC-023: sentences are turn-based (ticked in go_home_button_system as
    // runs complete) - nothing to sweep on load

    info!(
        "Loaded roster: {} dealer(s); active: {} (Heat: {}, Decks played: {})",
        save_data.dealers.len(),
        save_data.active_dealer_state().name,
        save_data.active_character().heat,
        save_data.active_character().decks_played
    );

    commands.insert_resource(save_data);
    character_loaded.0 = true;
}

/// System to apply heat decay when starting a new deck (entering DeckBuilding)
pub fn apply_decay_system(
    mut save_data: ResMut<SaveData>,
    save_manager: Res<SaveManager>,
    mut decay_info: ResMut<DecayInfo>,
) {
    // RFC-023: decay applies per dealer; jailed dealers skip it - serving
    // the term IS their heat reset, decaying them too would double-dip
    let active_idx = save_data.active_dealer;
    let mut total_decay = 0;
    let mut active_decay = 0;
    for (idx, dealer) in save_data.dealers.iter_mut().enumerate() {
        if !dealer.is_available() {
            continue;
        }
        let decay = dealer.character.apply_decay();
        if idx == active_idx {
            active_decay = decay;
        }
        total_decay += decay;
    }

    if total_decay > 0 {
        info!("Heat decayed by {} across the roster (from time elapsed)", total_decay);
        // The on-screen callout describes the dealer you're about to send out
        decay_info.decay_amount = active_decay;
        decay_info.displayed = false;

        // Save the decayed state
        if let Err(e) = save_manager.save(&save_data) {
            warn!("Failed to save after decay: {:?}", e);
        }
    }
}

/// RFC-023: Defensive roster guard at run start. The roster invariant says
/// this never fires (a fresh save recruits one dealer), but a run must
/// always have someone to send out.
pub fn ensure_roster_on_run_start(
    mut save_data: ResMut<SaveData>,
    save_manager: Res<SaveManager>,
) {
    if save_data.dealers.is_empty() {
        warn!("Roster empty at run start - recruiting a replacement");
        save_data.dealers.push(crate::save::DealerState::recruit(&[]));
        save_data.active_dealer = 0;

        if let Err(e) = save_manager.save(&save_data) {
            warn!("Failed to save new recruit: {:?}", e);
        }
    }
}

/// System to save character heat and account cash after hand resolution
pub fn save_after_resolution_system(
    hand_state_query: Query<&HandState, Changed<HandState>>,
    mut save_data: ResMut<SaveData>,
    save_manager: Res<SaveManager>,
    mut commands: Commands,
) {
    for hand_state in hand_state_query.iter() {
        // Only save when hand reaches terminal state with an outcome
        if hand_state.current_state != HandPhase::Bust {
            continue;
        }

        let Some(outcome) = &hand_state.outcome else {
            continue;
        };

        // SOW-025: a successful deal earns the runner +1 street cred in the
        // run's area - reputation is per dealer, per territory, and permanent
        if *outcome == HandOutcome::Safe {
            let area = hand_state.run_area.clone();
            let dealer = save_data.active_dealer_state_mut();
            dealer.add_cred(&area);
            info!(
                "{} earned street cred in {} (now {})",
                dealer.name,
                area,
                dealer.cred_in(&area)
            );
        }

        // RFC-016: Add profit to account-wide cash on Safe outcome
        if *outcome == HandOutcome::Safe && hand_state.last_profit > 0 {
            save_data.account.add_profit(hand_state.last_profit);
            info!(
                "Profit earned: ${} (Account total: ${})",
                hand_state.last_profit,
                save_data.account.cash_on_hand
            );

            // RFC-017: Increment play counts for player cards on successful deal
            // Only player card types get upgrades (not Narc Evidence/Conviction)
            {
                let character = save_data.active_character_mut();
                for card in &hand_state.cards_played {
                    // Player card types: Product, Location, Cover, DealModifier, Insurance
                    let is_player_card = matches!(
                        card.card_type,
                        crate::CardType::Product { .. }
                            | crate::CardType::Location { .. }
                            | crate::CardType::Cover { .. }
                            | crate::CardType::DealModifier { .. }
                            | crate::CardType::Insurance { .. }
                    );

                    if is_player_card {
                        character.increment_play_count(&card.name);
                        let tier = character.get_card_tier(&card.name);
                        info!(
                            "Card '{}' play count: {} (Tier: {})",
                            card.name,
                            character.get_play_count(&card.name),
                            tier.name()
                        );

                        // RFC-019: Check if this card has earned a new upgrade
                        if character.queue_pending_upgrade(&card.name, &card.card_type) {
                            info!(
                                "Card '{}' earned an upgrade! Queued for player choice.",
                                card.name
                            );
                        }
                    }
                }
            }
        }

        // Log outcome (heat transfer happens in go_home_button_system)
        match outcome {
            HandOutcome::Busted => {
                info!("{} busted! Jail time incoming. Account cash preserved: ${}",
                      save_data.active_dealer_state().name,
                      save_data.account.cash_on_hand);
            }
            HandOutcome::Safe => {
                info!("Hand resolved Safe, Deck heat: {}, Dealer heat: {}",
                      hand_state.current_heat, save_data.active_character().heat);
            }
            HandOutcome::Folded => {
                info!("Hand resolved as Folded, Deck heat: {}", hand_state.current_heat);
            }
            HandOutcome::InvalidDeal | HandOutcome::BuyerBailed => {
                info!("Deal incomplete: {:?}", outcome);
            }
        }

        // RFC-023: a bust JAILS the active dealer - sentence scales with
        // their heat at the moment of bust (session heat transferred first
        // so the crime is priced at the heat it happened at). If the
        // KINGPIN busts, the empire ends: the one remaining permadeath.
        if *outcome == HandOutcome::Busted {
            if save_data.active_dealer_state().is_kingpin {
                info!("THE KINGPIN WAS BUSTED - the empire falls. Starting fresh.");
                save_data.reset_empire();
                // Drop the stale deck selection so the fresh empire rebuilds
                // its deck builder from the fresh account's collection
                commands.remove_resource::<crate::models::deck_builder::DeckBuilder>();
            } else {
                let session_heat = hand_state.current_heat;
                let dealer = save_data.active_dealer_state_mut();
                dealer.character.apply_session_heat(session_heat);
                dealer.jail();
                if let Some(runs) = dealer.jail_remaining() {
                    info!("{} jailed for {} run(s) (heat {} at bust)",
                          dealer.name, runs, dealer.character.heat);
                }
            }
        }

        // Save updated state
        if let Err(e) = save_manager.save(&save_data) {
            warn!("Failed to save after resolution: {:?}", e);
        }
    }
}

/// System that runs on OnExit(GameState::InRun)
/// Note: Heat transfer now happens in go_home_button_system before HandState is despawned
pub fn mark_deck_completed_system(
    _save_data: ResMut<SaveData>,
    _save_manager: Res<SaveManager>,
    _hand_state_query: Query<&HandState>,
) {
    // Heat transfer and deck completion now handled in go_home_button_system
    // This system kept for potential future cleanup needs on state exit
}

// SOW-023: update_character_heat_display_system removed - the roster panel is
// the per-dealer heat display (the old stats-block line duplicated it)

/// System to display decay information (the only decay surface; names the
/// active dealer since the roster shows several heats now)
pub fn update_decay_display_system(
    decay_info: Res<DecayInfo>,
    save_data: Option<Res<SaveData>>,
    mut decay_text_query: Query<(&mut Text, &mut Visibility), With<DecayInfoDisplay>>,
) {
    for (mut text, mut visibility) in decay_text_query.iter_mut() {
        if decay_info.decay_amount > 0 && !decay_info.displayed {
            let who = save_data
                .as_ref()
                .map(|save| save.active_dealer_state().name.clone())
                .unwrap_or_else(|| "Your dealer".to_string());
            **text = format!(
                "While you were away: {} cooled off by {}",
                who, decay_info.decay_amount
            );
            *visibility = Visibility::Visible;
        } else if decay_info.displayed || decay_info.decay_amount == 0 {
            *visibility = Visibility::Hidden;
        }
    }
}

/// System to clear decay display after it has been shown
pub fn clear_decay_display_system(
    mut decay_info: ResMut<DecayInfo>,
    decay_text_query: Query<&Visibility, With<DecayInfoDisplay>>,
) {
    // Mark as displayed once shown
    if decay_info.decay_amount > 0 && !decay_info.displayed {
        for visibility in decay_text_query.iter() {
            if *visibility == Visibility::Visible {
                decay_info.displayed = true;
            }
        }
    }
}

/// RFC-016: System to update account cash display UI
pub fn update_account_cash_display_system(
    save_data: Option<Res<SaveData>>,
    mut cash_text_query: Query<&mut Text, (With<AccountCashText>, Without<LifetimeRevenueText>)>,
    mut revenue_text_query: Query<&mut Text, (With<LifetimeRevenueText>, Without<AccountCashText>)>,
) {
    let Some(save_data) = save_data else {
        return;
    };

    let cash = save_data.account.cash_on_hand;
    let revenue = save_data.account.lifetime_revenue;

    // Update cash on hand text
    for mut text in cash_text_query.iter_mut() {
        **text = format!("Cash: ${}", format_number(cash));
    }

    // Update lifetime revenue text
    for mut text in revenue_text_query.iter_mut() {
        **text = format!("Lifetime: ${}", format_number(revenue));
    }
}

/// Format a number with comma separators
fn format_number(n: u64) -> String {
    let s = n.to_string();
    let mut result = String::new();
    for (i, c) in s.chars().rev().enumerate() {
        if i > 0 && i % 3 == 0 {
            result.push(',');
        }
        result.push(c);
    }
    result.chars().rev().collect()
}

/// System to update story history display in overlay
pub fn update_story_history_display_system(
    save_data: Option<Res<SaveData>>,
    mut story_text_query: Query<&mut Text, With<StoryHistoryText>>,
) {
    let Some(save_data) = save_data else {
        return;
    };

    // RFC-023: the active dealer's rap sheet (kingpin-wide ledger is SOW-026)
    let history = &save_data.active_character().story_history;
    let stories = (!history.is_empty()).then_some(history);

    for mut text in story_text_query.iter_mut() {
        if let Some(history) = stories {
            // Most recent first, subtle separator between stories
            let story_text = history
                .iter()
                .rev()
                .map(|s| s.as_str())
                .collect::<Vec<_>>()
                .join("\n───\n");
            **text = story_text;
        } else {
            **text = "No stories yet...".to_string();
        }
    }
}

/// System to toggle story history overlay visibility
pub fn story_history_button_system(
    open_query: Query<&Interaction, (Changed<Interaction>, With<StoryHistoryButton>)>,
    close_query: Query<&Interaction, (Changed<Interaction>, With<StoryHistoryCloseButton>)>,
    mut overlay_query: Query<&mut Node, With<StoryHistoryOverlay>>,
) {
    // Open overlay when book button clicked
    for interaction in open_query.iter() {
        if *interaction == Interaction::Pressed {
            if let Ok(mut node) = overlay_query.single_mut() {
                node.display = Display::Flex;
            }
        }
    }

    // Close overlay when close button clicked
    for interaction in close_query.iter() {
        if *interaction == Interaction::Pressed {
            if let Ok(mut node) = overlay_query.single_mut() {
                node.display = Display::None;
            }
        }
    }
}
