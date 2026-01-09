use bevy::prelude::*;

use crate::storage::Storage;

/// Event fired when an item is deposited into storage
#[derive(Event, Debug, Clone)]
pub struct ItemDeposited {
    pub item_name: String,
}

/// Event fired when an item is withdrawn from storage
#[derive(Event, Debug, Clone)]
pub struct ItemWithdrawn {
    pub item_name: String,
}

/// Plugin that initializes the Storage and registers storage-related events
pub struct StoragePlugin;

impl Plugin for StoragePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Storage>()
            .add_event::<ItemDeposited>()
            .add_event::<ItemWithdrawn>();
    }
}
