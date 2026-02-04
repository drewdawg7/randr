use bevy::prelude::*;

use crate::dungeon::MerchantInteraction;
use crate::ui::screens::merchant_modal::MerchantStock;
use crate::ui::screens::modal::{ModalType, OpenModal};

pub struct NpcInteractionsPlugin;

impl Plugin for NpcInteractionsPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(on_merchant_interaction);
    }
}

fn on_merchant_interaction(_trigger: On<MerchantInteraction>, mut commands: Commands) {
    commands.insert_resource(MerchantStock::generate());
    commands.trigger(OpenModal(ModalType::MerchantModal));
}
