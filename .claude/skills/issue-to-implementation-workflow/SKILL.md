---
name: issue-to-implementation-workflow
description: Guides the complete workflow from GitHub issue to implementation. Use when starting work on a GitHub issue that involves code changes.
---

# Issue to Implementation Workflow

## Quick Start

When user says "work on issue #X":

1. **Analyze Issue** (skip if trivially simple)
   - Invoke: github-issue-analyzer
   - Output: Requirements, acceptance criteria, complexity estimate

2. **Research Code** (always for Rust)
   - Invoke: rust-codebase-researcher
   - Focus: Areas mentioned in issue, related modules
   - Output: Affected files, patterns, dependencies

3. **Plan Implementation**
   - Match pattern to approach:
     - Refactoring/deduplication -> Consider refactor-extract-pattern skill
     - New feature -> Identify integration points
     - Bug fix -> Trace the bug path

4. **Execute with Pre-Edit Checklist**
   - [ ] Checked if batch operation appropriate?
   - [ ] Ran findReferences for any removals?
   - [ ] Using LSP for Rust navigation?

## Decision Points

### Skip github-issue-analyzer when:
- Issue is one sentence AND no comments AND no linked issues
- You're the issue author
- Issue was analyzed in current session

### Use refactor-extract-pattern skill when:
- Issue involves "extract", "deduplicate", "consolidate"
- Code research reveals >5 similar patterns
- ast-grep would be more efficient than manual edits

## Agent Chain Examples

### Refactoring Task
```
1. gh issue view #X -> understand requirements
2. rust-codebase-researcher -> find all instances, understand patterns
3. refactor-extract-pattern skill -> batch apply changes (if >5 instances)
4. cargo check -> verify
```

### Bug Fix Task
```
1. github-issue-analyzer -> extract reproduction steps, expected behavior
2. rust-codebase-researcher -> trace code path, find root cause
3. Implement fix with LSP navigation
4. cargo test -> verify
```

### New Feature Task
```
1. github-issue-analyzer -> extract requirements, acceptance criteria
2. rust-codebase-researcher -> find integration points, existing patterns
3. Plan implementation with architectural decisions
4. Implement with Pre-Edit Checklist
5. cargo check && cargo test -> verify
```
