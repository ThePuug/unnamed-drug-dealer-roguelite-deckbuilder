// SOW-AAA Phase 1: Data creators module
// Pure data creation functions with no game logic dependencies

mod narc_deck;
mod player_deck;
mod buyer_personas;
mod presets;

pub use narc_deck::create_narc_deck;
pub use player_deck::create_player_deck;
pub use buyer_personas::create_buyer_personas;
pub use presets::{validate_deck, create_default_deck, create_aggro_deck, create_control_deck};
