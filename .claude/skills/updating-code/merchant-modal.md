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

## Detail Pane Systems

Detail pane logic is split into two systems for efficient change detection:

### `update_merchant_detail_pane_source`
Updates `pane.source` based on focus and grid selection. Only runs when:
- `FocusState` changes (tab between grids)
- `ItemGrid.selected_index` changes (navigation)

Uses `Ref<ItemGrid>` to check `is_changed()` on each grid.

### `populate_merchant_detail_pane_content`
Renders content when source or data changes. Only runs when:
- `pane.source` changed (via source update system)
- `stock.is_changed() || inventory.is_changed()` (data changed after buy/sell)

Uses `Ref<ItemDetailPane>` to check `is_changed()` for pane updates.

**Content rendered:**
- Item name (quality-colored with black outline)
- Item type, quality label
- Quantity (if > 1)
- Price label: "Price: Xg" for store items, "Sell: Xg" for player items
- `ItemStatsDisplay` with stat comparison for equipment

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
