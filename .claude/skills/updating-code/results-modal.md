# Results Modal

Reusable modal for displaying loot/reward results. Used after mob victories and chest openings.

## Module Structure

**Location:** `src/ui/screens/results_modal/`

| File | Purpose |
|------|---------|
| `mod.rs` | Module declarations, exports `ResultsModalPlugin`, `SpawnResultsModal`, `ResultsModalData`, `ResultsSprite` |
| `plugin.rs` | `ResultsModalPlugin` and system registration |
| `state.rs` | Components, resources, sprite marker |
| `input.rs` | Close handling (Enter or Escape) |
| `render.rs` | UI spawning via Modal builder |

## Key Resources

### ResultsModalData

Stores the results for display:

```rust
#[derive(Resource)]
pub struct ResultsModalData {
    pub title: String,              // Modal title ("Victory!", "Chest Opened!")
    pub subtitle: Option<String>,   // Secondary text (mob name, etc.)
    pub sprite: Option<ResultsSprite>, // What sprite to show
    pub gold_gained: Option<i32>,   // None = don't show gold line
    pub xp_gained: Option<i32>,     // None = don't show XP line
    pub loot_drops: Vec<LootDrop>,
}
```

### ResultsSprite

```rust
pub enum ResultsSprite {
    Mob(MobId),  // Shows animated mob sprite via SpriteMarker
}
```

### SpawnResultsModal

Marker resource to trigger modal spawn. Removed by `spawn_results_modal` system.

## Usage

### After Mob Victory (fight_modal/input.rs)

```rust
commands.insert_resource(ResultsModalData {
    title: "Victory!".to_string(),
    subtitle: Some(mob_name),
    sprite: Some(ResultsSprite::Mob(mob_id)),
    gold_gained: Some(rewards.gold_gained),
    xp_gained: Some(rewards.xp_gained),
    loot_drops: death_result.loot_drops,
});
commands.insert_resource(SpawnResultsModal);
```

### After Chest Opening (dungeon/plugin.rs)

```rust
commands.insert_resource(ResultsModalData {
    title: "Chest Opened!".to_string(),
    subtitle: None,
    sprite: None,
    gold_gained: None,
    xp_gained: None,
    loot_drops,
});
commands.insert_resource(SpawnResultsModal);
```

## UI Layout

Uses `Modal::new().title(&data.title).size(300, 0)` with a Column containing:
1. Sprite (128x128, if `ResultsSprite::Mob`)
2. Subtitle (24px white, if present)
3. Gold gained (22px gold color, if present)
4. XP gained (22px light blue, if present)
5. Loot items (20px white, each on own line)

## Flow

1. Caller inserts `ResultsModalData` and `SpawnResultsModal` resources
2. `spawn_results_modal` system runs (gated on `resource_exists::<SpawnResultsModal>`)
3. Sets `ActiveModal.modal = Some(ModalType::ResultsModal)`
4. Modal displays until Enter or Escape
5. `handle_results_modal_close` removes `ResultsModalData` on close

## Not a RegisteredModal

Like the old VictoryModal and FightModal, the results modal is spawned programmatically and not toggled via the navigation system. Its `ModalType::ResultsModal` variant is in the non-toggle arm of `handle_modal_toggle`.

## Text Colors

- Title: cream (0.95, 0.9, 0.7) - from Modal builder
- Subtitle: white
- Gold: gold (1.0, 0.84, 0.0)
- XP: light blue (0.6, 0.8, 1.0)
- Loot items: white

## Related Documentation

- [fight-modal.md](fight-modal.md) - Fight modal (spawns results modal on mob death)
- [dungeon/entities.md](dungeon/entities.md) - Chest interaction system
- [modals.md](modals.md) - General modal patterns
- [modal-builder.md](modal-builder.md) - Modal builder API
- [sprite-marker.md](sprite-marker.md) - SpriteMarker trait
