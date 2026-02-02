# Project Guidelines

## Important Reminders (MANDATORY)
1. Do not add comments to code unless there is a really good reason too. Clean up exisitng comments you come across.
2. Do not use unwrap in non-test code.
3. BEFORE ANY CHANGE TO ANYTHING, WHETHER ITS DOCS, CODE, CLAUDE SKILLS, ETC, MAKE SURE YOU ARE NOT ON THE MAIN BRANCH.
4. WHENEVER YOU MAKE A BUNDLE OF CHANGES COMMIT THEM.
5. DO NOT LOOK AT SOURCE CODE FOR CRATES, SEARCH FOR DOCS ONLINE INSTEAD
6. CONSULT .claude/docs FREQUENTLY.
## Code Change Workflow
**Use the `updating-code` skill for ALL code changes.** It contains the full workflow.

**Use the `sprites` skill for sprite or UI changes.**

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

## Sprites
Use the `sprites` skill when working with sprite sheets, Aseprite exports, or adding sprites to UI.

## Project Documentation

Reference docs in `.claude/docs/` organized by domain:
- `dungeon/` - floors, layouts, movement, spawning, entities
- `combat/` - ECS combat, fight modal, health bars
- `modals/` - modal system, registry, individual modals
- `widgets/` - reusable UI components
- `ui/` - layout, navigation, display patterns
- `sprites/` - sprite sheets, slices, animations

Each domain has a README.md with its index.
