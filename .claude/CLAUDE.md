# Project Guidelines

## Self Improvement
- If the user corrects you in anyway, immediately update .claude/lessons.md with the pattern. This must happen immediately, before proceeding with anything else.
- Write rules for yourself to prevent the same mistake from occuring
- Review the lessons prior to starting work- whether that's planning, exploration, or making changes.
- Continually update and iterate on lessons in order to lessen how often you need to be corrected.

## Subagent Strategy
- Use subagents liberally to keep main context window clean.
- Offload research, exploration, and parallel analysis to subagents.
- One task per subagent for focused execution.

## Important Reminders (MANDATORY)
1. BEFORE ANY CHANGE TO ANYTHING, WHETHER ITS DOCS, CODE, CLAUDE SKILLS, ETC, MAKE SURE YOU ARE NOT ON THE MAIN BRANCH.
2. WHENEVER YOU MAKE A BUNDLE OF CHANGES COMMIT THEM.
3. DO NOT LOOK AT SOURCE CODE FOR CRATES, SEARCH FOR DOCS ONLINE INSTEAD
4. CONSULT .claude/docs FREQUENTLY.
5. USE THE styleguide SKILL TO REVIEW CODE CHANGES IN YOUR PLAN OR AFTER THEY HAVE BEEN MADE
6. YOU DO NOT NEED TO OPEN A PR. ONCE THE USER VERIFIES YOU JUST NEED TO MERGE INTO MAIN AND PUSH.

## Code Change Workflow
**Use the `updating-code` skill for ALL code changes.** It contains the full workflow.


## Rust Navigation Rules (MANDATORY)

**NEVER use Grep to search Rust code.** Use these LSP operations instead:

| Task | Tool | Example |
|------|------|---------|
| Find where symbol is defined | `LSP goToDefinition` | Find struct/function definition |
| Find all usages | `LSP findReferences` | **REQUIRED before any deletion** |
| Find trait implementations | `LSP goToImplementation` | Find all Components |
| Get type/docs | `LSP hover` | Check inferred types |
| Search by name | `LSP workspaceSymbol` | Find symbols across codebase |

For structural patterns across files, use `ast-grep --pattern 'PATTERN' --lang rust src/`

## Conventions
- Branches: `type/description` (e.g., `feat/add-inventory`)
- Commits: conventional (`feat:`, `fix:`, `refactor:`)
- Tests: changed modules only

## Project Documentation

Reference docs in `.claude/docs/` organized by domain:
- `dungeon/` - floors, layouts, movement, spawning, entities
- `combat/` - ECS combat, fight modal, health bars
- `modals/` - modal system, registry, individual modals
- `widgets/` - reusable UI components
- `ui/` - layout, navigation, display patterns
- `sprites/` - sprite sheets, slices, animations

Each domain has a README.md with its index.
