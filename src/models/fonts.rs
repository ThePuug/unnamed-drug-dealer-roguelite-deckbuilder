// Font Resources
// Provides access to custom fonts like emoji support
// Default font (DejaVuSans) is set in main.rs and supports Unicode including ★ U+2605

use bevy::prelude::*;

/// Emoji font for emoji icons on cards (NotoEmoji)
#[derive(Resource)]
pub struct EmojiFont(pub Handle<Font>);
