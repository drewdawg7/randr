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

// No-background modal (grid-based modals where widgets have their own backgrounds)
commands.spawn_modal(
    Modal::new()
        .background(ModalBackground::None)
        .with_root_marker(|e| { e.insert(InventoryModalRoot); })
        .content(|c| { /* widgets provide their own backgrounds */ })
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
| `.size(f32, f32)` | Width and height in pixels | Solid, Atlas |
| `.max_width_percent(f32)` | Max width constraint (default 90%) | Solid, Atlas |
| `.max_height_percent(f32)` | Max height constraint (default 80%) | Solid, Atlas |
| `.padding(f32)` | Container padding (default 30px) | Solid only |
| `.border(f32)` | Border width (default 3px) | Solid only |
| `.background(ModalBackground)` | Set background type | All |
| `.modal_type(ModalType)` | For ActiveModal tracking | All |
| `.with_root_marker(FnOnce)` | Add marker component to overlay | All |
| `.content(FnOnce)` | Spawn child content | All |

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

    /// No container - content is added directly to the overlay.
    /// Use for grid-based modals where child widgets provide their own backgrounds.
    None,
}
```

## Behavioral Differences by Background

| Behavior | Solid | Atlas | None |
|----------|-------|-------|------|
| Container | `BackgroundColor` + `BorderColor` | `ImageNode::from_atlas_image()` | No container |
| Positioning | Default flex | `PositionType::Relative` | Content on overlay |
| Padding | Applied (default 30px) | Ignored | N/A |
| Border | Applied (3px) | None | N/A |
| Title | Spawned if set | Skipped | Skipped |
| Hints | Spawned if set | Skipped | Skipped |

**When to use each:**
- `Solid` - Standard modals with title, hints, and solid background (ProfileModal, ResultsModal)
- `Atlas` - Sprite-based modals using an image as background (MonsterCompendium book)
- `None` - Grid-based modals where widgets provide their own backgrounds (InventoryModal, MerchantModal, AnvilModal, ForgeModal)

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

### Grid-Based Modal (No Container)

```rust
fn spawn_inventory_modal(commands: &mut Commands, inventory: &Inventory) {
    let items = inventory.items.clone();

    commands.spawn_modal(
        Modal::new()
            .background(ModalBackground::None)
            .with_root_marker(|e| { e.insert(InventoryModalRoot); })
            .content(move |c| {
                c.spawn(modal_content_row()).with_children(|row| {
                    spawn_equipment_grid(row, &items);
                    spawn_backpack_grid(row, &items);
                });
            })
    );
}
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

- `src/ui/screens/modal.rs` - `ModalOverlayBundle`, `ModalOverlay`, `ModalContent`, `ActiveModal`, `ModalType`
- `src/ui/hints.rs` - `spawn_modal_hint()` used for hint rendering
