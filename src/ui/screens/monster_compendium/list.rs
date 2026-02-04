use bevy::prelude::*;

use crate::ui::{FocusPanel, FocusState, MobSpriteSheets, SpriteAnimation};

use super::constants::*;
use super::state::{CompendiumListState, CompendiumMobSprite, CompendiumMonsters, MonsterListItem};

pub fn update_monster_list_display(
    list_state: Res<CompendiumListState>,
    focus_state: Option<Res<FocusState>>,
    mut items: Query<(&MonsterListItem, &mut TextColor)>,
) {
    let Some(focus_state) = focus_state else { return };
    if !list_state.is_changed() && !focus_state.is_changed() {
        return;
    }

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
    mob_sheets: Res<MobSpriteSheets>,
    query: Query<Entity, With<CompendiumMobSprite>>,
    added: Query<Entity, Added<CompendiumMobSprite>>,
) {
    let needs_update = list_state.is_changed() || !added.is_empty();
    if !needs_update {
        return;
    }

    let Some(monsters) = monsters else { return };
    let Some(entry) = monsters.get(list_state.selected) else {
        return;
    };

    for entity in &query {
        if let Some(sheet) = mob_sheets.get(entry.mob_id) {
            commands.entity(entity).insert((
                ImageNode::from_atlas_image(
                    sheet.texture.clone(),
                    TextureAtlas {
                        layout: sheet.layout.clone(),
                        index: sheet.animation.first_frame,
                    },
                ),
                SpriteAnimation::new(&sheet.animation.clone().into()),
            ));
        } else {
            commands
                .entity(entity)
                .remove::<ImageNode>()
                .remove::<SpriteAnimation>();
        }
    }
}
