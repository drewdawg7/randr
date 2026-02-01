# Health Bar System

Sprite-based health bars with HP text overlay.

**File:** `src/ui/screens/health_bar.rs`

## Core Components

| Component | Purpose |
|-----------|---------|
| `SpriteHealthBar` | Marker for sprite-based health bar entity |
| `HealthBarValues` | Drives updates: stores `current` and `max` HP |
| `HealthBarText` | Marker for the HP text child entity |

## How It Works

### Spawning a Health Bar

Use `SpriteHealthBarBundle` for spawning health bars:

```rust
use crate::ui::screens::health_bar::SpriteHealthBarBundle;

parent.spawn((
    MyHealthBarMarker,
    SpriteHealthBarBundle::new(current_hp, max_hp, width, height),
));
```

The bundle includes `SpriteHealthBar`, `HealthBarValues`, and `Node` with the specified dimensions.

### Initialization (`init_sprite_health_bars`)

Runs once per health bar entity (detects `SpriteHealthBar` without `ImageNode`):
1. Inserts the `ImageNode` with the full-health sprite atlas and 9-slice border
2. Sets `justify_content: Center` and `align_items: Center` on the node
3. Spawns a `HealthBarText` child entity (white, 10px font) for the HP text overlay

### Updating (`update_sprite_health_bar_visuals`)

Runs every frame for entities with `SpriteHealthBar` + `HealthBarValues` + `ImageNode` + `Children`:
1. Computes percentage from `values.current / values.max`
2. Updates the `ImageNode` texture atlas index via `HealthBarSlice::for_percent()`
3. Updates the `HealthBarText` child with format `"{current} / {max}"`

Only mutates when values actually differ (avoids unnecessary change detection).

### Data-Driven Pattern

The fight modal demonstrates the intended pattern:
- **Data writers** (domain-specific systems) write to `HealthBarValues`
- **Visual updater** (generic `update_sprite_health_bar_visuals`) handles rendering

```rust
// Domain system: just writes values
pub fn update_mob_health_bar(
    fight_mob: Res<FightModalMob>,
    mut bar_query: Query<&mut HealthBarValues, With<FightModalMobHealthBar>>,
) {
    let Ok(mut values) = bar_query.get_single_mut() else { return };
    values.current = fight_mob.mob.hp();
    values.max = fight_mob.mob.max_hp();
}
```

## System Registration

Both `init_sprite_health_bars` and `update_sprite_health_bar_visuals` must be registered in plugins that use sprite health bars:

```rust
use crate::ui::screens::health_bar::{init_sprite_health_bars, update_sprite_health_bar_visuals};

app.add_systems(Update, (
    init_sprite_health_bars,
    update_sprite_health_bar_visuals,
    // ... other systems that write HealthBarValues ...
));
```

## Legacy Pattern: Container-Based Health Bars

The `fight` screen (not `fight_modal`) uses an older container pattern with `HealthBar` + `HealthBarBundle` + child `SpriteHealthBar` and `HealthBarText`. This uses the `update_health_bar()` helper function which takes explicit entity/query parameters.

The newer `HealthBarValues` + `update_sprite_health_bar_visuals` pattern is preferred for new code.
