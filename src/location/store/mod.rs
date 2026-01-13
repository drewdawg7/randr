pub mod definition;
pub mod enums;
pub mod events;
pub mod store_item;
pub mod traits;

#[cfg(test)]
mod tests;

use bevy::prelude::*;

use crate::item::ItemId;

pub use definition::{sell_player_item, Store};
pub use enums::StoreError;
pub use events::{PurchaseEvent, SellEvent, TransactionResult};
pub use store_item::StoreItem;

/// Plugin for store functionality.
pub struct StorePlugin;

impl Plugin for StorePlugin {
    fn build(&self, app: &mut App) {
        let store = Store::new("Village Store", vec![
            (ItemId::BasicHPPotion, 5),
            (ItemId::Sword, 3),
            (ItemId::BasicShield, 3),
            (ItemId::CopperHelmet, 2),
        ]);

        app.insert_resource(store)
            .add_event::<PurchaseEvent>()
            .add_event::<SellEvent>()
            .add_event::<TransactionResult>()
            .add_systems(
                Update,
                (
                    events::handle_purchase.run_if(on_event::<PurchaseEvent>),
                    events::handle_sell.run_if(on_event::<SellEvent>),
                ),
            );
    }
}
