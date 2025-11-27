// SOW-AAA: Systems module
// Bevy systems extracted from main.rs

pub mod input;
pub mod ui_update;
pub mod game_loop;
pub mod save_integration;
pub mod upgrade_choice;
pub mod shop;

pub use input::*;
pub use ui_update::*;
pub use game_loop::*;
pub use save_integration::*;
pub use upgrade_choice::*;
pub use shop::*;
