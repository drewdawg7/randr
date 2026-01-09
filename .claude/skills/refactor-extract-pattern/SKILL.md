---
name: refactor-extract-pattern
description: Extract repeated code patterns into a shared module using ast-grep for batch operations. Use when refactoring duplicated code, extracting utilities, or consolidating repeated patterns across multiple files.
allowed-tools:
  - Bash(ast-grep:*)
  - Bash(cargo check:*)
  - Read
  - Write
  - Edit
---

# Refactor: Extract Repeated Pattern

Use this skill when extracting repeated code patterns into a shared module.

## Workflow

### 1. Identify Pattern Instances

```bash
ast-grep --pattern 'YOUR_PATTERN_HERE' --lang rust src/
```

Count instances to determine if batch operation is appropriate (>5 similar changes).

### 2. Create Shared Module

- Create new file (e.g., `src/ui/theme.rs`)
- Define constants and helper functions
- Export from parent module (`mod.rs` or `lib.rs`)

### 3. Batch Replace Pattern

```bash
ast-grep --pattern 'OLD_PATTERN' --rewrite 'NEW_CALL' --lang rust src/ --update-all
```

### 4. Add Imports

If >5 files need imports, batch with ast-grep:

```bash
ast-grep --pattern 'use bevy::prelude::*;' \
  --rewrite 'use bevy::prelude::*;

use crate::new_module::helper;' \
  --lang rust src/target_dir/ --update-all
```

Otherwise, add imports manually to each file.

### 5. Verify

```bash
cargo check
```

## Checklist

- [ ] Pattern identified with ast-grep search
- [ ] Instance count determined (>5 = batch, ≤5 = manual OK)
- [ ] Shared module created with helpers
- [ ] All instances replaced via ast-grep
- [ ] Imports added (batch if >5 files)
- [ ] Compilation verified with `cargo check`
