---
name: updating-code
description: Required workflow for ALL code changes - invoke FIRST before any implementation. Use when adding features, fixing bugs, refactoring, editing files, modifying code, updating functions, changing behavior, writing new code, deleting code, working on issues, or making any changes to the codebase. Covers git branching, LSP navigation, testing, and merge process.
---

## Workflow
Follow this workflow for ALL code changes:
**IMPORTANT**: CREATE NEW BRANCHES EVEN FOR CHANGES NOT RELATED TO GITHUB ISSUES.

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
- [blacksmith.md](blacksmith.md) - Blacksmith module, crafting helper pattern, `CraftingOperation` enum, `UpgradeOperation` enum, recipe system, `RecipeId::material()`
- [event-systems.md](event-systems.md) - Event handler patterns, `run_if(on_event::<T>)` requirement, files in `src/game/`
- [mob-sprites.md](mob-sprites.md) - Adding mob sprites, `SpriteAssets::mob_sprite()`, `populate_mob_sprite` system
- [ui-nodes.md](ui-nodes.md) - UI node helpers (`row_node`, `column_node`), reducing boilerplate for flex containers


## Documentation
- Upon completion of a code change, documentation must be added to the updating-code skill.
- Each module should get its own file in .claude/updating-code. If a file starts to get close to 500 lines, create a subdirectory, break out the file, and place them all the subdirectory.
- If any new files are created, update the documentation index above. The goal of the documentation index is to make it easy to find relevant code or guidance in the future.
- Use examples, file names, function names, etc in the document to keep research quick and efficient.
- Documentation should cover both low-level and high-level concepts and areas. It should cover game systems and UI implementations.
- Documentation can include decisions made if it will help guide changes in the future.
- When new files are added or files are changed update the documentation index.
- Even if the change is consistent with existing coding patterns, it should still be added to the documentation to ensure similar changes are consistent in the future.
