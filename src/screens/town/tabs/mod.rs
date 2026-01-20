mod alchemist;
mod blacksmith;
mod field;
pub mod store;

use bevy::prelude::*;

pub use alchemist::AlchemistTabPlugin;
pub use blacksmith::BlacksmithTabPlugin;
pub use field::FieldTabPlugin;
pub use store::{InfoPanelSource, StoreTabPlugin};

/// Plugin that bundles all tab plugins together.
pub struct TabsPlugin;

impl Plugin for TabsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            StoreTabPlugin,
            BlacksmithTabPlugin,
            AlchemistTabPlugin,
            FieldTabPlugin,
        ));
    }
}
