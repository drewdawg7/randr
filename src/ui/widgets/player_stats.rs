use bevy::prelude::*;
use crate::entities::Progression;
use crate::entities::progression::HasProgression;
use crate::game::Player;
use crate::stats::HasStats;

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
            // HP
            stats.spawn((
                Text::new(format!("HP: {}/{}", player.hp(), player.max_hp())),
                TextFont { font_size: 16.0, ..default() },
                TextColor(Color::srgb(0.8, 0.3, 0.3)),
            ));

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
