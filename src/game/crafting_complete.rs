use bevy::prelude::*;

use crate::crafting_station::{AnvilCraftingState, ForgeCraftingState};
use crate::inventory::{Inventory, ManagesItems};
use crate::skills::{blacksmith_bonus_item_chance, SkillType, SkillXpGained, Skills};

#[derive(Message, Debug, Clone)]
pub struct ForgeCraftingCompleteEvent {
    pub entity: Entity,
}

#[derive(Message, Debug, Clone)]
pub struct AnvilCraftingCompleteEvent {
    pub entity: Entity,
}

pub struct CraftingCompletePlugin;

impl Plugin for CraftingCompletePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ForgeCraftingCompleteEvent>()
            .add_event::<AnvilCraftingCompleteEvent>()
            .add_systems(
                Update,
                (
                    handle_forge_crafting_complete.run_if(on_event::<ForgeCraftingCompleteEvent>),
                    handle_anvil_crafting_complete.run_if(on_event::<AnvilCraftingCompleteEvent>),
                ),
            );
    }
}

fn handle_forge_crafting_complete(
    mut events: MessageReader<ForgeCraftingCompleteEvent>,
    mut xp_events: MessageWriter<SkillXpGained>,
    skills: Res<Skills>,
    mut forge_query: Query<&mut ForgeCraftingState>,
) {
    let blacksmith_level = skills
        .skill(SkillType::Blacksmith)
        .map(|s| s.level)
        .unwrap_or(1);
    let bonus_chance = blacksmith_bonus_item_chance(blacksmith_level);

    for event in events.read() {
        let Ok(mut state) = forge_query.get_mut(event.entity) else {
            continue;
        };

        let coal_qty = state.coal_slot.as_ref().map(|(_, q)| *q).unwrap_or(0);
        let ore_qty = state.ore_slot.as_ref().map(|(_, q)| *q).unwrap_or(0);
        let ingot_count = coal_qty.min(ore_qty);

        state.complete_crafting_with_bonus(bonus_chance);

        if ingot_count > 0 {
            xp_events.write(SkillXpGained {
                skill: SkillType::Blacksmith,
                amount: ingot_count as u64 * 25,
            });
        }
    }
}

fn handle_anvil_crafting_complete(
    mut events: MessageReader<AnvilCraftingCompleteEvent>,
    mut xp_events: MessageWriter<SkillXpGained>,
    mut inventory: ResMut<Inventory>,
    skills: Res<Skills>,
    mut anvil_query: Query<&mut AnvilCraftingState>,
) {
    let blacksmith_level = skills
        .skill(SkillType::Blacksmith)
        .map(|s| s.level)
        .unwrap_or(1);

    for event in events.read() {
        let Ok(mut state) = anvil_query.get_mut(event.entity) else {
            continue;
        };

        let Some(recipe_id) = state.complete_crafting() else {
            continue;
        };

        let spec = recipe_id.spec();
        let item = spec.output.spawn_with_quality_bonus(blacksmith_level);
        let _ = inventory.add_to_inv(item);

        let ingredient_count: u32 = spec.ingredients.values().sum();
        let xp_amount = 75 + (ingredient_count.saturating_sub(1) * 25);
        xp_events.write(SkillXpGained {
            skill: SkillType::Blacksmith,
            amount: xp_amount as u64,
        });
    }
}
