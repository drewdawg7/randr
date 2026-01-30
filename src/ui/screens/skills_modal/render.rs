use bevy::prelude::*;

use crate::skills::{xp_for_level, SkillType, Skills};
use crate::ui::widgets::Column;
use crate::ui::{Modal, SpawnModalExt};

use super::state::SkillsModalRoot;

const MODAL_WIDTH: f32 = 350.0;
const SKILL_NAME_SIZE: f32 = 22.0;
const STAT_SIZE: f32 = 18.0;
const BAR_WIDTH: f32 = 280.0;
const BAR_HEIGHT: f32 = 16.0;

const XP_BAR_BG: Color = Color::srgb(0.2, 0.2, 0.2);
const XP_BAR_FILL: Color = Color::srgb(0.2, 0.7, 0.3);

pub fn do_spawn_skills_modal(mut commands: Commands, skills: Res<Skills>) {
    let skill_data: Vec<_> = SkillType::all()
        .iter()
        .map(|&skill_type| {
            let skill = skills.skill(skill_type);
            let (level, xp) = skill.map(|s| (s.level, s.xp)).unwrap_or((1, 0));
            let current_level_xp = xp_for_level(level);
            let next_level_xp = xp_for_level(level + 1);
            let xp_in_level = xp.saturating_sub(current_level_xp);
            let xp_needed = next_level_xp.saturating_sub(current_level_xp);
            let progress = if xp_needed > 0 {
                xp_in_level as f32 / xp_needed as f32
            } else {
                1.0
            };
            (skill_type, level, xp, xp_needed, progress)
        })
        .collect();

    let total_level = skills.player_level();

    commands.spawn_modal(
        Modal::builder()
            .title("Skills")
            .size((MODAL_WIDTH, 0.0))
            .root_marker(Box::new(|e| {
                e.insert(SkillsModalRoot);
            }))
            .content(Box::new(move |c| {
                c.spawn(Column::new().gap(16.0).align_center())
                    .with_children(|col| {
                        for (skill_type, level, xp, xp_needed, progress) in &skill_data {
                            col.spawn(Column::new().gap(4.0).align_center())
                                .with_children(|skill_col| {
                                    skill_col.spawn((
                                        Text::new(format!(
                                            "{}  Level {}/99",
                                            skill_type.display_name(),
                                            level
                                        )),
                                        TextFont {
                                            font_size: SKILL_NAME_SIZE,
                                            ..default()
                                        },
                                        TextColor(Color::WHITE),
                                    ));

                                    skill_col
                                        .spawn(Node {
                                            width: Val::Px(BAR_WIDTH),
                                            height: Val::Px(BAR_HEIGHT),
                                            ..default()
                                        })
                                        .insert(BackgroundColor(XP_BAR_BG))
                                        .with_children(|bar| {
                                            bar.spawn((
                                                Node {
                                                    width: Val::Percent(progress * 100.0),
                                                    height: Val::Percent(100.0),
                                                    ..default()
                                                },
                                                BackgroundColor(XP_BAR_FILL),
                                            ));
                                        });

                                    skill_col.spawn((
                                        Text::new(format!(
                                            "{} / {} XP",
                                            format_number(*xp),
                                            format_number(*xp_needed)
                                        )),
                                        TextFont {
                                            font_size: STAT_SIZE,
                                            ..default()
                                        },
                                        TextColor(Color::srgb(0.7, 0.7, 0.7)),
                                    ));
                                });
                        }

                        col.spawn(Node {
                            height: Val::Px(8.0),
                            ..default()
                        });

                        col.spawn((
                            Text::new(format!("Total Level: {}", total_level)),
                            TextFont {
                                font_size: SKILL_NAME_SIZE,
                                ..default()
                            },
                            TextColor(Color::srgb(1.0, 0.84, 0.0)),
                        ));
                    });
            }))
            .build(),
    );
}

fn format_number(n: u64) -> String {
    if n >= 1_000_000 {
        format!("{:.1}M", n as f64 / 1_000_000.0)
    } else if n >= 1_000 {
        format!("{:.1}K", n as f64 / 1_000.0)
    } else {
        n.to_string()
    }
}
