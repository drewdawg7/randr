use bevy::prelude::*;

use super::StoreTabPlugin;

/// Plugin that bundles all tab plugins together.
pub struct TabsPlugin;

impl Plugin for TabsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(StoreTabPlugin);
    }
}
