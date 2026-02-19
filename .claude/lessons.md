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
- Don't split simple log queries into multiple commands. Combine into one call or run them in parallel.
  - Bad: 1) find latest log, 2) filter it, 3) filter again separately
  - Good: single jq command with inline log path lookup

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

## Tracing Code
- When asked to trace where something comes from, READ THE CODEBASE directly. Don't use git log, git blame, or search for random files.
- Follow imports and function calls to find the source of data.
- Don't brute force search - trace the actual code path.

## Planning
- Understand the actual goal before proposing solutions. Don't just revert to old patterns - find a forward path that achieves the user's intent.
- When a refactor breaks something, the fix should align with the refactor's goals, not undo them.
- Don't enter plan mode unless the user explicitly asks. When given a well-defined issue with a detailed checklist, just execute it.
- When something "doesn't work", verify the specific failure mode before assuming the cause. "X doesn't work" could mean X fails silently, X triggers wrong behavior, or X is never called.
- Avoid special cases and fallbacks - there should be one consistent way of doing things. If similar things are handled differently, unify the approach.

## Bevy Systems
- When a system doesn't run, check its `run_if` conditions. If it uses `any_with_component::<T>`, verify that `T` is actually added to entities (check spawn/bundle code).
- Systems that poll for transient state (like animation markers) should have `run_if` conditions to avoid wasting cycles every frame. Don't dismiss unnecessary per-frame work as "harmless".
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
- Before creating a new branch, check for uncommitted changes on any existing branch. If there are uncommitted changes, commit them and merge that branch into main FIRST, then create the new branch. Never just switch branches and leave work behind.

## Following Instructions
- When the user says they updated files to match a pattern you already applied, just apply the same pattern. Don't re-inspect the files — trust the user.
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
- When the user asks for research, go DEEP. Don't present surface-level summaries with "simple alternatives". Research the actual topic requested, not workarounds.
- Don't propose simple workarounds when the user explicitly asks about a specific technique (e.g., don't suggest sprite tinting when they ask about Material2d shaders).

## Skills & Project Knowledge
- Check project skills BEFORE searching the filesystem for paths, tools, or conventions. Skills contain critical info like tool locations and workflows.
- Aseprite is on the PATH - just use `aseprite` directly. Don't use the full application path.
- When the user says information "is already there", check skills and docs before searching elsewhere.

## Planning
- When a plan is approved, execute it. Don't keep second-guessing or changing the approach based on misinterpretations of feedback.
- Show actual code in plans, not just high-level descriptions. The user needs to see what will be implemented.
- When data already exists in the file (slices, metadata, etc.), USE IT directly via the API. Don't write manual scanning/detection code when the information is already structured.
- Never assume a bug is in a library/API. Always assume you are doing something wrong first.

## Aseprite Scripting
- The `aseprite_add_tags.lua` script is DESTRUCTIVE — it overwrites the source file. ALWAYS copy source files from Downloads/aseprite_sprites first, then run the script on the copies. Never run on originals.
- When the user says "convert files from X to Y", the workflow is: copy from X, then convert. Don't skip the copy step.
- When copying multiple files from a source, verify ALL files were copied — don't assume any are "already correct" without checking file size/integrity.
- Source filenames in Downloads don't match destination names (e.g., MiniNobleMan → merchant, MiniDwarfShieldbreaker → dwarf_defender). Always refer to the issue's filename mapping.
- `aseprite -b --list-tags` can return empty for valid multi-layer files. Don't trust CLI output alone — check file size and compare against originals.
- Some aseprite files already have proper tags (e.g., merchant has idle/walk/jump/damage/death) and don't need the conversion script. Check before converting.
- Aseprite tag names must be short (~4 chars max like "a_1") or the timeline display breaks with staggering/nesting artifacts.
- When creating tags programmatically, create ALL frames first, THEN add all tags in a separate pass. Aseprite auto-extends existing tags when new frames are added after them.
- When scanning sprite sheet rows for content, don't break at the first empty cell — scan the entire row and include up to the last non-empty cell (animations can have gaps).

## Bevy Ecosystem
- ALWAYS prefer Bevy ecosystem crates over building custom solutions. The entire point of using Bevy is to leverage its ecosystem.
- Never recommend "build in-house" when a Bevy crate exists for the functionality.
- When evaluating solutions, search for existing Bevy crates first before designing custom implementations.
- When adopting a crate, use it directly. Don't create wrapper abstractions or registry patterns that just duplicate the crate's functionality. Simplify by removing existing code, not by wrapping new code in old patterns.
- When an issue says existing implementations are being REPLACED, do NOT explore or reference those old implementations. They are the wrong pattern. Only follow the issue's target pattern.

## GitHub Issues
- NEVER create checklist-style issues. Checklists are not actionable.
- Issues must detail concrete code changes: what structs/systems to add/modify/remove, with code examples showing the target state.
- Include enough context and code examples that someone could implement from the issue alone.
- Show what the code should look like, not just what steps to take.
