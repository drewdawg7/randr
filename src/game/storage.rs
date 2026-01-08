use bevy::prelude::*;

use crate::storage::Storage;

/// Bevy Resource that wraps the existing Storage struct
#[derive(Resource, Debug)]
pub struct StorageResource(pub Storage);

impl Default for StorageResource {
    fn default() -> Self {
        Self(Storage::default())
    }
}

impl std::ops::Deref for StorageResource {
    type Target = Storage;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for StorageResource {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

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

/// Plugin that initializes the StorageResource and registers storage-related events
pub struct StoragePlugin;

impl Plugin for StoragePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<StorageResource>()
            .add_event::<ItemDeposited>()
            .add_event::<ItemWithdrawn>();
    }
}
