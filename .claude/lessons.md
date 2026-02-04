# Lessons

## Bash Commands
- Do not use variables/aliases for single-use commands. Pipe directly or use command substitution inline instead of storing in a variable first.
  - Bad: `LOG=$(ls -t logs | head -1) && tail -100 "$LOG"`
  - Good: `tail -100 logs/$(ls -t logs | head -1)`
- Use relative paths from the working directory, not absolute paths.
  - Bad: `/Users/drewstewart/code/game/logs/`
  - Good: `logs/`

## Rust Code Navigation
- Use LSP operations instead of Grep for Rust code. See CLAUDE.md for the full table.
- Use `ast-grep --pattern 'PATTERN' --lang rust src/` for structural patterns across files.

## Logging
- Use `#[instrument]` attribute for tracing, not `debug!` macro calls or `Span::current().record()`.
- Put computed values directly in `fields()`: `#[instrument(fields(count = query.iter().count()))]`

## Debugging
- When debugging, add instrumentation and test immediately rather than endlessly analyzing code. Logs provide concrete answers faster than speculation.
- When logs are available, read them first before jumping to code analysis.
- Follow through: if you add instrumentation, actually run and read the logs before implementing a fix. Don't skip steps.
- Always include logging/instrumentation in fix plans - never propose a fix without visibility into what's happening.
- Don't try to run the game - you can't interact with it. Ask the user to run and report back.

## Planning
- Understand the actual goal before proposing solutions. Don't just revert to old patterns - find a forward path that achieves the user's intent.
- When a refactor breaks something, the fix should align with the refactor's goals, not undo them.

## bevy_ecs_tiled / bevy_ecs_tilemap
- Tiles have `TilePos` component (grid position), NOT Transform/GlobalTransform
- Do not add Transform to tile entities - rendering is handled at the TiledTilemap level
- To get world position: query the tilemap entity and use its transform + tile grid position
- TileBundle fields: position (TilePos), texture_index, tilemap_id, visible, flip, color, old_position, sync

## Workflow
- Commit changes as you go. After completing a logical unit of work, run styleguide then commit immediately. Don't accumulate uncommitted changes.
- Run the styleguide skill before every commit to catch issues early.
- Don't run unnecessary commands (like `git status`) when you already know the state from your own actions.
