use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::*;

use crate::ui::widgets::Column;
use crate::ui::{AseMobSheets, Modal, SpawnModalExt};

use super::state::{ResultsModalData, ResultsModalRoot, ResultsSprite};

const SPRITE_SIZE: f32 = 128.0;
const MODAL_WIDTH: f32 = 300.0;
const SUBTITLE_FONT_SIZE: f32 = 24.0;
const STAT_FONT_SIZE: f32 = 22.0;
const LOOT_FONT_SIZE: f32 = 20.0;

const GOLD_COLOR: Color = Color::srgb(1.0, 0.84, 0.0);
const XP_COLOR: Color = Color::srgb(0.6, 0.8, 1.0);

pub fn do_spawn_results_modal(
    mut commands: Commands,
    data: Res<ResultsModalData>,
    ase_sheets: Res<AseMobSheets>,
) {
    let title = data.title.clone();
    let subtitle = data.subtitle.clone();
    let sprite = data.sprite.clone();
    let gold_gained = data.gold_gained;
    let xp_gained = data.xp_gained;
    let loot_drops: Vec<(String, i32)> = data
        .loot_drops
        .iter()
        .map(|drop| (drop.item.name.clone(), drop.quantity))
        .collect();

    let mob_ase: Option<(Handle<Aseprite>, &'static str)> = sprite.as_ref().and_then(|s| {
        match s {
            ResultsSprite::Mob(mob_id) => {
                let sheet = ase_sheets.get(*mob_id)?;
                let tag = sheet.death_tag.unwrap_or(sheet.idle_tag);
                Some((sheet.aseprite.clone(), tag))
            }
        }
    });

    commands.spawn_modal(
        Modal::builder()
            .title(&title)
            .size((MODAL_WIDTH, 0.0))
            .root_marker(Box::new(|e| {
                e.insert(ResultsModalRoot);
            }))
            .content(Box::new(move |c| {
                c.spawn(Column::new().gap(8.0).align_center())
                    .with_children(|col| {
                        if let Some((aseprite, tag)) = &mob_ase {
                            col.spawn((
                                AseAnimation {
                                    aseprite: aseprite.clone(),
                                    animation: Animation::tag(tag)
                                        .with_repeat(AnimationRepeat::Count(1)),
                                },
                                ImageNode::default(),
                                Node {
                                    width: Val::Px(SPRITE_SIZE),
                                    height: Val::Px(SPRITE_SIZE),
                                    ..default()
                                },
                            ));
                        }

                        if let Some(subtitle) = &subtitle {
                            col.spawn((
                                Text::new(subtitle),
                                TextFont {
                                    font_size: SUBTITLE_FONT_SIZE,
                                    ..default()
                                },
                                TextColor(Color::WHITE),
                            ));
                        }

                        if let Some(gold) = gold_gained {
                            col.spawn((
                                Text::new(format!("+{gold} Gold")),
                                TextFont {
                                    font_size: STAT_FONT_SIZE,
                                    ..default()
                                },
                                TextColor(GOLD_COLOR),
                            ));
                        }

                        if let Some(xp) = xp_gained {
                            col.spawn((
                                Text::new(format!("+{xp} XP")),
                                TextFont {
                                    font_size: STAT_FONT_SIZE,
                                    ..default()
                                },
                                TextColor(XP_COLOR),
                            ));
                        }

                        for (item_name, quantity) in &loot_drops {
                            let text = if *quantity > 1 {
                                format!("{item_name} x{quantity}")
                            } else {
                                item_name.clone()
                            };
                            col.spawn((
                                Text::new(text),
                                TextFont {
                                    font_size: LOOT_FONT_SIZE,
                                    ..default()
                                },
                                TextColor(Color::WHITE),
                            ));
                        }
                    });
            }))
            .build(),
    );
}
