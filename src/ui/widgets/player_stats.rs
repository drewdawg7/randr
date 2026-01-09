use bevy::prelude::*;
use crate::assets::GameSprites;
use crate::entities::Progression;
use crate::entities::progression::HasProgression;
use crate::game::Player;
use crate::stats::HasStats;

/// Plugin for player stats widget systems.
pub struct PlayerStatsPlugin;

impl Plugin for PlayerStatsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, populate_heart_icons);
    }
}

/// Marker component for heart icon placeholder.
#[derive(Component)]
struct HeartIconPlaceholder;

/// Spawn player stats display (HP, Level/XP, Gold).
pub fn spawn_player_stats(parent: &mut ChildBuilder, player: &Player) {
    parent
        .spawn((Node {
            flex_direction: FlexDirection::Column,
            padding: UiRect::all(Val::Px(10.0)),
            row_gap: Val::Px(5.0),
            ..default()
        },))
        .with_children(|stats| {
            // HP row with heart icon + values
            stats
                .spawn(Node {
                    flex_direction: FlexDirection::Row,
                    align_items: AlignItems::Center,
                    column_gap: Val::Px(4.0),
                    ..default()
                })
                .with_children(|hp_row| {
                    // Heart icon placeholder - will be populated by system
                    hp_row.spawn((
                        HeartIconPlaceholder,
                        Node {
                            width: Val::Px(16.0),
                            height: Val::Px(16.0),
                            ..default()
                        },
                    ));

                    // HP values
                    hp_row.spawn((
                        Text::new(format!("{}/{}", player.hp(), player.max_hp())),
                        TextFont { font_size: 16.0, ..default() },
                        TextColor(Color::srgb(0.8, 0.3, 0.3)),
                    ));
                });

            // Level & XP
            stats.spawn((
                Text::new(format!(
                    "Level: {}  XP: {}/{}",
                    player.level(),
                    player.prog.xp,
                    Progression::xp_to_next_level(player.level())
                )),
                TextFont { font_size: 16.0, ..default() },
                TextColor(Color::srgb(0.5, 0.8, 0.5)),
            ));

            // Gold
            stats.spawn((
                Text::new(format!("Gold: {}", player.gold)),
                TextFont { font_size: 16.0, ..default() },
                TextColor(Color::srgb(0.9, 0.8, 0.3)),
            ));
        });
}

/// System to populate heart icon placeholders with the actual sprite.
fn populate_heart_icons(
    mut commands: Commands,
    query: Query<Entity, With<HeartIconPlaceholder>>,
    game_sprites: Res<GameSprites>,
) {
    let Some(ui_all) = &game_sprites.ui_all else {
        return;
    };
    let Some(index) = ui_all.get("Slice_3013") else {
        return;
    };

    for entity in &query {
        commands.entity(entity).remove::<HeartIconPlaceholder>().insert(
            ImageNode::from_atlas_image(
                ui_all.texture.clone(),
                TextureAtlas {
                    layout: ui_all.layout.clone(),
                    index,
                },
            ),
        );
    }
}
