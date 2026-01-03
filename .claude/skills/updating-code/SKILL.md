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

## Phase 1: Setup
**Skill**: `git-workflow`

Checkout a new descriptive branch from main.

---

## Phase 2: Research

1. **Reference Docs**: Check `.claude/skills/updating-code/` for relevant documentation before checking the codebase. For GitHub issues, read all comments.
2. **Activate Skills**: Invoke any necessary skills (ascii-art, log-issue, tests, etc.)
3. **Ask Questions**: Clarify any ambiguity before proceeding.

---

## Phase 3: Development
**Skill**: `parallel-dev`

1. **Evaluate Parallelization**: Assess if changes can be made in parallel with subagents.
2. **Make Changes**: Proceed with changes. Use `ast-grep` for renames/refactoring (see `refactoring.md`). Commit logical chunks for easy rollback.

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
