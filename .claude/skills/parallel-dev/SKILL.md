---
name: parallel-dev
description: Safe parallel development patterns. Use for multi-issue or single-issue parallelization.
---

## Mode Selection

| Mode | Use When | Isolation |
|------|----------|-----------|
| **Multi-Issue** | Working on multiple separate issues | Git worktrees (complete) |
| **Single-Issue** | Parallelizing work within one issue | Subagents (file-level) |

---

## Mode 1: Multi-Issue Parallel (Git Worktrees)

Use git worktrees for complete isolation when working on multiple issues simultaneously.

### Setup
```bash
git worktree add ../project-issue-1 -b fix/issue-1
git worktree add ../project-issue-2 -b feat/issue-2
# Run separate Claude sessions in each directory
```

### Hierarchy
Within each worktree, if the issue needs parallel work, use **Single-Issue Mode**:
```
Multi-Issue (worktrees for isolation)
├── Issue 1 (worktree 1)
│   └── Single-Issue Mode (if parallelizing)
└── Issue 2 (worktree 2)
    └── Single-Issue Mode (if parallelizing)
```

### Merge Back
```bash
# After completing work in worktrees
cd ../project-issue-1 && git push -u origin fix/issue-1
cd ../project-issue-2 && git push -u origin feat/issue-2

# Clean up worktrees when done
git worktree remove ../project-issue-1
git worktree remove ../project-issue-2
```

---

## Mode 2: Single-Issue Parallel (Subagents)

Use subagents for parallel work on non-overlapping files within a single issue.

### Pre-Launch Checklist
- [ ] Files are independent (no shared edits)
- [ ] Each subagent has explicit file assignment
- [ ] Task is large enough to benefit from parallelization

### Subagent Contract

**CRITICAL RULES:**

| Action | Allowed | Notes |
|--------|---------|-------|
| Read any file | ✅ Yes | For context |
| Edit assigned files | ✅ Yes | Explicit list only |
| Run `cargo check` | ❌ No | Parent only |
| Run `cargo build` | ❌ No | Parent only |
| Run `cargo test` | ❌ No | Parent only |
| Run any verification | ❌ No | Parent only |

**Limits:**
- **5 turn maximum** per subagent
- Commit before returning (atomic checkpoint)
- Return with status report

### File Assignment Example
```
Subagent A: Edit only src/module_a.rs, src/module_a/helpers.rs
Subagent B: Edit only src/module_b.rs
Subagent C: Edit only tests/integration.rs
```

### Safe Exit Pattern

When subagent completes OR reaches 5 turn limit:
```bash
git add <assigned-files-only>
git commit -m "WIP: <task-description> (partial if incomplete)"
# Return control with status report
```

**Status report must include:**
- Files modified
- Work completed
- Work remaining (if any)
- Any blockers encountered

---

## Parent Agent Responsibilities

After all subagents return:

1. **Verify compilation**: `cargo check`
2. **Run tests**: `cargo test`
3. **If errors found**:
   - Fix directly (simple issues), OR
   - Spin up new subagents using Single-Issue Mode rules
4. **Squash commits** if desired: `git rebase -i`

### WIP Handling Options

**Option A: Review and Complete**
1. Review WIP commit(s)
2. Complete remaining work sequentially
3. Squash into clean history

**Option B: Rollback and Retry**
```bash
git reset --soft HEAD~N  # N = number of WIP commits
# Changes preserved in working directory
# Try different approach
```

---

## Conflict Prevention

| Scenario | Prevention |
|----------|------------|
| Same file edited by multiple agents | Strict file assignment upfront |
| Verification during incomplete edits | Only parent runs cargo check/build/test |
| Endless loops | 5 turn hard limit |
| Lost work on exit | Commit before return |
| Partial edits | WIP commit with clear message |

---

## Anti-Patterns (Avoid)

❌ Subagent runs `cargo check` while other subagents still editing
❌ Multiple subagents edit same file
❌ Subagent exceeds 5 turns without returning
❌ Subagent returns without committing changes
❌ Parent runs verification before all subagents complete
❌ No explicit file assignment before launch
