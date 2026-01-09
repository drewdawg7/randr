# Project Guidelines

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
