# Documentation Index

## Quick Reference

| Topic | Location |
|-------|----------|
| Workflow overview | `.claude/CLAUDE.md` |
| **Workflow goals** | `.claude/docs/workflow-goals.md` |
| Scripts reference | `.claude/scripts/SCRIPTS.index.md` |
| Git workflow | `.claude/skills/git-workflow/SKILL.md` |
| Code navigation | `.claude/skills/code-nav/SKILL.md` |
| Rust patterns | `.claude/skills/rust-patterns/SKILL.md` |
| Testing guide | `.claude/skills/testing/SKILL.md` |

## Agents

| Agent | Purpose | Model |
|-------|---------|-------|
| code-change | Orchestrates workflow | Sonnet |
| coder | Writes code | Opus |
| reviewer | Reviews changes | Sonnet |
| test-writer | Writes tests | Sonnet |

## Architecture

See `architecture.md` for codebase overview.

## How to Find Information

1. **Workflow questions** → `CLAUDE.md`
2. **Script usage** → `scripts/SCRIPTS.index.md`
3. **Code patterns** → `skills/rust-patterns/SKILL.md`
4. **Architecture** → `docs/architecture.md`

## Search Tips

Use grep patterns to find docs:
```bash
# Find workflow info
grep -r "workflow" .claude/

# Find pattern examples
grep -r "impl.*for" .claude/skills/
```
