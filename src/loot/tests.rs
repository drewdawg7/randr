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
        name: "Test Material".to_string(),
        is_equipped: false,
        is_locked: false,
        num_upgrades: 0,
        max_upgrades: 0,
        max_stack_quantity: 99,
        base_stats: StatSheet::new(),
        stats: StatSheet::new(),
        gold_value,
        quality: ItemQuality::Normal,
    }
}

#[cfg(test)]
fn mock_spawn_item(id: ItemId) -> Option<Item> {
    Some(create_test_material(id, 10))
}

// ==================== LootItem::new() tests ====================

#[test]
fn loot_item_new_creates_valid_loot_item() {
    let result = LootItem::new(ItemId::IronOre, 1, 4, 1..=3);
    assert!(result.is_ok());
}

#[test]
fn loot_item_new_returns_invalid_division_when_denominator_is_zero() {
    let result = LootItem::new(ItemId::IronOre, 1, 0, 1..=1);
    assert!(matches!(result, Err(LootError::InvalidDivision)));
}

#[test]
fn loot_item_new_returns_invalid_division_when_denominator_less_than_numerator() {
    let result = LootItem::new(ItemId::IronOre, 5, 3, 1..=1);
    assert!(matches!(result, Err(LootError::InvalidDivision)));
}

#[test]
fn loot_item_new_accepts_equal_numerator_and_denominator() {
    // 100% chance (1/1) should be valid
    let result = LootItem::new(ItemId::IronOre, 1, 1, 1..=1);
    assert!(result.is_ok());
}

#[test]
fn loot_item_new_accepts_zero_numerator() {
    // 0% chance (0/4) should be valid
    let result = LootItem::new(ItemId::IronOre, 0, 4, 1..=1);
    assert!(result.is_ok());
}

// ==================== LootTable::new() tests ====================

#[test]
fn loot_table_new_creates_empty_table() {
    let table = LootTable::new().build();
    assert_eq!(table.ore_proportions().count(), 0);
}

// ==================== LootTable::with() builder tests ====================

#[test]
fn loot_table_with_chains_loot_item_additions() {
    let table = LootTable::new()
        .with(ItemId::IronOre, 1, 4, 1..=1)
        .with(ItemId::GoldOre, 1, 8, 1..=2)
        .build();

    assert_eq!(table.ore_proportions().count(), 2);
}

#[test]
fn loot_table_with_silently_ignores_invalid_items() {
    let table = LootTable::new()
        .with(ItemId::IronOre, 1, 4, 1..=1)
        .with(ItemId::GoldOre, 5, 3, 1..=1)
        .build();

    assert_eq!(table.ore_proportions().count(), 1);
}

#[test]
fn loot_table_with_ignores_duplicate_items() {
    let table = LootTable::new()
        .with(ItemId::IronOre, 1, 4, 1..=1)
        .with(ItemId::IronOre, 1, 8, 1..=2)
        .build();

    assert_eq!(table.ore_proportions().count(), 1);
}

// ==================== LootTable::add_loot_item() tests ====================

#[test]
fn loot_table_add_loot_item_adds_items_correctly() {
    let mut table = LootTable::new().build();
    let item = LootItem::new(ItemId::IronOre, 1, 4, 1..=1).unwrap();

    let result = table.add_loot_item(item);
    assert_eq!(result, Ok(0));
    assert_eq!(table.ore_proportions().count(), 1);

    let item2 = LootItem::new(ItemId::GoldOre, 1, 4, 1..=1).unwrap();
    let result2 = table.add_loot_item(item2);
    assert_eq!(result2, Ok(1));
    assert_eq!(table.ore_proportions().count(), 2);
}

#[test]
fn loot_table_add_loot_item_returns_item_already_in_table_for_duplicates() {
    let mut table = LootTable::new().build();
    let item1 = LootItem::new(ItemId::IronOre, 1, 4, 1..=1).unwrap();
    let item2 = LootItem::new(ItemId::IronOre, 1, 8, 1..=2).unwrap();

    table.add_loot_item(item1).unwrap();
    let result = table.add_loot_item(item2);

    assert!(matches!(result, Err(LootError::ItemAlreadyInTable)));
}

// ==================== LootTable::check_item_kind() tests ====================

#[test]
fn loot_table_check_item_kind_returns_true_when_item_exists() {
    let table = LootTable::new()
        .with(ItemId::IronOre, 1, 4, 1..=1)
        .build();

    assert!(table.check_item_kind(&ItemId::IronOre));
}

#[test]
fn loot_table_check_item_kind_returns_false_when_item_missing() {
    let table = LootTable::new()
        .with(ItemId::IronOre, 1, 4, 1..=1)
        .build();

    assert!(!table.check_item_kind(&ItemId::GoldOre));
}

#[test]
fn loot_table_check_item_kind_returns_false_for_empty_table() {
    let table = LootTable::new().build();
    assert!(!table.check_item_kind(&ItemId::IronOre));
}

// ==================== LootTable::get_loot_item_from_kind() tests ====================

#[test]
fn loot_table_get_loot_item_from_kind_finds_item() {
    let table = LootTable::new()
        .with(ItemId::IronOre, 1, 4, 1..=1)
        .build();

    let result = table.get_loot_item_from_kind(&ItemId::IronOre);
    assert!(result.is_some());
}

#[test]
fn loot_table_get_loot_item_from_kind_returns_none_when_missing() {
    let table = LootTable::new()
        .with(ItemId::IronOre, 1, 4, 1..=1)
        .build();

    let result = table.get_loot_item_from_kind(&ItemId::GoldOre);
    assert!(result.is_none());
}

// ==================== LootTable::ore_proportions() tests ====================

#[test]
fn loot_table_ore_proportions_returns_correct_drop_chances() {
    let table = LootTable::new()
        .with(ItemId::IronOre, 1, 4, 1..=1)
        .with(ItemId::GoldOre, 1, 2, 1..=1)
        .build();

    let proportions: Vec<(ItemId, f32)> = table.ore_proportions().collect();

    assert_eq!(proportions.len(), 2);

    // Find each item and check its chance
    let copper = proportions.iter().find(|(id, _)| *id == ItemId::IronOre);
    let tin = proportions.iter().find(|(id, _)| *id == ItemId::GoldOre);

    assert!(copper.is_some());
    assert!((copper.unwrap().1 - 0.25).abs() < 0.001);

    assert!(tin.is_some());
    assert!((tin.unwrap().1 - 0.5).abs() < 0.001);
}

#[test]
fn loot_table_ore_proportions_returns_empty_for_empty_table() {
    let table = LootTable::new().build();
    assert_eq!(table.ore_proportions().count(), 0);
}

// ==================== LootTable::roll_drops_with_spawner() tests ====================

#[test]
fn loot_table_roll_drops_empty_table_returns_no_drops() {
    let table = LootTable::new().build();
    let drops = table.roll_drops_with_spawner(0, mock_spawn_item);
    assert!(drops.is_empty());
}

#[test]
fn loot_table_roll_drops_100_percent_always_drops() {
    let table = LootTable::new()
        .with(ItemId::IronOre, 1, 1, 1..=1)
        .build();

    for _ in 0..100 {
        let drops = table.roll_drops_with_spawner(0, mock_spawn_item);
        assert_eq!(drops.len(), 1);
        assert_eq!(drops[0].item.item_id, ItemId::IronOre);
    }
}

#[test]
fn loot_table_roll_drops_0_percent_never_drops() {
    let table = LootTable::new()
        .with(ItemId::IronOre, 0, 4, 1..=1)
        .build();

    for _ in 0..100 {
        let drops = table.roll_drops_with_spawner(0, mock_spawn_item);
        assert!(drops.is_empty());
    }
}

#[test]
fn loot_table_roll_drops_probability_statistical_test() {
    let table = LootTable::new()
        .with(ItemId::IronOre, 1, 2, 1..=1)
        .build();

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
    let table = LootTable::new()
        .with(ItemId::IronOre, 1, 1, 1..=5)
        .build();

    for _ in 0..100 {
        let drops = table.roll_drops_with_spawner(0, mock_spawn_item);
        assert_eq!(drops.len(), 1);
        assert!(drops[0].quantity >= 1 && drops[0].quantity <= 5);
    }
}

#[test]
fn loot_table_roll_drops_single_quantity_always_returns_that_value() {
    let table = LootTable::new()
        .with(ItemId::IronOre, 1, 1, 3..=3)
        .build();

    for _ in 0..50 {
        let drops = table.roll_drops_with_spawner(0, mock_spawn_item);
        assert_eq!(drops.len(), 1);
        assert_eq!(drops[0].quantity, 3);
    }
}

#[test]
fn loot_table_roll_drops_multiple_items_roll_independently() {
    let table = LootTable::new()
        .with(ItemId::IronOre, 1, 1, 1..=1)
        .with(ItemId::GoldOre, 1, 1, 1..=1)
        .build();

    let drops = table.roll_drops_with_spawner(0, mock_spawn_item);
    assert_eq!(drops.len(), 2);
}

#[test]
fn loot_table_roll_drops_handles_spawn_failure() {
    let table = LootTable::new()
        .with(ItemId::IronOre, 1, 1, 1..=1)
        .build();

    let drops = table.roll_drops_with_spawner(0, |_| None);
    assert!(drops.is_empty());
}

// ==================== LootDrop tests ====================

#[test]
fn loot_drop_contains_spawned_item_with_quantity() {
    let table = LootTable::new()
        .with(ItemId::IronOre, 1, 1, 2..=2)
        .build();

    let drops = table.roll_drops_with_spawner(0, mock_spawn_item);

    assert_eq!(drops.len(), 1);
    assert_eq!(drops[0].item.item_id, ItemId::IronOre);
    assert_eq!(drops[0].quantity, 2);
}

// ==================== WorthGold trait tests (Item implementation) ====================

#[test]
fn worth_gold_gold_value_returns_base_value_for_normal_quality() {
    let item = create_test_material(ItemId::IronOre, 100);
    // Normal quality has 1.0 value multiplier
    assert_eq!(item.gold_value(), 100);
}

#[test]
fn worth_gold_gold_value_applies_quality_multiplier() {
    let mut item = create_test_material(ItemId::IronOre, 100);
    item.quality = ItemQuality::Mythic; // 1.4x value multiplier

    assert_eq!(item.gold_value(), 140);
}

#[test]
fn worth_gold_purchase_price_equals_gold_value() {
    let item = create_test_material(ItemId::IronOre, 100);
    assert_eq!(item.purchase_price(), item.gold_value());
}

#[test]
fn worth_gold_sell_price_is_half_gold_value() {
    let item = create_test_material(ItemId::IronOre, 100);
    assert_eq!(item.sell_price(), 50);
}

#[test]
fn worth_gold_sell_price_rounds_down_for_odd_values() {
    let item = create_test_material(ItemId::IronOre, 101);
    // 101 / 2 = 50 (integer division)
    assert_eq!(item.sell_price(), 50);
}

// ==================== Magic Find bonus tests ====================

#[test]
fn loot_table_roll_drops_magic_find_zero_no_bonus() {
    let table = LootTable::new()
        .with(ItemId::IronOre, 1, 1, 1..=1)
        .build();

    for _ in 0..10 {
        let drops = table.roll_drops_with_spawner(0, mock_spawn_item);
        assert_eq!(drops.len(), 1);
    }
}

#[test]
fn loot_table_roll_drops_magic_find_increases_chances() {
    let table = LootTable::new()
        .with(ItemId::IronOre, 1, 10, 1..=1)
        .build();

    let iterations = 1000;

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
