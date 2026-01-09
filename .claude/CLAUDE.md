# Project Guidelines

## Code Change Workflow (Required)

Follow this workflow for ALL code changes:

1. **Branch**: Create a new branch with descriptive name (e.g., `feat/add-inventory`)
2. **Analyze**: Use ast-grep and Rust LSP to understand the codebase. The `rust-codebase-researcher` agent is skilled at this.
3. **Ask**: Clarify any ambiguity with the user before proceeding
4. **Compare**: Check similar functionality in the codebase for patterns
5. **Make Changes**: Execute your plan
6. **Test**: Run tests for changed modules only
7. **Clean-Up**: Fix any compiler warnings related to your changes
8. **Merge**: Commit, merge, and push. No PR necessary.
9. **Close**: If working on a GitHub issue, close it

## Pre-Edit Checklist
- [ ] Batch operation appropriate? (>5 similar changes → use ast-grep)
- [ ] findReferences for removals? (REQUIRED before deleting code)
- [ ] Using LSP for Rust navigation? (never grep for Rust code)

## Code Rules
- New public APIs must be used in production code, not just tests
- Never add `#[allow(dead_code)]` to hide unused new code
- Think simplest solution first

## Conventions
- Branches: `type/description` (e.g., `feat/add-inventory`)
- Commits: conventional (`feat:`, `fix:`, `refactor:`)
- Tests: changed modules only

## Sprites
Use the `sprites` skill when working with sprite sheets, Aseprite exports, or adding sprites to UI.
