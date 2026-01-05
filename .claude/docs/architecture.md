# Codebase Architecture

## Overview

Terminal-based RPG game built with Rust using `ratatui` and `tuirealm` for TUI.

## Key Directories

```
src/
├── main.rs              # Entry point, main loop
├── system.rs            # Global GameState singleton
├── ui/                  # Screen & component system
│   ├── components/      # TUI components
│   └── screens/         # Screen definitions
├── entities/
│   ├── player.rs        # Player struct
│   ├── mob.rs           # Enemy definitions
│   └── progression.rs   # XP/leveling traits
├── item/                # Item system
├── combat/              # Combat system
├── mine/                # Mining system
├── blacksmith/          # Upgrade system
├── store/               # Shop system
├── stats/               # Stat system
└── town/                # Town hub
```

## Core Patterns

### Global State (`system.rs`)
```rust
pub fn game_state() -> &'static mut GameState {
    unsafe { STATE.as_mut().unwrap() }
}
```

### Registry Pattern (`registry.rs`)
```rust
Registry<K, V>  // Generic registry
RegistryDefaults<K>  // Default specs
SpawnFromSpec<K>  // Instance creation
```

### Trait Composition
- `Combatant` - Combat capabilities
- `HasStats` - Stat manipulation
- `HasInventory` - Equipment
- `HasProgression` - XP/levels
- `GivesXP`, `DropsGold` - Rewards

### UI System
- Screens identified by `ui::Id` enum
- Components implement `tuirealm::Component`
- Wrappers: `ModalWrapper`, `TabbedContainer`

## Data Flow

```
User Input
    ↓
Screen Handler
    ↓
Component Message
    ↓
GameState Mutation
    ↓
UI Re-render
```

## Adding New Features

1. Define spec in registry (if entity)
2. Implement required traits
3. Add UI component if needed
4. Register with appropriate system
