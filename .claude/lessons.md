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
- Use `#[instrument]` attribute for tracing, not `debug!` macro calls.

## Debugging
- When debugging, add instrumentation and test immediately rather than endlessly analyzing code. Logs provide concrete answers faster than speculation.
- When logs are available, read them first before jumping to code analysis.
- Follow through: if you add instrumentation, actually run and read the logs before implementing a fix. Don't skip steps.

## Workflow
- Commit changes as you go. After completing a logical unit of work, run styleguide then commit immediately. Don't accumulate uncommitted changes.
- Run the styleguide skill before every commit to catch issues early.
- Don't run unnecessary commands (like `git status`) when you already know the state from your own actions.
