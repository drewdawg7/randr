# Economy System

## Overview

The economy module (`src/economy/`) handles gold-based value traits for items that can be bought, sold, or traded.

## Key Files

| File | Purpose |
|------|---------|
| `src/economy/mod.rs` | Defines `WorthGold` trait for economic value |

## WorthGold Trait

Trait for items with gold-based economic value. Used by shop system, blacksmith, and trading.

```rust
pub trait WorthGold {
    fn gold_value(&self) -> i32;           // Required: base gold value
    fn purchase_price(&self) -> i32;       // Default: gold_value()
    fn sell_price(&self) -> i32;           // Default: gold_value() / 2
}
```

### Implementors

- `Item` (`src/item/traits.rs`) - applies quality multiplier to base value

### Usage Locations

| File | Purpose |
|------|---------|
| `src/location/store/store_item.rs` | Shop pricing display |
| `src/location/store/definition.rs` | Purchase/sell transactions |
| `src/ui/components/widgets/item_list/impls.rs` | UI price display |

## Design Notes

The `WorthGold` trait was moved from `src/loot/` to `src/economy/` because:
1. Economic value is not loot-specific - items are bought, sold, crafted
2. Only `Item` implements this trait
3. Better discoverability and module organization
