# Issue #[NUMBER]: [TITLE]

## Summary
- **Type:** [feat/fix/refactor]
- **Complexity:** [Low/Medium/High] ([X] files modified)
- **Outcome:** [Completed/Failed/Partial]

## Goal Metrics (see workflow-goals.md)

### P1: Stability
| Metric | Value | Target |
|--------|-------|--------|
| Reverts needed | X | 0 |
| Compilation errors from removals | X | 0 |
| findReferences before removal | Yes/No | Yes |

### P2: Token Usage
| Metric | Value | Target |
|--------|-------|--------|
| Session tokens used | X | - |
| Context utilization | X% | - |
| Manual edit count | X | - |
| ast-grep operations | X | - |
| Manual edit ratio | X% | <20% |
| Similar edits without ast-grep | X | 0 |

*Run `/context` to get session token metrics*

### P3: Speed
| Metric | Value |
|--------|-------|
| LSP operations | X |
| Grep on .rs (blocked) | X |
| Parallel read batches | X |

## Tool Stats
| Metric | Count |
|--------|-------|
| Bash invocations | X |
| Read operations | X |
| Edit operations | X |
| LSP operations | X |
| Grep attempts blocked | X |

## Workflow Compliance
- [ ] LSP used for Rust navigation (not grep)
- [ ] Batch operations used where applicable (>5 similar changes)
- [ ] `findReferences` run before any code removal
- [ ] Agent delegation followed (orchestrator doesn't write code)

## Agent Usage
| Agent | Invocations |
|-------|-------------|
| Coder (Opus) | X |
| Reviewer (Sonnet) | X |
| Test-writer (Sonnet) | X |
| Direct edits (should be 0) | X |

## Quality Metrics
| Metric | Start | End |
|--------|-------|-----|
| Compilation warnings | X | 0 |
| Tests passing | X | X |
| Reverts needed | - | X (target: 0) |

## Lessons Learned

### What Went Well
- [List successes]

### What Could Improve
- [List areas for improvement]

### Specific Recommendations
- [Actionable next steps]
