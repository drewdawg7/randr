# Code Navigation Skill

## Prefer LSP Over Grep for Rust

The LSP tool provides semantic understanding of code:

### Find Where Something is Defined
```
LSP goToDefinition on symbol
```

### Find All Usages
```
LSP findReferences on symbol
```

### Find Trait Implementations
```
LSP goToImplementation on trait
```

### Get Type Information
```
LSP hover on symbol
```

### List Symbols in File
```
LSP documentSymbol on file
```

### Search Workspace for Symbol
```
LSP workspaceSymbol with query
```

## When to Use ast-grep

For pattern-based searches that LSP can't handle:

```bash
# Find all functions
python3 .claude/scripts/code/find_symbol.py function my_function

# Find all structs
python3 .claude/scripts/code/find_symbol.py struct MyStruct

# Find impl blocks
python3 .claude/scripts/code/find_symbol.py impl MyType

# Custom pattern
python3 .claude/scripts/code/find_symbol.py pattern "fn $NAME($$$) -> Result<$$$>"
```

## When to Use Grep

Only for:
- Markdown documentation (`.md` files)
- String literals in code
- Comments
- When LSP returns no results

## Navigation Workflow

1. **Start with LSP** - Most accurate for Rust code
2. **Fall back to ast-grep** - For pattern matching
3. **Use Grep last** - For non-Rust files or strings
