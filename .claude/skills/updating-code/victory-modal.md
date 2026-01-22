# Victory Modal

Post-fight victory modal displaying combat results after defeating a mob. Spawned programmatically from the fight modal.

## Module Structure

**Location:** `src/ui/screens/victory_modal/`

| File | Purpose |
|------|---------|
| `mod.rs` | Module declarations, exports `VictoryModalPlugin`, `SpawnVictoryModal`, `VictoryModalData` |
| `plugin.rs` | `VictoryModalPlugin` and system registration |
| `state.rs` | Components, resources, sprite marker |
| `input.rs` | Close handling (Enter or Escape) |
| `render.rs` | UI spawning via Modal builder |

## Key Resources

### VictoryModalData

Stores the victory results for display:

```rust
#[derive(Resource)]
pub struct VictoryModalData {
    pub mob_name: String,
    pub mob_id: MobId,
    pub gold_gained: i32,
    pub xp_gained: i32,
    pub loot_drops: Vec<LootDrop>,
}
```

### SpawnVictoryModal

Marker resource to trigger modal spawn. Removed by `spawn_victory_modal` system.

## Flow

1. **Fight Modal Victory** (`fight_modal/input.rs`): When mob dies:
   - Calls `on_death()` to get `MobDeathResult`
   - Calls `apply_victory_rewards()` to get gold/XP (with goldfind bonus)
   - Calls `collect_loot_drops()` to add loot to inventory
   - Closes fight modal
   - Inserts `VictoryModalData` and `SpawnVictoryModal` resources

2. **Modal Spawn** (`render.rs:spawn_victory_modal`):
   - Uses `Modal::new().title("Victory!").size(300, 350)` builder
   - Displays mob sprite (128x128), mob name, gold gained, XP gained, loot items
   - Sets `ActiveModal.modal = Some(ModalType::VictoryModal)`

3. **Dismiss** (`input.rs:handle_victory_modal_close`):
   - Enter (`GameAction::Select`) or Escape (`GameAction::CloseModal`) closes
   - Removes `VictoryModalData` resource on close

## UI Components

| Component | Purpose |
|-----------|---------|
| `VictoryModalRoot` | Modal overlay root entity |
| `VictoryModalMobSprite { mob_id }` | Mob sprite marker (SpriteMarker impl) |

## Sprite Marker

`VictoryModalMobSprite` resolves from `MobSpriteSheets` by `mob_id`, with `flip_x: false`.

## Text Colors

- Title "Victory!": cream (0.95, 0.9, 0.7) - from Modal builder
- Mob name: white
- Gold: gold (1.0, 0.84, 0.0)
- XP: light blue (0.6, 0.8, 1.0)
- Loot items: white

## Not a RegisteredModal

The victory modal is NOT registered via `RegisteredModal` trait. Like `FightModal`, it's spawned programmatically and not toggled via the navigation system. Its `ModalType::VictoryModal` variant is listed in the non-toggle arm of `handle_modal_toggle` in `navigation/systems.rs`.

## Related Documentation

- [fight-modal.md](fight-modal.md) - Fight modal (spawns victory modal on mob death)
- [modals.md](modals.md) - General modal patterns
- [modal-builder.md](modal-builder.md) - Modal builder API
- [sprite-marker.md](sprite-marker.md) - SpriteMarker trait
