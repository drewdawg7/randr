# Batch Operations Guide

When making 5+ similar changes, use batch operations instead of one-by-one edits.

## Decision Tree

```
> 5 similar changes needed?
  YES → Use batch operation (this guide)
  NO  → Individual edits OK
```

## ast-grep Patterns

### Add Attributes to Functions

```bash
# Add #[allow(dead_code)] to unused functions
ast-grep -p 'fn $NAME($$$ARGS) { $$$BODY }' \
  --rewrite '#[allow(dead_code)]
fn $NAME($$$ARGS) { $$$BODY }'

# Add #[inline] to small functions
ast-grep -p 'fn $NAME($$$ARGS) -> $RET { $SINGLE }' \
  --rewrite '#[inline]
fn $NAME($$$ARGS) -> $RET { $SINGLE }'
```

### Find and Review Before Changing

```bash
# Find all structs with a specific field
python3 .claude/scripts/code/find_symbol.py pattern "struct $NAME { $$$BEFORE gold: $TYPE, $$$AFTER }"

# Find all impl blocks for a trait
python3 .claude/scripts/code/find_symbol.py impl HasInventory

# Find functions matching a pattern
python3 .claude/scripts/code/find_symbol.py pattern "fn $NAME($$$) -> Result<$$$>"
```

### cargo fix for Warnings

```bash
# Fix all automatically fixable warnings
cargo fix --allow-dirty --allow-staged

# Fix specific lint
cargo fix --allow-dirty --allow-staged -- -A dead_code

# Preview what would be fixed
cargo check 2>&1 | grep "warning:"
```

## Common Batch Scenarios

| Scenario | Tool | Command |
|----------|------|---------|
| Add attribute to many functions | ast-grep | `--rewrite` with pattern |
| Rename symbol everywhere | LSP + Edit | `findReferences` then batch edit |
| Remove unused imports | cargo fix | `cargo fix --allow-dirty` |
| Add derive macro to structs | ast-grep | Pattern match struct definitions |
| Update function signatures | ast-grep | Pattern with `$$$` wildcards |

## Pattern Syntax (ast-grep)

- `$NAME` - Single identifier
- `$$$` - Zero or more items (args, fields, etc.)
- `$_` - Any single node
- `$$$_` - Any sequence

## Workflow

1. **Identify pattern** - What makes these changes similar?
2. **Write ast-grep pattern** - Test with `--json` first
3. **Review matches** - Ensure pattern catches only intended targets
4. **Apply rewrite** - Use `--rewrite` to transform
5. **Verify** - Run `cargo check` to confirm no errors

## Anti-Patterns

- Editing 10+ files one-by-one manually
- Running cargo check after each small edit instead of batching
- Not using `findReferences` before removing code
