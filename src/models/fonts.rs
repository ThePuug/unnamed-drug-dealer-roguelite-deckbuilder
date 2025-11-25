// Font Resources
// Provides access to custom fonts like emoji support

use bevy::prelude::*;

#[derive(Resource)]
pub struct EmojiFont(pub Handle<Font>);

/// NotoSans font - has good Unicode coverage including filled star (★ U+2605)
#[derive(Resource)]
pub struct UiFont(pub Handle<Font>);
