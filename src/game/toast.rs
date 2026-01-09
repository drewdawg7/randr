use bevy::prelude::*;
use std::time::Duration;

/// Maximum number of toasts to display at once
const MAX_TOASTS: usize = 5;
/// How long toasts stay visible before auto-dismissing
const TOAST_DURATION: Duration = Duration::from_secs(3);

/// Types of toast notifications, each with different visual styling
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ToastType {
    Error,
    Success,
    Info,
    Warning,
}

impl ToastType {
    /// Returns the icon to display for this toast type
    pub fn icon(&self) -> &'static str {
        match self {
            ToastType::Error => "[!]",
            ToastType::Success => "[+]",
            ToastType::Info => "[i]",
            ToastType::Warning => "[?]",
        }
    }

    /// Returns the label text for this toast type
    pub fn label(&self) -> &'static str {
        match self {
            ToastType::Error => "ERROR",
            ToastType::Success => "SUCCESS",
            ToastType::Info => "INFO",
            ToastType::Warning => "WARNING",
        }
    }

    /// Returns the color for this toast type
    pub fn color(&self) -> Color {
        match self {
            ToastType::Error => Color::srgb(0.9, 0.2, 0.2),     // Red
            ToastType::Success => Color::srgb(0.2, 0.8, 0.2),   // Green
            ToastType::Info => Color::srgb(0.2, 0.5, 0.9),      // Blue
            ToastType::Warning => Color::srgb(0.9, 0.7, 0.2),   // Yellow
        }
    }

    /// Returns the background color for this toast type
    pub fn bg_color(&self) -> Color {
        match self {
            ToastType::Error => Color::srgb(0.3, 0.1, 0.1),
            ToastType::Success => Color::srgb(0.1, 0.3, 0.1),
            ToastType::Info => Color::srgb(0.1, 0.2, 0.3),
            ToastType::Warning => Color::srgb(0.3, 0.25, 0.1),
        }
    }
}

/// A single toast notification
#[derive(Debug, Clone)]
pub struct Toast {
    pub toast_type: ToastType,
    pub message: String,
    pub created_at: Duration,
}

impl Toast {
    /// Creates a new toast notification
    pub fn new(toast_type: ToastType, message: impl Into<String>, current_time: Duration) -> Self {
        Self {
            toast_type,
            message: message.into(),
            created_at: current_time,
        }
    }

    /// Checks if the toast has expired based on current time
    pub fn is_expired(&self, current_time: Duration) -> bool {
        current_time.saturating_sub(self.created_at) >= TOAST_DURATION
    }
}

/// Resource that manages the queue of active toast notifications
#[derive(Resource, Default)]
pub struct ToastQueue {
    toasts: Vec<Toast>,
}

impl ToastQueue {
    /// Adds a new toast to the queue
    pub fn push(&mut self, toast: Toast) {
        // Add to front (most recent on top)
        self.toasts.insert(0, toast);
        // Limit total toasts
        if self.toasts.len() > MAX_TOASTS {
            self.toasts.pop();
        }
    }

    /// Removes expired toasts based on current time
    pub fn cleanup(&mut self, current_time: Duration) {
        self.toasts.retain(|t| !t.is_expired(current_time));
    }

    /// Returns all active toasts
    pub fn toasts(&self) -> &[Toast] {
        &self.toasts
    }

    /// Clears all toasts
    pub fn clear(&mut self) {
        self.toasts.clear();
    }

    // Convenience methods for common toast types

    /// Adds an error toast
    pub fn error(&mut self, message: impl Into<String>, current_time: Duration) {
        self.push(Toast::new(ToastType::Error, message, current_time));
    }

    /// Adds a success toast
    pub fn success(&mut self, message: impl Into<String>, current_time: Duration) {
        self.push(Toast::new(ToastType::Success, message, current_time));
    }

    /// Adds an info toast
    pub fn info(&mut self, message: impl Into<String>, current_time: Duration) {
        self.push(Toast::new(ToastType::Info, message, current_time));
    }

    /// Adds a warning toast
    pub fn warning(&mut self, message: impl Into<String>, current_time: Duration) {
        self.push(Toast::new(ToastType::Warning, message, current_time));
    }
}

/// Event fired to trigger a toast notification
#[derive(Event, Debug, Clone)]
pub struct ShowToast {
    pub toast_type: ToastType,
    pub message: String,
}

impl ShowToast {
    pub fn error(message: impl Into<String>) -> Self {
        Self {
            toast_type: ToastType::Error,
            message: message.into(),
        }
    }

    pub fn success(message: impl Into<String>) -> Self {
        Self {
            toast_type: ToastType::Success,
            message: message.into(),
        }
    }

    pub fn info(message: impl Into<String>) -> Self {
        Self {
            toast_type: ToastType::Info,
            message: message.into(),
        }
    }

    pub fn warning(message: impl Into<String>) -> Self {
        Self {
            toast_type: ToastType::Warning,
            message: message.into(),
        }
    }
}

/// Component marker for the toast container UI
#[derive(Component)]
pub struct ToastContainer;

/// Component marker for individual toast UI elements
#[derive(Component)]
pub struct ToastElement {
    pub index: usize,
}

/// Plugin that handles toast notifications
pub struct ToastPlugin;

impl Plugin for ToastPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ToastQueue>()
            .add_event::<ShowToast>()
            .add_systems(Startup, spawn_toast_container)
            .add_systems(
                Update,
                (handle_toast_events, cleanup_toasts, update_toast_ui).chain(),
            );
    }
}

/// System to spawn the toast container on startup
fn spawn_toast_container(mut commands: Commands) {
    commands.spawn((
        ToastContainer,
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(10.0),
            right: Val::Px(10.0),
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(10.0),
            ..default()
        },
        ZIndex(1000), // High z-index to appear above other UI
    ));
}

/// System to handle ShowToast events and add them to the queue
fn handle_toast_events(
    mut toast_events: EventReader<ShowToast>,
    mut toast_queue: ResMut<ToastQueue>,
    time: Res<Time>,
) {
    let current_time = time.elapsed();
    for event in toast_events.read() {
        toast_queue.push(Toast::new(
            event.toast_type,
            event.message.clone(),
            current_time,
        ));
    }
}

/// System to clean up expired toasts
fn cleanup_toasts(mut toast_queue: ResMut<ToastQueue>, time: Res<Time>) {
    toast_queue.cleanup(time.elapsed());
}

/// System to update the toast UI to match the current queue
fn update_toast_ui(
    mut commands: Commands,
    toast_queue: Res<ToastQueue>,
    container_query: Query<Entity, With<ToastContainer>>,
    toast_elements: Query<Entity, With<ToastElement>>,
) {
    // Only update if the queue has changed
    if !toast_queue.is_changed() {
        return;
    }

    let container = match container_query.get_single() {
        Ok(entity) => entity,
        Err(_) => return,
    };

    // Despawn all existing toast elements
    for entity in toast_elements.iter() {
        commands.entity(entity).despawn_recursive();
    }

    // Spawn new toast elements for each active toast
    let toasts = toast_queue.toasts();
    for (index, toast) in toasts.iter().enumerate() {
        spawn_toast_element(&mut commands, container, index, toast);
    }
}

/// Helper function to spawn a single toast UI element
fn spawn_toast_element(commands: &mut Commands, parent: Entity, index: usize, toast: &Toast) {
    let toast_type = toast.toast_type;
    let color = toast_type.color();
    let bg_color = toast_type.bg_color();

    commands.entity(parent).with_children(|parent| {
        parent
            .spawn((
                ToastElement { index },
                Node {
                    width: Val::Px(350.0),
                    padding: UiRect::all(Val::Px(12.0)),
                    border: UiRect::all(Val::Px(2.0)),
                    ..default()
                },
                BackgroundColor(bg_color),
                BorderColor(color),
            ))
            .with_children(|parent| {
                // Icon and label
                parent.spawn((
                    Text::new(format!(
                        "{} {}: {}",
                        toast_type.icon(),
                        toast_type.label(),
                        &toast.message
                    )),
                    TextFont {
                        font_size: 16.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.95, 0.95, 0.95)),
                ));
            });
    });
}
