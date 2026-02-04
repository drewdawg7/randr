use bevy::prelude::*;
use rand::Rng;

use crate::item::ItemId;
use crate::skills::{blacksmith_speed_multiplier, SkillType, Skills};

use super::events::{ForgeCraftingStarted, TryStartForgeCrafting};
use super::ForgeActiveTimer;

#[derive(Component, Default, Clone)]
pub struct ForgeCraftingState {
    pub coal_slot: Option<(ItemId, u32)>,
    pub ore_slot: Option<(ItemId, u32)>,
    pub product_slot: Option<(ItemId, u32)>,
}

impl ForgeCraftingState {
    pub fn can_start_crafting(&self) -> bool {
        self.coal_slot.is_some() && self.ore_slot.is_some() && self.product_slot.is_none()
    }

    pub fn get_output_item(&self) -> Option<ItemId> {
        self.ore_slot.as_ref().map(|(ore_id, _)| match ore_id {
            ItemId::CopperOre => ItemId::CopperIngot,
            ItemId::IronOre => ItemId::IronIngot,
            ItemId::GoldOre => ItemId::GoldIngot,
            _ => ItemId::IronIngot,
        })
    }

    pub fn complete_crafting(&mut self) {
        self.complete_crafting_with_bonus(0.0);
    }

    pub fn complete_crafting_with_bonus(&mut self, bonus_chance: f32) {
        let Some(output_id) = self.get_output_item() else {
            return;
        };

        let coal_qty = self.coal_slot.as_ref().map(|(_, q)| *q).unwrap_or(0);
        let ore_qty = self.ore_slot.as_ref().map(|(_, q)| *q).unwrap_or(0);
        let base_output = coal_qty.min(ore_qty);

        if base_output > 0 {
            let mut rng = rand::thread_rng();
            let mut bonus_count = 0u32;
            for _ in 0..base_output {
                if rng.gen_range(0.0..1.0) < bonus_chance {
                    bonus_count += 1;
                }
            }
            let output_qty = base_output + bonus_count;

            if let Some((_, qty)) = self.coal_slot.as_mut() {
                *qty -= base_output;
                if *qty == 0 {
                    self.coal_slot = None;
                }
            }
            if let Some((_, qty)) = self.ore_slot.as_mut() {
                *qty -= base_output;
                if *qty == 0 {
                    self.ore_slot = None;
                }
            }
            self.product_slot = Some((output_id, output_qty));
        }
    }
}

const BASE_FORGE_DURATION: f32 = 5.0;

pub fn handle_try_start_forge_crafting(
    mut commands: Commands,
    mut try_events: MessageReader<TryStartForgeCrafting>,
    mut started_events: MessageWriter<ForgeCraftingStarted>,
    skills: Res<Skills>,
    query: Query<&ForgeCraftingState, Without<ForgeActiveTimer>>,
) {
    for event in try_events.read() {
        let entity = event.entity;

        let Ok(state) = query.get(entity) else {
            continue;
        };

        if !state.can_start_crafting() {
            continue;
        }

        let blacksmith_level = skills
            .skill(SkillType::Blacksmith)
            .map(|s| s.level)
            .unwrap_or(1);
        let speed_mult = blacksmith_speed_multiplier(blacksmith_level);
        let duration = BASE_FORGE_DURATION * speed_mult;

        commands
            .entity(entity)
            .insert(ForgeActiveTimer(Timer::from_seconds(duration, TimerMode::Once)));

        started_events.write(ForgeCraftingStarted { entity });
    }
}
