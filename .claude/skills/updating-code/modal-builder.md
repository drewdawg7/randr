# Modal Builder

Fluent builder API for creating consistent modal UIs.

**File:** `src/ui/modal_builder.rs`

## Quick Reference

```rust
use crate::ui::{Modal, ModalBackground, SpawnModalExt};

// Standard solid-background modal
commands.spawn_modal(
    Modal::new()
        .title("Inventory")
        .size(600.0, 400.0)
        .hint("[Esc] Close")
        .with_root_marker(|e| { e.insert(InventoryModalRoot); })
        .content(|c| {
            c.spawn(Text::new("Content here"));
        })
);

// Atlas/image-background modal (e.g., book sprite)
commands.spawn_modal(
    Modal::new()
        .background(ModalBackground::Atlas { texture, layout, index })
        .size(672.0, 399.0)
        .with_root_marker(|e| { e.insert(MonsterCompendiumRoot); })
        .content(|c| { /* children use absolute positioning */ })
);
```

## API Reference

### Modal::new()

Creates a new modal builder with default settings:
- Background: Solid with `MODAL_BG_COLOR` and `MODAL_BORDER_COLOR`
- Width: 800px, max width 90%, max height 80%
- Padding: 30px, border: 3px

### Builder Methods

| Method | Description | Applies To |
|--------|-------------|------------|
| `.title(impl Into<String>)` | Title at top (48px cream) | Solid only |
| `.hint(impl Into<String>)` | Hint at bottom (16px gray), can call multiple times | Solid only |
| `.size(f32, f32)` | Width and height in pixels | Both |
| `.max_width_percent(f32)` | Max width constraint (default 90%) | Both |
| `.max_height_percent(f32)` | Max height constraint (default 80%) | Both |
| `.padding(f32)` | Container padding (default 30px) | Solid only |
| `.border(f32)` | Border width (default 3px) | Solid only |
| `.background(ModalBackground)` | Set background type | Both |
| `.modal_type(ModalType)` | For ActiveModal tracking | Both |
| `.with_root_marker(FnOnce)` | Add marker component to overlay | Both |
| `.content(FnOnce)` | Spawn child content | Both |

### ModalBackground Enum

```rust
pub enum ModalBackground {
    /// Solid color with border (default)
    Solid { background: Color, border: Color },

    /// Texture atlas image (e.g., book sprite)
    /// Container uses PositionType::Relative for absolute child positioning
    Atlas {
        texture: Handle<Image>,
        layout: Handle<TextureAtlasLayout>,
        index: usize,
    },
}
```

## Behavioral Differences by Background

| Behavior | Solid | Atlas |
|----------|-------|-------|
| Container | `BackgroundColor` + `BorderColor` | `ImageNode::from_atlas_image()` |
| Positioning | Default flex | `PositionType::Relative` |
| Padding | Applied (default 30px) | Ignored |
| Border | Applied (3px) | None |
| Title | Spawned if set | Skipped |
| Hints | Spawned if set | Skipped |

## Styling Constants

Defined in `src/ui/modal_builder.rs`:

```rust
// Overlay
pub const MODAL_OVERLAY_COLOR: Color = Color::srgba(0.0, 0.0, 0.0, 0.7);
pub const MODAL_OVERLAY_Z_INDEX: i32 = 100;

// Container
pub const MODAL_BG_COLOR: Color = Color::srgb(0.15, 0.12, 0.1);
pub const MODAL_BORDER_COLOR: Color = Color::srgb(0.6, 0.5, 0.3);
pub const MODAL_DEFAULT_WIDTH: f32 = 800.0;
pub const MODAL_DEFAULT_MAX_WIDTH_PERCENT: f32 = 90.0;
pub const MODAL_DEFAULT_MAX_HEIGHT_PERCENT: f32 = 80.0;
pub const MODAL_DEFAULT_PADDING: f32 = 30.0;
pub const MODAL_DEFAULT_BORDER_WIDTH: f32 = 3.0;

// Title
pub const MODAL_TITLE_COLOR: Color = Color::srgb(0.95, 0.9, 0.7);
pub const MODAL_TITLE_FONT_SIZE: f32 = 48.0;
pub const MODAL_TITLE_MARGIN_BOTTOM: f32 = 20.0;
```

## Usage Examples

### Standard Modal with Two Columns

```rust
fn spawn_profile_modal(commands: &mut Commands, player: &Player) {
    commands.spawn_modal(
        Modal::new()
            .title("Character Profile")
            .with_root_marker(|e| { e.insert(ProfileModalRoot); })
            .hint("[P] or [Esc] to close")
            .content(|modal| {
                modal
                    .spawn(Node {
                        width: Val::Percent(100.0),
                        flex_direction: FlexDirection::Row,
                        column_gap: Val::Px(40.0),
                        ..default()
                    })
                    .with_children(|columns| {
                        spawn_left_column(columns, player);
                        spawn_right_column(columns, player);
                    });
            })
    );
}
```

### Custom Size and Multiple Hints

```rust
commands.spawn_modal(
    Modal::new()
        .title("Inventory")
        .size(1000.0, 700.0)
        .max_height_percent(90.0)
        .with_root_marker(|e| { e.insert(InventoryModalRoot); })
        .hint("[↑↓] Navigate")
        .hint("[Enter] Equip/Unequip")
        .hint("[I/Esc] Close")
        .content(|modal| {
            spawn_item_list(modal, &items, selected_index);
        })
);
```

### Atlas Background (Monster Compendium Book)

```rust
fn spawn_monster_compendium(
    mut commands: Commands,
    game_sprites: Res<GameSprites>,
) {
    let ui_all = game_sprites.get(SpriteSheetKey::UiAll).unwrap();
    let book_idx = ui_all.get(UiAllSlice::Book.as_str()).unwrap();

    commands.spawn_modal(
        Modal::new()
            .background(ModalBackground::Atlas {
                texture: ui_all.texture.clone(),
                layout: ui_all.layout.clone(),
                index: book_idx,
            })
            .size(BOOK_WIDTH, BOOK_HEIGHT)
            .with_root_marker(|e| { e.insert(MonsterCompendiumRoot); })
            .content(|book| {
                // Children use absolute positioning within the book image
                book.spawn(Node {
                    position_type: PositionType::Absolute,
                    left: Val::Px(45.0),
                    top: Val::Px(30.0),
                    ..default()
                }).with_children(|left_page| {
                    // Left page content
                });
            })
    );
}
```

### Minimal Modal (No Title/Hints)

```rust
commands.spawn_modal(
    Modal::new()
        .size(400.0, 200.0)
        .with_root_marker(|e| { e.insert(ConfirmDialogRoot); })
        .content(|modal| {
            modal.spawn(Text::new("Are you sure?"));
        })
);
```

## Content Closure Pattern

The content closure is `FnOnce(&mut ChildBuilder) + Send + Sync + 'static`. When capturing data:

```rust
// Clone/extract data before building
let items = get_all_inventory_items(&inventory);
let selected = selection.index;

commands.spawn_modal(
    Modal::new()
        .content(move |c| {
            // items and selected are moved into the closure
            for item in &items {
                spawn_item_row(c, item);
            }
        })
);
```

## Re-exports

From `src/ui/mod.rs`:

```rust
pub use modal_builder::{Modal, ModalBackground, SpawnModalExt};
```

## Related Files

- `src/ui/screens/modal.rs` - `ModalOverlayBundle`, `spawn_modal_overlay()`, `ModalOverlay`, `ModalContent`, `ActiveModal`
- `src/ui/hints.rs` - `spawn_modal_hint()` used for hint rendering
