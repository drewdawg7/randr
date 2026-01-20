use bevy::prelude::*;

/// Plugin for stat row widget.
pub struct StatRowPlugin;

impl Plugin for StatRowPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(on_add_stat_row);
    }
}

/// Default styling constants for stat rows.
pub mod defaults {
    use bevy::prelude::Color;

    pub const LABEL_WIDTH: f32 = 120.0;
    pub const FONT_SIZE: f32 = 20.0;
    pub const COLUMN_GAP: f32 = 10.0;
    pub const LABEL_COLOR: Color = Color::srgb(0.75, 0.75, 0.75);
    pub const VALUE_COLOR: Color = Color::WHITE;
}

/// Widget that displays a label-value row with optional bonus text.
///
/// Spawns a row with: [Label] [Value] [Optional Bonus]
///
/// # Examples
///
/// ```ignore
/// // Basic usage
/// parent.spawn(StatRow::new("Attack", "12"));
///
/// // With bonus
/// parent.spawn(
///     StatRow::new("Attack", "12")
///         .with_bonus("+3", Color::srgb(0.3, 0.8, 0.3))
/// );
///
/// // Fully customized
/// parent.spawn(
///     StatRow::new("HP", "10/20")
///         .label_width(140.0)
///         .font_size(22.0)
///         .value_color(Color::srgb(0.8, 0.3, 0.3))
/// );
/// ```
#[derive(Component)]
pub struct StatRow {
    /// Label text (e.g., "Attack")
    pub label: String,
    /// Value text (e.g., "12")
    pub value: String,
    /// Optional bonus text and color (e.g., "+3" in green)
    pub bonus: Option<(String, Color)>,
    /// Width of the label column in pixels
    pub label_width: f32,
    /// Font size for all text
    pub font_size: f32,
    /// Gap between columns
    pub column_gap: f32,
    /// Color for the label text
    pub label_color: Color,
    /// Color for the value text
    pub value_color: Color,
    /// Optional bottom margin
    pub bottom_margin: Option<f32>,
}

impl StatRow {
    /// Creates a new StatRow with the given label and value.
    pub fn new(label: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            value: value.into(),
            bonus: None,
            label_width: defaults::LABEL_WIDTH,
            font_size: defaults::FONT_SIZE,
            column_gap: defaults::COLUMN_GAP,
            label_color: defaults::LABEL_COLOR,
            value_color: defaults::VALUE_COLOR,
            bottom_margin: None,
        }
    }

    /// Adds bonus text with a color (e.g., "+3" in green).
    pub fn with_bonus(mut self, bonus: impl Into<String>, color: Color) -> Self {
        self.bonus = Some((bonus.into(), color));
        self
    }

    /// Sets the label column width in pixels.
    pub fn label_width(mut self, width: f32) -> Self {
        self.label_width = width;
        self
    }

    /// Sets the font size for all text.
    pub fn font_size(mut self, size: f32) -> Self {
        self.font_size = size;
        self
    }

    /// Sets the gap between columns.
    pub fn column_gap(mut self, gap: f32) -> Self {
        self.column_gap = gap;
        self
    }

    /// Sets the label text color.
    pub fn label_color(mut self, color: Color) -> Self {
        self.label_color = color;
        self
    }

    /// Sets the value text color.
    pub fn value_color(mut self, color: Color) -> Self {
        self.value_color = color;
        self
    }

    /// Adds a bottom margin to the row.
    pub fn bottom_margin(mut self, margin: f32) -> Self {
        self.bottom_margin = Some(margin);
        self
    }
}

fn on_add_stat_row(
    trigger: Trigger<OnAdd, StatRow>,
    mut commands: Commands,
    query: Query<&StatRow>,
) {
    let entity = trigger.entity();
    let Ok(stat_row) = query.get(entity) else {
        return;
    };

    // Capture values before removing component
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
            // Label
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

            // Value
            row.spawn((
                Text::new(&value),
                TextFont {
                    font_size,
                    ..default()
                },
                TextColor(value_color),
            ));

            // Bonus (if present)
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
