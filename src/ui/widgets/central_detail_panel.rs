use bevy::prelude::*;

use super::nine_slice::spawn_nine_slice_panel;
use crate::assets::{DetailPanelSlice, GameSprites};
use crate::ui::screens::InfoPanelSource;

pub struct CentralDetailPanelPlugin;

impl Plugin for CentralDetailPanelPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(on_add_central_detail_panel);
    }
}

#[derive(Component)]
pub struct CentralDetailPanel {
    pub source: InfoPanelSource,
}

fn on_add_central_detail_panel(
    trigger: Trigger<OnAdd, CentralDetailPanel>,
    mut commands: Commands,
    game_sprites: Res<GameSprites>,
    panels: Query<&CentralDetailPanel>,
) {
    let entity = trigger.entity();
    let _panel = panels.get(entity).ok();

    let panel_width = 240.0;
    let panel_height = 288.0;

    let mut panel_entity = commands.entity(entity);
    panel_entity.insert(Node {
        width: Val::Px(panel_width),
        height: Val::Px(panel_height),
        position_type: PositionType::Relative,
        flex_direction: FlexDirection::Column,
        justify_content: JustifyContent::FlexStart,
        align_items: AlignItems::Center,
        padding: UiRect::all(Val::Px(12.0)),
        row_gap: Val::Px(4.0),
        overflow: Overflow::clip(),
        ..default()
    });

    panel_entity.with_children(|parent| {
        spawn_nine_slice_panel::<DetailPanelSlice>(parent, &game_sprites, panel_width, panel_height);
    });
}
