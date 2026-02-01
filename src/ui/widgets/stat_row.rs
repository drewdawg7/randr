use bevy::prelude::*;
use bon::Builder;

pub struct StatRowPlugin;

impl Plugin for StatRowPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(on_add_stat_row);
    }
}

pub mod defaults {
    use bevy::prelude::Color;

    pub const LABEL_WIDTH: f32 = 120.0;
    pub const FONT_SIZE: f32 = 20.0;
    pub const COLUMN_GAP: f32 = 10.0;
    pub const LABEL_COLOR: Color = Color::srgb(0.75, 0.75, 0.75);
    pub const VALUE_COLOR: Color = Color::WHITE;
}

#[derive(Component, Builder)]
#[builder(on(String, into))]
pub struct StatRow {
    #[builder(start_fn)]
    pub label: String,
    #[builder(start_fn)]
    pub value: String,
    pub bonus: Option<(String, Color)>,
    #[builder(default = defaults::LABEL_WIDTH)]
    pub label_width: f32,
    #[builder(default = defaults::FONT_SIZE)]
    pub font_size: f32,
    #[builder(default = defaults::COLUMN_GAP)]
    pub column_gap: f32,
    #[builder(default = defaults::LABEL_COLOR)]
    pub label_color: Color,
    #[builder(default = defaults::VALUE_COLOR)]
    pub value_color: Color,
    pub bottom_margin: Option<f32>,
}

fn on_add_stat_row(
    trigger: On<Add, StatRow>,
    mut commands: Commands,
    query: Query<&StatRow>,
) {
    let entity = trigger.entity;
    let Ok(stat_row) = query.get(entity) else {
        return;
    };

    let label = stat_row.label.clone();
    let value = stat_row.value.clone();
    let bonus = stat_row.bonus.clone();
    let label_width = stat_row.label_width;
    let font_size = stat_row.font_size;
    let column_gap = stat_row.column_gap;
    let label_color = stat_row.label_color;
    let value_color = stat_row.value_color;
    let bottom_margin = stat_row.bottom_margin;

    let margin = bottom_margin.map_or(UiRect::default(), |m| UiRect::bottom(Val::Px(m)));

    commands
        .entity(entity)
        .remove::<StatRow>()
        .insert(Node {
            flex_direction: FlexDirection::Row,
            column_gap: Val::Px(column_gap),
            margin,
            ..default()
        })
        .with_children(|row| {
            row.spawn((
                Text::new(&label),
                TextFont {
                    font_size,
                    ..default()
                },
                TextColor(label_color),
                Node {
                    width: Val::Px(label_width),
                    ..default()
                },
            ));

            row.spawn((
                Text::new(&value),
                TextFont {
                    font_size,
                    ..default()
                },
                TextColor(value_color),
            ));

            if let Some((bonus_text, bonus_color)) = bonus {
                row.spawn((
                    Text::new(format!(" {}", bonus_text)),
                    TextFont {
                        font_size,
                        ..default()
                    },
                    TextColor(bonus_color),
                ));
            }
        });
}
