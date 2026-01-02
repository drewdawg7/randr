# Toast Notification System

## Overview
Displays non-blocking notifications in the top-right corner of the terminal. Toasts auto-dismiss after 3 seconds and stack vertically with most recent on top.

## Module Structure
```
src/toast/
  mod.rs          # Exports: Toast, ToastQueue, ToastType
  definition.rs   # Core types (no UI dependencies)
  render.rs       # Rendering logic
```

## Key Types

### ToastType (src/toast/definition.rs)
```rust
pub enum ToastType {
    Error,   // Red, icon [!]
    Success, // Green, icon [+]
    Info,    // Blue, icon [i]
}
```

### Toast (src/toast/definition.rs)
```rust
pub struct Toast {
    pub toast_type: ToastType,
    pub message: String,
    pub created_at: Instant,
}
```

### ToastQueue (src/toast/definition.rs)
Main API for triggering toasts:
```rust
impl ToastQueue {
    pub fn error(&mut self, message: impl Into<String>);
    pub fn success(&mut self, message: impl Into<String>);
    pub fn info(&mut self, message: impl Into<String>);
    pub fn cleanup(&mut self); // Called each frame to remove expired toasts
    pub fn toasts(&self) -> &[Toast]; // For rendering
}
```

## Usage Pattern

### From game logic:
```rust
use crate::system::game_state;

// Simple error
game_state().toasts.error("Not enough gold");

// Success with formatted message
game_state().toasts.success(format!("Purchased {}!", item.name));

// Match on Result
match some_operation() {
    Ok(result) => game_state().toasts.success("Success!"),
    Err(e) => {
        let msg = match e {
            SomeError::Variant1 => "Error message 1",
            SomeError::Variant2 => "Error message 2",
            _ => "Operation failed",
        };
        game_state().toasts.error(msg);
    }
}
```

## Integration Points

### GameState (src/system.rs)
- Field: `pub toasts: ToastQueue`
- `run_current_screen()` calls `self.toasts.cleanup()` each frame
- Rendering done after screen content in `terminal.draw()` closure

### Rendering (src/toast/render.rs)
- `render_toasts(frame: &mut Frame, toasts: &[Toast])`
- Position: top-right corner, 2-char margin from edge
- Dimensions: 35 wide x 3 tall per toast
- Max 5 toasts visible at once
- Uses `Clear` widget to render on top of content

## Wired Error Locations

| Location | File | Function |
|----------|------|----------|
| Blacksmith Upgrade | `ui/components/blacksmith/upgrade.rs` | `handle()` |
| Blacksmith Quality | `ui/components/blacksmith/quality.rs` | `handle()` |
| Blacksmith Smelt | `ui/components/blacksmith/smelt.rs` | `handle()` |
| Store Purchase | `ui/components/store/tab.rs` | `handle_buy_cmd()` |
| Store Sell | `ui/components/store/tab.rs` | `handle_sell_cmd()` |
| Alchemist Brew | `ui/components/alchemist/brew.rs` | `handle()` |
| Field Mob Spawn | `ui/components/field/tab.rs` | `perform()` |
| Inventory Use | `ui/components/player/inventory_modal.rs` | `handle_input()` |

## Error Types with Toast Messages

### BlacksmithError (src/location/blacksmith/enums.rs)
- `MaxUpgradesReached` -> "Max upgrades reached"
- `NotEnoughGold` -> "Not enough gold"
- `NoUpgradeStones` -> "No upgrade stones"
- `NotEquipment` -> "Cannot upgrade this item"
- `NotEnoughFuel` -> "Not enough fuel"
- `NoFuel` -> "No fuel to add"
- `RecipeError(_)` -> "Missing ingredients"
- `InventoryFull` -> "Inventory is full"

### StoreError (src/location/store/enums.rs)
- `OutOfStock` -> "Out of stock"
- `NotEnoughGold` -> "Not enough gold"
- `InventoryFull` -> "Inventory is full"
- `InvalidIndex` -> "Item not found"

### RecipeError (src/item/recipe/enums.rs)
- `NotEnoughIngredients` -> "Missing ingredients"

## Colors (from theme.rs)
- Error: `RED` (244, 67, 54)
- Success: `GREEN` (76, 175, 80)
- Info: `BLUE` (33, 150, 243)
- Toast background: `HEADER_BG` (45, 50, 70)
- Message text: `WHITE` (240, 240, 240)

## Adding New Toast Locations

1. Import game_state: `use crate::system::game_state;`
2. After the operation, call the appropriate toast method:
   ```rust
   game_state().toasts.error("Error message");
   game_state().toasts.success("Success message");
   ```
3. For Result types, match on the error and provide appropriate messages
