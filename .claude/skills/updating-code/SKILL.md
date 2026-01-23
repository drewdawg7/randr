---
name: updating-code
description: Required workflow for ALL code changes - invoke FIRST before any implementation. Use when adding features, fixing bugs, refactoring, editing files, modifying code, updating functions, changing behavior, writing new code, deleting code, working on issues, or making any changes to the codebase. Covers git branching, LSP navigation, testing, and merge process.
---

## Workflow
Follow this workflow for ALL code changes:
**IMPORTANT**: CREATE NEW BRANCHES EVEN FOR CHANGES NOT RELATED TO GITHUB ISSUES.
**IMPORTANT**: REFERENCE THE DOCUMENTATION EARLY AND OFTEN

1. **Branch**: Create a new branch with descriptive name (e.g., `feat/add-inventory`)
2. **Analyze and Research**: Use ast-grep and Rust LSP to understand the codebase. The `rust-codebase-researcher` agent is skilled at this.
    1. Use the sprites skill when working with sprites or UI. 
    2. Use the documentation index to quickly find relevant documentation to the issue at hand.
3. **Ask**: Clarify any ambiguity with the user before proceeding
4. **Compare**: Check similar functionality in the codebase for patterns
5. **Make Changes**: Execute your plan
6. **Test**: Run tests for changed modules only.
7. **Clean-Up**: Fix any compiler warnings related to your changes
8. **Verify**: Ask user to verify changes
9. **Merge**: Commit, merge, and push. No PR necessary.
10. **Close**: If working on a GitHub issue, close it
11. **Document**: Update documentation based on the documentation section below.
    1. Documentation is NOT a choice. You should always add additional documentation. 


## Documentation Index

Read the relevant doc BEFORE making changes.

| When working on... | Read |
|--------------------|------|
| Crafting recipes, upgrades, blacksmith | [blacksmith.md](blacksmith.md) |
| Store buy/sell, Store resource, StoreItem | [store.md](store.md) |
| Store screen UI, item grids, detail panels | [store-ui.md](store-ui.md) |
| Dungeon layouts, TileType, DungeonLayout | [dungeon/mod.md](dungeon/mod.md) |
| DungeonPlugin, DungeonRegistry, location floors | [dungeon/mod.md](dungeon/mod.md) |
| FloorSpec, FloorId, floor definitions | [dungeon/mod.md](dungeon/mod.md) |
| DungeonEntity enum, entity spawning | [dungeon/entities.md](dungeon/entities.md) |
| DungeonCommands, entity despawning, occupancy vacate | [dungeon/entities.md](dungeon/entities.md) |
| Dungeon tab rendering, tile slices | [dungeon/ui.md](dungeon/ui.md) |
| Dungeon movement, SmoothPosition, interpolation | [dungeon/ui.md](dungeon/ui.md) |
| EntityLayer, entity absolute positioning | [dungeon/ui.md](dungeon/ui.md) |
| Animated tiles, torch walls, TorchWall | [dungeon/ui.md](dungeon/ui.md) |
| Fight screen UI, action menu, post-combat overlay | [fight-screen.md](fight-screen.md) |
| Fight modal, dungeon combat, mob encounters | [fight-modal.md](fight-modal.md) |
| Results modal, post-fight/chest rewards, loot display | [results-modal.md](results-modal.md) |
| Chest interaction, adjacency detection, chest loot | [dungeon/entities.md](dungeon/entities.md) |
| Rock mining, RockType, rock loot, mine interaction | [dungeon/entities.md](dungeon/entities.md) |
| Health bars, HealthBarValues, SpriteHealthBar, HP text | [health-bar.md](health-bar.md) |
| Event handlers, run_if(on_event::<T>) | [event-systems.md](event-systems.md) |
| SelectionState trait, list navigation, focus | [focus.md](focus.md) |
| Mob animations, MobSpriteSheets, MobAnimationConfig | [mob-sprites.md](mob-sprites.md) |
| Option handling, Query aliases, saturating math | [rust-idioms.md](rust-idioms.md) |
| Item sprite assignments, icon-to-slice mapping | [item-sprites.md](item-sprites.md) |
| SpriteSheet, GameSprites, image_bundle helpers | [sprites.md](sprites.md) |
| SpriteMarker trait, SpriteAnimation, sprite population | [sprite-marker.md](sprite-marker.md) |
| UiAllSlice, HealthBarSlice, NineSlice, ThreeSlice traits | [sprite-slices.md](sprite-slices.md) |
| ItemDetailIconsSlice, stat display icons | [stat-icons.md](stat-icons.md) |
| StatRow, ItemGrid, GoldDisplay widgets | [widgets/](widgets/table-of-contents.md) |
| Row, Column, Stack layout components | [widgets/layout_primitives.md](widgets/layout_primitives.md) |
| row_node, column_node, overflow, layout | [ui-nodes.md](ui-nodes.md) |
| Decoupling UI from registries, display structs | [ui-display-data.md](ui-display-data.md) |
| Modal screens, ActiveModal, toggle/close patterns | [modals.md](modals.md) |
| Modal builder API, SpawnModalExt, ModalBackground | [modal-builder.md](modal-builder.md) |
| RegisteredModal trait, ModalCommands, toggle_modal | [modal-registry.md](modal-registry.md) |
| Navigation system, state transitions, NavigationPlugin | [navigation.md](navigation.md) |
| Key repeat, InputPlugin, arrow key hold behavior | [navigation.md](navigation.md) |
| Player walk animation, sprite flip, PlayerWalkTimer | [sprite-marker.md](sprite-marker.md) |
| Inventory modal, ItemGrid display, inventory UI | [inventory-modal.md](inventory-modal.md) |
| Player stats banner, HP/XP/Gold HUD, reactive text | [player-stats.md](player-stats.md) |


## Documentation
- Upon completion of a code change, documentation must be added to the updating-code skill.
- Each module should get its own file in .claude/skills/updating-code. If a file starts to get close to 500 lines, create a subdirectory, break out the file, and place them all the subdirectory.
- If any new files are created, update the documentation index above. The goal of the documentation index is to make it easy to find relevant code or guidance in the future.
- Use examples, file names, function names, etc in the document to keep research quick and efficient.
- Documentation should cover both low-level and high-level concepts and areas. It should cover game systems and UI implementations.
- Documentation can include decisions made if it will help guide changes in the future.
- When new files are added or files are changed update the documentation index.
- Even if the change is consistent with existing coding patterns, it should still be added to the documentation to ensure similar changes are consistent in the future.
- Consider if the documentation should be placed in multiple places. i.e., updating how blacksmith recipes work should go into blacksmith.md, and also possibly a recipes.md file.
