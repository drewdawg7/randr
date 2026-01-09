use bevy::prelude::*;

use crate::assets::{GameFonts, GameSprites};

/// Plugin for gold display widget.
pub struct GoldDisplayPlugin;

impl Plugin for GoldDisplayPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(on_add_gold_display);
    }
}

/// Widget that displays a gold amount with the coin icon.
/// Spawns a row with: [coin icon] [amount]
#[derive(Component)]
pub struct GoldDisplay {
    pub amount: i32,
    pub font_size: f32,
    pub text_color: Color,
}

impl GoldDisplay {
    pub fn new(amount: i32) -> Self {
        Self {
            amount,
            font_size: 16.0,
            text_color: Color::srgb(0.4, 0.25, 0.15),
        }
    }

    pub fn with_font_size(mut self, size: f32) -> Self {
        self.font_size = size;
        self
    }

    pub fn with_color(mut self, color: Color) -> Self {
        self.text_color = color;
        self
    }
}

fn on_add_gold_display(
    trigger: Trigger<OnAdd, GoldDisplay>,
    mut commands: Commands,
    query: Query<&GoldDisplay>,
    game_sprites: Res<GameSprites>,
    game_fonts: Res<GameFonts>,
) {
    let entity = trigger.entity();
    let Ok(gold_display) = query.get(entity) else {
        return;
    };

    // Get gold icon
    let gold_image = game_sprites.ui_all.as_ref().and_then(|s| s.image_node("Slice_3019"));

    // Icon size scales with font size
    let icon_size = gold_display.font_size;

    commands
        .entity(entity)
        .remove::<GoldDisplay>()
        .insert(Node {
            flex_direction: FlexDirection::Row,
            align_items: AlignItems::Center,
            column_gap: Val::Px(4.0),
            ..default()
        })
        .with_children(|row| {
            // Gold icon
            let mut icon = row.spawn(Node {
                width: Val::Px(icon_size),
                height: Val::Px(icon_size),
                ..default()
            });
            if let Some(img) = gold_image {
                icon.insert(img);
            }

            // Amount text
            row.spawn((
                Text::new(format!("{}", gold_display.amount)),
                game_fonts.pixel_font(gold_display.font_size),
                TextColor(gold_display.text_color),
            ));
        });
}
