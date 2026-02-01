use bevy::prelude::*;
use bon::Builder;

use crate::assets::{GameSprites, SpriteSheetKey, UiAllSlice};
use crate::ui::row_node;

pub struct GoldDisplayPlugin;

impl Plugin for GoldDisplayPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(on_add_gold_display);
    }
}

#[derive(Component, Builder)]
pub struct GoldDisplay {
    #[builder(start_fn)]
    pub amount: i32,
    #[builder(default = 16.0)]
    pub font_size: f32,
    #[builder(default = Color::srgb(0.4, 0.25, 0.15))]
    pub text_color: Color,
}

fn on_add_gold_display(
    trigger: On<Add, GoldDisplay>,
    mut commands: Commands,
    query: Query<&GoldDisplay>,
    game_sprites: Res<GameSprites>,
) {
    let entity = trigger.entity();
    let Ok(gold_display) = query.get(entity) else {
        return;
    };

    let gold_image = game_sprites
        .get(SpriteSheetKey::UiAll)
        .and_then(|s| s.image_node(UiAllSlice::GoldIcon.as_str()));

    let icon_size = gold_display.font_size;

    commands
        .entity(entity)
        .remove::<GoldDisplay>()
        .insert(row_node(4.0))
        .with_children(|row| {
            let mut icon = row.spawn(Node {
                width: Val::Px(icon_size),
                height: Val::Px(icon_size),
                ..default()
            });
            if let Some(img) = gold_image {
                icon.insert(img);
            }

            row.spawn((
                Text::new(format!("{}", gold_display.amount)),
                TextFont {
                    font_size: gold_display.font_size,
                    ..default()
                },
                TextColor(gold_display.text_color),
            ));
        });
}
