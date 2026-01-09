use bevy::prelude::*;

use crate::assets::GameSprites;
use crate::ui::UiText;
use crate::entities::Progression;
use crate::entities::progression::HasProgression;
use crate::game::Player;
use crate::stats::HasStats;

/// Plugin for player stats widget.
pub struct PlayerStatsPlugin;

impl Plugin for PlayerStatsPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(on_add_player_stats);
    }
}

/// Marker for player stats widget. Observer populates with sprites.
#[derive(Component)]
pub struct PlayerStats;

fn on_add_player_stats(
    trigger: Trigger<OnAdd, PlayerStats>,
    mut commands: Commands,
    player: Res<Player>,
    game_sprites: Res<GameSprites>,
) {
    let entity = trigger.entity();

    // Get sprite images
    let heart_image = game_sprites.ui_all.as_ref().and_then(|s| s.image_node("Slice_3013"));
    let gold_image = game_sprites.ui_all.as_ref().and_then(|s| s.image_node("Slice_3019"));
    let background_image = game_sprites.ui_all.as_ref().and_then(|s| s.image_node_sliced("Slice_8", 8.0));

    let mut entity_commands = commands.entity(entity);
    entity_commands.insert(Node {
        flex_direction: FlexDirection::Row,
        padding: UiRect::all(Val::Px(10.0)),
        column_gap: Val::Px(15.0),
        align_items: AlignItems::Center,
        ..default()
    });
    if let Some(bg) = background_image {
        entity_commands.insert(bg);
    }
    entity_commands
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
                // Heart icon
                let mut icon = hp_row.spawn(Node {
                    width: Val::Px(16.0),
                    height: Val::Px(16.0),
                    ..default()
                });
                if let Some(img) = heart_image {
                    icon.insert(img);
                }

                // HP values
                hp_row.spawn(
                    UiText::new(format!("{}/{}", player.hp(), player.max_hp()))
                        .size(16.0)
                        .red()
                        .build(),
                );
            });

        // Level & XP
        stats.spawn(
            UiText::new(format!(
                "Level: {}  XP: {}/{}",
                player.level(),
                player.prog.xp,
                Progression::xp_to_next_level(player.level())
            ))
            .size(16.0)
            .green()
            .build(),
        );

        // Gold row with coin icon + value
        stats
            .spawn(Node {
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::Center,
                column_gap: Val::Px(4.0),
                ..default()
            })
            .with_children(|gold_row| {
                // Gold icon
                let mut icon = gold_row.spawn(Node {
                    width: Val::Px(16.0),
                    height: Val::Px(16.0),
                    ..default()
                });
                if let Some(img) = gold_image {
                    icon.insert(img);
                }

                // Gold value
                gold_row.spawn(
                    UiText::new(format!("{}", player.gold)).size(16.0).gold().build(),
                );
            });
    });
}
