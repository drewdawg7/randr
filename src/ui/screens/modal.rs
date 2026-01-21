use bevy::prelude::*;

/// Resource tracking which modal is currently open, if any.
#[derive(Resource, Default)]
pub struct ActiveModal {
    pub modal: Option<ModalType>,
}

/// Types of modals available in the game.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ModalType {
    Profile,
    Inventory,
    Keybinds,
    MonsterCompendium,
    FightModal,
}

/// Component marker for modal overlay background.
#[derive(Component)]
pub struct ModalOverlay;

/// Component marker for modal content container.
#[derive(Component)]
pub struct ModalContent;

/// Plugin that manages modal state and lifecycle.
pub struct ModalPlugin;

impl Plugin for ModalPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ActiveModal>();
    }
}

/// Helper to spawn a modal overlay with semi-transparent background.
pub fn spawn_modal_overlay(commands: &mut Commands) -> Entity {
    commands
        .spawn((
            ModalOverlay,
            Node {
                position_type: PositionType::Absolute,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.7)),
            ZIndex(100),
        ))
        .id()
}

/// Helper to create a modal container node.
pub fn create_modal_container() -> Node {
    Node {
        width: Val::Px(800.0),
        max_width: Val::Percent(90.0),
        height: Val::Auto,
        max_height: Val::Percent(80.0),
        flex_direction: FlexDirection::Column,
        padding: UiRect::all(Val::Px(30.0)),
        border: UiRect::all(Val::Px(3.0)),
        ..default()
    }
}

/// Helper to create a modal title text.
pub fn create_modal_title(title: &str) -> impl Bundle {
    (
        Text::new(title),
        TextFont {
            font_size: 48.0,
            ..default()
        },
        TextColor(Color::srgb(0.95, 0.9, 0.7)),
        Node {
            margin: UiRect::bottom(Val::Px(20.0)),
            ..default()
        },
    )
}

/// Helper to create modal section text.
pub fn create_modal_section(text: &str, color: Color) -> impl Bundle {
    (
        Text::new(text),
        TextFont {
            font_size: 24.0,
            ..default()
        },
        TextColor(color),
        Node {
            margin: UiRect::bottom(Val::Px(10.0)),
            ..default()
        },
    )
}

/// Helper to create modal instruction text.
pub fn create_modal_instruction(text: &str) -> impl Bundle {
    (
        Text::new(text),
        TextFont {
            font_size: 18.0,
            ..default()
        },
        TextColor(Color::srgb(0.6, 0.6, 0.6)),
        Node {
            margin: UiRect::top(Val::Px(20.0)),
            ..default()
        },
    )
}

/// Result of a modal toggle operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ModalAction {
    /// Modal was closed (was previously open).
    Closed,
    /// Modal should be opened (no other modal was active).
    Open,
}

/// Toggle a modal open/closed.
///
/// Returns `Some(ModalAction::Closed)` if the modal was despawned,
/// `Some(ModalAction::Open)` if no modal is active and this one should open,
/// or `None` if another modal is already open.
///
/// The caller is responsible for:
/// - Spawning the modal UI when `ModalAction::Open` is returned
/// - Any custom cleanup (removing resources) when `ModalAction::Closed` is returned
pub fn toggle_modal<T: Component>(
    commands: &mut Commands,
    active_modal: &mut ActiveModal,
    modal_query: &Query<Entity, With<T>>,
    modal_type: ModalType,
) -> Option<ModalAction> {
    if let Ok(entity) = modal_query.get_single() {
        commands.entity(entity).despawn_recursive();
        active_modal.modal = None;
        Some(ModalAction::Closed)
    } else if active_modal.modal.is_none() {
        active_modal.modal = Some(modal_type);
        Some(ModalAction::Open)
    } else {
        None
    }
}

/// Close a modal if it's currently active.
///
/// Returns `true` if the modal was closed, `false` if it wasn't active.
/// The caller is responsible for any custom cleanup (removing resources).
pub fn close_modal<T: Component>(
    commands: &mut Commands,
    active_modal: &mut ActiveModal,
    modal_query: &Query<Entity, With<T>>,
    modal_type: ModalType,
) -> bool {
    if active_modal.modal == Some(modal_type) {
        if let Ok(entity) = modal_query.get_single() {
            commands.entity(entity).despawn_recursive();
            active_modal.modal = None;
            return true;
        }
    }
    false
}
