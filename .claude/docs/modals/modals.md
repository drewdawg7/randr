# Modal System

Modals are full-screen UI overlays that block game interaction until closed.

## Core Infrastructure

**File:** `src/ui/screens/modal.rs`

- `ActiveModal` - Resource tracking which modal is open
- `ModalType` - Enum of all modal types
- `spawn_modal_overlay()` - Creates semi-transparent background
- `in_*_modal()` - Run condition functions for each modal

## Modal Registry

**File:** `src/ui/modal_registry.rs`

- `RegisteredModal` trait - Implement for type-safe modal handling
- `ModalCommands` extension - `toggle_modal::<M>()`, `close_modal::<M>()`
- `modal_close_system::<M>` - Generic close handler

See [modal-registry.md](modal-registry.md) for details.

## Module Structure

```
src/ui/screens/my_modal/
├── mod.rs        # Re-exports
├── plugin.rs     # Plugin, system registration
├── state.rs      # Components, resources, RegisteredModal impl
├── input.rs      # Navigation/select handlers
└── render.rs     # Spawn and display systems
```

Reference: `src/ui/screens/inventory_modal/` for a complete example.

## Shared Systems

**File:** `src/ui/focus.rs`

- `tab_toggle_system(FocusPanel, FocusPanel)` - Generic tab handler for dual-panel modals
- `FocusState` - Resource tracking focused panel
- `FocusPanel` - Enum of all focusable panels

**File:** `src/ui/widgets/detail_pane_system.rs`

- `DetailPaneContext` trait - Implement in `state.rs` for modals with two ItemGrids
- `update_detail_pane_source::<C>` - Generic system that updates `ItemDetailPane.source` based on focus/grid state

Reference: `inventory_modal/state.rs` for `InventoryDetailPane` implementation.

## Input Blocking

Non-modal input handlers must check `ActiveModal` and return early if a modal is open.

Files implementing this:
- `src/ui/screens/town/systems.rs`
- `src/ui/screens/town/tabs/*/input.rs`

## Existing Modals

| Modal | Directory | Type |
|-------|-----------|------|
| Inventory | `inventory_modal/` | `InventoryModal` |
| Merchant | `merchant_modal/` | `MerchantModal` |
| Forge | `forge_modal/` | `ForgeModal` |
| Anvil | `anvil_modal/` | `AnvilModal` |
| Profile | `profile_modal.rs` | `ProfileModal` |
| Monster Compendium | `monster_compendium/` | `MonsterCompendiumModal` |

## Adding a New Modal

1. Add variant to `ModalType` in `src/ui/screens/modal.rs`
2. Create module with structure above
3. Implement `RegisteredModal` in `state.rs`
4. Add match arm in `handle_modal_toggle` in `src/navigation/systems.rs`
5. Configure in `NavigationPlugin` in `src/plugins/game.rs`
6. Register plugin in `src/plugins/game.rs`

See [modal-registry.md](modal-registry.md) and [navigation.md](../ui/navigation.md).
