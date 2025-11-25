// Character state and save integration systems

use bevy::prelude::*;
use crate::save::{SaveManager, SaveData, CharacterState, HeatTier};
use crate::models::hand_state::{HandState, HandPhase, HandOutcome};
use crate::ui::components::{CharacterHeatText, CharacterTierText, DecayInfoDisplay};

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

/// System to save character heat after hand resolution
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

        // Update character state from HandState
        if let Some(ref mut character) = save_data.character {
            // Transfer heat from hand to character
            character.heat = hand_state.current_heat;

            match outcome {
                HandOutcome::Busted => {
                    // Permadeath - delete character
                    info!("Character busted! Permadeath triggered.");
                    save_data.character = None;
                }
                HandOutcome::Safe | HandOutcome::Folded => {
                    // Character survives, update last_played
                    info!("Hand resolved as {:?}, Heat: {}", outcome, character.heat);
                }
                HandOutcome::InvalidDeal | HandOutcome::BuyerBailed => {
                    // Deal didn't complete, no major state change
                    info!("Deal incomplete: {:?}", outcome);
                }
            }

            // Save updated state
            if let Err(e) = save_manager.save(&save_data) {
                warn!("Failed to save after resolution: {:?}", e);
            }
        }
    }
}

/// System to mark deck as completed and update timestamp when returning home
pub fn mark_deck_completed_system(
    mut save_data: ResMut<SaveData>,
    save_manager: Res<SaveManager>,
    hand_state_query: Query<&HandState>,
) {
    // This runs when transitioning back to DeckBuilding after a run
    // Only count if we had a character and outcome was Safe (completed successfully)

    for hand_state in hand_state_query.iter() {
        if hand_state.current_state != HandPhase::Bust {
            continue;
        }

        if matches!(hand_state.outcome, Some(HandOutcome::Safe)) {
            if let Some(ref mut character) = save_data.character {
                character.mark_deck_completed();
                info!("Deck completed! Total decks: {}", character.decks_played);

                if let Err(e) = save_manager.save(&save_data) {
                    warn!("Failed to save deck completion: {:?}", e);
                }
            }
        }
    }
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
