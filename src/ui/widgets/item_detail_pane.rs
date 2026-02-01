use bevy::prelude::*;

use super::nine_slice::spawn_nine_slice_panel;
use crate::assets::{DetailPanelSlice, GameSprites};
use crate::ui::InfoPanelSource;

pub struct ItemDetailPanePlugin;

impl Plugin for ItemDetailPanePlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(on_add_item_detail_pane);
    }
}

#[derive(Component)]
pub struct ItemDetailPane {
    pub source: InfoPanelSource,
}

/// Marker component for the content container inside the detail pane.
#[derive(Component)]
pub struct ItemDetailPaneContent;

fn on_add_item_detail_pane(
    trigger: On<Add, ItemDetailPane>,
    mut commands: Commands,
    game_sprites: Res<GameSprites>,
    panels: Query<&ItemDetailPane>,
) {
    let entity = trigger.entity;
    let _panel = panels.get(entity).ok();

    let panel_width = 280.0;
    let panel_height = 288.0;

    let mut panel_entity = commands.entity(entity);
    panel_entity.insert(Node {
        width: Val::Px(panel_width),
        height: Val::Px(panel_height),
        position_type: PositionType::Relative,
        ..default()
    });

    panel_entity.with_children(|parent| {
        spawn_nine_slice_panel::<DetailPanelSlice>(parent, &game_sprites, panel_width, panel_height);
        parent.spawn((
            ItemDetailPaneContent,
            Node {
                position_type: PositionType::Absolute,
                left: Val::Px(48.0),
                top: Val::Px(48.0),
                width: Val::Px(184.0),   // 280 - 2*48
                height: Val::Px(192.0),  // 288 - 2*48
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::FlexStart,
                row_gap: Val::Px(4.0),
                overflow: Overflow::clip(),
                ..default()
            },
        ));
    });
}
