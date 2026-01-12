# Event Handler Systems

## Overview
Event handler systems process Bevy events (e.g., `UpgradeItemEvent`, `StorePurchaseEvent`). These systems live in `src/game/` and handle game logic triggered by UI or other systems.

## Required Pattern: run_if with on_event

**All event handler systems MUST use `run_if(on_event::<EventType>)` to avoid running every frame.**

### Correct
```rust
app.add_event::<MyEvent>()
    .add_systems(Update, handle_my_event.run_if(on_event::<MyEvent>));
```

### Incorrect
```rust
// BAD: Runs every frame even when no events exist
app.add_event::<MyEvent>()
    .add_systems(Update, handle_my_event);
```

## Examples

### Single Event Handler
From `src/game/crafting.rs`:
```rust
app.add_event::<BrewPotionEvent>()
    .add_event::<BrewingResult>()
    .add_systems(Update, handle_brew_potion.run_if(on_event::<BrewPotionEvent>));
```

### Multiple Event Handlers
From `src/game/blacksmith.rs`:
```rust
app.add_event::<UpgradeItemEvent>()
    .add_event::<UpgradeQualityEvent>()
    .add_event::<SmeltRecipeEvent>()
    .add_event::<ForgeRecipeEvent>()
    .add_event::<BlacksmithResult>()
    .add_systems(
        Update,
        (
            handle_upgrade_item.run_if(on_event::<UpgradeItemEvent>),
            handle_upgrade_quality.run_if(on_event::<UpgradeQualityEvent>),
            handle_smelt_recipe.run_if(on_event::<SmeltRecipeEvent>),
            handle_forge_recipe.run_if(on_event::<ForgeRecipeEvent>),
        ),
    );
```

From `src/game/store_transactions.rs`:
```rust
app.add_event::<StorePurchaseEvent>()
    .add_event::<StoreSellEvent>()
    .add_event::<StorageWithdrawEvent>()
    .add_event::<StorageDepositEvent>()
    .add_event::<StoreTransactionResult>()
    .add_systems(
        Update,
        (
            handle_store_purchase.run_if(on_event::<StorePurchaseEvent>),
            handle_store_sell.run_if(on_event::<StoreSellEvent>),
            handle_storage_withdraw.run_if(on_event::<StorageWithdrawEvent>),
            handle_storage_deposit.run_if(on_event::<StorageDepositEvent>),
        ),
    );
```

## Event Handler Files
- `src/game/blacksmith.rs` - Item upgrades, smelting, forging
- `src/game/crafting.rs` - Potion brewing
- `src/game/store_transactions.rs` - Store purchases, sales, storage operations

## Note on Result Events
Result events (e.g., `BlacksmithResult`, `BrewingResult`, `StoreTransactionResult`) are written by handler systems but read by UI systems. The UI listeners typically use `EventReader` directly since they need to display feedback.
