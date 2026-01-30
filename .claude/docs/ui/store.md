# Store Module

Core store logic lives in `src/location/store/`.

## Store Resource (`src/location/store/definition.rs`)

```rust
#[derive(Debug, Resource)]
pub struct Store {
    pub name: String,
    pub inventory: Vec<StoreItem>,
    refresh_timer: Timer,
    // ...
}
```

### Creating a Store

`Store::new` accepts initial stock as a vector of `(ItemId, quantity)` tuples:

```rust
let store = Store::new("Village Store", vec![
    (ItemId::BasicHPPotion, 5),
    (ItemId::Sword, 3),
    (ItemId::BasicShield, 3),
]);
```

Items are spawned immediately in `StoreItem::new`:
- Equipment items stock 1 at a time (regardless of max_quantity)
- Consumables stock the full quantity

## StoreItem (`src/location/store/store_item.rs`)

```rust
pub struct StoreItem {
    pub item_id: ItemId,
    pub items: Vec<Item>,      // Actual items in stock
    pub max_quantity: i32,
}

impl StoreItem {
    pub fn new(item_id: ItemId, quantity: i32) -> Self  // Spawns items immediately
    pub fn quantity(&self) -> i32
    pub fn is_in_stock(&self) -> bool
    pub fn take_item(&mut self) -> Option<Item>         // For purchasing
    pub fn restock(&mut self)                           // Respawn up to max_quantity
    pub fn display_item(&self) -> Option<&Item>         // For UI display
}
```

## Events (`src/location/store/events.rs`)

```rust
#[derive(Event)]
pub struct PurchaseEvent {
    pub index: usize,  // Index into store.inventory
}

#[derive(Event)]
pub struct SellEvent {
    pub inventory_index: usize,
}

pub enum TransactionResult {
    PurchaseSuccess { item_name: String, price: i32 },
    PurchaseFailedInsufficientGold { need: i32, have: i32 },
    PurchaseFailedInventoryFull,
    PurchaseFailedOutOfStock,
    SellSuccess { item_name: String, price: i32 },
}
```

Handlers use `run_if(on_event::<T>)` pattern per Bevy idioms.

## StorePlugin (`src/location/store/mod.rs`)

Registers events, handlers, and initializes the store resource:

```rust
impl Plugin for StorePlugin {
    fn build(&self, app: &mut App) {
        let store = Store::new("Village Store", vec![
            (ItemId::BasicHPPotion, 5),
            (ItemId::Sword, 3),
        ]);

        app.insert_resource(store)
            .add_event::<PurchaseEvent>()
            .add_event::<SellEvent>()
            .add_event::<TransactionResult>()
            .add_systems(Update, (
                handle_purchase.run_if(on_event::<PurchaseEvent>),
                handle_sell.run_if(on_event::<SellEvent>),
            ));
    }
}
```

## Purchasing and Selling

### Purchase Flow (`Store::purchase_item`)
1. Validates index
2. Takes item from `StoreItem.items`
3. Applies store discount from passive effects
4. Checks player gold
5. Adds to player inventory
6. Deducts gold

### Sell Flow (`sell_player_item`)
1. Checks if item is locked (cannot sell locked items)
2. Calculates sell price (50% of gold_value)
3. Adds gold to player
4. Decreases item quantity in inventory

## Key Files

- `src/location/store/definition.rs` - Store struct, purchase_item, sell_player_item
- `src/location/store/store_item.rs` - StoreItem with items vector
- `src/location/store/events.rs` - PurchaseEvent, SellEvent, TransactionResult, handlers
- `src/location/store/mod.rs` - StorePlugin
- `src/location/store/traits.rs` - Location, Refreshable implementations
- `src/location/store/tests.rs` - 31 tests covering store functionality
