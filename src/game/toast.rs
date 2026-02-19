use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::*;
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

#[derive(Resource)]
struct ToastSprite {
    aseprite: Handle<Aseprite>,
    slice_name: String,
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
struct ToastElement;

#[derive(Component)]
struct ToastTimer(Timer);

pub struct ToastPlugin;

impl Plugin for ToastPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ToastConfig>()
            .add_message::<ShowToast>()
            .add_systems(PreStartup, load_toast_sprite)
            .add_systems(Startup, spawn_toast_container)
            .add_systems(
                Update,
                (
                    spawn_toast.run_if(on_message::<ShowToast>),
                    tick_toast_timers.run_if(any_with_component::<ToastTimer>),
                ),
            );
    }
}

fn load_toast_sprite(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(ToastSprite {
        aseprite: asset_server.load("sprites/toast_1.aseprite"),
        slice_name: "Slice 1".into(),
    });
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

fn spawn_toast(
    mut commands: Commands,
    mut events: MessageReader<ShowToast>,
    toast_sprite: Res<ToastSprite>,
    config: Res<ToastConfig>,
    container: Query<Entity, With<ToastContainer>>,
) {
    let container = match container.single() {
        Ok(e) => e,
        Err(_) => return,
    };

    for event in events.read() {
        commands.entity(container).with_children(|parent| {
            parent
                .spawn((
                    ToastElement,
                    ToastTimer(Timer::new(config.duration, TimerMode::Once)),
                    Node::default(),
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Node {
                            position_type: PositionType::Absolute,
                            left: Val::Px(0.0),
                            top: Val::Px(0.0),
                            width: Val::Percent(100.0),
                            height: Val::Percent(100.0),
                            ..default()
                        },
                        ImageNode::default(),
                        AseSlice {
                            name: toast_sprite.slice_name.clone().into(),
                            aseprite: toast_sprite.aseprite.clone(),
                        },
                    ));
                    parent.spawn((
                        Text::new(event.message.clone()),
                        TextFont {
                            font_size: 16.0,
                            ..default()
                        },
                        TextColor(Color::srgb(0.95, 0.95, 0.95)),
                    ));
                });
        });
    }
}

fn tick_toast_timers(
    mut commands: Commands,
    time: Res<Time>,
    mut toasts: Query<(Entity, &mut ToastTimer)>,
) {
    for (entity, mut timer) in &mut toasts {
        timer.0.tick(time.delta());
        if timer.0.just_finished() {
            commands.entity(entity).despawn();
        }
    }
}
