# Merchant Modal

Modal that opens when pressing Space next to a Merchant NPC in the dungeon.

## Module Structure

```
src/ui/screens/merchant_modal/
  mod.rs         - Module exports
  state.rs       - Components, resources, RegisteredModal impl
  render.rs      - UI spawning, detail pane population
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
- Remove item via `inventory.remove_item()`

## Detail Pane Population

`populate_merchant_detail_pane` system:
- Checks which grid is focused
- Updates `ItemDetailPane.source` accordingly
- Populates `ItemDetailPaneContent` with item name, type, quality, quantity, price, stats

## Stock Generation

`MerchantStock::generate()` creates random stock from a pool:
- Potions (3-8 qty)
- Basic weapons (1 qty)
- Ores (5-15 qty)
- Ingots (1-5 qty)

Selects 6-10 random items from the pool.
