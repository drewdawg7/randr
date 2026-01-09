# LSP Operations Guide

## goToDefinition
Navigate to where a symbol is defined.

**Use for:**
- Finding struct/enum definitions
- Finding function implementations
- Finding module declarations
- Tracing imports to their source

**Example:**
```
LSP goToDefinition on `Player` → jumps to struct definition
```

## findReferences
Find ALL usages of a symbol across the codebase.

**MANDATORY before:**
- Removing any symbol
- Renaming any symbol
- Changing function signatures
- Removing struct fields

**Example:**
```
LSP findReferences on `spawn_player` → shows all call sites
```

## goToImplementation
Find implementations of traits or abstract methods.

**Use for:**
- Finding all types that implement a trait
- Finding concrete implementations
- Understanding trait hierarchies

**Example:**
```
LSP goToImplementation on `Component` trait → shows all components
```

## hover
Get type information, documentation, and signatures.

**Use for:**
- Checking inferred types
- Reading documentation
- Understanding function signatures
- Verifying generic constraints

## documentSymbol
List all symbols in a document.

**Use for:**
- Getting file overview
- Finding functions/structs in large files
- Understanding module structure

## workspaceSymbol
Search for symbols by name across workspace.

**Use for:**
- Finding types by name
- Searching for functions
- Locating modules
