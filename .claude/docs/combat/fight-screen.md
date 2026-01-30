# Fight Screen

Full-screen combat UI that appears during dungeon fights. Renders player/enemy health bars, mob sprite, action menu, and post-combat overlay.

## Module Structure

**Location:** `src/ui/screens/fight/`

| File | Purpose |
|------|---------|
| `mod.rs` | Module declarations and re-exports |
| `plugin.rs` | `FightPlugin`, system sets, system registration |
| `components.rs` | Marker components (`FightScreenRoot`, `PlayerHealthBar`, `EnemyHealthBar`, `NeedsMobSprite`, etc.) |
| `state.rs` | `FightScreenState` resource, `ActionSelection` / `PostCombatSelection` wrappers |
| `styles.rs` | Color constants and helpers for action menu selection styling |
| `spawn.rs` | `spawn_fight_screen` and UI tree construction (combatants, health bars, action items) |
| `actions.rs` | Action menu rendering, post-combat overlay, `reset_fight_state`, `update_action_visuals` |
| `systems.rs` | Populate systems (background, mob sprite, popup), `update_combat_visuals`, `cleanup_fight_screen`, `update_enemy_name` |
| `input.rs` | Player turn input handling, post-combat input handling |

## Style Constants (`styles.rs`)

Action menu uses a brown-themed color scheme distinct from the nav theme:

```rust
pub const SELECTED_TEXT_COLOR: Color = Color::srgb(0.15, 0.1, 0.05);
pub const UNSELECTED_TEXT_COLOR: Color = Color::srgb(0.4, 0.35, 0.3);
pub const SELECTED_SUFFIX: &str = " <";
```

Helper functions:
- `action_text_color(selected: bool) -> Color` - returns the appropriate color
- `action_label(label: &str, selected: bool) -> String` - formats label with suffix

## Populate Systems (`systems.rs`)

Three "needs" marker components trigger asset population when resources become available:

| Marker | System | Asset Source |
|--------|--------|--------------|
| `NeedsFightBackground` | `populate_fight_background` | `GameSprites::FightBackgrounds` |
| `NeedsMobSprite` | `populate_mob_sprite` | `MobSpriteSheets` |
| `NeedsFightPopup` | `populate_fight_popup` | `GameSprites::FightPopup` |

Pattern: Query for entities with marker → check if resource is ready → remove marker and insert image/sprite.

## Action Menu (`actions.rs`)

- `reset_fight_state` - resets selection to index 0 and updates all action item visuals
- `update_action_visuals` - called from input handlers after navigation; updates text and color
- `spawn_post_combat_overlay` / `despawn_post_combat_overlay` - victory/defeat overlay with "Fight Again" / "Continue" options
- Post-combat items use `nav_selection_text()` from the theme module (white/gray scheme)

## System Sets

```rust
pub enum FightSystemSet {
    Input,  // Handle player input
    Ui,     // Update visuals and overlays
}
```

Configured to run in order (Input → Ui) only when `AppState::Fight` is active.

## Related Documentation

- [fight-modal.md](fight-modal.md) - The older fight modal (different module at `fight_modal/`)
- [health-bar.md](health-bar.md) - Health bar system
- [sprite-marker.md](../sprites/sprite-marker.md) - SpriteMarker trait pattern
- [mob-sprites.md](../sprites/mob-sprites.md) - Mob sprite sheet loading
- [focus.md](../ui/focus.md) - SelectionState trait used by ActionSelection/PostCombatSelection
