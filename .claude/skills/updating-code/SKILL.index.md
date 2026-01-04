# updating-code Documentation Index

> Codebase documentation for a Rust TUI RPG game using ratatui/tuirealm.
> Use LSP for code navigation. Use ast-grep for refactoring. Always branch first.

## Quick Reference

- [SKILL.md](SKILL.md): Step-by-step code change procedure, parallelization decision tree
- [refactoring.md](refactoring.md): ast-grep patterns for automated renames, function/type/field refactoring

## Game Systems

- [dungeon/overview.md](dungeon/overview.md): Procedural 5x5 grid dungeon, DungeonRoom, DungeonState, minimap with fog of war, room types (Monster/Chest/Rest/Boss), dragon boss fight, compass navigation
- [combat/overview.md](combat/overview.md): Turn-based combat, Combatant/DealsDamage/IsKillable traits, damage formula with ±25% variance, defense reduction (K=50), CombatSource, ActiveCombat state
- [magic/overview.md](magic/overview.md): Word-based spell system, WordId/WordSpec/WordProperties, Page/Tome structures, compute_spell() hybrid computation, ActiveEffect/PassiveEffect enums, tome equipment, spell casting in combat

## Entities

- [entities/items.md](entities/items.md): ItemId registry, ItemSpec, EquipmentType/EquipmentSlot enums, ItemQuality multipliers (0.8-1.8x), armor crafting costs
- [entities/mob.md](entities/mob.md): MobId registry, MobSpec with stat ranges, MobRegistry, spawn weights, IsKillable death_processed guard, HasLoot trait integration
- [entities/loot.md](entities/loot.md): LootTable probability system, LootDrop struct, HasLoot trait, roll_drops() flow for combat and mining
- [entities/recipes.md](entities/recipes.md): RecipeId/RecipeType enums, Recipe::craft() returns ItemId, RecipeSpec, forging/smelting/alchemy recipes
- [entities/stats.md](entities/stats.md): StatSheet HashMap wrapper, StatType (Health/Attack/Defense/GoldFind/Mining), StatInstance, HasStats trait, Healable trait with blanket impl

## Locations

- [location/overview.md](location/overview.md): Location trait (identity, timer, entry/exit), LocationId, LocationSpec, LocationData enum, Town integration with location fields
- [location/adding-locations.md](location/adding-locations.md): Step-by-step checklist for new location types, key files to modify, submodule structure
- [location/mine.md](location/mine.md): Mine system with CaveLayout, rock respawn (2 min), mine regeneration (10 min), timer display, procedural cave generation

## UI Framework

- [ui/architecture.md](ui/architecture.md): Screen lifecycle (just_entered, came_from), UIState struct, selection widgets (ListSelection/BinaryToggle/GridSelection/DirectionalSelection), children-first event routing, MockComponent/Component traits, ModalWrapper, TabbedContainer, command architecture
- [ui/layout.md](ui/layout.md): Layout centering with Constraint::Fill ratios, MENU_HEIGHT/MENU_WIDTH patterns
- [ui/backgrounds.md](ui/backgrounds.md): Stone wall/decorative border patterns, tiling approach, explicit foreground colors, direct buffer rendering for background preservation
- [ui/toast-system.md](ui/toast-system.md): ToastQueue API (error/success/info), ToastType enum, 3-second auto-dismiss, wired error locations
- [ui/item-list-widget.md](ui/item-list-widget.md): ItemList<T,F> reusable widget, ListItem/ItemFilter traits, InventoryFilter/ForgeFilter, wrapper types (InventoryListItem, StoreBuyItem, etc.)

## Economy

- [economy.md](economy.md): WorthGold trait, gold_value/purchase_price/sell_price methods, quality multipliers

## Storage

- [storage.md](storage.md): Storage system for depositing/withdrawing items, accessible from Store screen, DepositItem/WithdrawItem commands, dual-panel UI
