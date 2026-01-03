# UI Architecture

## Overview

The UI uses `ratatui` for rendering and `tuirealm` for component/event management. Key architectural elements:

- **Screens**: Full-page views (`MainMenuScreen`, `TownScreen`, `FightScreen`, etc.)
- **Tabs**: Sub-views within screens (Store, Blacksmith, Field in TownScreen)
- **Widgets**: Reusable UI components (Menu, ItemList, selection types)
- **Wrappers**: Decorators adding functionality (ModalWrapper, TabbedContainer)

## Key Modules

| Module | Location | Purpose |
|--------|----------|---------|
| Screen lifecycle | `src/ui/screen/lifecycle.rs` | Track screen transitions |
| UIState | `src/ui/state.rs` | Grouped UI-specific state |
| Selection widgets | `src/ui/components/widgets/selection.rs` | Navigation abstractions |
| Event routing | `src/ui/components/wrappers/tabbed_container.rs` | Children-first event handling |

---

## Screen Lifecycle

**Location**: `src/ui/screen/lifecycle.rs`

Tracks screen transitions to enable state reset on entry:

```rust
// In any screen's view() or on() method:
let gs = game_state();
if gs.screen_lifecycle().just_entered() {
    self.reset_state();
}

// Check where we came from:
if gs.screen_lifecycle().came_from(Id::Fight) {
    // Handle return from combat
}
```

### ScreenLifecycle API

- `just_entered()` - True on first frame after transition
- `came_from(Id)` - True if just transitioned from specific screen
- `previous_screen()` - Get the previous screen ID
- `current_screen()` - Get the current screen ID
- `acknowledge_entry()` - Clear just_entered flag (optional)

### ScreenMetadata

Provides info about screens for validation:

```rust
let meta = ScreenMetadata::for_screen(Id::Fight);
assert!(meta.requires_combat);  // Fight screen needs active combat
```

---

## UIState

**Location**: `src/ui/state.rs`

Groups all UI-specific state separate from game logic:

```rust
pub struct UIState {
    pub current_screen: Id,
    pub lifecycle: ScreenLifecycle,
    pub active_modal: ModalType,
    pub inventory_modal: InventoryModal,
    pub show_item_details: bool,
    pub toasts: ToastQueue,
}
```

### Access Pattern

```rust
// New code should use:
game_state().ui.current_screen
game_state().ui.active_modal
game_state().ui.toasts.error("message")

// Legacy accessors still work:
game_state().current_screen  // Deprecated, use ui.current_screen
game_state().toasts          // Deprecated, use ui.toasts
```

### UIState Methods

- `go_to_screen(Id)` - Navigate to a screen
- `toggle_modal(ModalType)` - Toggle a modal
- `open_modal(ModalType)` / `close_modal()` - Modal control
- `toast_error/success/info(msg)` - Add toast notifications
- `just_entered_screen()` - Shortcut for lifecycle check

---

## Selection Abstractions

**Location**: `src/ui/components/widgets/selection.rs`

Unified navigation patterns to replace scattered implementations.

### ListSelection

Wrapping vertical list navigation:

```rust
let mut sel = ListSelection::new(5);  // 5 items
sel.move_down();  // 0 -> 1
sel.move_up();    // 1 -> 0
sel.move_up();    // 0 -> 4 (wraps)

// For ratatui List widget:
frame.render_stateful_widget(list, area, sel.list_state_mut());
```

### BinaryToggle<T>

Toggle between exactly two options:

```rust
let mut toggle = BinaryToggle::new(FightAction::Attack, FightAction::Run);
toggle.toggle();  // Switch to Run
if toggle.is_first() { /* Attack selected */ }
```

### BoundedSelection

Bounded numeric selection (no wrapping):

```rust
let mut sel = BoundedSelection::new(2);  // 0, 1, 2
sel.move_right();  // 0 -> 1
sel.move_right();  // 1 -> 2
sel.move_right();  // 2 -> 2 (stays at max)
```

### GridSelection

2D row/column selection:

```rust
let mut grid = GridSelection::new(2, 3);  // 2 rows, 3 cols
grid.move_right();  // (0,0) -> (0,1)
grid.move_down();   // (0,1) -> (1,1)
let (row, col) = grid.position();
```

### DirectionalSelection

Compass-style navigation with availability:

```rust
let mut compass = DirectionalSelection::new();
compass.set_available(NavDirection::Up, true);   // North available
compass.set_available(NavDirection::Right, true); // East available

compass.navigate(NavDirection::Up);    // Center -> North
compass.navigate(NavDirection::Right); // North -> East
compass.navigate(NavDirection::Left);  // East -> Center (opposite)

if compass.is_center() { /* at center */ }
if compass.is_at(NavDirection::Up) { /* at north */ }
```

---

## Event Routing

**Location**: `src/ui/components/wrappers/tabbed_container.rs`

### Children-First Pattern

TabbedContainer uses children-first event routing:

1. Event is passed to active tab child first
2. If child consumes it (returns None), routing stops
3. If child doesn't consume it (returns Some), container handles it

```rust
fn on(&mut self, ev: Event<NoUserEvent>) -> Option<Event<NoUserEvent>> {
    // Forward to child first
    if let Some(tab) = self.tabs.get_mut(self.active_tab) {
        if tab.content.on(ev.clone()).is_none() {
            return None;  // Child consumed
        }
    }
    // Only then handle tab switching
    match ev {
        Event::Keyboard(KeyEvent { code: Key::Left, .. }) => {
            self.switch_tab(-1);
            None
        }
        // ...
    }
}
```

**Why this matters**: Child components (like DungeonTab's compass) can use Left/Right arrows for their navigation without tab switching interfering.

---

## Component Patterns

### MockComponent + Component Traits

All UI elements implement two traits:

- `MockComponent` - Rendering (`view`), commands (`perform`), state
- `Component<Event<NoUserEvent>, NoUserEvent>` - Event handling (`on`)

```rust
impl MockComponent for MyScreen {
    fn view(&mut self, frame: &mut Frame, area: Rect) {
        // Rendering logic
    }

    fn perform(&mut self, cmd: Cmd) -> CmdResult {
        // Command handling
    }
}

impl Component<Event<NoUserEvent>, NoUserEvent> for MyScreen {
    fn on(&mut self, ev: Event<NoUserEvent>) -> Option<Event<NoUserEvent>> {
        // Return None if consumed, Some(ev) to pass through
    }
}
```

### ModalWrapper

Decorator that adds modal overlay capability to any screen:

```rust
let menu = ModalWrapper::new(MainMenuScreen::default());
```

Modals (Inventory, Keybinds, Profile) are rendered on top of the wrapped screen. Event handling is blocked for the wrapped screen when a modal is open.

Modal instances are stored in `GameState`:
- `inventory_modal: InventoryModal`
- `profile_modal: ProfileModal`
- `spell_test_modal: SpellTestModal`

### ProfileModal

**Location**: `src/ui/components/player/profile_modal.rs`

Displays player stats and passive effects in a two-column parchment-styled layout:
- **Left column**: Player stats (name, level, HP, attack, defense, gold, XP bar)
- **Right column**: Scrollable list of passive effects from equipped tome

**Key features**:
- Uses `ListSelection` for navigating passive effects
- Displays spell name as label, effect description below selected item
- Uses `player.tome_passive_effects_with_names()` to get spell names with effects
- Parchment background with fiber texture pattern

**Input handling**:
- `p` / `Esc`: Close modal
- `Up` / `Down`: Navigate passive effects list

### TabbedContainer

Container for managing multiple tabs with themed borders:

```rust
TabbedContainer::new(vec![
    TabEntry::with_border(Line::from("Store"), StoreTab::new(), BorderTheme::Wood),
    TabEntry::with_border(Line::from("Blacksmith"), BlacksmithTab::new(), BorderTheme::Ember),
])
```

---

## Key Files

| File | Purpose |
|------|---------|
| `src/system.rs` | GameState, global state access |
| `src/ui/mod.rs` | UI module exports |
| `src/ui/state.rs` | UIState struct |
| `src/ui/screen/lifecycle.rs` | Screen transition tracking |
| `src/ui/components/widgets/selection.rs` | Selection abstractions |
| `src/ui/components/wrappers/tabbed_container.rs` | Tab container with event routing |
| `src/ui/components/wrappers/modal_wrapper.rs` | Modal overlay wrapper |

---

## DungeonScreen Module Structure

**Location**: `src/ui/components/screens/dungeon/`

The DungeonScreen was split into focused sub-modules for maintainability:

| File | Purpose |
|------|---------|
| `mod.rs` | Main component, DungeonState enum, core methods |
| `room_entry.rs` | Initial room interaction (Monster, Chest, etc.) |
| `navigation.rs` | Compass-based movement between rooms |
| `rest_room.rs` | Rest areas where player can heal |
| `boss_room.rs` | Boss fight handling (trapped until victory/death) |

### DungeonState Enum

```rust
pub enum DungeonState {
    RoomEntry,    // Player just entered a room
    Navigation,   // Room cleared, can navigate
    RestRoom,     // In a rest area
    BossRoom,     // Boss fight (trapped!)
}
```

### State Module Pattern

Each state module exports:
- `render(frame, area, &state_fields)` - Render the state UI
- `handle_submit(state_fields) -> Option<DungeonState>` - Handle action, return new state

---

## Command Architecture

**Location**: `src/commands/`

The command layer decouples game logic from UI components. Instead of UI directly mutating game state, it dispatches commands that are executed by handlers.

### Usage

```rust
use crate::commands::{execute, apply_result, GameCommand};

// In a UI event handler:
match selection {
    FightSelection::Attack => {
        let result = execute(GameCommand::PlayerAttack);
        apply_result(&result);  // Shows toast, changes screen
    }
}
```

### Command Categories

| Module | Commands |
|--------|----------|
| `combat.rs` | PlayerAttack, PlayerRun, ReturnFromCombat, StartNewFight |
| `dungeon.rs` | EnterRoom, MoveDungeon, LeaveDungeon, Rest, AttackBoss |
| `store.rs` | PurchaseItem, SellItem |
| `inventory.rs` | EquipItem, UnequipItem, ToggleLock, UseConsumable |

### Inventory Keybinds

The following keybinds are available when the inventory modal is open:

| Key | Action | Location |
|-----|--------|----------|
| `i` | Open/close inventory | `modal_wrapper.rs` |
| `Shift+I` | Toggle keybinds help | `modal_wrapper.rs` |
| `e` | Equip/unequip item | `inventory_modal.rs`, `store/tab.rs` |
| `l` | Lock/unlock item | `inventory_modal.rs`, `store/tab.rs` |
| `d` | Toggle item details | `inventory_modal.rs` |
| `u` | Use consumable | `inventory_modal.rs` |
| `f` | Cycle filter | `inventory_modal.rs` |
| `Up/Down` | Navigate list | `inventory_modal.rs` |

| `mining.rs` | MineRock |

### CommandResult

Commands return a `CommandResult` with:
- `success: bool` - Whether the command succeeded
- `message: Option<CommandMessage>` - Toast to display (Success/Info/Error)
- `screen_change: Option<Id>` - Screen to navigate to

### Migration Status

The command layer is complete but UI components haven't been migrated yet.
Migration can be done incrementally - existing direct state mutation still works.

---

## Future Improvements (Deferred)

### Phase 7: Reorganize File Structure
Deferred as LOW priority. Current structure is reasonable:
- Tabs: `components/{store,blacksmith,alchemist,field,dungeon}/`
- Screens: `components/screens/{main_menu,town,fight,dungeon/}`
- Wrappers: `components/wrappers/`
- Widgets: `components/widgets/`
