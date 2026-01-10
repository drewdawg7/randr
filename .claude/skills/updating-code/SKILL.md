---
name: updating-code
description: Required workflow for ALL code changes - invoke FIRST before any implementation. Use when adding features, fixing bugs, refactoring, editing files, modifying code, updating functions, changing behavior, writing new code, deleting code, working on issues, or making any changes to the codebase. Covers git branching, LSP navigation, testing, and merge process.
---

## Workflow
1. **Branch**: Create a new github branch with a descriptive name.
2. **Analyze**: Analyze the code base using LSP and ast-grep. **NEVER use Grep for Rust code.**
3. **Ask**: If there is any ambiguity, ask the user questions for clarification.
4. **Compare**: Compare your plan to similar functionality in the codebase.
5. **Make Changes**: Execute your plan
6. **Test**: Run the tests only relevant to code you've changed.
7. **Clean-Up**: Clean up any compiler warnings that relate to your changes
8. **Merge**: Commit, Merge, and Push your changes. No PR is necessary.
9. **Close**: If working on a github issue, close it out.

## LSP Quick Reference (Use Instead of Grep)

| Task | Operation | When to Use |
|------|-----------|-------------|
| Find definition | `LSP goToDefinition` | Locate where symbol is defined |
| Find all usages | `LSP findReferences` | **REQUIRED** before any removal/rename |
| Find implementations | `LSP goToImplementation` | Find trait implementations |
| Get type info | `LSP hover` | Check types, docs, signatures |
| List symbols | `LSP documentSymbol` | Overview of file structure |
| Search workspace | `LSP workspaceSymbol` | Find symbols by name |

## ast-grep for Pattern Matching

```bash
# Find patterns across files
ast-grep --pattern '$EXPR.unwrap()' --lang rust src/

# Batch replacement (>5 similar changes)
ast-grep --pattern 'OLD' --rewrite 'NEW' --lang rust src/ --update-all
```

Common patterns:
- `$EXPR.unwrap()` - Find unwrap calls
- `impl $TRAIT for $TYPE { $$$ }` - Find trait implementations
- `pub fn $NAME($$$) $$$` - Find public functions
