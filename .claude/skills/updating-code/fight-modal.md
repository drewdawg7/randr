# Fight Modal

Modal overlay for dungeon combat encounters. Appears when player collides with a mob in the dungeon.

## Module Structure

**Location:** `src/ui/screens/fight_modal/`

| File | Purpose |
|------|---------|
| `mod.rs` | Module declarations and re-exports |
| `plugin.rs` | `FightModalPlugin` and system registration |
| `state.rs` | Components, resources, markers |
| `input.rs` | Navigation, close, and attack handling |
| `render.rs` | UI spawning and health bar updates |

## Key Resources

- **FightModalMob** (`state.rs`): Stores `mob_id`, `pos`, and `entity` for the mob being fought
- **FightModalButtonSelection** (`state.rs`): OK/Cancel selection, implements `SelectionState`

## Combat Flow

1. **Collision Detection** (`dungeon/plugin.rs:handle_move_result`)
   - `MoveResult::TriggeredCombat` contains `(mob_id, entity, pos)`
   - Guards against duplicate triggers if `FightModalMob` resource exists
   - Inserts `FightModalMob` resource and triggers `OpenModal(ModalType::FightModal)`

2. **Modal Spawn** (`render.rs:do_spawn_fight_modal`)
   - Spawns player and mob sprites with health bars
   - OK/Cancel buttons below player sprite

3. **Attack Handling** (`input.rs:handle_fight_modal_select`)
   - Enter with OK: Player attacks mob via `player_attacks_entity()`
   - Triggers attack animation via `PlayerAttackTimer`
   - If mob dies: Apply rewards, collect loot, despawn mob, close modal, spawn results modal
   - If mob survives: Mob counter-attacks player
   - If player dies: Process defeat, close modal
   - Enter with Cancel: Close modal, no combat

4. **Health Bar Updates** - See [health-bar.md](health-bar.md)

## Combat Integration

Uses functions from `crate::combat`:
- `player_attacks_entity()` / `entity_attacks_player()` - ECS-based combat
- `apply_victory_rewards_direct()` - Grant rewards
- `process_player_defeat()` - Handle player death

Uses `collect_loot_drops()` from `crate::loot` to add dropped items to inventory.

## Victory Transition

When mob dies:
1. Roll loot via `loot_table.0.roll_drops(magic_find)`
2. Apply rewards via `apply_victory_rewards_direct()`
3. Collect loot into inventory
4. Send `MobDefeated` event
5. Despawn mob entity and clear occupancy
6. Close fight modal
7. Insert `ResultsModalData` and trigger `OpenModal(ModalType::ResultsModal)`

## Error Handling

- **Invalid entity**: If mob entity query fails in `handle_fight_modal_select` (e.g., despawned during floor transition), modal closes gracefully
- **Duplicate triggers**: `handle_move_result` skips combat if `FightModalMob` resource exists, preventing race conditions during floor transitions

## UI Components

| Component | Purpose |
|-----------|---------|
| `FightModalRoot` | Modal overlay root entity |
| `FightModalPlayerSprite` | Player sprite marker |
| `FightModalMobSprite` | Mob sprite marker (contains `mob_id`) |
| `FightModalPlayerHealthBar` | Player health bar marker |
| `FightModalMobHealthBar` | Mob health bar marker |
| `FightModalOkButton` | OK button marker |
| `FightModalCancelButton` | Cancel button marker |

## Related Documentation

- [results-modal.md](results-modal.md) - Results modal (spawned after mob death or chest opening)
- [health-bar.md](health-bar.md) - HealthBarValues, SpriteHealthBar, HP text overlay
- [modals.md](modals.md) - General modal patterns
- [focus.md](focus.md) - SelectionState trait
- [dungeon/mod.md](dungeon/mod.md) - Dungeon and grid systems
