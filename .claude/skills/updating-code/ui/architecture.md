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

Modals (Inventory, Keybinds) are rendered on top of the wrapped screen. Event handling is blocked for the wrapped screen when a modal is open.

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

## Future Improvements (Deferred)

### Phase 1: Command Architecture
Decouple game logic from UI by introducing a command layer. Currently, game logic (combat, shopping, mining) executes directly in UI event handlers.

### Phase 6: Split DungeonScreen
DungeonScreen is 1270 lines. Should be split into:
- `dungeon/room_entry.rs`
- `dungeon/navigation.rs`
- `dungeon/rest_room.rs`
- `dungeon/boss_room.rs`

### Phase 7: Reorganize File Structure
Consolidate screens under `src/ui/screens/` for consistent organization.
