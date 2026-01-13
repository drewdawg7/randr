use bevy::prelude::*;

use crate::assets::{GameSprites, SpriteSheetKey, TravelBookSlice, UiAllSlice};
use crate::entities::Progression;
use crate::player::PlayerGold;
use crate::stats::{StatSheet, StatType};
use crate::ui::{row_node, UiText};

/// Plugin for player stats widget.
pub struct PlayerStatsPlugin;

impl Plugin for PlayerStatsPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(on_add_player_stats)
            .add_systems(Update, update_gold_display);
    }
}

/// Marker for player stats widget. Observer populates with sprites.
#[derive(Component)]
pub struct PlayerStats;

/// Marker for the gold text so it can be updated reactively.
#[derive(Component)]
pub struct PlayerGoldText;

fn on_add_player_stats(
    trigger: Trigger<OnAdd, PlayerStats>,
    mut commands: Commands,
    stats: Res<StatSheet>,
    progression: Res<Progression>,
    gold: Res<PlayerGold>,
    game_sprites: Res<GameSprites>,
) {
    let entity = trigger.entity();

    // Get sprite images
    let heart_image = game_sprites
        .get(SpriteSheetKey::UiAll)
        .and_then(|s| s.image_node(UiAllSlice::HeartIcon.as_str()));
    let gold_image = game_sprites
        .get(SpriteSheetKey::UiAll)
        .and_then(|s| s.image_node(UiAllSlice::GoldIcon.as_str()));
    let background_image = game_sprites
        .get(SpriteSheetKey::TravelBook)
        .and_then(|s| s.image_node_sliced(TravelBookSlice::Banner.as_str(), 16.0));

    let hp = stats.value(StatType::Health);
    let max_hp = stats.max_value(StatType::Health);

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
    entity_commands.with_children(|stats_node| {
        // HP row with heart icon + values
        stats_node
            .spawn(row_node(4.0))
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
                hp_row.spawn(UiText::new(format!("{}/{}", hp, max_hp)).size(16.0).red().build());
            });

        // Level & XP
        stats_node.spawn(
            UiText::new(format!(
                "Level: {}  XP: {}/{}",
                progression.level,
                progression.xp,
                Progression::xp_to_next_level(progression.level)
            ))
            .size(16.0)
            .green()
            .build(),
        );

        // Gold row with coin icon + value
        stats_node
            .spawn(row_node(4.0))
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
                gold_row.spawn((
                    PlayerGoldText,
                    UiText::new(format!("{}", gold.0)).size(16.0).gold().build(),
                ));
            });
    });
}

/// System to update gold display when PlayerGold resource changes.
fn update_gold_display(gold: Res<PlayerGold>, mut query: Query<&mut Text, With<PlayerGoldText>>) {
    if gold.is_changed() {
        for mut text in query.iter_mut() {
            **text = format!("{}", gold.0);
        }
    }
}
