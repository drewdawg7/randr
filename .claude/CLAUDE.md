# Claude Code Workflow


## Tool Selection (MANDATORY)

Before ANY code change, check:

```
1. Similar changes needed?
   > 5 similar → ast-grep (NOT manual edits)
   ≤ 5 → individual edits OK

2. Removing code?
   YES → LSP findReferences FIRST (mandatory)
   NO → proceed

3. Navigating Rust code?
   YES → LSP (grep blocked by hook)
   NO → grep OK for .md, strings

4. Reading multiple files?
   > 3 files → parallel Read calls
   ≤ 3 → sequential OK
```

## Pre-Edit Checklist
Before making changes:
- [ ] Checked if batch operation appropriate (>5 similar changes)?
- [ ] Ran findReferences for any removals?
- [ ] Using LSP for Rust navigation?
- [ ] Considered delegation for large changes (>50 lines)?

## New Code Rules
- New public APIs must be used in production code, not just tests
- Never add `#[allow(dead_code)]` to hide unused new code
- Think simplest solution first (e.g., re-exports vs transforming imports)

## Code Navigation
**PREFER LSP over grep for Rust:**
- `goToDefinition` - Find where symbol is defined
- `findReferences` - Find all usages
- `goToImplementation` - Find trait impls
- `hover` - Get type info


## Conventions
- Branch naming: `type/description` (e.g., `feat/add-inventory`)
- Commits: Conventional (`feat:`, `fix:`, `refactor:`)
- Tests: Changed modules only
- Issues: Auto-close on merge

