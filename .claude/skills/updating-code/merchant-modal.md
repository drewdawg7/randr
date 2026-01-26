# Merchant Modal

Modal that opens when pressing Space next to a Merchant NPC in the dungeon.

## Module Structure

```
src/ui/screens/merchant_modal/
  mod.rs         - Module exports
  state.rs       - Components, resources, RegisteredModal impl
  render.rs      - UI spawning, sync_merchant_grids, detail pane population
  input.rs       - Tab, navigation, buy/sell handlers
  plugin.rs      - Plugin registration
```

## Key Types

| Type | Location | Purpose |
|------|----------|---------|
| `MerchantModalRoot` | state.rs | Root marker component |
| `MerchantStockGrid` | state.rs | Marker for merchant's ItemGrid |
| `MerchantPlayerGrid` | state.rs | Marker for player's ItemGrid |
| `MerchantStock` | state.rs | Resource holding `Vec<StoreItem>` |
| `SpawnMerchantModal` | state.rs | Trigger resource |
| `MerchantDetailRefresh` | state.rs | Trigger resource for detail pane refresh |
| `MerchantModal` | state.rs | RegisteredModal implementation |

## Spawning the Modal

From dungeon plugin (`src/ui/screens/dungeon/plugin.rs`):

```rust
// In handle_mine_interaction, check for adjacent NPC
if let Some((_, _, mob_id)) = find_adjacent_npc(&state, &occupancy, &entity_query) {
    match mob_id {
        MobId::Merchant => {
            commands.insert_resource(MerchantStock::generate());
            commands.insert_resource(SpawnMerchantModal);
        }
        _ => {}
    }
}
```

## UI Layout

Uses `spawn_modal_overlay` directly (no Modal builder):
- Row container with 3 children:
  1. `ItemGrid` (5x5) with `MerchantStockGrid` marker - focused by default
  2. `ItemGrid` (5x5) with `MerchantPlayerGrid` marker
  3. `ItemDetailPane` showing selected item info

## Input Handling

| Action | Handler | Behavior |
|--------|---------|----------|
| Tab | `handle_merchant_modal_tab` | Toggle focus between grids |
| Arrows | `handle_merchant_modal_navigation` | Move selection in focused grid |
| Enter | `handle_merchant_modal_select` | Buy (stock focused) or sell (player focused) |
| Esc | `modal_close_system` | Close modal |

## Buy/Sell Logic

**Buying** (when MerchantStockGrid focused):
- Check player has enough gold
- Check inventory has space
- Deduct gold via `PlayerGold::subtract()`
- Add item via `inventory.add_to_inv()`
- Remove from `MerchantStock`

**Selling** (when MerchantPlayerGrid focused):
- Skip locked items
- Add gold via `PlayerGold::add()`
- Decrease quantity by 1 via `inventory.decrease_item_quantity()` (removes item when quantity reaches 0)

Grids update automatically via `sync_merchant_grids` which uses Bevy's change detection.

## Reactive Grid Sync

The `sync_merchant_grids` system uses Bevy's native change detection to automatically update grids:

```rust
pub fn sync_merchant_grids(
    inventory: Res<Inventory>,
    stock: Option<Res<MerchantStock>>,
    mut stock_grids: Query<&mut ItemGrid, (With<MerchantStockGrid>, Without<MerchantPlayerGrid>)>,
    mut player_grids: Query<&mut ItemGrid, (With<MerchantPlayerGrid>, Without<MerchantStockGrid>)>,
) {
    if !inventory.is_changed() && !stock.is_changed() {
        return;
    }
    // Update stock grid if stock changed, player grid if inventory changed...
}
```

This replaces manual `refresh_grids()` calls after buy/sell transactions.

## Detail Pane Population

`populate_merchant_detail_pane` system:
- Checks which grid is focused
- Updates `ItemDetailPane.source` accordingly
- Detects data changes via `stock.is_changed() || inventory.is_changed()`
- Populates `ItemDetailPaneContent` with item name, type, quality, quantity, price, stats

The system automatically refreshes when stock or inventory changes (using Bevy's change detection), so no manual refresh trigger is needed after transactions.

## Stock Generation

`MerchantStock::generate()` creates random stock from a pool of 35 items:
- Consumables: BasicHPPotion (3-8 qty)
- Weapons: Sword, Dagger, TinSword, CopperSword, BronzeSword (1 qty each)
- Shields: BasicShield (1 qty)
- Armor sets: Full Copper, Tin, and Bronze armor (helmet, chestplate, gauntlets, greaves, leggings) (1 qty each)
- Tools: BronzePickaxe (1 qty)
- Accessories: GoldRing (1 qty)
- Ores: CopperOre, TinOre, Coal (5-15 qty each)
- Ingots: CopperIngot, TinIngot (2-5 qty), BronzeIngot (1-3 qty)
- Materials: Cowhide, SlimeGel (3-8 qty each)

Selects 8-12 random items from the pool.
