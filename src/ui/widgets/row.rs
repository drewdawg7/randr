use bevy::prelude::*;

/// Plugin for Row layout widget.
pub struct RowPlugin;

impl Plugin for RowPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(on_add_row);
    }
}

/// A horizontal row layout component.
///
/// Spawning this component triggers an observer that replaces it with a properly
/// configured `Node`. This provides a declarative API for creating row layouts.
///
/// # Examples
///
/// ```ignore
/// // Basic row with gap
/// parent.spawn(Row::new().gap(10.0));
///
/// // Row with centered items
/// parent.spawn(Row::new().gap(8.0).align_center());
///
/// // Row with space between items
/// parent.spawn(
///     Row::new()
///         .gap(10.0)
///         .justify(JustifyContent::SpaceBetween)
///         .padding(UiRect::all(Val::Px(10.0)))
/// );
/// ```
#[derive(Component)]
pub struct Row {
    /// Gap between child elements (column_gap)
    pub gap: f32,
    /// How to justify content along the main axis
    pub justify: JustifyContent,
    /// How to align items along the cross axis
    pub align: AlignItems,
    /// Padding inside the row
    pub padding: UiRect,
    /// Width of the row
    pub width: Option<Val>,
    /// Height of the row
    pub height: Option<Val>,
}

impl Default for Row {
    fn default() -> Self {
        Self::new()
    }
}

impl Row {
    /// Creates a new Row with default settings.
    pub fn new() -> Self {
        Self {
            gap: 0.0,
            justify: JustifyContent::FlexStart,
            align: AlignItems::Center,
            padding: UiRect::default(),
            width: None,
            height: None,
        }
    }

    /// Sets the gap between child elements.
    pub fn gap(mut self, gap: f32) -> Self {
        self.gap = gap;
        self
    }

    /// Sets how to justify content along the main axis.
    pub fn justify(mut self, justify: JustifyContent) -> Self {
        self.justify = justify;
        self
    }

    /// Sets how to align items along the cross axis.
    pub fn align(mut self, align: AlignItems) -> Self {
        self.align = align;
        self
    }

    /// Convenience method to center-align items.
    pub fn align_center(self) -> Self {
        self.align(AlignItems::Center)
    }

    /// Sets padding inside the row.
    pub fn padding(mut self, padding: UiRect) -> Self {
        self.padding = padding;
        self
    }

    /// Sets the width of the row.
    pub fn width(mut self, width: Val) -> Self {
        self.width = Some(width);
        self
    }

    /// Sets the height of the row.
    pub fn height(mut self, height: Val) -> Self {
        self.height = Some(height);
        self
    }
}

fn on_add_row(trigger: On<Add, Row>, mut commands: Commands, query: Query<&Row>) {
    let entity = trigger.entity;
    let Ok(row) = query.get(entity) else {
        return;
    };

    // Capture values before removing component
    let gap = row.gap;
    let justify = row.justify;
    let align = row.align;
    let padding = row.padding;
    let width = row.width;
    let height = row.height;

    commands.entity(entity).remove::<Row>().insert(Node {
        flex_direction: FlexDirection::Row,
        column_gap: Val::Px(gap),
        justify_content: justify,
        align_items: align,
        padding,
        width: width.unwrap_or(Val::Auto),
        height: height.unwrap_or(Val::Auto),
        ..default()
    });
}
