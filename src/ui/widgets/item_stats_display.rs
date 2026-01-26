use bevy::prelude::*;

use crate::assets::{GameFonts, GameSprites, ItemDetailIconsSlice};
use crate::stats::StatType;

/// Plugin for item stats display widget.
pub struct ItemStatsDisplayPlugin;

impl Plugin for ItemStatsDisplayPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(on_add_item_stats_display);
    }
}

/// Display mode for item stats.
#[derive(Clone, Copy, Default)]
pub enum StatsDisplayMode {
    /// Text-only format: "HP: +5"
    TextOnly,
    /// Icon + value format: [icon] 5
    #[default]
    IconValue,
}

/// Widget that displays item stats.
///
/// Spawns a column with stat rows based on the display mode:
/// - `TextOnly`: "HP: +5", "ATK: +3"
/// - `IconValue`: [icon] 5, [icon] 3
#[derive(Component)]
pub struct ItemStatsDisplay {
    /// Stats to display as (StatType, value) pairs.
    pub stats: Vec<(StatType, i32)>,
    /// Comparison stats from equipped item (for showing deltas).
    pub comparison: Option<Vec<(StatType, i32)>>,
    /// Font size for stat text.
    pub font_size: f32,
    /// Text color for stat values.
    pub text_color: Color,
    /// Display mode (text-only or icon+value).
    pub mode: StatsDisplayMode,
}

impl ItemStatsDisplay {
    /// Creates a new ItemStatsDisplay with the given stats.
    ///
    /// Only includes stats with non-zero values.
    pub fn from_stats_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = (StatType, i32)>,
    {
        Self {
            stats: iter.into_iter().filter(|(_, v)| *v > 0).collect(),
            comparison: None,
            font_size: 18.0,
            text_color: Color::srgb(0.4, 0.25, 0.15),
            mode: StatsDisplayMode::default(),
        }
    }

    /// Set comparison stats from equipped item for showing deltas.
    pub fn with_comparison(mut self, comparison: Vec<(StatType, i32)>) -> Self {
        self.comparison = Some(comparison);
        self
    }

    pub fn with_font_size(mut self, size: f32) -> Self {
        self.font_size = size;
        self
    }

    pub fn with_color(mut self, color: Color) -> Self {
        self.text_color = color;
        self
    }

    pub fn with_mode(mut self, mode: StatsDisplayMode) -> Self {
        self.mode = mode;
        self
    }

    pub fn text_only(self) -> Self {
        self.with_mode(StatsDisplayMode::TextOnly)
    }

    pub fn icon_value(self) -> Self {
        self.with_mode(StatsDisplayMode::IconValue)
    }
}

fn on_add_item_stats_display(
    trigger: Trigger<OnAdd, ItemStatsDisplay>,
    mut commands: Commands,
    query: Query<&ItemStatsDisplay>,
    game_sprites: Res<GameSprites>,
    game_fonts: Res<GameFonts>,
) {
    let entity = trigger.entity();
    let Ok(display) = query.get(entity) else {
        return;
    };

    // Capture values before removing component
    let stats = display.stats.clone();
    let font_size = display.font_size;
    let text_color = display.text_color;
    let mode = display.mode;

    commands
        .entity(entity)
        .remove::<ItemStatsDisplay>()
        .insert(Node {
            flex_direction: FlexDirection::Column,
            ..default()
        })
        .with_children(|parent| {
            for (stat_type, value) in stats {
                match mode {
                    StatsDisplayMode::TextOnly => {
                        // Text-only format: "HP: +5"
                        parent.spawn((
                            Text::new(format!("{}: +{}", stat_type.display_name(), value)),
                            game_fonts.pixel_font(font_size),
                            TextColor(text_color),
                        ));
                    }
                    StatsDisplayMode::IconValue => {
                        // Icon + value format
                        let icon_slice = ItemDetailIconsSlice::for_stat(stat_type);
                        parent
                            .spawn(Node {
                                flex_direction: FlexDirection::Row,
                                align_items: AlignItems::Center,
                                column_gap: Val::Px(4.0),
                                ..default()
                            })
                            .with_children(|row| {
                                if let Some(sheet) =
                                    game_sprites.get(icon_slice.sprite_sheet_key())
                                {
                                    if let Some(img) = sheet.image_node(icon_slice.as_str()) {
                                        row.spawn((img, Node::default()));
                                    }
                                }
                                row.spawn((
                                    Text::new(format!("{}", value)),
                                    game_fonts.pixel_font(font_size),
                                    TextColor(text_color),
                                ));
                            });
                    }
                }
            }
        });
}
