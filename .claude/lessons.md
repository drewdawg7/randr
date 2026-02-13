# Lessons

## Edits
- Make edits in bulk. Don't make many small sequential edits to the same file or across files - batch them together to reduce approval overhead.
- Use ast-grep for bulk code transformations across multiple files, not sed or individual Edit calls.
- When using ast-grep, don't specify individual file names - just pass the directory and let it find all matches. That's the point of using it.

## Bash Commands
- Do not use variables/aliases for single-use commands. Pipe directly or use command substitution inline instead of storing in a variable first.
  - Bad: `LOG=$(ls -t logs | head -1) && tail -100 "$LOG"`
  - Good: `tail -100 logs/$(ls -t logs | head -1)`
- Use relative paths from the working directory, not absolute paths.
  - Bad: `/Users/drewstewart/code/game/logs/`
  - Good: `logs/`

## GitHub CLI (gh)
- Don't specify `--repo` when inside a git repository. The `gh` command automatically detects the repo from the git remote.
  - Bad: `gh issue view 514 --repo username/repo`
  - Good: `gh issue view 514`

## Rust Code Navigation
- Use LSP operations instead of Grep for Rust code. See CLAUDE.md for the full table.
- Use `ast-grep --pattern 'PATTERN' --lang rust src/` for structural patterns across files.

## Logging
- Use `#[instrument]` attribute for tracing, not `debug!` macro calls.
- Put computed values directly in `fields()` when possible: `#[instrument(fields(count = query.iter().count()))]`
- When values can only be computed inside the function body, use `Span::current().record()` with empty field declarations: `#[instrument(fields(coal_qty))]` then `tracing::Span::current().record("coal_qty", value);`

## Debugging
- When debugging, add instrumentation and test immediately rather than endlessly analyzing code. Logs provide concrete answers faster than speculation.
- When logs are available, read them first before jumping to code analysis.
- Follow through: if you add instrumentation, actually run and read the logs before implementing a fix. Don't skip steps.
- Always include logging/instrumentation in fix plans - never propose a fix without visibility into what's happening.
- Don't try to run the game - you can't interact with it. Ask the user to run and report back.

## Planning
- Understand the actual goal before proposing solutions. Don't just revert to old patterns - find a forward path that achieves the user's intent.
- When a refactor breaks something, the fix should align with the refactor's goals, not undo them.
- Don't enter plan mode unless the user explicitly asks. When given a well-defined issue with a detailed checklist, just execute it.
- When something "doesn't work", verify the specific failure mode before assuming the cause. "X doesn't work" could mean X fails silently, X triggers wrong behavior, or X is never called.
- Avoid special cases and fallbacks - there should be one consistent way of doing things. If similar things are handled differently, unify the approach.

## Bevy Systems
- When a system doesn't run, check its `run_if` conditions. If it uses `any_with_component::<T>`, verify that `T` is actually added to entities (check spawn/bundle code).
- `resource_changed::<T>()` requires the resource to exist - it panics if the resource is missing. For optional resources (like `FocusState` which is only inserted when modals open), wrap with: `resource_exists::<T>().and(resource_changed::<T>())`

## bevy_ecs_tiled / bevy_ecs_tilemap
- Tiles have `TilePos` component (grid position), NOT Transform/GlobalTransform
- Do not add Transform to tile entities - rendering is handled at the TiledTilemap level
- To get world position: query the tilemap entity and use its transform + tile grid position
- TileBundle fields: position (TilePos), texture_index, tilemap_id, visible, flip, color, old_position, sync

## Workflow
- Commit changes as you go. After completing a logical unit of work, run styleguide then commit immediately. Don't accumulate uncommitted changes.
- Run the styleguide skill before every commit to catch issues early.
- Don't run unnecessary commands (like `git status`) when you already know the state from your own actions.
- Don't ask permission to commit. Just commit when the work is ready.
- User verification happens at the end, after all commits are made. Don't wait for verification before committing.

## Following Instructions
- When the user interrupts, STOP immediately and update lessons.md with what was learned from the correction before doing anything else.
- When the user explicitly says NOT to do something (e.g., "do not look at other project skills"), follow that instruction exactly.
- Don't assume existing project patterns are valid examples when told to use external sources instead.
- Read user instructions carefully before starting work.
- Only do what the user explicitly asks. Do not take additional steps beyond the request. If asked to create a GitHub issue, create the issue and stop - don't start implementation, create branches, or take other actions.
- When corrected, don't immediately take more actions to "fix" things. Stop and wait for instructions.
- ALWAYS read lessons.md before starting any work - planning, exploration, or implementation. This is explicitly stated in CLAUDE.md.

## Comments
- Do not add comments to code. The styleguide explicitly forbids unnecessary comments.
- Doc comments on public structs/types are also unnecessary unless they explain something non-obvious.

## Naming
- Don't use "handle_X" naming pattern - this is JavaScript/React, not idiomatic Rust/Bevy.
- Name systems for what they do, not "handle_X".

## Module Organization
- Organize modules by domain, not by technical concern.
  - Good: `input/combat.rs`, `input/navigation.rs`
  - Bad: `combat/systems/attack_input.rs` (input logic in combat module)
- Don't create redundant files like `action.rs` + `action_combat.rs` - consolidate.

## Bevy Patterns
- Use observers for reactive logic (e.g., `OnAdd<Component>` to spawn related entities).
- Input systems should only set state/components, not perform game logic.
- Don't couple input handling to game systems - use components/observers to decouple.
- Use `With<T>` filter instead of querying a component you don't need to read. Avoids unused variables.

## Constants and Values
- A constant is still a magic number if it's not derived from anything meaningful.
- Derive values from existing config/constants when possible (e.g., hitbox offset from player collider config).
- When values depend on other entities, query them at runtime rather than hardcoding.

## Research
- When asked about best practices, do web research - don't just explore the codebase.
- Search for relevant terms (e.g., "hitbox" when working on hitboxes, not just "ECS patterns").

## Bevy Ecosystem
- ALWAYS prefer Bevy ecosystem crates over building custom solutions. The entire point of using Bevy is to leverage its ecosystem.
- Never recommend "build in-house" when a Bevy crate exists for the functionality.
- When evaluating solutions, search for existing Bevy crates first before designing custom implementations.
- When adopting a crate, use it directly. Don't create wrapper abstractions or registry patterns that just duplicate the crate's functionality. Simplify by removing existing code, not by wrapping new code in old patterns.
