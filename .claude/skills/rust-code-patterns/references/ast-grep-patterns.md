# ast-grep Patterns for Rust

## Pattern Syntax

- `$VAR` - Single metavariable (matches one node)
- `$$$VAR` - Multiple metavariable (matches zero or more nodes)
- `$$VAR` - Optional metavariable

## Common Patterns

### Error Handling
```bash
# Find unwrap() calls
ast-grep --pattern '$EXPR.unwrap()' --lang rust src/

# Find expect() calls
ast-grep --pattern '$EXPR.expect($MSG)' --lang rust src/

# Find ? operator usage
ast-grep --pattern '$EXPR?' --lang rust src/
```

### Functions
```bash
# All public functions
ast-grep --pattern 'pub fn $NAME($$$ARGS) $$$REST' --lang rust src/

# Async functions
ast-grep --pattern 'pub async fn $NAME($$$ARGS) $$$REST' --lang rust src/

# Functions returning Result
ast-grep --pattern 'fn $NAME($$$ARGS) -> Result<$OK, $ERR> { $$$ }' --lang rust src/
```

### Structs and Traits
```bash
# Find trait implementations
ast-grep --pattern 'impl $TRAIT for $TYPE { $$$ }' --lang rust src/

# Find structs with specific derives
ast-grep --pattern '#[derive($$$DERIVES)] struct $NAME { $$$ }' --lang rust src/

# Find impl blocks
ast-grep --pattern 'impl $TYPE { $$$ }' --lang rust src/
```

### Imports
```bash
# Find specific imports
ast-grep --pattern 'use $PATH::$ITEM;' --lang rust src/

# Find wildcard imports
ast-grep --pattern 'use $PATH::*;' --lang rust src/
```

### Bevy-Specific
```bash
# Find system functions (Query parameters)
ast-grep --pattern 'fn $NAME($$$ARGS: Query<$$$QUERY>) { $$$ }' --lang rust src/

# Find Commands usage
ast-grep --pattern '$VAR.spawn($$$ARGS)' --lang rust src/

# Find component insertions
ast-grep --pattern '.insert($COMPONENT)' --lang rust src/
```

## Batch Replacement

```bash
# Replace pattern with new code
ast-grep --pattern 'OLD_PATTERN' --rewrite 'NEW_CODE' --lang rust src/ --update-all

# Preview changes first (no --update-all)
ast-grep --pattern 'OLD_PATTERN' --rewrite 'NEW_CODE' --lang rust src/
```

## Tips

1. **Count first**: Run search before replacement to see scope
2. **Preview**: Omit `--update-all` to preview changes
3. **Specific paths**: Target specific directories to limit scope
4. **Verify**: Always run `cargo check` after batch changes
