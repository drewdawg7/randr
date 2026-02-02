---
name: logging
description: Adds tracing instrumentation to Rust code. Use when adding [instrument], debugging systems, or viewing logs.
---

# Logging

## Quick Start

Add `#[instrument]` to functions that need tracing:

```rust
use tracing::instrument;

#[instrument(level = "debug", skip_all)]
fn my_system(query: Query<&Player>, state: Res<GameState>) {
    // Function body
}
```

## Log Location

Logs are written to `logs/game_{timestamp}.log`.

View the latest log:
```bash
tail -f logs/$(ls -t logs | head -1)
```

## Patterns

| Pattern | When to Use |
|---------|-------------|
| `skip_all` | Default for systems (avoids logging large Bevy types) |
| `skip(self)` + `fields()` | Methods where you want specific context |
| `ret` | Functions with meaningful return values |
| `fields(key = ?value)` | Add custom context (positions, counts, entities) |

## Examples

```rust
// System with skip_all (most common)
#[instrument(level = "debug", skip_all)]
fn handle_player_move(...) { }

// Method with explicit fields
#[instrument(level = "debug", skip(self), fields(pos = ?pos, entity = ?entity))]
pub fn occupy(&mut self, pos: GridPosition, entity: Entity) { }

// Log return value
#[instrument(level = "debug", ret)]
pub fn apply_defense(raw_damage: i32, defense: i32) -> i32 { }

// Computed field
#[instrument(level = "debug", skip_all, fields(entity_count = layout.entities().len()))]
fn spawn_entities(...) { }
```

## When to Add Logging

- New Bevy systems handling game logic
- Functions with complex branching or state changes
- Code you're debugging
- Entry points for major operations

## Configuration

Filter in `src/main.rs`: `"warn,game=debug"`
- `debug` level for game crate
- `warn+` for dependencies

## Checklist

- [ ] Added `use tracing::instrument;`
- [ ] Used `level = "debug"`
- [ ] Used `skip_all` for systems
- [ ] Added `fields()` only for key context
- [ ] Avoided instrumenting hot loops

## Querying JSON Logs

Logs are stored as JSON Lines. Use `jq` to filter efficiently:

```bash
LOG=$(ls -t logs/*.log | head -1)

# Errors only
jq 'select(.level == "ERROR")' "$LOG"

# Specific module
jq 'select(.target | contains("combat"))' "$LOG"

# Last N entries
tail -50 "$LOG" | jq .

# Span entries for function
jq 'select(.span.name == "handle_player_move")' "$LOG"

# Events with specific field
jq 'select(.fields.entity != null)' "$LOG"
```
