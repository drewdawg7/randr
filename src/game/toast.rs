use bevy::prelude::*;
use std::time::Duration;

#[derive(Resource, Clone, Debug)]
pub struct ToastConfig {
    pub max_toasts: usize,
    pub duration: Duration,
}

impl Default for ToastConfig {
    fn default() -> Self {
        Self {
            max_toasts: 5,
            duration: Duration::from_secs(3),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ToastType {
    Error,
    Success,
    Info,
    Warning,
}

impl ToastType {
    pub fn icon(&self) -> &'static str {
        match self {
            ToastType::Error => "[!]",
            ToastType::Success => "[+]",
            ToastType::Info => "[i]",
            ToastType::Warning => "[?]",
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            ToastType::Error => "ERROR",
            ToastType::Success => "SUCCESS",
            ToastType::Info => "INFO",
            ToastType::Warning => "WARNING",
        }
    }

    pub fn color(&self) -> Color {
        match self {
            ToastType::Error => Color::srgb(0.9, 0.2, 0.2),
            ToastType::Success => Color::srgb(0.2, 0.8, 0.2),
            ToastType::Info => Color::srgb(0.2, 0.5, 0.9),
            ToastType::Warning => Color::srgb(0.9, 0.7, 0.2),
        }
    }

    pub fn bg_color(&self) -> Color {
        match self {
            ToastType::Error => Color::srgb(0.3, 0.1, 0.1),
            ToastType::Success => Color::srgb(0.1, 0.3, 0.1),
            ToastType::Info => Color::srgb(0.1, 0.2, 0.3),
            ToastType::Warning => Color::srgb(0.3, 0.25, 0.1),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Toast {
    pub toast_type: ToastType,
    pub message: String,
    timer: Timer,
}

impl Toast {
    pub fn new(toast_type: ToastType, message: impl Into<String>, config: &ToastConfig) -> Self {
        Self {
            toast_type,
            message: message.into(),
            timer: Timer::new(config.duration, TimerMode::Once),
        }
    }

    pub fn tick(&mut self, delta: Duration) {
        self.timer.tick(delta);
    }

    pub fn is_expired(&self) -> bool {
        self.timer.is_finished()
    }
}

#[derive(Resource, Default)]
pub struct ToastQueue {
    toasts: Vec<Toast>,
}

impl ToastQueue {
    pub fn push(&mut self, toast: Toast, config: &ToastConfig) {
        self.toasts.insert(0, toast);
        if self.toasts.len() > config.max_toasts {
            self.toasts.pop();
        }
    }

    pub fn tick_and_cleanup(&mut self, delta: Duration) {
        for toast in &mut self.toasts {
            toast.tick(delta);
        }
        self.toasts.retain(|t| !t.is_expired());
    }

    pub fn toasts(&self) -> &[Toast] {
        &self.toasts
    }

    pub fn clear(&mut self) {
        self.toasts.clear();
    }
}

#[derive(Message, Debug, Clone)]
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

#[derive(Component)]
pub struct ToastContainer;

#[derive(Component)]
pub struct ToastElement {
    pub index: usize,
}

pub struct ToastPlugin;

impl Plugin for ToastPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ToastConfig>()
            .init_resource::<ToastQueue>()
            .add_message::<ShowToast>()
            .add_systems(Startup, spawn_toast_container)
            .add_systems(
                Update,
                (
                    handle_toast_events,
                    cleanup_toasts,
                    update_toast_ui.run_if(resource_changed::<ToastQueue>),
                )
                    .chain(),
            );
    }
}

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
        ZIndex(1000),
    ));
}

fn handle_toast_events(
    mut toast_events: MessageReader<ShowToast>,
    mut toast_queue: ResMut<ToastQueue>,
    config: Res<ToastConfig>,
) {
    for event in toast_events.read() {
        toast_queue.push(
            Toast::new(event.toast_type, event.message.clone(), &config),
            &config,
        );
    }
}

fn cleanup_toasts(mut toast_queue: ResMut<ToastQueue>, time: Res<Time>) {
    toast_queue.tick_and_cleanup(time.delta());
}

fn update_toast_ui(
    mut commands: Commands,
    toast_queue: Res<ToastQueue>,
    container_query: Query<Entity, With<ToastContainer>>,
    toast_elements: Query<Entity, With<ToastElement>>,
) {
    let container = match container_query.single() {
        Ok(entity) => entity,
        Err(_) => return,
    };

    for entity in toast_elements.iter() {
        commands.entity(entity).despawn();
    }

    let toasts = toast_queue.toasts();
    for (index, toast) in toasts.iter().enumerate() {
        spawn_toast_element(&mut commands, container, index, toast);
    }
}

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
                BorderColor::all(color),
            ))
            .with_children(|parent| {
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
