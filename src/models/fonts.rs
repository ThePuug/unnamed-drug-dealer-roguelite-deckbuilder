// Font Resources
// Provides access to custom fonts like emoji support

use bevy::prelude::*;

#[derive(Resource)]
pub struct EmojiFont(pub Handle<Font>);
