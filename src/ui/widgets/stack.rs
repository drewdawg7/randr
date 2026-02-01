use bevy::prelude::*;

/// Plugin for Stack layout widget.
pub struct StackPlugin;

impl Plugin for StackPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(on_add_stack);
    }
}

/// A z-layered stack layout component.
///
/// Creates a container where children can be layered on top of each other
/// using absolute positioning. Children should use `PositionType::Absolute`
/// to stack properly.
///
/// # Examples
///
/// ```ignore
/// // Basic stack container
/// parent.spawn(Stack::new())
///     .with_children(|stack| {
///         // Background layer
///         stack.spawn((
///             ImageNode::new(background_image),
///             Node {
///                 position_type: PositionType::Absolute,
///                 width: Val::Percent(100.0),
///                 height: Val::Percent(100.0),
///                 ..default()
///             },
///         ));
///         // Foreground layer (renders on top)
///         stack.spawn((
///             ImageNode::new(foreground_image),
///             Node {
///                 position_type: PositionType::Absolute,
///                 width: Val::Percent(100.0),
///                 height: Val::Percent(100.0),
///                 ..default()
///             },
///             ZIndex(1),
///         ));
///     });
///
/// // Stack with specific size
/// parent.spawn(Stack::new().width(Val::Px(200.0)).height(Val::Px(150.0)));
/// ```
#[derive(Component)]
pub struct Stack {
    /// Width of the stack container
    pub width: Option<Val>,
    /// Height of the stack container
    pub height: Option<Val>,
    /// How to justify content (affects non-absolute children)
    pub justify: JustifyContent,
    /// How to align items (affects non-absolute children)
    pub align: AlignItems,
}

impl Default for Stack {
    fn default() -> Self {
        Self::new()
    }
}

impl Stack {
    /// Creates a new Stack with default settings.
    pub fn new() -> Self {
        Self {
            width: None,
            height: None,
            justify: JustifyContent::Center,
            align: AlignItems::Center,
        }
    }

    /// Sets the width of the stack.
    pub fn width(mut self, width: Val) -> Self {
        self.width = Some(width);
        self
    }

    /// Sets the height of the stack.
    pub fn height(mut self, height: Val) -> Self {
        self.height = Some(height);
        self
    }

    /// Sets how to justify content.
    pub fn justify(mut self, justify: JustifyContent) -> Self {
        self.justify = justify;
        self
    }

    /// Sets how to align items.
    pub fn align(mut self, align: AlignItems) -> Self {
        self.align = align;
        self
    }
}

fn on_add_stack(trigger: On<Add, Stack>, mut commands: Commands, query: Query<&Stack>) {
    let entity = trigger.entity();
    let Ok(stack) = query.get(entity) else {
        return;
    };

    // Capture values before removing component
    let width = stack.width;
    let height = stack.height;
    let justify = stack.justify;
    let align = stack.align;

    commands.entity(entity).remove::<Stack>().insert(Node {
        // Relative positioning allows absolute children to position relative to this container
        position_type: PositionType::Relative,
        width: width.unwrap_or(Val::Auto),
        height: height.unwrap_or(Val::Auto),
        justify_content: justify,
        align_items: align,
        ..default()
    });
}
