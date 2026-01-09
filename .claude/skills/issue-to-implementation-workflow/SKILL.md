---
name: issue-to-implementation-workflow
description: End-to-end workflow from GitHub issue to implementation. Use when starting work on an issue that involves code changes.
---

# Issue to Implementation Workflow

## Quick Start

When user says "work on issue #X":

1. **Analyze Issue**
   - Use `analyzing-issues` skill for requirements extraction
   - Skip if trivially simple (one sentence, no comments)

2. **Research Code**
   - Invoke `rust-codebase-researcher` agent
   - Focus: areas mentioned in issue, related modules

3. **Plan Implementation**
   - Match pattern to approach (see below)
   - For >5 similar changes → use `refactor-extract-pattern` skill

4. **Execute with Pre-Edit Checklist**
   - [ ] Batch operation appropriate?
   - [ ] findReferences for removals?
   - [ ] Using LSP for Rust navigation?

## Workflow Patterns

### Bug Fix
```
1. Analyze issue (extract reproduction steps)
2. rust-codebase-researcher (trace code path)
3. Implement fix
4. cargo test
```

### New Feature
```
1. Analyze issue (extract requirements)
2. rust-codebase-researcher (find integration points)
3. Plan implementation
4. Execute with checklist
5. cargo check && cargo test
```

### Refactoring
```
1. Analyze issue (understand scope)
2. rust-codebase-researcher (find all instances)
3. If >5 instances → refactor-extract-pattern skill
4. cargo check
```

## Decision Points

### Skip Issue Analysis When
- One sentence + no comments + no linked issues
- You're the issue author
- Already analyzed in current session

### Use refactor-extract-pattern When
- Issue mentions "extract", "deduplicate", "consolidate"
- Research reveals >5 similar patterns
- ast-grep more efficient than manual edits
