#[cfg(test)]
use uuid::Uuid;

#[cfg(test)]
use crate::assets::SpriteSheetKey;
#[cfg(test)]
use crate::item::enums::{EquipmentType, ItemQuality, ItemType, MaterialType};
#[cfg(test)]
use crate::item::{Item, ItemId, SpriteInfo};
#[cfg(test)]
use crate::stats::{StatSheet, StatType};

#[cfg(test)]
use super::{EquipmentSlot, FindsItems, HasInventory, Inventory, InventoryError, InventoryItem, ManagesEquipment, ManagesItems};

// ==================== Test Helpers ====================

#[cfg(test)]
fn create_test_weapon(id: ItemId, attack: i32) -> Item {
    Item {
        item_uuid: Uuid::new_v4(),
        item_id: id,
        item_type: ItemType::Equipment(EquipmentType::Weapon),
        name: "Test Weapon".to_string(),
        is_equipped: false,
        is_locked: false,
        num_upgrades: 0,
        max_upgrades: 5,
        max_stack_quantity: 1,
        base_stats: StatSheet::new().with(StatType::Attack, attack),
        stats: StatSheet::new().with(StatType::Attack, attack),
        gold_value: 100,
        quality: ItemQuality::Normal,
        sprite: SpriteInfo { name: String::new(), sheet_key: SpriteSheetKey::IconItems },
    }
}

#[cfg(test)]
fn create_test_shield(id: ItemId, defense: i32) -> Item {
    Item {
        item_uuid: Uuid::new_v4(),
        item_id: id,
        item_type: ItemType::Equipment(EquipmentType::Shield),
        name: "Test Shield".to_string(),
        is_equipped: false,
        is_locked: false,
        num_upgrades: 0,
        max_upgrades: 5,
        max_stack_quantity: 1,
        base_stats: StatSheet::new().with(StatType::Defense, defense),
        stats: StatSheet::new().with(StatType::Defense, defense),
        gold_value: 80,
        quality: ItemQuality::Normal,
        sprite: SpriteInfo { name: String::new(), sheet_key: SpriteSheetKey::IconItems },
    }
}

#[cfg(test)]
fn create_test_material(id: ItemId) -> Item {
    Item {
        item_uuid: Uuid::new_v4(),
        item_id: id,
        item_type: ItemType::Material(MaterialType::Ore),
        name: "Test Material".to_string(),
        is_equipped: false,
        is_locked: false,
        num_upgrades: 0,
        max_upgrades: 0,
        max_stack_quantity: 99,
        base_stats: StatSheet::new(),
        stats: StatSheet::new(),
        gold_value: 10,
        quality: ItemQuality::Normal,
        sprite: SpriteInfo { name: String::new(), sheet_key: SpriteSheetKey::IconItems },
    }
}

#[cfg(test)]
struct MockInventoryHolder {
    inventory: Inventory,
}

#[cfg(test)]
impl MockInventoryHolder {
    fn new() -> Self {
        Self {
            inventory: Inventory::new(),
        }
    }
}

#[cfg(test)]
impl HasInventory for MockInventoryHolder {
    fn inventory(&self) -> &Inventory {
        &self.inventory
    }

    fn inventory_mut(&mut self) -> &mut Inventory {
        &mut self.inventory
    }
}

// ==================== InventoryItem::new() tests ====================

#[test]
fn inventory_item_new_creates_with_quantity_one() {
    let weapon = create_test_weapon(ItemId::Sword, 10);
    let inv_item = InventoryItem::new(weapon);

    assert_eq!(inv_item.quantity, 1);
}

#[test]
fn inventory_item_new_preserves_item_data() {
    let weapon = create_test_weapon(ItemId::Sword, 25);
    let original_uuid = weapon.item_uuid;
    let inv_item = InventoryItem::new(weapon);

    assert_eq!(inv_item.item.item_id, ItemId::Sword);
    assert_eq!(inv_item.item.item_uuid, original_uuid);
    assert_eq!(inv_item.item.stats.value(StatType::Attack), 25);
}

// ==================== InventoryItem::uuid() tests ====================

#[test]
fn inventory_item_uuid_returns_item_uuid() {
    let weapon = create_test_weapon(ItemId::Sword, 10);
    let expected_uuid = weapon.item_uuid;
    let inv_item = InventoryItem::new(weapon);

    assert_eq!(inv_item.uuid(), expected_uuid);
}

// ==================== InventoryItem::decrease_quantity() tests ====================

#[test]
fn inventory_item_decrease_quantity_decreases_correctly() {
    let material = create_test_material(ItemId::IronOre);
    let mut inv_item = InventoryItem::new(material);
    inv_item.quantity = 10;

    inv_item.decrease_quantity(3);
    assert_eq!(inv_item.quantity, 7);
}

#[test]
fn inventory_item_decrease_quantity_floors_at_zero() {
    let material = create_test_material(ItemId::IronOre);
    let mut inv_item = InventoryItem::new(material);
    inv_item.quantity = 5;

    inv_item.decrease_quantity(10); // Decrease by more than available
    assert_eq!(inv_item.quantity, 0);
}

#[test]
fn inventory_item_decrease_quantity_to_exactly_zero() {
    let material = create_test_material(ItemId::IronOre);
    let mut inv_item = InventoryItem::new(material);
    inv_item.quantity = 5;

    inv_item.decrease_quantity(5);
    assert_eq!(inv_item.quantity, 0);
}

// ==================== InventoryItem::increase_quantity() tests ====================

#[test]
fn inventory_item_increase_quantity_increases_correctly() {
    let material = create_test_material(ItemId::IronOre);
    let mut inv_item = InventoryItem::new(material);

    inv_item.increase_quantity(5);
    assert_eq!(inv_item.quantity, 6); // Started at 1
}

#[test]
fn inventory_item_increase_quantity_multiple_times() {
    let material = create_test_material(ItemId::IronOre);
    let mut inv_item = InventoryItem::new(material);

    inv_item.increase_quantity(3);
    inv_item.increase_quantity(2);
    inv_item.increase_quantity(1);
    assert_eq!(inv_item.quantity, 7); // 1 + 3 + 2 + 1
}

// ==================== Inventory::new() tests ====================

#[test]
fn inventory_new_initializes_empty_items() {
    let inv = Inventory::new();
    assert!(inv.items.is_empty());
}

#[test]
fn inventory_new_initializes_empty_equipment() {
    let inv = Inventory::new();
    assert!(inv.equipment().is_empty());
}

#[test]
fn inventory_new_sets_max_slots_to_fifteen() {
    let inv = Inventory::new();
    assert_eq!(inv.max_slots(), 15);
}

// ==================== Inventory::new_unlimited() tests ====================

#[test]
fn inventory_new_unlimited_has_max_slots() {
    let inv = Inventory::new_unlimited();
    assert_eq!(inv.max_slots(), usize::MAX);
}

// ==================== Inventory::max_slots() tests ====================

#[test]
fn inventory_max_slots_returns_correct_value() {
    let inv = Inventory::new();
    assert_eq!(inv.max_slots(), 15);
}

// ==================== Inventory::sum_equipment_stats() tests ====================

#[test]
fn inventory_sum_equipment_stats_returns_zero_when_empty() {
    let inv = Inventory::new();
    assert_eq!(inv.sum_equipment_stats(StatType::Attack), 0);
    assert_eq!(inv.sum_equipment_stats(StatType::Defense), 0);
}

#[test]
fn inventory_sum_equipment_stats_sums_single_item() {
    let mut holder = MockInventoryHolder::new();
    let weapon = create_test_weapon(ItemId::Sword, 25);
    holder.equip_item(weapon, EquipmentSlot::Weapon);

    assert_eq!(holder.inventory().sum_equipment_stats(StatType::Attack), 25);
}

#[test]
fn inventory_sum_equipment_stats_sums_multiple_items() {
    let mut holder = MockInventoryHolder::new();

    let weapon = create_test_weapon(ItemId::Sword, 25);
    holder.equip_item(weapon, EquipmentSlot::Weapon);

    let shield = create_test_shield(ItemId::BasicShield, 15);
    holder.equip_item(shield, EquipmentSlot::OffHand);

    assert_eq!(holder.inventory().sum_equipment_stats(StatType::Attack), 25);
    assert_eq!(holder.inventory().sum_equipment_stats(StatType::Defense), 15);
}

// ==================== Inventory::equipment() accessor tests ====================

#[test]
fn inventory_equipment_accessor_returns_reference() {
    let mut holder = MockInventoryHolder::new();
    let weapon = create_test_weapon(ItemId::Sword, 20);
    holder.equip_item(weapon, EquipmentSlot::Weapon);

    let equipment = holder.inventory().equipment();
    assert_eq!(equipment.len(), 1);
    assert!(equipment.contains_key(&EquipmentSlot::Weapon));
}

#[test]
fn inventory_equipment_mut_allows_modification() {
    let mut holder = MockInventoryHolder::new();
    let weapon = create_test_weapon(ItemId::Sword, 20);
    holder.equip_item(weapon, EquipmentSlot::Weapon);

    holder.inventory_mut().equipment_mut().clear();
    assert!(holder.inventory().equipment().is_empty());
}

// ==================== HasInventory::add_to_inv() tests ====================

#[test]
fn add_to_inv_adds_new_item() {
    let mut holder = MockInventoryHolder::new();
    let weapon = create_test_weapon(ItemId::Sword, 10);

    let result = holder.add_to_inv(weapon);
    assert!(result.is_ok());
    assert_eq!(holder.inventory().items.len(), 1);
}

#[test]
fn add_to_inv_stacks_non_equipment_items() {
    let mut holder = MockInventoryHolder::new();

    let material1 = create_test_material(ItemId::IronOre);
    let material2 = create_test_material(ItemId::IronOre);

    holder.add_to_inv(material1).unwrap();
    holder.add_to_inv(material2).unwrap();

    // Should stack into one slot
    assert_eq!(holder.inventory().items.len(), 1);
    assert_eq!(holder.inventory().items[0].quantity, 2);
}

#[test]
fn add_to_inv_does_not_stack_equipment() {
    let mut holder = MockInventoryHolder::new();

    let weapon1 = create_test_weapon(ItemId::Sword, 10);
    let weapon2 = create_test_weapon(ItemId::Sword, 10);

    holder.add_to_inv(weapon1).unwrap();
    holder.add_to_inv(weapon2).unwrap();

    // Equipment should not stack - each takes a slot
    assert_eq!(holder.inventory().items.len(), 2);
}

#[test]
fn add_to_inv_returns_error_when_full() {
    let mut holder = MockInventoryHolder::new();

    // Fill all 15 slots with equipment (which doesn't stack)
    for _ in 0..15 {
        let weapon = create_test_weapon(ItemId::Sword, 10);
        holder.add_to_inv(weapon).unwrap();
    }

    // 16th item should fail
    let extra_weapon = create_test_weapon(ItemId::Sword, 10);
    let result = holder.add_to_inv(extra_weapon);
    assert!(matches!(result, Err(InventoryError::Full)));
}

#[test]
fn add_to_inv_stacks_up_to_max_stack_quantity() {
    let mut holder = MockInventoryHolder::new();

    // Add materials that stack up to 99
    for _ in 0..99 {
        let material = create_test_material(ItemId::IronOre);
        holder.add_to_inv(material).unwrap();
    }

    // All should be in one stack
    assert_eq!(holder.inventory().items.len(), 1);
    assert_eq!(holder.inventory().items[0].quantity, 99);

    // Adding one more should create a new stack
    let material = create_test_material(ItemId::IronOre);
    holder.add_to_inv(material).unwrap();

    assert_eq!(holder.inventory().items.len(), 2);
}

// ==================== HasInventory::find_item_by_uuid() tests ====================

#[test]
fn find_item_by_uuid_finds_item_in_inventory() {
    let mut holder = MockInventoryHolder::new();
    let weapon = create_test_weapon(ItemId::Sword, 10);
    let target_uuid = weapon.item_uuid;

    holder.add_to_inv(weapon).unwrap();

    let found = holder.find_item_by_uuid(target_uuid);
    assert!(found.is_some());
    assert_eq!(found.unwrap().uuid(), target_uuid);
}

#[test]
fn find_item_by_uuid_returns_none_when_not_found() {
    let holder = MockInventoryHolder::new();
    let random_uuid = Uuid::new_v4();

    let found = holder.find_item_by_uuid(random_uuid);
    assert!(found.is_none());
}

// ==================== HasInventory::find_item_by_id() tests ====================

#[test]
fn find_item_by_id_finds_item_in_inventory() {
    let mut holder = MockInventoryHolder::new();
    let weapon = create_test_weapon(ItemId::Sword, 10);
    holder.add_to_inv(weapon).unwrap();

    let found = holder.find_item_by_id(ItemId::Sword);
    assert!(found.is_some());
    assert_eq!(found.unwrap().item.item_id, ItemId::Sword);
}

#[test]
fn find_item_by_id_finds_item_in_equipment() {
    let mut holder = MockInventoryHolder::new();
    let weapon = create_test_weapon(ItemId::Sword, 10);
    holder.equip_item(weapon, EquipmentSlot::Weapon);

    let found = holder.find_item_by_id(ItemId::Sword);
    assert!(found.is_some());
    assert_eq!(found.unwrap().item.item_id, ItemId::Sword);
}

#[test]
fn find_item_by_id_returns_none_when_not_found() {
    let holder = MockInventoryHolder::new();

    let found = holder.find_item_by_id(ItemId::Sword);
    assert!(found.is_none());
}

// ==================== HasInventory::decrease_item_quantity() tests ====================

#[test]
fn decrease_item_quantity_decreases_in_inventory() {
    let mut holder = MockInventoryHolder::new();
    let material = create_test_material(ItemId::IronOre);
    holder.add_to_inv(material.clone()).unwrap();
    holder.add_to_inv(create_test_material(ItemId::IronOre)).unwrap();
    holder.add_to_inv(create_test_material(ItemId::IronOre)).unwrap();

    // Should have 3 stacked
    assert_eq!(holder.inventory().items[0].quantity, 3);

    holder.decrease_item_quantity(ItemId::IronOre, 1);

    assert_eq!(holder.inventory().items[0].quantity, 2);
}

#[test]
fn decrease_item_quantity_removes_when_zero() {
    let mut holder = MockInventoryHolder::new();
    let material = create_test_material(ItemId::IronOre);
    holder.add_to_inv(material).unwrap();

    holder.decrease_item_quantity(ItemId::IronOre, 1);

    // Item should be removed
    assert!(holder.inventory().items.is_empty());
}

#[test]
fn decrease_item_quantity_works_on_equipment() {
    let mut holder = MockInventoryHolder::new();
    let weapon = create_test_weapon(ItemId::Sword, 10);
    holder.equip_item(weapon, EquipmentSlot::Weapon);

    holder.decrease_item_quantity(ItemId::Sword, 1);

    // Equipment item should be removed from equipment slot
    assert!(holder.inventory().equipment().is_empty());
}

// ==================== HasInventory::remove_item_from_inventory() tests ====================

#[test]
fn remove_item_from_inventory_removes_by_uuid() {
    let mut holder = MockInventoryHolder::new();
    let weapon = create_test_weapon(ItemId::Sword, 10);
    let target_uuid = weapon.item_uuid;
    holder.add_to_inv(weapon).unwrap();

    let inv_item = holder.find_item_by_uuid(target_uuid).unwrap().clone();
    holder.remove_item_from_inventory(&inv_item);

    assert!(holder.inventory().items.is_empty());
}

#[test]
fn remove_item_from_inventory_only_removes_matching_item() {
    let mut holder = MockInventoryHolder::new();
    let weapon1 = create_test_weapon(ItemId::Sword, 10);
    let weapon2 = create_test_weapon(ItemId::Dagger, 8);
    let target_uuid = weapon1.item_uuid;

    holder.add_to_inv(weapon1).unwrap();
    holder.add_to_inv(weapon2).unwrap();

    let inv_item = holder.find_item_by_uuid(target_uuid).unwrap().clone();
    holder.remove_item_from_inventory(&inv_item);

    assert_eq!(holder.inventory().items.len(), 1);
    assert_eq!(holder.inventory().items[0].item.item_id, ItemId::Dagger);
}

// ==================== HasInventory::equip_item() tests ====================

#[test]
fn equip_item_adds_to_equipment_slot() {
    let mut holder = MockInventoryHolder::new();
    let weapon = create_test_weapon(ItemId::Sword, 10);

    holder.equip_item(weapon, EquipmentSlot::Weapon);

    assert!(holder.inventory().equipment().contains_key(&EquipmentSlot::Weapon));
}

#[test]
fn equip_item_sets_is_equipped_flag() {
    let mut holder = MockInventoryHolder::new();
    let weapon = create_test_weapon(ItemId::Sword, 10);

    holder.equip_item(weapon, EquipmentSlot::Weapon);

    let equipped = holder.get_equipped_item(EquipmentSlot::Weapon).unwrap();
    assert!(equipped.item.is_equipped);
}

#[test]
fn equip_item_replaces_existing_equipped_item() {
    let mut holder = MockInventoryHolder::new();
    let weapon1 = create_test_weapon(ItemId::Sword, 10);
    let weapon2 = create_test_weapon(ItemId::Dagger, 15);

    holder.equip_item(weapon1, EquipmentSlot::Weapon);
    holder.equip_item(weapon2, EquipmentSlot::Weapon);

    let equipped = holder.get_equipped_item(EquipmentSlot::Weapon).unwrap();
    assert_eq!(equipped.item.item_id, ItemId::Dagger);

    // Old weapon should be in inventory
    assert!(holder.find_item_by_id(ItemId::Sword).is_some());
}

// ==================== HasInventory::unequip_item() tests ====================

#[test]
fn unequip_item_moves_to_inventory() {
    let mut holder = MockInventoryHolder::new();
    let weapon = create_test_weapon(ItemId::Sword, 10);
    holder.equip_item(weapon, EquipmentSlot::Weapon);

    let result = holder.unequip_item(EquipmentSlot::Weapon);
    assert!(result.is_ok());

    assert!(holder.inventory().equipment().get(&EquipmentSlot::Weapon).is_none());
    assert!(holder.find_item_by_id(ItemId::Sword).is_some());
}

#[test]
fn unequip_item_clears_is_equipped_flag() {
    let mut holder = MockInventoryHolder::new();
    let weapon = create_test_weapon(ItemId::Sword, 10);
    holder.equip_item(weapon, EquipmentSlot::Weapon);

    holder.unequip_item(EquipmentSlot::Weapon).unwrap();

    let item = holder.find_item_by_id(ItemId::Sword).unwrap();
    assert!(!item.item.is_equipped);
}

#[test]
fn unequip_item_fails_when_inventory_full() {
    let mut holder = MockInventoryHolder::new();

    // Equip a weapon
    let weapon = create_test_weapon(ItemId::Sword, 10);
    holder.equip_item(weapon, EquipmentSlot::Weapon);

    // Fill all 15 inventory slots
    for _ in 0..15 {
        let item = create_test_weapon(ItemId::Dagger, 5);
        holder.add_to_inv(item).unwrap();
    }

    // Unequip should fail
    let result = holder.unequip_item(EquipmentSlot::Weapon);
    assert!(matches!(result, Err(InventoryError::Full)));

    // Weapon should still be equipped
    assert!(holder.inventory().equipment().contains_key(&EquipmentSlot::Weapon));
}

#[test]
fn unequip_item_succeeds_for_empty_slot() {
    let mut holder = MockInventoryHolder::new();

    let result = holder.unequip_item(EquipmentSlot::Weapon);
    assert!(result.is_ok());
}

// ==================== HasInventory::equip_from_inventory() tests ====================

#[test]
fn equip_from_inventory_moves_from_inventory_to_equipment() {
    let mut holder = MockInventoryHolder::new();
    let weapon = create_test_weapon(ItemId::Sword, 10);
    let weapon_uuid = weapon.item_uuid;
    holder.add_to_inv(weapon).unwrap();

    holder.equip_from_inventory(weapon_uuid, EquipmentSlot::Weapon);

    // Should be in equipment, not inventory
    assert!(holder.inventory().equipment().contains_key(&EquipmentSlot::Weapon));
    assert!(holder.find_item_by_uuid(weapon_uuid).is_none());
}

#[test]
fn equip_from_inventory_sets_is_equipped_flag() {
    let mut holder = MockInventoryHolder::new();
    let weapon = create_test_weapon(ItemId::Sword, 10);
    let weapon_uuid = weapon.item_uuid;
    holder.add_to_inv(weapon).unwrap();

    holder.equip_from_inventory(weapon_uuid, EquipmentSlot::Weapon);

    let equipped = holder.get_equipped_item(EquipmentSlot::Weapon).unwrap();
    assert!(equipped.item.is_equipped);
}

#[test]
fn equip_from_inventory_swaps_with_existing_equipment() {
    let mut holder = MockInventoryHolder::new();

    let weapon1 = create_test_weapon(ItemId::Sword, 10);
    let weapon2 = create_test_weapon(ItemId::Dagger, 15);
    let weapon2_uuid = weapon2.item_uuid;

    holder.add_to_inv(weapon1).unwrap();
    holder.add_to_inv(weapon2).unwrap();

    // Equip first weapon
    let weapon1_uuid = holder.inventory().items[0].uuid();
    holder.equip_from_inventory(weapon1_uuid, EquipmentSlot::Weapon);

    // Equip second weapon (should swap)
    holder.equip_from_inventory(weapon2_uuid, EquipmentSlot::Weapon);

    let equipped = holder.get_equipped_item(EquipmentSlot::Weapon).unwrap();
    assert_eq!(equipped.item.item_id, ItemId::Dagger);

    // First weapon should be back in inventory
    let sword = holder.find_item_by_id(ItemId::Sword);
    assert!(sword.is_some());
}

#[test]
fn equip_from_inventory_does_nothing_for_invalid_uuid() {
    let mut holder = MockInventoryHolder::new();
    let random_uuid = Uuid::new_v4();

    holder.equip_from_inventory(random_uuid, EquipmentSlot::Weapon);

    assert!(holder.inventory().equipment().is_empty());
}

// ==================== HasInventory::get_equipped_item() tests ====================

#[test]
fn get_equipped_item_returns_item_in_slot() {
    let mut holder = MockInventoryHolder::new();
    let weapon = create_test_weapon(ItemId::Sword, 10);
    holder.equip_item(weapon, EquipmentSlot::Weapon);

    let equipped = holder.get_equipped_item(EquipmentSlot::Weapon);
    assert!(equipped.is_some());
    assert_eq!(equipped.unwrap().item.item_id, ItemId::Sword);
}

#[test]
fn get_equipped_item_returns_none_for_empty_slot() {
    let holder = MockInventoryHolder::new();

    let equipped = holder.get_equipped_item(EquipmentSlot::Weapon);
    assert!(equipped.is_none());
}

// ==================== HasInventory::remove_item() tests ====================

#[test]
fn remove_item_removes_from_equipment() {
    let mut holder = MockInventoryHolder::new();
    let weapon = create_test_weapon(ItemId::Sword, 10);
    let weapon_uuid = weapon.item_uuid;
    holder.equip_item(weapon, EquipmentSlot::Weapon);

    let removed = holder.remove_item(weapon_uuid);

    assert!(removed.is_some());
    assert!(holder.inventory().equipment().is_empty());
}

#[test]
fn remove_item_removes_from_inventory() {
    let mut holder = MockInventoryHolder::new();
    let weapon = create_test_weapon(ItemId::Sword, 10);
    let weapon_uuid = weapon.item_uuid;
    holder.add_to_inv(weapon).unwrap();

    let removed = holder.remove_item(weapon_uuid);

    assert!(removed.is_some());
    assert!(holder.inventory().items.is_empty());
}

#[test]
fn remove_item_returns_none_for_missing_uuid() {
    let mut holder = MockInventoryHolder::new();
    let random_uuid = Uuid::new_v4();

    let removed = holder.remove_item(random_uuid);
    assert!(removed.is_none());
}

#[test]
fn remove_item_returns_removed_item() {
    let mut holder = MockInventoryHolder::new();
    let weapon = create_test_weapon(ItemId::Sword, 10);
    let weapon_uuid = weapon.item_uuid;
    holder.add_to_inv(weapon).unwrap();

    let removed = holder.remove_item(weapon_uuid);

    assert!(removed.is_some());
    let removed_item = removed.unwrap();
    assert_eq!(removed_item.uuid(), weapon_uuid);
    assert_eq!(removed_item.item.item_id, ItemId::Sword);
}

// ==================== EquipmentSlot::all() tests ====================

#[test]
fn equipment_slot_all_returns_all_slots() {
    let all_slots = EquipmentSlot::all();

    assert_eq!(all_slots.len(), 9);
    assert!(all_slots.contains(&EquipmentSlot::Weapon));
    assert!(all_slots.contains(&EquipmentSlot::OffHand));
    assert!(all_slots.contains(&EquipmentSlot::Ring));
    assert!(all_slots.contains(&EquipmentSlot::Tool));
    assert!(all_slots.contains(&EquipmentSlot::Head));
    assert!(all_slots.contains(&EquipmentSlot::Chest));
    assert!(all_slots.contains(&EquipmentSlot::Hands));
    assert!(all_slots.contains(&EquipmentSlot::Feet));
    assert!(all_slots.contains(&EquipmentSlot::Legs));
}

// ==================== Integration/User Flow tests ====================

#[test]
fn user_flow_add_equip_unequip_remove() {
    let mut holder = MockInventoryHolder::new();

    // Player picks up a sword
    let sword = create_test_weapon(ItemId::Sword, 15);
    let sword_uuid = sword.item_uuid;
    holder.add_to_inv(sword).unwrap();
    assert_eq!(holder.inventory().items.len(), 1);

    // Player equips the sword
    holder.equip_from_inventory(sword_uuid, EquipmentSlot::Weapon);
    assert!(holder.inventory().items.is_empty());
    assert!(holder.get_equipped_item(EquipmentSlot::Weapon).is_some());

    // Verify stats are counted
    assert_eq!(holder.inventory().sum_equipment_stats(StatType::Attack), 15);

    // Player finds a better sword
    let better_sword = create_test_weapon(ItemId::Dagger, 25);
    let better_sword_uuid = better_sword.item_uuid;
    holder.add_to_inv(better_sword).unwrap();

    // Player equips the better sword (old one goes to inventory)
    holder.equip_from_inventory(better_sword_uuid, EquipmentSlot::Weapon);
    assert_eq!(holder.inventory().items.len(), 1);
    assert_eq!(holder.inventory().sum_equipment_stats(StatType::Attack), 25);

    // Player sells the old sword
    holder.remove_item(sword_uuid);
    assert!(holder.inventory().items.is_empty());
}

#[test]
fn user_flow_collect_materials_and_stack() {
    let mut holder = MockInventoryHolder::new();

    // Player mines some copper ore
    for _ in 0..5 {
        let ore = create_test_material(ItemId::IronOre);
        holder.add_to_inv(ore).unwrap();
    }

    // All should be in one stack
    assert_eq!(holder.inventory().items.len(), 1);
    assert_eq!(holder.inventory().items[0].quantity, 5);

    // Player uses 3 ore for crafting
    holder.decrease_item_quantity(ItemId::IronOre, 3);

    assert_eq!(holder.inventory().items[0].quantity, 2);

    // Player uses remaining ore
    holder.decrease_item_quantity(ItemId::IronOre, 2);

    assert!(holder.inventory().items.is_empty());
}

#[test]
fn user_flow_full_inventory_management() {
    let mut holder = MockInventoryHolder::new();

    // Player has full inventory (15 weapons)
    for i in 0..15 {
        let weapon = create_test_weapon(ItemId::Sword, i as i32);
        holder.add_to_inv(weapon).unwrap();
    }
    assert_eq!(holder.inventory().items.len(), 15);

    // Cannot pick up more
    let extra = create_test_weapon(ItemId::Dagger, 100);
    assert!(matches!(holder.add_to_inv(extra), Err(InventoryError::Full)));

    // But can still equip from inventory (frees a slot)
    let first_uuid = holder.inventory().items[0].uuid();
    holder.equip_from_inventory(first_uuid, EquipmentSlot::Weapon);
    assert_eq!(holder.inventory().items.len(), 14);

    // Now can pick up one more
    let extra = create_test_weapon(ItemId::Dagger, 100);
    assert!(holder.add_to_inv(extra).is_ok());
    assert_eq!(holder.inventory().items.len(), 15);
}
