use bevy::prelude::*;

use super::{
    AlchemistTabPlugin, BlacksmithTabPlugin, DungeonTabPlugin, FieldTabPlugin, StoreTabPlugin,
};

/// Plugin that bundles all tab plugins together.
pub struct TabsPlugin;

impl Plugin for TabsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            StoreTabPlugin,
            BlacksmithTabPlugin,
            AlchemistTabPlugin,
            FieldTabPlugin,
            DungeonTabPlugin,
        ));
    }
}
