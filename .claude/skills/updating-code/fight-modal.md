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

### FightModalMob

Stores the mob being fought along with data for despawning after defeat:

```rust
#[derive(Resource)]
pub struct FightModalMob {
    pub mob_id: MobId,       // For sprite lookup
    pub mob: Mob,            // Live mob instance for combat
    pub pos: GridPosition,   // For clearing occupancy
    pub entity: Entity,      // For despawning from dungeon
}
```

### FightModalButtonSelection

Tracks OK/Cancel button selection:

```rust
#[derive(Resource, Default)]
pub struct FightModalButtonSelection {
    pub selected: FightModalButton,  // Ok or Cancel
}
```

Implements `SelectionState` trait for left/right navigation.

## Combat Flow

1. **Collision Detection** (`dungeon/plugin.rs:check_entity_collision`)
   - Returns `(DungeonEntity, Entity, GridPosition)` tuple
   - Mob collision triggers `FightModalMob` and `SpawnFightModal` resources

2. **Modal Spawn** (`render.rs:spawn_fight_modal`)
   - Spawns player and mob sprites with health bars
   - OK/Cancel buttons below player sprite

3. **Attack Handling** (`input.rs:handle_fight_modal_select`)
   - Enter with OK: Player attacks mob using `combat::attack()`
   - If mob dies: Apply rewards, despawn mob, close modal
   - If mob survives: Mob counter-attacks player
   - If player dies: Process defeat, close modal
   - Enter with Cancel: Close modal, no combat

4. **Health Bar Updates** (data → visuals separation)
   - `update_mob_health_bar` (`render.rs`): Writes mob HP from `FightModalMob` into `HealthBarValues`
   - `update_player_health_bar` (`render.rs`): Writes player HP from `StatSheet` into `HealthBarValues`
   - `update_sprite_health_bar_visuals` (`health_bar.rs`): Generic system that reads `HealthBarValues` and updates both the sprite atlas index and the HP text overlay
   - Health bars are spawned with initial `HealthBarValues` so they display correctly from the first frame
   - See [health-bar.md](health-bar.md) for the generic health bar system

## Combat Integration

Uses functions from `crate::combat`:
- `attack(&attacker, &mut defender)` - Execute single attack
- `apply_victory_rewards(&mut player, gold, xp)` - Grant rewards
- `process_defeat(&mut player)` - Handle player death
- `IsKillable::on_death(magic_find)` - Get mob death rewards

Uses `PlayerGuard` pattern for auto-writeback of player resources.

## Despawning Mobs

When mob is defeated:
```rust
// Clear from dungeon occupancy
occupancy.vacate(fight_mob.pos, GridSize::single());
// Remove entity from ECS
commands.entity(fight_mob.entity).despawn_recursive();
```

## UI Components

| Component | Purpose |
|-----------|---------|
| `FightModalRoot` | Modal overlay root entity |
| `FightModalPlayerSprite` | Player sprite marker |
| `FightModalMobSprite { mob_id }` | Mob sprite marker |
| `FightModalPlayerHealthBar` | Player health bar marker |
| `FightModalMobHealthBar` | Mob health bar marker |
| `FightModalOkButton` | OK button marker |
| `FightModalCancelButton` | Cancel button marker |

## Related Documentation

- [health-bar.md](health-bar.md) - HealthBarValues, SpriteHealthBar, HP text overlay
- [modals.md](modals.md) - General modal patterns
- [focus.md](focus.md) - SelectionState trait
- [dungeon/mod.md](dungeon/mod.md) - Dungeon and grid systems
