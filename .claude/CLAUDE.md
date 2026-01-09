# Claude Code Workflow


## Tool Selection (MANDATORY)

Before ANY code change, check:

## Pre-Edit Checklist
Before making changes:
- [ ] Checked if batch operation appropriate?
- [ ] Ran findReferences for any removals?
- [ ] Using LSP for Rust navigation?
- [ ] Considered delegation for large changes (>50 lines)?

## New Code Rules
- New public APIs must be used in production code, not just tests
- Never add `#[allow(dead_code)]` to hide unused new code
- Think simplest solution first (e.g., re-exports vs transforming imports)

## Code Navigation
**PREFER LSP over grep for Rust:**
- `goToDefinition` - Find where symbol is defined
- `findReferences` - Find all usages
- `goToImplementation` - Find trait impls
- `hover` - Get type info


## Conventions
- Branch naming: `type/description` (e.g., `feat/add-inventory`)
- Commits: Conventional (`feat:`, `fix:`, `refactor:`)
- Tests: Changed modules only
- Issues: Auto-close on merge

## Agents

Use the Task tool with these specialized agents for complex tasks:

| Agent | When to Use |
|-------|-------------|
| `rust-codebase-researcher` | Exploring Rust code: finding definitions, tracing code paths, understanding type hierarchies, mapping module structures. Uses LSP + ast-grep. |
| `github-issue-analyzer` | Analyzing GitHub issues: extracting requirements, scoping work, breaking down complex issues into actionable items. |
| `codebase-issue-finder` | Auditing code quality: finding anti-patterns, non-idiomatic Rust/Bevy code, architectural issues, bloated modules. Use after feature work. |
| `skill-optimizer` | Creating/refining skills: analyzing sessions for patterns, improving existing skills with progressive disclosure. |
| `session-orchestrator` | Coordinating complex tasks: multi-domain work, ambiguous requests, tasks needing multiple agent types. |

### Quick Decision Guide
- Exploring unfamiliar Rust code → `rust-codebase-researcher`
- Working on a GitHub issue → `github-issue-analyzer`
- Reviewing code quality → `codebase-issue-finder`
- Creating automation → `skill-optimizer`
- Complex multi-step task → `session-orchestrator`

### Agent Selection Triggers

Start with the right agent based on task type:

| Task Type | First Agent | Why |
|-----------|-------------|-----|
| "Work on issue #X" | `github-issue-analyzer` | Extract requirements before exploring code |
| "Explore/find/trace Rust" | `rust-codebase-researcher` | LSP + ast-grep for accurate navigation |
| "Audit/review code" | `codebase-issue-finder` | Systematic quality analysis |
| "Complex multi-domain" | `session-orchestrator` | Coordinate multiple agents |
| "Create/improve skill" | `skill-optimizer` | Session pattern analysis |

