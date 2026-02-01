use bevy::prelude::*;

/// Plugin for Column layout widget.
pub struct ColumnPlugin;

impl Plugin for ColumnPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(on_add_column);
    }
}

/// A vertical column layout component.
///
/// Spawning this component triggers an observer that replaces it with a properly
/// configured `Node`. This provides a declarative API for creating column layouts.
///
/// # Examples
///
/// ```ignore
/// // Basic column with gap
/// parent.spawn(Column::new().gap(5.0));
///
/// // Column with centered items
/// parent.spawn(Column::new().gap(10.0).align_center());
///
/// // Column with specific width
/// parent.spawn(
///     Column::new()
///         .gap(8.0)
///         .width(Val::Px(200.0))
///         .padding(UiRect::all(Val::Px(10.0)))
/// );
/// ```
#[derive(Component)]
pub struct Column {
    /// Gap between child elements (row_gap)
    pub gap: f32,
    /// How to justify content along the main axis
    pub justify: JustifyContent,
    /// How to align items along the cross axis
    pub align: AlignItems,
    /// Padding inside the column
    pub padding: UiRect,
    /// Width of the column
    pub width: Option<Val>,
    /// Height of the column
    pub height: Option<Val>,
}

impl Default for Column {
    fn default() -> Self {
        Self::new()
    }
}

impl Column {
    /// Creates a new Column with default settings.
    pub fn new() -> Self {
        Self {
            gap: 0.0,
            justify: JustifyContent::FlexStart,
            align: AlignItems::Stretch,
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

    /// Sets padding inside the column.
    pub fn padding(mut self, padding: UiRect) -> Self {
        self.padding = padding;
        self
    }

    /// Sets the width of the column.
    pub fn width(mut self, width: Val) -> Self {
        self.width = Some(width);
        self
    }

    /// Sets the height of the column.
    pub fn height(mut self, height: Val) -> Self {
        self.height = Some(height);
        self
    }
}

fn on_add_column(
    trigger: On<OnAdd, Column>,
    mut commands: Commands,
    query: Query<&Column>,
) {
    let entity = trigger.entity();
    let Ok(column) = query.get(entity) else {
        return;
    };

    // Capture values before removing component
    let gap = column.gap;
    let justify = column.justify;
    let align = column.align;
    let padding = column.padding;
    let width = column.width;
    let height = column.height;

    commands.entity(entity).remove::<Column>().insert(Node {
        flex_direction: FlexDirection::Column,
        row_gap: Val::Px(gap),
        justify_content: justify,
        align_items: align,
        padding,
        width: width.unwrap_or(Val::Auto),
        height: height.unwrap_or(Val::Auto),
        ..default()
    });
}
