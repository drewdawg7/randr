use bevy::prelude::*;

use crate::assets::{DetailPanelSlice, GameSprites, SpriteSheetKey};
use crate::screens::InfoPanelSource;

const SLICE_SIZE: f32 = 48.0;

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
        spawn_detail_panel_background(parent, &game_sprites, panel_width, panel_height);
    });
}

fn spawn_detail_panel_background(
    parent: &mut ChildBuilder,
    game_sprites: &GameSprites,
    width: f32,
    height: f32,
) {
    let Some(sheet) = game_sprites.get(SpriteSheetKey::DetailPanelBg) else {
        return;
    };

    let stretch_width = width - (SLICE_SIZE * 2.0);
    let stretch_height = height - (SLICE_SIZE * 2.0);

    parent
        .spawn(Node {
            position_type: PositionType::Absolute,
            left: Val::Px(0.0),
            top: Val::Px(0.0),
            width: Val::Px(width),
            height: Val::Px(height),
            display: Display::Grid,
            grid_template_columns: vec![
                GridTrack::px(SLICE_SIZE),
                GridTrack::px(stretch_width),
                GridTrack::px(SLICE_SIZE),
            ],
            grid_template_rows: vec![
                GridTrack::px(SLICE_SIZE),
                GridTrack::px(stretch_height),
                GridTrack::px(SLICE_SIZE),
            ],
            ..default()
        })
        .with_children(|grid| {
            for slice in DetailPanelSlice::ALL {
                let (w, h) = match slice {
                    DetailPanelSlice::TopLeft
                    | DetailPanelSlice::TopRight
                    | DetailPanelSlice::BottomLeft
                    | DetailPanelSlice::BottomRight => (SLICE_SIZE, SLICE_SIZE),
                    DetailPanelSlice::TopCenter | DetailPanelSlice::BottomCenter => {
                        (stretch_width, SLICE_SIZE)
                    }
                    DetailPanelSlice::MiddleLeft | DetailPanelSlice::MiddleRight => {
                        (SLICE_SIZE, stretch_height)
                    }
                    DetailPanelSlice::Center => (stretch_width, stretch_height),
                };

                let mut cell = grid.spawn(Node {
                    width: Val::Px(w),
                    height: Val::Px(h),
                    ..default()
                });

                if let Some(img) = sheet.image_node(slice.as_str()) {
                    cell.insert(img);
                }
            }
        });
}
