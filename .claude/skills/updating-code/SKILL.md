---
name: updating-code
description: Code change workflow. Covers branching, LSP navigation, testing, and merge process.
---

# Code Change Workflow

1. **Branch** - Create descriptive branch (e.g., `feat/add-inventory`)
2. **Research** - Use LSP and ast-grep. Check relevant domain in `.claude/docs/`
3. **Ask** - Clarify ambiguity before proceeding
4. **Compare** - Check similar functionality for patterns
5. **Make Changes** - Execute plan
6. **Test** - Run tests for changed modules
7. **Clean-Up** - Fix compiler warnings from changes
8. **Verify** - User verifies before proceeding
9. **Document** - Update relevant `.claude/docs/` domain
10. **Merge** - Commit, merge, push (no PR)
11. **Close** - Close GitHub issue if applicable
