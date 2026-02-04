use bevy::prelude::*;

use crate::item::recipe::RecipeId;
use crate::skills::{blacksmith_speed_multiplier, SkillType, Skills};

use super::events::{AnvilCraftingStarted, TryStartAnvilCrafting};
use super::AnvilActiveTimer;

#[derive(Component, Default, Clone)]
pub struct AnvilCraftingState {
    pub selected_recipe: Option<RecipeId>,
}

impl AnvilCraftingState {
    pub fn complete_crafting(&mut self) -> Option<RecipeId> {
        self.selected_recipe.take()
    }
}

const BASE_ANVIL_DURATION: f32 = 3.0;

pub fn handle_try_start_anvil_crafting(
    mut commands: Commands,
    mut try_events: MessageReader<TryStartAnvilCrafting>,
    mut started_events: MessageWriter<AnvilCraftingStarted>,
    skills: Res<Skills>,
    query: Query<&AnvilCraftingState, Without<AnvilActiveTimer>>,
) {
    for event in try_events.read() {
        let entity = event.entity;

        let Ok(state) = query.get(entity) else {
            continue;
        };

        if state.selected_recipe.is_none() {
            continue;
        }

        let blacksmith_level = skills
            .skill(SkillType::Blacksmith)
            .map(|s| s.level)
            .unwrap_or(1);
        let speed_mult = blacksmith_speed_multiplier(blacksmith_level);
        let duration = BASE_ANVIL_DURATION * speed_mult;

        commands
            .entity(entity)
            .insert(AnvilActiveTimer(Timer::from_seconds(duration, TimerMode::Once)));

        started_events.write(AnvilCraftingStarted { entity });
    }
}
