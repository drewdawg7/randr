# Claude Code Workflow

## 10-Step Code Change Process
```
1. Checkout branch → 2. Load docs → 3. Clarify → 4. Plan
                                              ↓
8. Add tests ← 7. Run tests ← 6. cargo check ← 5. Make changes
       ↓
9. Update docs → 10. Merge/Push/Close
```

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
- `close.py` - Close issue

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
