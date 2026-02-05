use bevy::prelude::*;

use crate::combat::ActiveCombat;
use crate::dungeon::MoveResult;
use crate::ui::screens::fight_modal::state::FightModalMob;
use crate::ui::screens::modal::{ModalType, OpenModal};

pub fn handle_move_result(
    mut commands: Commands,
    mut events: MessageReader<MoveResult>,
    fight_mob: Option<Res<FightModalMob>>,
) {
    for event in events.read() {
        if let MoveResult::TriggeredCombat { mob_id, entity, pos } = event {
            if fight_mob.is_some() {
                continue;
            }
            commands.insert_resource(FightModalMob {
                mob_id: *mob_id,
                pos: *pos,
                entity: *entity,
            });
            commands.insert_resource(ActiveCombat { mob_entity: *entity });
            commands.trigger(OpenModal(ModalType::FightModal));
        }
    }
}
