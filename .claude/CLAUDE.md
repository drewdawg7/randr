# Claude Code Workflow

## 11-Step Code Change Process
```
1. Checkout branch → 2. Load docs → 3. Clarify → 4. Plan
                                              ↓
8. Add tests ← 7. Run tests ← 6. cargo check ← 5. Make changes
       ↓
9. Update docs → 10. Feedback → 11. Merge/Push/Close
```

**Step 3 - Clarify**: Ask user about PR vs direct merge preference, push to origin?

**Step 10 - Feedback**: Run `/context` to capture token usage, then merge (feedback auto-generated)

## Required Skills
Invoke skills BEFORE starting work:
- `git-workflow` - Branch/commit/merge operations
- `code-nav` - Code navigation (LSP and ast-grep, NOT grep)
- `rust-patterns` - Rust idioms and this project's trait patterns
- `testing` - Writing or running tests

## Enforcement Rules (Hooks)
| Action | Enforcement |
|--------|-------------|
| Grep on .rs files | Blocked → Use LSP |
| Raw `gh` commands | Blocked → Use scripts |
| Edit on main branch | Warned → Checkout first |
| Edit .rs files | Auto cargo check |
| Merge to main | Auto-generates feedback file |

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

## Scripts (Use Instead of CLI)
All scripts output JSON. Located in `.claude/scripts/`:

### Git: `scripts/git/`
- `branch.py` - Create/switch branches
- `commit.py` - Conventional commits
- `merge.py` - Merge and cleanup

### Issues: `scripts/issue/`
- `list.py` - Query issues
- `view.py` - Get issue details
- `create.py` - Create new issue
- `edit.py` - Edit issue (title, body, labels)
- `close.py` - Close issue
- `labels.py` - List repository labels

### Code: `scripts/check/`
- `cargo_check.py` - Type check with JSON output
- `run_tests.py` - Run tests for module

## Code Navigation
**PREFER LSP over grep for Rust:**
- `goToDefinition` - Find where symbol is defined
- `findReferences` - Find all usages
- `goToImplementation` - Find trait impls
- `hover` - Get type info

## Agent Architecture
```
code-change (Sonnet) - Orchestrates, NO code writing
    ├── delegates to → coder (Opus) - Writes code
    ├── delegates to → reviewer (Sonnet) - Reviews
    └── delegates to → test-writer (Sonnet) - Tests
```

## Conventions
- Branch naming: `type/description` (e.g., `feat/add-inventory`)
- Commits: Conventional (`feat:`, `fix:`, `refactor:`)
- Tests: Changed modules only
- Issues: Auto-close on merge

## Documentation
Search `.claude/docs/` for detailed guides. Use grep patterns to find relevant docs.

## Quick Reference
```bash
# Start work on issue
python3 .claude/scripts/issue/view.py 42
python3 .claude/scripts/git/branch.py feat/issue-42

# Check code
python3 .claude/scripts/check/cargo_check.py

# Complete work
python3 .claude/scripts/git/commit.py "feat: add feature"
python3 .claude/scripts/git/merge.py
```
