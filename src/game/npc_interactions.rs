use bevy::prelude::*;

use crate::dungeon::MerchantInteraction;
use crate::item::ItemRegistry;
use crate::ui::screens::merchant_modal::MerchantStock;
use crate::ui::screens::modal::{ModalType, OpenModal};

pub struct NpcInteractionsPlugin;

impl Plugin for NpcInteractionsPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(on_merchant_interaction);
    }
}

fn on_merchant_interaction(_trigger: On<MerchantInteraction>, mut commands: Commands, registry: Res<ItemRegistry>) {
    commands.insert_resource(MerchantStock::generate(&registry));
    commands.trigger(OpenModal(ModalType::MerchantModal));
}
