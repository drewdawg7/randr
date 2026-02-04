# System Ordering

## Quick Reference

```rust
// Chain - sequential execution
app.add_systems(Update, (read_input, process_input, apply_result).chain());

// After - run after specific system
app.add_systems(Update, apply_damage.after(calculate_damage));

// Before - run before specific system
app.add_systems(Update, prepare_combat.before(combat_logic));

// Combined ordering
app.add_systems(Update, (
    physics_step,
    collision_detection.after(physics_step),
    damage_application.after(collision_detection),
));

// Batch ordering
app.add_systems(Update, (
    (spawn_a, spawn_b, spawn_c).before(process_spawns),
    process_spawns,
));
```

## Overview

By default, Bevy runs systems in parallel with no guaranteed order. Use ordering constraints when:
- One system writes data another reads
- Processing must happen in a specific sequence
- Events must be handled in order

## Chaining Systems

`.chain()` enforces sequential execution in tuple order:

```rust
// Systems run in order: a -> b -> c
app.add_systems(Update, (system_a, system_b, system_c).chain());

// Equivalent to:
app.add_systems(Update, (
    system_a,
    system_b.after(system_a),
    system_c.after(system_b),
));
```

### Codebase Examples

**Combat chain** from `src/combat/plugin.rs`:
```rust
app.add_systems(
    Update,
    (
        process_player_attack.run_if(on_message::<PlayerAttackMob>),
        handle_mob_death.run_if(on_message::<EntityDied>),
        handle_player_death.run_if(on_message::<EntityDied>),
    )
        .chain()
        .run_if(in_state(AppState::Dungeon))
        .run_if(resource_exists::<ActiveCombat>),
);
```

**Dungeon flow** from `src/ui/screens/dungeon/plugin.rs`:
```rust
app.add_systems(
    Update,
    (
        handle_floor_ready.run_if(on_message::<FloorReady>),
        spawn_player_when_ready.run_if(resource_exists::<PendingPlayerSpawn>),
        handle_dungeon_movement.run_if(on_message::<GameAction>),
        handle_move_result.run_if(on_message::<MoveResult>),
        update_player_sprite_direction,
    )
        .chain()
        .run_if(in_state(AppState::Dungeon)),
);
```

**State entry** from `src/ui/screens/main_menu.rs`:
```rust
app.add_systems(OnEnter(AppState::Menu),
    (spawn_main_menu, reset_menu_selection).chain()
);
```

**Visual updates** from `src/ui/screens/forge_modal/plugin.rs`:
```rust
app.add_systems(
    PostUpdate,
    (update_forge_slot_selector, animate_forge_slot_selector)
        .chain()
        .run_if(in_forge_modal),
);
```

## Explicit Ordering

### after() and before()

For fine-grained control:

```rust
app.add_systems(Update, (
    read_input,
    process_movement.after(read_input),
    apply_physics.after(process_movement),
    render_frame.after(apply_physics),
));

// Or using before()
app.add_systems(Update, (
    setup_frame.before(game_logic),
    game_logic,
    cleanup_frame.after(game_logic),
));
```

### Batch Ordering

Order multiple systems relative to one:

```rust
// Multiple systems before one
app.add_systems(Update, (
    (spawn_enemies, spawn_items, spawn_hazards).before(process_entities),
    process_entities,
));

// Multiple systems after one
app.add_systems(Update, (
    calculate_damage,
    (apply_damage, show_damage_numbers, play_hit_sound).after(calculate_damage),
));
```

## Ordering with Run Conditions

Run conditions and ordering interact:

```rust
// Chain with individual conditions
app.add_systems(Update, (
    handle_attack.run_if(on_message::<Attack>),
    handle_death.run_if(on_message::<Death>),
).chain());
// handle_death waits for handle_attack, but only runs if Death events exist

// Shared condition on chain
app.add_systems(Update, (
    handle_attack,
    handle_death,
).chain().run_if(in_state(GameState::Combat)));
// Entire chain skipped if not in Combat state
```

## Ambiguity Detection

Enable warnings for systems that access the same data without ordering:

```rust
app.edit_schedule(Update, |schedule| {
    schedule.set_build_settings(ScheduleBuildSettings {
        ambiguity_detection: LogLevel::Warn,
        ..default()
    });
});
```

### What Causes Ambiguity

Systems are ambiguous when:
1. They access the same resource/component mutably
2. One reads and another writes the same data
3. No ordering constraint exists between them

```rust
// Ambiguous: both write to Score
fn system_a(mut score: ResMut<Score>) { score.0 += 1; }
fn system_b(mut score: ResMut<Score>) { score.0 += 2; }

app.add_systems(Update, (system_a, system_b)); // Warning!
```

### Resolving Ambiguity

**Option 1: Add ordering**
```rust
app.add_systems(Update, (system_a, system_b.after(system_a)));
```

**Option 2: Use chain**
```rust
app.add_systems(Update, (system_a, system_b).chain());
```

**Option 3: Mark as intentionally ambiguous**
```rust
app.add_systems(Update, (
    system_a,
    system_b.ambiguous_with(system_a),
));
```

**Option 4: Use system sets**
```rust
app.configure_sets(Update, SetA.before(SetB));
app.add_systems(Update, (
    system_a.in_set(SetA),
    system_b.in_set(SetB),
));
```

## When to Use Ordering

### Use chain() when:
- Systems form a clear pipeline
- Processing must happen in sequence
- Reading/writing the same data

### Use after()/before() when:
- Ordering relative to specific systems
- Complex dependency graphs
- Mixing ordered and unordered systems

### Don't order when:
- Systems are truly independent
- Parallel execution is beneficial
- No data dependencies exist

## Ordering Across Schedules

Ordering only works within a schedule:

```rust
// These run in different schedules - no ordering possible
app.add_systems(PreUpdate, system_a);
app.add_systems(Update, system_b);  // Always runs after PreUpdate

// For cross-schedule ordering, use the schedule order itself
// PreUpdate -> StateTransition -> Update -> PostUpdate
```

## Common Mistakes

### Forgetting to chain event handlers
```rust
// Wrong: race condition, events may be processed out of order
app.add_systems(Update, (
    emit_damage.run_if(on_message::<Attack>),
    handle_damage.run_if(on_message::<Damage>),
));

// Correct: chain ensures damage emitted before handled
app.add_systems(Update, (
    emit_damage.run_if(on_message::<Attack>),
    handle_damage.run_if(on_message::<Damage>),
).chain());
```

### Over-ordering (hurts parallelism)
```rust
// Wrong: unnecessary ordering prevents parallelism
app.add_systems(Update, (
    update_health,
    update_mana.after(update_health),  // No dependency!
    update_stamina.after(update_mana), // No dependency!
));

// Correct: independent systems run in parallel
app.add_systems(Update, (update_health, update_mana, update_stamina));
```

### Ordering without checking conditions
```rust
// Potential issue: system_b waits even if system_a doesn't run
app.add_systems(Update, (
    system_a.run_if(rare_condition),
    system_b.after(system_a),
));

// If system_b truly depends on system_a's output, this is correct.
// If not, remove the ordering for better parallelism.
```
