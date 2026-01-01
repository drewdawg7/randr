# CLAUDE.md



This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.
## Skills
1. updating-code: The skill must be used for any changes to code, this includes while planning changes. Plans should include information from this skill.
2. ascii-art: The skill must be used whenever ascii art needs to be made or edited.

## Build Commands

```bash
cargo build          # Build the project
cargo run            # Run the game
cargo check          # Type check without building
cargo test           # Run tests (none currently exist)
```

## Architecture Overview

This is a terminal-based RPG game built with Rust using the `ratatui` and `tuirealm` libraries for TUI rendering.

### Global State Pattern

The game uses a global mutable `GameState` singleton accessed via `game_state()` (defined in `src/system.rs`). This pattern allows any module to access game state, though it requires `unsafe` access.

### Screen/Component System

The UI is screen-based using `tuirealm::Application`. Screens are identified by `ui::Id` enum (Menu, Town, Fight, Profile, Mine, Quit). The main loop in `main.rs` calls `run_current_screen()` until `Id::Quit` is reached.

UI components in `src/ui/components/` implement `tuirealm::Component`. Common wrapper patterns:
- `ModalWrapper` - wraps screens with modal overlay support (keybinds, inventory)
- `TabbedContainer` - combines multiple components as tabs

### Registry Pattern

Entities (items, mobs, rocks) use a generic `Registry<K, V>` pattern (`src/registry.rs`) with:
- `RegistryDefaults<K>` trait - provides default specs
- `SpawnFromSpec<K>` trait - creates instances from specs
- Specs define static data, spawned instances are mutable

### Trait-Based Systems

Core behaviors are implemented as traits allowing composition:
- **Combat**: `Combatant`, `DropsGold`, `HasGold` (in `combat/traits.rs`)
- **Stats**: `HasStats` for stat manipulation (hp, attack, defense)
- **Progression**: `HasProgression`, `GivesXP` (in `entities/progression.rs`)
- **Inventory**: `HasInventory` for equipment management

### Key Modules

- `entities/player` - Player struct with stats, inventory, progression
- `entities/mob` - Enemy definitions with MobRegistry
- `combat/system` - Turn-based combat logic via `enter_combat()` function
- `item/` - Item system with ItemId, ItemType (Weapon/Shield), upgrades
- `mine/` - Mining system with RockRegistry for mineable resources
- `blacksmith/` - Item upgrade system with gold cost
- `store/` - Shop system for purchasing items
- `stats/` - StatSheet with StatType enum (Health, Attack, Defense)
- `town/` - Town structure containing store, blacksmith, field, and mine
