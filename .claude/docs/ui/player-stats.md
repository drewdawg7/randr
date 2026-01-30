# Player Stats Banner

Top-of-screen HUD widget displaying player HP, XP/Level, and Gold.

**File:** `src/ui/widgets/player_stats.rs`

## Architecture

The widget uses an **observer + reactive update systems** pattern:

1. `PlayerStats` marker component triggers `on_add_player_stats` observer on spawn
2. Observer builds the full UI with icons and text, reading initial values from resources
3. Per-frame update systems with change detection keep text in sync with resources

## Components

| Component | Purpose |
|-----------|---------|
| `PlayerStats` | Marker; triggers observer to build widget UI |
| `PlayerHpText` | Marker on HP text entity for reactive updates |
| `PlayerXpText` | Marker on XP/Level text entity for reactive updates |
| `PlayerGoldText` | Marker on gold text entity for reactive updates |

## Data Sources

| Display | Resource | Format |
|---------|----------|--------|
| HP | `Res<StatSheet>` | `"{current}/{max}"` (red, 16px) |
| XP/Level | `Res<Progression>` | `"Level: {lvl}  XP: {xp}/{needed}"` (green, 16px) |
| Gold | `Res<PlayerGold>` | `"{amount}"` (gold, 16px) |

## Reactive Update Systems

All three follow the same pattern — check `resource.is_changed()`, reformat text:

```rust
fn update_hp_display(stats: Res<StatSheet>, mut query: Query<&mut Text, With<PlayerHpText>>) {
    if stats.is_changed() { /* update text */ }
}

fn update_xp_display(progression: Res<Progression>, mut query: Query<&mut Text, With<PlayerXpText>>) {
    if progression.is_changed() { /* update text */ }
}

fn update_gold_display(gold: Res<PlayerGold>, mut query: Query<&mut Text, With<PlayerGoldText>>) {
    if gold.is_changed() { /* update text */ }
}
```

All registered in `PlayerStatsPlugin::build` as `Update` systems.

## Plugin Registration

```rust
pub struct PlayerStatsPlugin;

impl Plugin for PlayerStatsPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(on_add_player_stats)
            .add_systems(Update, (update_gold_display, update_hp_display, update_xp_display));
    }
}
```

## Where the Banner is Spawned

- **Town screen:** `src/ui/screens/town/systems.rs` — `parent.spawn(PlayerStats);`
- **Dungeon screen:** `src/ui/screens/dungeon/plugin.rs` — `parent.spawn(PlayerStats);`

The banner is rebuilt on each screen entry (`OnEnter` state transition), but the reactive update systems ensure values stay current even within a single screen (e.g., after combat in the dungeon).

## Visual Layout

```
[heart icon] 45/100    Level: 3  XP: 120/200    [coin icon] 350
```

Background uses a nine-slice sprite: `TravelBookSlice::Banner` (16px border).

## Adding New Stats

To add a new reactive stat display:
1. Create a marker component (e.g., `PlayerNewStatText`)
2. Spawn the text entity with the marker in `on_add_player_stats`
3. Add an update system following the `is_changed()` pattern
4. Register the system in `PlayerStatsPlugin::build`
