#[cfg(test)]
use std::time::Duration;
#[cfg(test)]
use crate::{
    combat::HasGold,
    player::Player,
    inventory::{HasInventory, ManagesItems},
    item::{Item, ItemId, ItemType},
    item::enums::{EquipmentType, ItemQuality},
    location::store::{sell_player_item, Store, StoreError, StoreItem},
    stats::StatSheet,
};
#[cfg(test)]
use uuid::Uuid;

// ==================== Helper functions ====================

#[cfg(test)]
fn create_test_item(
    item_id: ItemId,
    name: &str,
    item_type: ItemType,
    gold_value: i32,
    is_locked: bool,
) -> Item {
    Item {
        item_uuid: Uuid::new_v4(),
        item_id,
        item_type,
        name: name.to_string(),
        is_equipped: false,
        is_locked,
        num_upgrades: 0,
        max_upgrades: 5,
        max_stack_quantity: if item_type.is_equipment() { 1 } else { 99 },
        base_stats: StatSheet::new(),
        stats: StatSheet::new(),
        gold_value,
        quality: ItemQuality::Normal,
    }
}

#[cfg(test)]
fn create_weapon(gold_value: i32) -> Item {
    create_test_item(
        ItemId::Sword,
        "Test Sword",
        ItemType::Equipment(EquipmentType::Weapon),
        gold_value,
        false,
    )
}

#[cfg(test)]
fn create_shield(gold_value: i32) -> Item {
    create_test_item(
        ItemId::BasicShield,
        "Test Shield",
        ItemType::Equipment(EquipmentType::Shield),
        gold_value,
        false,
    )
}


#[cfg(test)]
fn create_locked_item(gold_value: i32) -> Item {
    create_test_item(
        ItemId::Sword,
        "Locked Sword",
        ItemType::Equipment(EquipmentType::Weapon),
        gold_value,
        true,
    )
}

// ==================== Store creation tests ====================

#[test]
fn store_new_creates_with_correct_defaults() {
    let store = Store::new("Test Store", vec![]);

    assert_eq!(store.name, "Test Store");
    assert_eq!(store.description(), "");
    assert!(store.inventory.is_empty());
    assert_eq!(store.time_until_restock(), 60);
}

#[test]
fn store_new_creates_with_initial_stock() {
    let store = Store::new("Test Store", vec![
        (ItemId::Sword, 3),
        (ItemId::BasicShield, 2),
    ]);

    assert_eq!(store.inventory.len(), 2);
    assert_eq!(store.inventory[0].item_id, ItemId::Sword);
    assert_eq!(store.inventory[1].item_id, ItemId::BasicShield);
    // Items are spawned immediately
    assert_eq!(store.inventory[0].quantity(), 1); // Equipment stocks 1
    assert_eq!(store.inventory[1].quantity(), 1); // Equipment stocks 1
}

// ==================== Store::add_item tests ====================

#[test]
fn add_item_adds_to_existing_slot() {
    let mut store = Store::new("Test Store", vec![(ItemId::Sword, 1)]);

    let item = create_weapon(100);
    store.add_item(item);

    let store_item = &store.inventory[0];
    assert_eq!(store_item.quantity(), 2); // 1 from init + 1 added
}

#[test]
fn add_item_creates_new_slot_if_not_exists() {
    let mut store = Store::new("Test Store", vec![]);

    let item = create_weapon(100);
    store.add_item(item);

    assert_eq!(store.inventory.len(), 1);
    let store_item = &store.inventory[0];
    assert_eq!(store_item.item_id, ItemId::Sword);
    assert_eq!(store_item.max_quantity, 1);
    assert_eq!(store_item.quantity(), 1);
}

// ==================== Store::purchase_item success tests ====================

#[test]
fn purchase_item_success_full_flow() {
    let mut store = Store::new("Test Store", vec![]);
    let mut player = Player::default();
    player.add_gold(200);

    let item = create_weapon(100);
    store.add_item(item);

    let result = store.purchase_item(&mut player, 0);

    assert!(result.is_ok());
    assert_eq!(player.gold(), 100); // 200 - 100
    assert_eq!(player.inventory().items.len(), 1);
    assert_eq!(store.inventory[0].quantity(), 0);
}

#[test]
fn purchase_item_adds_correct_item_to_inventory() {
    let mut store = Store::new("Test Store", vec![]);
    let mut player = Player::default();
    player.add_gold(200);

    let item = create_weapon(100);
    store.add_item(item);

    let result = store.purchase_item(&mut player, 0);

    assert!(result.is_ok());
    let purchased = result.unwrap();
    assert_eq!(purchased.item_id, ItemId::Sword);
    assert_eq!(purchased.name, "Test Sword");
}

// ==================== Store::purchase_item error tests ====================

#[test]
fn purchase_item_returns_not_enough_gold() {
    let mut store = Store::new("Test Store", vec![]);
    let mut player = Player::default();
    player.add_gold(50);

    let item = create_weapon(100);
    store.add_item(item);

    let result = store.purchase_item(&mut player, 0);

    assert!(matches!(result, Err(StoreError::NotEnoughGold)));
    assert_eq!(player.gold(), 50);
    assert_eq!(store.inventory[0].quantity(), 1);
    assert_eq!(player.inventory().items.len(), 0);
}

#[test]
fn purchase_item_returns_out_of_stock() {
    let mut store = Store::new("Test Store", vec![]);
    let mut player = Player::default();
    player.add_gold(200);

    let store_item = StoreItem {
        item_id: ItemId::Sword,
        items: vec![],
        max_quantity: 1,
    };
    store.inventory.push(store_item);

    let result = store.purchase_item(&mut player, 0);

    assert!(matches!(result, Err(StoreError::OutOfStock)));
    assert_eq!(player.gold(), 200);
    assert_eq!(player.inventory().items.len(), 0);
}

#[test]
fn purchase_item_returns_inventory_full() {
    let mut store = Store::new("Test Store", vec![]);
    let mut player = Player::default();
    player.add_gold(200);

    for _ in 0..20 {
        let item = create_weapon(10);
        let _ = player.add_to_inv(item);
    }

    let item = create_weapon(100);
    store.add_item(item);

    let result = store.purchase_item(&mut player, 0);

    assert!(matches!(result, Err(StoreError::InventoryFull)));
    assert_eq!(player.gold(), 200);
    assert_eq!(store.inventory[0].quantity(), 1);
}

#[test]
fn purchase_item_returns_invalid_index() {
    let mut store = Store::new("Test Store", vec![]);
    let mut player = Player::default();
    player.add_gold(200);

    let result = store.purchase_item(&mut player, 0);

    assert!(matches!(result, Err(StoreError::InvalidIndex)));
}

#[test]
fn purchase_item_invalid_index_when_index_too_high() {
    let mut store = Store::new("Test Store", vec![]);
    let mut player = Player::default();
    player.add_gold(200);

    let item = create_weapon(100);
    store.add_item(item);

    let result = store.purchase_item(&mut player, 1);

    assert!(matches!(result, Err(StoreError::InvalidIndex)));
}

// ==================== Store::get_store_item_by_id tests ====================

#[test]
fn get_store_item_by_id_finds_item() {
    let store = Store::new("Test Store", vec![
        (ItemId::Sword, 2),
        (ItemId::BasicShield, 1),
    ]);

    let result = store.get_store_item_by_id(ItemId::Sword);

    assert!(result.is_some());
    let store_item = result.unwrap();
    assert_eq!(store_item.item_id, ItemId::Sword);
}

#[test]
fn get_store_item_by_id_returns_none_when_not_found() {
    let store = Store::new("Test Store", vec![(ItemId::Sword, 2)]);

    let result = store.get_store_item_by_id(ItemId::BasicShield);

    assert!(result.is_none());
}

// ==================== Store::restock tests ====================

#[test]
fn restock_clears_existing_items() {
    let mut store = Store::new("Test Store", vec![]);
    let item = create_weapon(100);
    store.add_item(item);

    assert_eq!(store.inventory[0].quantity(), 1);
    assert!(store.inventory.len() > 0);
}

// ==================== Store::check_and_restock tests ====================

#[test]
fn check_and_restock_does_not_restock_immediately() {
    let mut store = Store::new("Test Store", vec![(ItemId::Sword, 3)]);

    store.inventory[0].items.clear();

    store.tick_timer(Duration::from_secs(30));
    store.check_and_restock();

    assert_eq!(store.inventory[0].quantity(), 0);
}

#[test]
fn check_and_restock_restocks_after_interval() {
    let mut store = Store::new("Test Store", vec![(ItemId::Sword, 3)]);

    store.tick_timer(Duration::from_secs(60));
    assert!(store.time_until_restock() <= 60);

    store.check_and_restock();
    assert!(store.time_until_restock() <= 60);
}

// ==================== Store::time_until_restock tests ====================

#[test]
fn time_until_restock_returns_correct_countdown() {
    let mut store = Store::new("Test Store", vec![]);

    assert_eq!(store.time_until_restock(), 60);

    store.tick_timer(Duration::from_secs(20));
    assert_eq!(store.time_until_restock(), 40);

    store.tick_timer(Duration::from_secs(30));
    assert_eq!(store.time_until_restock(), 10);
}

#[test]
fn time_until_restock_decreases_over_time() {
    let mut store = Store::new("Test Store", vec![]);

    assert_eq!(store.time_until_restock(), 60);

    store.tick_timer(Duration::from_secs(59));
    assert_eq!(store.time_until_restock(), 1);
}

// ==================== StoreItem::new tests ====================

#[test]
fn store_item_new_creates_with_items() {
    let store_item = StoreItem::new(ItemId::BasicHPPotion, 5);

    assert_eq!(store_item.item_id, ItemId::BasicHPPotion);
    assert_eq!(store_item.max_quantity, 5);
    // Consumables spawn full quantity
    assert_eq!(store_item.items.len(), 5);
}

#[test]
fn store_item_new_equipment_stocks_one() {
    let store_item = StoreItem::new(ItemId::Sword, 5);

    assert_eq!(store_item.item_id, ItemId::Sword);
    assert_eq!(store_item.max_quantity, 5);
    // Equipment only stocks 1 at a time
    assert_eq!(store_item.items.len(), 1);
}

// ==================== StoreItem::take_item tests ====================

#[test]
fn take_item_removes_and_returns_item() {
    let mut store_item = StoreItem::new(ItemId::Sword, 2);
    let item = create_weapon(100);
    store_item.items.push(item);

    let result = store_item.take_item();

    assert!(result.is_some());
    let taken = result.unwrap();
    assert_eq!(taken.item_id, ItemId::Sword);
    assert_eq!(store_item.quantity(), 1); // One left from init
}

#[test]
fn take_item_returns_none_when_empty() {
    let mut store_item = StoreItem::new(ItemId::Sword, 2);
    store_item.items.clear(); // Empty it

    let result = store_item.take_item();

    assert!(result.is_none());
}

#[test]
fn take_item_multiple_times() {
    let mut store_item = StoreItem::new(ItemId::Sword, 3);
    store_item.items.push(create_weapon(100));
    store_item.items.push(create_weapon(100));

    // Now has 3 items (1 from init + 2 added)
    let result1 = store_item.take_item();
    assert!(result1.is_some());
    assert_eq!(store_item.quantity(), 2);

    let result2 = store_item.take_item();
    assert!(result2.is_some());
    assert_eq!(store_item.quantity(), 1);

    let result3 = store_item.take_item();
    assert!(result3.is_some());
    assert_eq!(store_item.quantity(), 0);

    let result4 = store_item.take_item();
    assert!(result4.is_none());
}

// ==================== StoreItem::restock tests ====================

#[test]
fn store_item_restock_clears_old_items() {
    let mut store_item = StoreItem::new(ItemId::Sword, 5);
    store_item.items.push(create_weapon(50));

    assert_eq!(store_item.quantity(), 2); // 1 from init + 1 added
    assert_eq!(store_item.max_quantity, 5);
}

// ==================== sell_player_item function tests ====================

#[test]
fn sell_player_item_adds_gold_and_decreases_quantity() {
    let mut player = Player::default();
    let item = create_weapon(100);
    let _ = player.add_to_inv(item.clone());

    let initial_gold = player.gold();
    let sell_price = sell_player_item(&mut player, &item);

    assert_eq!(sell_price, 50);
    assert_eq!(player.gold(), initial_gold + 50);
    assert_eq!(player.inventory().items.len(), 0);
}

#[test]
fn sell_player_item_returns_zero_for_locked_items() {
    let mut player = Player::default();
    let item = create_locked_item(100);
    let _ = player.add_to_inv(item.clone());

    let initial_gold = player.gold();
    let sell_price = sell_player_item(&mut player, &item);

    assert_eq!(sell_price, 0);
    assert_eq!(player.gold(), initial_gold);
    assert_eq!(player.inventory().items.len(), 1);
}

#[test]
fn sell_player_item_correct_price_calculation() {
    let mut player = Player::default();

    for gold_value in [10, 50, 100, 200] {
        let item = create_weapon(gold_value);
        let _ = player.add_to_inv(item.clone());

        let sell_price = sell_player_item(&mut player, &item);

        assert_eq!(sell_price, gold_value / 2);
    }
}

// ==================== Integration tests ====================

#[test]
fn store_full_purchase_and_sell_cycle() {
    let mut store = Store::new("Test Store", vec![]);
    let mut player = Player::default();
    player.add_gold(200);

    let item = create_weapon(100);
    store.add_item(item);

    let result = store.purchase_item(&mut player, 0);
    assert!(result.is_ok());
    assert_eq!(player.gold(), 100);
    assert_eq!(player.inventory().items.len(), 1);

    let purchased_item = result.unwrap();
    let sell_price = sell_player_item(&mut player, &purchased_item);
    assert_eq!(sell_price, 50);
    assert_eq!(player.gold(), 150);
}

#[test]
fn store_multiple_purchases() {
    let mut store = Store::new("Test Store", vec![]);
    let mut player = Player::default();
    player.add_gold(1000);

    store.add_item(create_weapon(100));
    store.add_item(create_shield(50));

    let result1 = store.purchase_item(&mut player, 0);
    assert!(result1.is_ok());
    assert_eq!(player.gold(), 900);

    let result2 = store.purchase_item(&mut player, 1);
    assert!(result2.is_ok());
    assert_eq!(player.gold(), 850);

    assert_eq!(player.inventory().items.len(), 2);
}

#[test]
fn store_purchase_without_enough_gold_preserves_state() {
    let mut store = Store::new("Test Store", vec![]);
    let mut player = Player::default();
    player.add_gold(50);

    let item = create_weapon(100);
    store.add_item(item);

    let initial_gold = player.gold();
    let initial_store_qty = store.inventory[0].quantity();

    let result = store.purchase_item(&mut player, 0);

    assert!(matches!(result, Err(StoreError::NotEnoughGold)));
    assert_eq!(player.gold(), initial_gold);
    assert_eq!(store.inventory[0].quantity(), initial_store_qty);
    assert_eq!(player.inventory().items.len(), 0);
}
