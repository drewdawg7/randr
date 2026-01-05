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

### ast-grep Limitations

ast-grep works well for:
- Adding/removing attributes
- Renaming symbols
- Simple structural changes

ast-grep does NOT work well for:
- Complex import splitting (e.g., `use {A, B, C}` where A stays, B/C move)
- Context-dependent transformations
- Changes requiring semantic understanding

For complex cases, use LSP + targeted manual edits.

### Pattern Examples

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

## Before Multi-File Changes

**STOP** and ask yourself:
1. How many files will I touch?
2. Are the changes similar (same pattern)?
3. Could ast-grep handle this?

If answers: **many, yes, yes** → Use ast-grep FIRST

### Batch Operation Patterns
Use ast-grep for:
- Rename across files → `ast-grep --rewrite`
- Add attribute to many items → `ast-grep --rewrite`
- Update import paths → `ast-grep --rewrite`
- Change method signatures → `ast-grep --rewrite`

### Self-Check Before Editing
After planning but before first edit:
- Am I about to make > 5 similar changes?
- Should I use ast-grep instead?
- Did I run findReferences for removals?

See `.claude/docs/batch-operations.md` for detailed examples.

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

## Before Removing Code

**MANDATORY** checklist before removing any struct field, function, or type:

1. [ ] Run `LSP findReferences` on the symbol
2. [ ] Check if any references exist outside the current file
3. [ ] If references exist: Update all callers FIRST, then remove
4. [ ] If no references: Safe to remove
5. [ ] Run `cargo check` after removal to verify

**Why this matters:** Removing code without checking references causes compilation errors that require reverts, wasting time and context.

```
# Example: Before removing a struct field
LSP operation="findReferences" filePath="src/combat/loot.rs" line=15 character=8

# If no references found, safe to remove
# If references found, update those files first
```
