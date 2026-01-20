use bevy::prelude::*;

use crate::assets::{GameSprites, ItemDetailIconsSlice};
use crate::stats::StatType;
use crate::ui::row_node;

/// Plugin for icon-value row widget.
pub struct IconValueRowPlugin;

impl Plugin for IconValueRowPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(on_add_icon_value_row);
    }
}

/// Default styling constants for icon-value rows.
pub mod defaults {
    use bevy::prelude::Color;

    pub const ICON_SIZE: f32 = 16.0;
    pub const FONT_SIZE: f32 = 18.0;
    pub const COLUMN_GAP: f32 = 4.0;
    pub const TEXT_COLOR: Color = Color::srgb(0.4, 0.25, 0.15);
}

/// Specifies the icon source for an IconValueRow.
#[derive(Clone)]
pub enum IconSource {
    /// Use an icon slice from the item detail icons sprite sheet.
    Slice(ItemDetailIconsSlice),
    /// Use an icon based on stat type (auto-maps to the appropriate slice).
    Stat(StatType),
}

/// Widget that displays an icon and value in a row.
///
/// Spawns a row with: [Icon] [Value]
///
/// # Examples
///
/// ```ignore
/// // With a specific icon slice
/// parent.spawn(
///     IconValueRow::new(ItemDetailIconsSlice::AttackIcon, "15")
/// );
///
/// // For a stat type (auto-selects appropriate icon)
/// parent.spawn(IconValueRow::for_stat(StatType::Attack, 15));
///
/// // Customized
/// parent.spawn(
///     IconValueRow::new(ItemDetailIconsSlice::HealthIcon, "10/20")
///         .icon_size(20.0)
///         .font_size(18.0)
///         .text_color(Color::srgb(0.8, 0.3, 0.3))
/// );
/// ```
#[derive(Component, Clone)]
pub struct IconValueRow {
    /// The icon to display.
    pub icon: IconSource,
    /// The value text to display.
    pub value: String,
    /// Icon dimensions (width and height).
    pub icon_size: f32,
    /// Font size for the value text.
    pub font_size: f32,
    /// Gap between icon and value.
    pub column_gap: f32,
    /// Color for the value text.
    pub text_color: Color,
}

impl IconValueRow {
    /// Creates a new IconValueRow with the given icon slice and value.
    pub fn new(icon: ItemDetailIconsSlice, value: impl Into<String>) -> Self {
        Self {
            icon: IconSource::Slice(icon),
            value: value.into(),
            icon_size: defaults::ICON_SIZE,
            font_size: defaults::FONT_SIZE,
            column_gap: defaults::COLUMN_GAP,
            text_color: defaults::TEXT_COLOR,
        }
    }

    /// Creates an IconValueRow for a stat type with an integer value.
    ///
    /// Automatically selects the appropriate icon based on the stat type.
    pub fn for_stat(stat_type: StatType, value: i32) -> Self {
        Self {
            icon: IconSource::Stat(stat_type),
            value: format!("{}", value),
            icon_size: defaults::ICON_SIZE,
            font_size: defaults::FONT_SIZE,
            column_gap: defaults::COLUMN_GAP,
            text_color: defaults::TEXT_COLOR,
        }
    }

    /// Sets the icon dimensions (both width and height).
    pub fn icon_size(mut self, size: f32) -> Self {
        self.icon_size = size;
        self
    }

    /// Sets the font size for the value text.
    pub fn font_size(mut self, size: f32) -> Self {
        self.font_size = size;
        self
    }

    /// Sets the gap between icon and value.
    pub fn column_gap(mut self, gap: f32) -> Self {
        self.column_gap = gap;
        self
    }

    /// Sets the color for the value text.
    pub fn text_color(mut self, color: Color) -> Self {
        self.text_color = color;
        self
    }
}

fn on_add_icon_value_row(
    trigger: Trigger<OnAdd, IconValueRow>,
    mut commands: Commands,
    query: Query<&IconValueRow>,
    game_sprites: Res<GameSprites>,
) {
    let entity = trigger.entity();
    let Ok(row_data) = query.get(entity) else {
        return;
    };

    // Capture values before removing component
    let icon = row_data.icon.clone();
    let value = row_data.value.clone();
    let icon_size = row_data.icon_size;
    let font_size = row_data.font_size;
    let column_gap = row_data.column_gap;
    let text_color = row_data.text_color;

    // Get the appropriate icon slice
    let icon_slice = match icon {
        IconSource::Slice(slice) => slice,
        IconSource::Stat(stat_type) => ItemDetailIconsSlice::for_stat(stat_type),
    };

    // Get the icon image
    let icon_image = game_sprites
        .get(icon_slice.sprite_sheet_key())
        .and_then(|sheet| sheet.image_node(icon_slice.as_str()));

    commands
        .entity(entity)
        .remove::<IconValueRow>()
        .insert(row_node(column_gap))
        .with_children(|row| {
            // Icon
            let mut icon_entity = row.spawn(Node {
                width: Val::Px(icon_size),
                height: Val::Px(icon_size),
                ..default()
            });
            if let Some(img) = icon_image {
                icon_entity.insert(img);
            }

            // Value
            row.spawn((
                Text::new(&value),
                TextFont {
                    font_size,
                    ..default()
                },
                TextColor(text_color),
            ));
        });
}
