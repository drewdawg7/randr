---
name: parallel-dev
description: Evaluate and execute parallel development using subagents for faster implementation.
---

## When to Use Parallel Development

Evaluate parallel development when:
- Multiple independent files need changes
- Changes don't have sequential dependencies
- Different areas of the codebase are affected
- Task can be cleanly split into non-overlapping work

## When NOT to Use

Avoid parallel development when:
- Changes depend on each other sequentially
- Same files need to be modified
- Order of operations matters
- Coordination overhead exceeds time saved

## Coordination Patterns

### Same Branch
All agents work off the same feature branch to keep changes together:

```
git checkout -b feat/my-feature  # Create once
# Agent 1: works on src/module_a/
# Agent 2: works on src/module_b/
# Agent 3: works on tests/
```

### Independent Files
Ensure each agent works on different files to avoid conflicts:

| Agent | Scope | Files |
|-------|-------|-------|
| 1 | Feature A | `src/feature_a.rs` |
| 2 | Feature B | `src/feature_b.rs` |
| 3 | Tests | `tests/integration.rs` |

### Merge Strategy
After parallel work completes:
1. Each agent commits their changes
2. Verify no conflicts exist
3. Run `cargo check` to verify combined changes compile
4. Run tests on the combined result

## Subagent Launch Pattern

When launching parallel agents:

```
Task tool with multiple invocations:
- Agent 1: "Implement X in src/x.rs"
- Agent 2: "Implement Y in src/y.rs"
- Agent 3: "Add tests in tests/z.rs"
```

## Quick Evaluation Checklist

- [ ] Are the changes independent?
- [ ] Do they touch different files?
- [ ] Is there no sequential dependency?
- [ ] Is the task large enough to benefit from parallelization?

If all yes → use parallel development
