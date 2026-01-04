---
name: updating-code
description: Outlines the necessary steps to make code changes. All steps must be followed. Use when planning or making code changes.
---

**IMPORTANT:** Read [SKILL.index.md](SKILL.index.md) first to find relevant documentation before diving into specific files.

IMPORTANT: Use grep patterns to search documentation for specific keywords.
IMPORTANT: **Use Rust LSP (rust-analyzer) instead of grep for Rust code navigation.**
IMPORTANT: Reference `ascii-art` skill when making UI changes.
IMPORTANT: Reference `log-issue` skill when issues are found.
IMPORTANT: Reference `tests` skill when creating tests.
IMPORTANT: Use `ast-grep` for refactoring (see `refactoring.md`).

---

## Phase 1: Research & Planning
**Skills**: `git-workflow`

1. **Read Documentation First** (before touching code):
   - Read `SKILL.index.md` to find relevant documentation
   - Identify which doc files relate to your task
   - Read those documentation files completely
2. **Code Exploration** (after reading docs):
   - Use LSP for code navigation (goToDefinition, findReferences, hover)
   - Use grep only for searching documentation and comments
3. **Activate Skills**: Invoke any necessary skills (ascii-art, log-issue, tests, etc.)
4. **Ask Questions**: Clarify any ambiguity before proceeding.

## Phase 2: Development

1. **Make Changes**: Make the necessary changes.
2. **Refactoring**: Use `ast-grep` for any renames or refactoring - see [refactoring.md](refactoring.md) for patterns. Prefer ast-grep over manual find-replace.
3. **Commit**: Commit logical chunks for easy rollback.

## Phase 3: Verification
**Skill**: `tests`

1. **Test**: Run tests in relevant modules. Don't update tests unless failure is due to structural changes.
2. **Cargo Check**: Run `cargo check` to verify compilation.
3. **User Review**: Ask user to verify changes work.

## Phase 4: Finalize
**Skills**: `documentation`, `git-workflow`, `fix-issue`

1. **Update Documentation**: Create/update docs in `.claude/skills/updating-code/` (see `documentation` skill).
2. **Merge**: Commit, push, merge branch into main, delete branch, push main.
3. **Additional Documentation**: Add any docs that would make this skill more efficient.
4. **Close GH Issue**: If working on a GH issue, close out the issue.
