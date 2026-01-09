---
name: rust-code-patterns
description: LSP operations and ast-grep patterns for Rust navigation. Use when exploring code, finding usages, or pattern matching.
---

# Rust Code Patterns

## LSP Quick Reference

| Task | Operation | When to Use |
|------|-----------|-------------|
| Find definition | `goToDefinition` | Locate where symbol is defined |
| Find all usages | `findReferences` | **REQUIRED** before any removal/rename |
| Find implementations | `goToImplementation` | Find trait implementations |
| Get type info | `hover` | Check types, docs, signatures |
| List symbols | `documentSymbol` | Overview of file structure |
| Search workspace | `workspaceSymbol` | Find symbols by name |

## ast-grep Quick Reference

| Pattern | Use Case |
|---------|----------|
| `$EXPR.unwrap()` | Find unwrap calls |
| `$EXPR.expect($MSG)` | Find expect calls |
| `impl $TRAIT for $TYPE { $$$ }` | Find trait implementations |
| `pub fn $NAME($$$ARGS) $$$REST` | Find public functions |
| `pub async fn $NAME($$$ARGS) $$$REST` | Find async functions |
| `#[derive($$$DERIVES)] struct $NAME { $$$ }` | Find derived structs |
| `use $PATH::$ITEM;` | Find specific imports |

## Tool Selection

```
Need to find where something is defined?
  → LSP goToDefinition

Need to find ALL usages before removing/renaming?
  → LSP findReferences (MANDATORY)

Need to find pattern across many files?
  → ast-grep --pattern 'PATTERN' --lang rust src/

Need semantic understanding (types, traits)?
  → LSP hover, goToImplementation

Never use grep for Rust code navigation.
```

## Common Workflows

### Before Removing Code
```bash
# 1. Find all references first
LSP findReferences on the symbol

# 2. Only if no external usages, proceed with removal
```

### Finding Similar Patterns
```bash
ast-grep --pattern 'PATTERN' --lang rust src/
# Count results to decide: >5 = batch operation
```

### Batch Replacement
```bash
ast-grep --pattern 'OLD' --rewrite 'NEW' --lang rust src/ --update-all
```

For detailed patterns, see [references/](references/).
