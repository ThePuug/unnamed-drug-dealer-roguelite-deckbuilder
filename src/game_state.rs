// SOW-AAA Phase 8: Game state types extracted from main.rs
// Core game state management

use bevy::prelude::*;

/// Game states for deck building vs gameplay (SOW-006)
#[derive(States, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum GameState {
    #[default]
    DeckBuilding,  // Pre-run deck selection
    InRun,         // Active gameplay
}

/// SOW-008 Phase 1: AI pacing timers
#[derive(Resource)]
pub struct AiActionTimer {
    pub ai_timer: Timer,
    pub dealer_timer: Timer,
    pub dealer_timer_started: bool, // Track if we've started the dealer timer this state
}

impl Default for AiActionTimer {
    fn default() -> Self {
        Self {
            ai_timer: Timer::from_seconds(1.0, TimerMode::Repeating), // 1s delay, repeating
            dealer_timer: Timer::from_seconds(1.0, TimerMode::Repeating), // 1s delay, repeating
            dealer_timer_started: false,
        }
    }
}
