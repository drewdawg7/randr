use crate::{inventory::InventoryItem, system::game_state};

use super::{ApplyEffect, ConsumableEffect, ConsumableError};

#[derive(Debug, Clone)]
pub struct ConsumableResult {
    #[allow(dead_code)]
    pub item_name: String,
    pub effect_applied: ConsumableEffect,
    pub actual_value: i32,
}

impl ConsumableResult {
    /// Get a human-readable description of what happened
    pub fn describe(&self) -> String {
        self.effect_applied.describe(self.actual_value)
    }
}

/// Use a consumable item on a target
///
/// # Arguments
/// * `target` - The entity receiving the effect
/// * `inv_item` - The inventory item being consumed
///
/// # Returns
/// * `Ok(ConsumableResult)` - Effect was applied successfully
/// * `Err(ConsumableError)` - Effect could not be applied
pub fn use_consumable<T: ApplyEffect>(
    target: &mut T,
    inv_item: &InventoryItem,
) -> Result<ConsumableResult, ConsumableError> {
    let item = &inv_item.item;

    // Verify item is a consumable
    if !item.item_type.is_consumable() {
        return Err(ConsumableError::NotConsumable);
    }

    // Look up effect in registry
    let gs = game_state();
    let effect = gs
        .consumable_registry()
        .get(&item.item_id)
        .ok_or(ConsumableError::NoEffectRegistered)?;

    // Check if effect can be applied
    if !target.can_apply_effect(effect) {
        return Err(ConsumableError::AlreadyAtFullHealth);
    }

    // Apply effect
    let actual_value = target.apply_effect(effect);

    Ok(ConsumableResult {
        item_name: item.name.clone(),
        effect_applied: effect.clone(),
        actual_value,
    })
}
