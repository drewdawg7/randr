---
name: updating-code
description: Outlines the necessary steps to make code changes. All steps must be followed. Use when planning or making code changes.
---

IMPORTANT: Use grep patterns to search documentation for specific keywords.
IMPORTANT: **Use Rust LSP (rust-analyzer) instead of grep for Rust code navigation.**
IMPORTANT: Reference `ascii-art` skill when making UI changes.
IMPORTANT: Reference `log-issue` skill when issues are found.
IMPORTANT: Reference `tests` skill when creating tests.
IMPORTANT: Use `ast-grep` for refactoring (see `refactoring.md`).

---

## Phase 1: Research & Planning
**Skills**: `parallel-dev`, `git-workflow`

1. **Reference Docs**: Check `.claude/skills/updating-code/` for relevant documentation before checking the codebase. For GitHub issues, read all comments.
2. **Activate Skills**: Invoke any necessary skills (ascii-art, log-issue, tests, etc.)
3. **Ask Questions**: Clarify any ambiguity before proceeding.
4. **Decide Parallelization Mode**: Evaluate before git setup (affects branch strategy).

### Parallelization Decision

**Multi-Issue Mode (Git Worktrees)** - Use when:
- User requests work on multiple unrelated issues/features simultaneously
- Issues touch overlapping files and need complete isolation
- Long-running tasks where context switching between issues is needed
- Each issue needs its own branch and independent testing

**Single-Issue Mode (Subagents)** - Use when:
- A single issue involves 3+ files that can be edited independently
- Work is clearly partitionable (e.g., separate modules, tests vs implementation)
- Task would take significantly longer sequentially
- Files have no cross-dependencies that require coordinated edits

**Sequential (no parallelization)** - Use when:
- Only 1-2 files need changes
- Files have tight coupling requiring coordinated edits
- Changes are simple enough that overhead outweighs benefit
- You need to see intermediate results before proceeding

---

## Phase 2: Setup
**Skill**: `git-workflow`

Based on parallelization decision:
- **Multi-Issue Mode**: Create git worktrees (see `parallel-dev` skill)
- **Single-Issue / Sequential**: Checkout a new descriptive branch from main

---

## Phase 3: Development

1. **Make Changes**: Use subagents if Single-Issue Mode, otherwise work sequentially.
2. **Refactoring**: Use `ast-grep` for renames/refactoring (see `refactoring.md`).
3. **Commit**: Commit logical chunks for easy rollback.

---

## Phase 4: Verification
**Skill**: `tests`

1. **Test**: Run tests in relevant modules. Don't update tests unless failure is due to structural changes.
2. **Cargo Check**: Run `cargo check` to verify compilation.
3. **User Review**: Ask user to verify changes work.

---

## Phase 5: Finalize
**Skills**: `documentation`, `git-workflow`

1. **Update Documentation**: Create/update docs in `.claude/skills/updating-code/` (see `documentation` skill).
2. **Merge**: Commit, push, merge branch into main, delete branch, push main.
3. **Additional Documentation**: Add any docs that would make this skill more efficient.
