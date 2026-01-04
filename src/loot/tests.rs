#[cfg(test)]
use uuid::Uuid;

#[cfg(test)]
use crate::item::enums::{ItemQuality, ItemType, MaterialType};
#[cfg(test)]
use crate::item::{Item, ItemId};
#[cfg(test)]
use crate::stats::StatSheet;
#[cfg(test)]
use crate::economy::WorthGold;

#[cfg(test)]
use super::definition::{LootItem, LootTable};
#[cfg(test)]
use super::enums::LootError;

// ==================== Test Helpers ====================

#[cfg(test)]
fn create_test_material(id: ItemId, gold_value: i32) -> Item {
    Item {
        item_uuid: Uuid::new_v4(),
        item_id: id,
        item_type: ItemType::Material(MaterialType::Ore),
        name: "Test Material",
        is_equipped: false,
        is_locked: false,
        num_upgrades: 0,
        max_upgrades: 0,
        max_stack_quantity: 99,
        base_stats: StatSheet::new(),
        stats: StatSheet::new(),
        gold_value,
        quality: ItemQuality::Normal,
        tome_data: None,
    }
}

#[cfg(test)]
fn mock_spawn_item(id: ItemId) -> Option<Item> {
    Some(create_test_material(id, 10))
}

// ==================== LootItem::new() tests ====================

#[test]
fn loot_item_new_creates_valid_loot_item() {
    let result = LootItem::new(ItemId::CopperOre, 1, 4, 1..=3);
    assert!(result.is_ok());
}

#[test]
fn loot_item_new_returns_invalid_division_when_denominator_is_zero() {
    let result = LootItem::new(ItemId::CopperOre, 1, 0, 1..=1);
    assert!(matches!(result, Err(LootError::InvalidDivision)));
}

#[test]
fn loot_item_new_returns_invalid_division_when_denominator_less_than_numerator() {
    let result = LootItem::new(ItemId::CopperOre, 5, 3, 1..=1);
    assert!(matches!(result, Err(LootError::InvalidDivision)));
}

#[test]
fn loot_item_new_accepts_equal_numerator_and_denominator() {
    // 100% chance (1/1) should be valid
    let result = LootItem::new(ItemId::CopperOre, 1, 1, 1..=1);
    assert!(result.is_ok());
}

#[test]
fn loot_item_new_accepts_zero_numerator() {
    // 0% chance (0/4) should be valid
    let result = LootItem::new(ItemId::CopperOre, 0, 4, 1..=1);
    assert!(result.is_ok());
}

// ==================== LootTable::new() tests ====================

#[test]
fn loot_table_new_creates_empty_table() {
    let table = LootTable::new();
    // Check that it's empty by verifying ore_proportions returns no items
    assert_eq!(table.ore_proportions().count(), 0);
}

// ==================== LootTable::with() builder tests ====================

#[test]
fn loot_table_with_chains_loot_item_additions() {
    let table = LootTable::new()
        .with(ItemId::CopperOre, 1, 4, 1..=1)
        .with(ItemId::TinOre, 1, 8, 1..=2);

    assert_eq!(table.ore_proportions().count(), 2);
}

#[test]
fn loot_table_with_silently_ignores_invalid_items() {
    let table = LootTable::new()
        .with(ItemId::CopperOre, 1, 4, 1..=1)
        .with(ItemId::TinOre, 5, 3, 1..=1); // Invalid: numerator > denominator

    // Only the valid item should be added
    assert_eq!(table.ore_proportions().count(), 1);
}

#[test]
fn loot_table_with_ignores_duplicate_items() {
    let table = LootTable::new()
        .with(ItemId::CopperOre, 1, 4, 1..=1)
        .with(ItemId::CopperOre, 1, 8, 1..=2); // Duplicate item

    // Only one item should be added
    assert_eq!(table.ore_proportions().count(), 1);
}

// ==================== LootTable::add_loot_item() tests ====================

#[test]
fn loot_table_add_loot_item_adds_items_correctly() {
    let mut table = LootTable::new();
    let item = LootItem::new(ItemId::CopperOre, 1, 4, 1..=1).unwrap();

    let result = table.add_loot_item(item);
    assert!(result.is_ok());
    assert_eq!(table.ore_proportions().count(), 1);
}

#[test]
fn loot_table_add_loot_item_returns_item_already_in_table_for_duplicates() {
    let mut table = LootTable::new();
    let item1 = LootItem::new(ItemId::CopperOre, 1, 4, 1..=1).unwrap();
    let item2 = LootItem::new(ItemId::CopperOre, 1, 8, 1..=2).unwrap();

    table.add_loot_item(item1).unwrap();
    let result = table.add_loot_item(item2);

    assert!(matches!(result, Err(LootError::ItemAlreadyInTable)));
}

// ==================== LootTable::check_item_kind() tests ====================

#[test]
fn loot_table_check_item_kind_returns_true_when_item_exists() {
    let table = LootTable::new()
        .with(ItemId::CopperOre, 1, 4, 1..=1);

    assert!(table.check_item_kind(&ItemId::CopperOre));
}

#[test]
fn loot_table_check_item_kind_returns_false_when_item_missing() {
    let table = LootTable::new()
        .with(ItemId::CopperOre, 1, 4, 1..=1);

    assert!(!table.check_item_kind(&ItemId::TinOre));
}

#[test]
fn loot_table_check_item_kind_returns_false_for_empty_table() {
    let table = LootTable::new();
    assert!(!table.check_item_kind(&ItemId::CopperOre));
}

// ==================== LootTable::get_loot_item_from_kind() tests ====================

#[test]
fn loot_table_get_loot_item_from_kind_finds_item() {
    let table = LootTable::new()
        .with(ItemId::CopperOre, 1, 4, 1..=1);

    let result = table.get_loot_item_from_kind(&ItemId::CopperOre);
    assert!(result.is_some());
}

#[test]
fn loot_table_get_loot_item_from_kind_returns_none_when_missing() {
    let table = LootTable::new()
        .with(ItemId::CopperOre, 1, 4, 1..=1);

    let result = table.get_loot_item_from_kind(&ItemId::TinOre);
    assert!(result.is_none());
}

// ==================== LootTable::ore_proportions() tests ====================

#[test]
fn loot_table_ore_proportions_returns_correct_drop_chances() {
    let table = LootTable::new()
        .with(ItemId::CopperOre, 1, 4, 1..=1)   // 25% chance
        .with(ItemId::TinOre, 1, 2, 1..=1);    // 50% chance

    let proportions: Vec<(ItemId, f32)> = table.ore_proportions().collect();

    assert_eq!(proportions.len(), 2);

    // Find each item and check its chance
    let copper = proportions.iter().find(|(id, _)| *id == ItemId::CopperOre);
    let tin = proportions.iter().find(|(id, _)| *id == ItemId::TinOre);

    assert!(copper.is_some());
    assert!((copper.unwrap().1 - 0.25).abs() < 0.001);

    assert!(tin.is_some());
    assert!((tin.unwrap().1 - 0.5).abs() < 0.001);
}

#[test]
fn loot_table_ore_proportions_returns_empty_for_empty_table() {
    let table = LootTable::new();
    assert_eq!(table.ore_proportions().count(), 0);
}

// ==================== LootTable::roll_drops_with_spawner() tests ====================

#[test]
fn loot_table_roll_drops_empty_table_returns_no_drops() {
    let table = LootTable::new();
    let drops = table.roll_drops_with_spawner(0, mock_spawn_item);
    assert!(drops.is_empty());
}

#[test]
fn loot_table_roll_drops_100_percent_always_drops() {
    // 100% drop chance (1/1) should always drop
    let table = LootTable::new()
        .with(ItemId::CopperOre, 1, 1, 1..=1);

    // Run 100 times to ensure it always drops
    for _ in 0..100 {
        let drops = table.roll_drops_with_spawner(0, mock_spawn_item);
        assert_eq!(drops.len(), 1);
        assert_eq!(drops[0].item.item_id, ItemId::CopperOre);
    }
}

#[test]
fn loot_table_roll_drops_0_percent_never_drops() {
    // 0% drop chance (0/4) should never drop
    let table = LootTable::new()
        .with(ItemId::CopperOre, 0, 4, 1..=1);

    // Run 100 times to ensure it never drops
    for _ in 0..100 {
        let drops = table.roll_drops_with_spawner(0, mock_spawn_item);
        assert!(drops.is_empty());
    }
}

#[test]
fn loot_table_roll_drops_probability_statistical_test() {
    // 50% drop chance (1/2)
    let table = LootTable::new()
        .with(ItemId::CopperOre, 1, 2, 1..=1);

    let iterations = 1000;
    let mut drop_count = 0;

    for _ in 0..iterations {
        let drops = table.roll_drops_with_spawner(0, mock_spawn_item);
        if !drops.is_empty() {
            drop_count += 1;
        }
    }

    // Should be roughly 50%, allow ±10% tolerance for randomness
    let drop_rate = drop_count as f64 / iterations as f64;
    assert!(
        drop_rate > 0.40 && drop_rate < 0.60,
        "Drop rate {} should be roughly 50%", drop_rate
    );
}

#[test]
fn loot_table_roll_drops_quantity_within_range() {
    // 100% drop, quantity 1-5
    let table = LootTable::new()
        .with(ItemId::CopperOre, 1, 1, 1..=5);

    for _ in 0..100 {
        let drops = table.roll_drops_with_spawner(0, mock_spawn_item);
        assert_eq!(drops.len(), 1);
        assert!(drops[0].quantity >= 1 && drops[0].quantity <= 5);
    }
}

#[test]
fn loot_table_roll_drops_single_quantity_always_returns_that_value() {
    // 100% drop, quantity always 3
    let table = LootTable::new()
        .with(ItemId::CopperOre, 1, 1, 3..=3);

    for _ in 0..50 {
        let drops = table.roll_drops_with_spawner(0, mock_spawn_item);
        assert_eq!(drops.len(), 1);
        assert_eq!(drops[0].quantity, 3);
    }
}

#[test]
fn loot_table_roll_drops_multiple_items_roll_independently() {
    // Both items have 100% drop
    let table = LootTable::new()
        .with(ItemId::CopperOre, 1, 1, 1..=1)
        .with(ItemId::TinOre, 1, 1, 1..=1);

    let drops = table.roll_drops_with_spawner(0, mock_spawn_item);
    assert_eq!(drops.len(), 2);
}

#[test]
fn loot_table_roll_drops_handles_spawn_failure() {
    let table = LootTable::new()
        .with(ItemId::CopperOre, 1, 1, 1..=1);

    // Spawn function that returns None
    let drops = table.roll_drops_with_spawner(0, |_| None);
    assert!(drops.is_empty());
}

// ==================== LootDrop tests ====================

#[test]
fn loot_drop_contains_spawned_item_with_quantity() {
    let table = LootTable::new()
        .with(ItemId::CopperOre, 1, 1, 2..=2);

    let drops = table.roll_drops_with_spawner(0, mock_spawn_item);

    assert_eq!(drops.len(), 1);
    assert_eq!(drops[0].item.item_id, ItemId::CopperOre);
    assert_eq!(drops[0].quantity, 2);
}

// ==================== WorthGold trait tests (Item implementation) ====================

#[test]
fn worth_gold_gold_value_returns_base_value_for_normal_quality() {
    let item = create_test_material(ItemId::CopperOre, 100);
    // Normal quality has 1.0 value multiplier
    assert_eq!(item.gold_value(), 100);
}

#[test]
fn worth_gold_gold_value_applies_quality_multiplier() {
    let mut item = create_test_material(ItemId::CopperOre, 100);
    item.quality = ItemQuality::Mythic; // 1.4x value multiplier

    assert_eq!(item.gold_value(), 140);
}

#[test]
fn worth_gold_purchase_price_equals_gold_value() {
    let item = create_test_material(ItemId::CopperOre, 100);
    assert_eq!(item.purchase_price(), item.gold_value());
}

#[test]
fn worth_gold_sell_price_is_half_gold_value() {
    let item = create_test_material(ItemId::CopperOre, 100);
    assert_eq!(item.sell_price(), 50);
}

#[test]
fn worth_gold_sell_price_rounds_down_for_odd_values() {
    let item = create_test_material(ItemId::CopperOre, 101);
    // 101 / 2 = 50 (integer division)
    assert_eq!(item.sell_price(), 50);
}

// ==================== Magic Find bonus tests ====================

#[test]
fn loot_table_roll_drops_magic_find_zero_no_bonus() {
    // 100% drop rate, we verify behavior is consistent
    let table = LootTable::new()
        .with(ItemId::CopperOre, 1, 1, 1..=1);

    // With 0 magic find, should still work normally
    for _ in 0..10 {
        let drops = table.roll_drops_with_spawner(0, mock_spawn_item);
        assert_eq!(drops.len(), 1);
    }
}

#[test]
fn loot_table_roll_drops_magic_find_increases_chances() {
    // Low drop rate (10% = 1/10) with high magic find
    let table = LootTable::new()
        .with(ItemId::CopperOre, 1, 10, 1..=1);

    let iterations = 1000;

    // Count drops with 0 magic find
    let mut drops_no_mf = 0;
    for _ in 0..iterations {
        let drops = table.roll_drops_with_spawner(0, mock_spawn_item);
        if !drops.is_empty() {
            drops_no_mf += 1;
        }
    }

    // Count drops with 200 magic find (2 guaranteed bonus rolls)
    let mut drops_with_mf = 0;
    for _ in 0..iterations {
        let drops = table.roll_drops_with_spawner(200, mock_spawn_item);
        if !drops.is_empty() {
            drops_with_mf += 1;
        }
    }

    // With 200 magic find, drop rate should be noticeably higher
    // 3 rolls at 10% each = ~27.1% chance of at least one success
    // vs 10% with no magic find
    assert!(
        drops_with_mf > drops_no_mf,
        "Magic find should increase drops: {} with MF vs {} without",
        drops_with_mf, drops_no_mf
    );
}
