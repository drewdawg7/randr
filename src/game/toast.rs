use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::*;
use std::time::Duration;

const TOAST_DURATION: Duration = Duration::from_secs(3);

#[derive(Resource)]
struct ToastSprite {
    aseprite: Handle<Aseprite>,
}

#[derive(Message, Debug, Clone)]
pub struct ShowToast(pub String);

impl ShowToast {
    pub fn new(message: impl Into<String>) -> Self {
        Self(message.into())
    }
}

#[derive(Component)]
struct ToastContainer;

#[derive(Component)]
struct ToastTimer(Timer);

pub struct ToastPlugin;

impl Plugin for ToastPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<ShowToast>()
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
    container: Query<Entity, With<ToastContainer>>,
) {
    let Ok(container) = container.single() else {
        return;
    };

    for event in events.read() {
        commands.entity(container).with_children(|parent| {
            parent
                .spawn((
                    ToastTimer(Timer::new(TOAST_DURATION, TimerMode::Once)),
                    Node {
                        padding: UiRect::all(Val::Px(8.0)),
                        ..default()
                    },
                    ImageNode::default(),
                    AseSlice {
                        name: "Slice 1".into(),
                        aseprite: toast_sprite.aseprite.clone(),
                    },
                ))
                .with_children(|toast| {
                    toast.spawn((
                        Text::new(event.0.clone()),
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
