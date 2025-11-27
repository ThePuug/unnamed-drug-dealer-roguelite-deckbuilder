// Character state and save integration systems

use bevy::prelude::*;
use crate::save::{SaveManager, SaveData, CharacterState, HeatTier};
use crate::models::hand_state::{HandState, HandPhase, HandOutcome};
use crate::ui::components::{CharacterHeatText, CharacterTierText, DecayInfoDisplay, AccountCashText, LifetimeRevenueText, StoryHistoryText, StoryHistoryButton, StoryHistoryOverlay, StoryHistoryCloseButton};

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

    if let Some(character) = &save_data.character {
        info!("Loaded character with Heat: {}, Decks played: {}",
            character.heat, character.decks_played);
    } else {
        info!("No existing character - will create on first run");
    }

    commands.insert_resource(save_data);
    character_loaded.0 = true;
}

/// System to apply heat decay when starting a new deck (entering DeckBuilding)
pub fn apply_decay_system(
    mut save_data: ResMut<SaveData>,
    save_manager: Res<SaveManager>,
    mut decay_info: ResMut<DecayInfo>,
) {
    if let Some(ref mut character) = save_data.character {
        let decay = character.apply_decay();
        if decay > 0 {
            info!("Heat decayed by {} (from time elapsed)", decay);
            decay_info.decay_amount = decay;
            decay_info.displayed = false;

            // Save the decayed state
            if let Err(e) = save_manager.save(&save_data) {
                warn!("Failed to save after decay: {:?}", e);
            }
        }
    }
}

/// System to create new character when starting a run without one
pub fn ensure_character_on_run_start(
    mut save_data: ResMut<SaveData>,
    save_manager: Res<SaveManager>,
) {
    if save_data.character.is_none() {
        info!("Creating new character for first run");
        save_data.character = Some(CharacterState::new());

        if let Err(e) = save_manager.save(&save_data) {
            warn!("Failed to save new character: {:?}", e);
        }
    }
}

/// System to save character heat and account cash after hand resolution
pub fn save_after_resolution_system(
    hand_state_query: Query<&HandState, Changed<HandState>>,
    mut save_data: ResMut<SaveData>,
    save_manager: Res<SaveManager>,
) {
    for hand_state in hand_state_query.iter() {
        // Only save when hand reaches terminal state with an outcome
        if hand_state.current_state != HandPhase::Bust {
            continue;
        }

        let Some(outcome) = &hand_state.outcome else {
            continue;
        };

        // Check if we have a character to update
        if save_data.character.is_none() {
            continue;
        }

        // RFC-016: Add profit to account-wide cash on Safe outcome
        // Do this first before potentially deleting character
        if *outcome == HandOutcome::Safe && hand_state.last_profit > 0 {
            save_data.account.add_profit(hand_state.last_profit);
            info!(
                "Profit earned: ${} (Account total: ${})",
                hand_state.last_profit,
                save_data.account.cash_on_hand
            );

            // RFC-017: Increment play counts for player cards on successful deal
            // Only player card types get upgrades (not Narc Evidence/Conviction)
            if let Some(ref mut character) = save_data.character {
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

        // Log outcome (heat transfer happens in mark_deck_completed_system when deck ends)
        if let Some(ref character) = save_data.character {
            match outcome {
                HandOutcome::Busted => {
                    // Permadeath - delete character (account cash survives!)
                    info!("Character busted! Permadeath triggered. Account cash preserved: ${}",
                          save_data.account.cash_on_hand);
                }
                HandOutcome::Safe => {
                    info!("Hand resolved Safe, Deck heat: {}, Character heat: {}",
                          hand_state.current_heat, character.heat);
                }
                HandOutcome::Folded => {
                    info!("Hand resolved as Folded, Deck heat: {}", hand_state.current_heat);
                }
                HandOutcome::InvalidDeal | HandOutcome::BuyerBailed => {
                    info!("Deal incomplete: {:?}", outcome);
                }
            }
        }

        // Handle permadeath after character borrow is released
        if *outcome == HandOutcome::Busted {
            save_data.character = None;
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

/// System to update character heat display UI
pub fn update_character_heat_display_system(
    save_data: Option<Res<SaveData>>,
    mut heat_text_query: Query<&mut Text, (With<CharacterHeatText>, Without<CharacterTierText>, Without<DecayInfoDisplay>)>,
    mut tier_text_query: Query<(&mut Text, &mut TextColor), (With<CharacterTierText>, Without<CharacterHeatText>, Without<DecayInfoDisplay>)>,
) {
    let Some(save_data) = save_data else {
        return;
    };

    let (heat, tier) = if let Some(ref character) = save_data.character {
        (character.heat, character.heat_tier())
    } else {
        (0, HeatTier::Cold)
    };

    // Update heat text
    for mut text in heat_text_query.iter_mut() {
        **text = format!("Heat: {}", heat);
    }

    // Update tier text with color
    for (mut text, mut color) in tier_text_query.iter_mut() {
        **text = format!("[{}]", tier.name());
        let (r, g, b) = tier.color();
        *color = TextColor(Color::srgb(r, g, b));
    }
}

/// System to display decay information
pub fn update_decay_display_system(
    decay_info: Res<DecayInfo>,
    mut decay_text_query: Query<(&mut Text, &mut Visibility), With<DecayInfoDisplay>>,
) {
    for (mut text, mut visibility) in decay_text_query.iter_mut() {
        if decay_info.decay_amount > 0 && !decay_info.displayed {
            **text = format!("Heat decayed by {} while away", decay_info.decay_amount);
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

    let stories = save_data.character.as_ref()
        .map(|c| &c.story_history)
        .filter(|h| !h.is_empty());

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
