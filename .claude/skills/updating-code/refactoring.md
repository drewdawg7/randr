# Automated Refactoring with ast-grep

Use `ast-grep` for automated refactoring instead of manually updating call sites. This is faster and less error-prone than manual edits.

## Quick Reference

```bash
# Function rename (updates all call sites)
ast-grep run -p 'old_fn($$$ARGS)' -r 'new_fn($$$ARGS)' -l rust src/ --update-all

# Struct/type rename
ast-grep run -p 'OldType' -r 'NewType' -l rust src/ --update-all

# Method rename
ast-grep run -p '$VAR.old_method($$$ARGS)' -r '$VAR.new_method($$$ARGS)' -l rust src/ --update-all
```

## Pattern Syntax

| Pattern | Matches |
|---------|---------|
| `$NAME` | Single AST node (identifier, expression, etc.) |
| `$$$ARGS` | Zero or more nodes (for argument lists, etc.) |
| `$$ITEMS` | One or more nodes |

## Common Refactoring Patterns

### Function/Method Renames

```bash
# Rename function and update all call sites
ast-grep run -p 'game_state()' -r 'get_game_state()' -l rust src/ --update-all

# Rename with arguments preserved
ast-grep run -p 'spawn_item($ID)' -r 'create_item($ID)' -l rust src/ --update-all

# Method on any receiver
ast-grep run -p '$OBJ.stats()' -r '$OBJ.get_stats()' -l rust src/ --update-all
```

### Type/Struct Renames

```bash
# Simple type rename
ast-grep run -p 'StatSheet' -r 'Stats' -l rust src/ --update-all

# Generic type rename
ast-grep run -p 'Registry<$K, $V>' -r 'EntityRegistry<$K, $V>' -l rust src/ --update-all
```

### Field Renames

```bash
# Struct field access
ast-grep run -p '$OBJ.old_field' -r '$OBJ.new_field' -l rust src/ --update-all

# Struct initialization
ast-grep run -p 'old_field: $VAL' -r 'new_field: $VAL' -l rust src/ --update-all
```

### Enum Variant Renames

```bash
# Enum variant
ast-grep run -p 'ItemType::OldVariant' -r 'ItemType::NewVariant' -l rust src/ --update-all
```

## Workflow

1. **Preview changes first** (no `--update-all`):
   ```bash
   ast-grep run -p 'old_name' -r 'new_name' -l rust src/
   ```

2. **Apply changes**:
   ```bash
   ast-grep run -p 'old_name' -r 'new_name' -l rust src/ --update-all
   ```

3. **Verify with cargo check**:
   ```bash
   cargo check
   ```

4. **Commit refactoring separately** for easy rollback

## When to Use ast-grep vs Manual Editing

| Use ast-grep | Use manual editing |
|--------------|-------------------|
| Function renames | Complex signature changes |
| Type renames | Different changes per call site |
| Consistent pattern changes | Context-dependent changes |
| Field/method renames | |

## Tips

- Use LSP `findReferences` first to see all usages before refactoring
- Test the pattern without `--update-all` to preview what will change
- Commit refactoring changes separately from feature changes
- For scoped changes, specify a file path instead of `src/`
