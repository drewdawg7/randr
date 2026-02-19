use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::*;

use crate::ui::{AseMobSheets, FocusPanel, FocusState};

use super::constants::*;
use super::state::{CompendiumListState, CompendiumMobSprite, CompendiumMonsters, MonsterListItem};

pub fn update_monster_list_display(
    list_state: Res<CompendiumListState>,
    focus_state: Option<Res<FocusState>>,
    mut items: Query<(&MonsterListItem, &mut TextColor)>,
) {
    let Some(focus_state) = focus_state else { return };
    let monsters_focused = focus_state.is_focused(FocusPanel::CompendiumMonsterList);

    for (item, mut color) in items.iter_mut() {
        if item.0 == list_state.selected && monsters_focused {
            *color = TextColor(SELECTED_COLOR);
        } else {
            *color = TextColor(NORMAL_COLOR);
        }
    }
}

pub fn update_compendium_mob_sprite(
    mut commands: Commands,
    list_state: Res<CompendiumListState>,
    monsters: Option<Res<CompendiumMonsters>>,
    ase_sheets: Res<AseMobSheets>,
    query: Query<Entity, With<CompendiumMobSprite>>,
) {
    let Some(monsters) = monsters else { return };
    let Some(entry) = monsters.get(list_state.selected) else {
        return;
    };

    for entity in &query {
        if let Some(sheet) = ase_sheets.get(entry.mob_id) {
            commands.entity(entity).insert((
                AseAnimation {
                    aseprite: sheet.aseprite.clone(),
                    animation: Animation::tag(sheet.idle_tag)
                        .with_repeat(AnimationRepeat::Loop),
                },
                ImageNode::default(),
            ));
        } else {
            commands
                .entity(entity)
                .remove::<AseAnimation>()
                .remove::<ImageNode>();
        }
    }
}
