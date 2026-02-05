use bevy::prelude::*;

use crate::assets::{GameSprites, SpriteSheetKey, UiSelectorsSlice};

const SELECTOR_FRAME_DURATION: f32 = 0.5;
const SELECTOR_SIZE: f32 = 48.0;

pub struct SelectorPlugin;

impl Plugin for SelectorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            PostUpdate,
            animate_selectors.run_if(any_with_component::<AnimatedSelector>),
        );
    }
}

#[derive(Component)]
pub struct AnimatedSelector {
    timer: Timer,
    frame: usize,
    frame_indices: [usize; 2],
}

impl AnimatedSelector {
    pub fn new(frame_indices: [usize; 2]) -> Self {
        Self {
            timer: Timer::from_seconds(SELECTOR_FRAME_DURATION, TimerMode::Repeating),
            frame: 0,
            frame_indices,
        }
    }
}

pub fn spawn_selector(parent: &mut ChildSpawnerCommands, game_sprites: &GameSprites) -> Option<Entity> {
    let sheet = game_sprites.get(SpriteSheetKey::UiSelectors)?;
    let idx1 = sheet.get(UiSelectorsSlice::SelectorFrame1.as_str())?;
    let idx2 = sheet.get(UiSelectorsSlice::SelectorFrame2.as_str())?;
    let img = sheet.image_node(UiSelectorsSlice::SelectorFrame1.as_str())?;

    Some(
        parent
            .spawn((
                AnimatedSelector::new([idx1, idx2]),
                Node {
                    position_type: PositionType::Absolute,
                    width: Val::Px(SELECTOR_SIZE),
                    height: Val::Px(SELECTOR_SIZE),
                    ..default()
                },
                img,
            ))
            .id(),
    )
}

fn animate_selectors(time: Res<Time>, mut selectors: Query<(&mut AnimatedSelector, &mut ImageNode)>) {
    for (mut selector, mut image) in &mut selectors {
        selector.timer.tick(time.delta());
        if selector.timer.just_finished() {
            selector.frame = (selector.frame + 1) % 2;
            if let Some(ref mut atlas) = image.texture_atlas {
                atlas.index = selector.frame_indices[selector.frame];
            }
        }
    }
}
