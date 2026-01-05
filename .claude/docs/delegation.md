# Agent Delegation Guide

## When to Delegate

The code-change orchestrator should delegate to specialist agents based on these triggers:

### Delegate to Coder Agent
| Trigger | Threshold | Rationale |
|---------|-----------|-----------|
| Lines of code | >30 lines | Complex implementations benefit from focused context |
| File count | >3 files | Multi-file changes need coordinated edits |
| Logic complexity | Uncertain design | Fresh perspective avoids overthinking |
| New features | Any size | Feature code should be reviewed |

### Delegate to Reviewer Agent
| Trigger | Threshold | Rationale |
|---------|-----------|-----------|
| After implementation | Always for >30 lines | Catch issues before commit |
| Security-sensitive | Any changes to auth/crypto | Security review mandatory |
| API changes | Public interface changes | Breaking change detection |

### Delegate to Test-writer Agent
| Trigger | Threshold | Rationale |
|---------|-----------|-----------|
| New functions | Any public function | Test coverage for new code |
| Bug fixes | After fix verified | Regression test for the bug |
| Refactors | If tests don't exist | Ensure behavior preserved |

## When NOT to Delegate

Skip delegation for:
- **Trivial changes** (<10 lines, single file)
- **Documentation only** (no code changes)
- **Config changes** (Cargo.toml, settings)
- **Simple renames** (file/variable renames)

## Delegation Workflow

```
1. Orchestrator analyzes task
2. Check delegation triggers (see above)
3. If triggers met → spawn specialist agent
4. Specialist completes work
5. Orchestrator verifies result
6. Continue workflow
```

## Example Delegation Prompts

### To Coder Agent
```
Implement [feature] following these specifications:
- Files to modify: [list]
- Requirements: [specs]
- Patterns to follow: [examples]
- Constraints: [limits]
```

### To Reviewer Agent
```
Review these changes for:
- Correctness (does it do what's intended?)
- Style (follows project patterns?)
- Security (no vulnerabilities?)
- Performance (any obvious issues?)

Changes: [summary or diff]
```

### To Test-writer Agent
```
Write tests for [module/function]:
- Test happy path
- Test edge cases: [list]
- Test error conditions: [list]
- Follow existing test patterns in [file]
```

## Monitoring Delegation

The `enforce_delegation.py` hook tracks:
- Edit count per session
- Lines edited per session
- Files touched per session

Advisory warnings appear at:
- 30+ lines edited without delegation
- 5+ files touched without delegation

## Goal Alignment

Delegation supports workflow goals:
- **P1 Stability**: Specialist review catches errors
- **P2 Token Usage**: Focused agents use context efficiently
- **P3 Speed**: Parallel work when possible
