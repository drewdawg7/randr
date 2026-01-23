use bevy::prelude::*;

use crate::combat::{ActiveCombatResource, CombatPhaseState};
use crate::ui::{nav_selection_text, MenuIndex};

use super::components::{
    ActionMenuItem, CombatResultText, FightScreenRoot, PostCombatMenuItem, PostCombatOverlay,
    RewardsText,
};
use super::state::FightScreenState;
use super::styles::{action_label, action_text_color};

/// Action menu labels in display order.
const ACTION_LABELS: [&str; 2] = ["Attack", "Run"];

pub fn spawn_post_combat_overlay(
    mut commands: Commands,
    combat_res: Res<ActiveCombatResource>,
    overlay_query: Query<Entity, With<PostCombatOverlay>>,
    fight_root: Query<Entity, With<FightScreenRoot>>,
    phase_state: Res<State<CombatPhaseState>>,
) {
    if !overlay_query.is_empty() {
        return;
    }

    let Some(combat) = combat_res.get() else {
        return;
    };
    let Ok(root_entity) = fight_root.get_single() else {
        return;
    };

    let is_victory = *phase_state.get() == CombatPhaseState::Victory;

    commands.entity(root_entity).with_children(|parent| {
        parent
            .spawn((
                PostCombatOverlay,
                Node {
                    position_type: PositionType::Absolute,
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    row_gap: Val::Px(20.0),
                    ..default()
                },
                BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.8)),
            ))
            .with_children(|overlay| {
                let (message, color) = if is_victory {
                    ("VICTORY!", Color::srgb(0.9, 0.9, 0.3))
                } else {
                    ("DEFEAT...", Color::srgb(0.8, 0.3, 0.3))
                };

                overlay.spawn((
                    CombatResultText,
                    Text::new(message),
                    TextFont { font_size: 64.0, ..default() },
                    TextColor(color),
                ));

                if is_victory {
                    overlay.spawn((
                        RewardsText,
                        Text::new(format!("Gold: {} | XP: {}", combat.gold_gained, combat.xp_gained)),
                        TextFont { font_size: 24.0, ..default() },
                        TextColor(Color::srgb(0.9, 0.9, 0.9)),
                    ));
                }

                overlay.spawn((
                    Text::new("What would you like to do?"),
                    TextFont { font_size: 20.0, ..default() },
                    TextColor(Color::srgb(0.8, 0.8, 0.8)),
                    Node { margin: UiRect::top(Val::Px(20.0)), ..default() },
                ));

                spawn_post_combat_item(overlay, 0, "Fight Again");
                spawn_post_combat_item(overlay, 1, "Continue");
            });
    });
}

fn spawn_post_combat_item(parent: &mut ChildBuilder, index: usize, label: &str) {
    let selected = index == 0;
    parent.spawn((
        PostCombatMenuItem,
        MenuIndex(index),
        Text::new(label),
        TextFont { font_size: 28.0, ..default() },
        TextColor(nav_selection_text(selected)),
    ));
}

pub fn despawn_post_combat_overlay(
    mut commands: Commands,
    overlay_query: Query<Entity, With<PostCombatOverlay>>,
) {
    for entity in &overlay_query {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn reset_fight_state(
    mut fight_state: ResMut<FightScreenState>,
    mut action_items: Query<(&MenuIndex, &mut TextColor, &mut Text), With<ActionMenuItem>>,
) {
    fight_state.reset();
    for (menu_index, mut color, mut text) in action_items.iter_mut() {
        let selected = menu_index.0 == fight_state.action_selection;
        *color = TextColor(action_text_color(selected));
        **text = action_label(ACTION_LABELS[menu_index.0], selected);
    }
}

pub fn update_action_visuals(
    state: &FightScreenState,
    items: &mut Query<(&MenuIndex, &mut TextColor, &mut Text), With<ActionMenuItem>>,
) {
    for (menu_index, mut color, mut text) in items.iter_mut() {
        let selected = menu_index.0 == state.action_selection;
        *color = TextColor(action_text_color(selected));
        **text = action_label(ACTION_LABELS[menu_index.0], selected);
    }
}
