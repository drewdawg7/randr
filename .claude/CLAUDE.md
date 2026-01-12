# Project Guidelines

## Code Change Workflow (Required)

Follow this workflow for ALL code changes:
**IMPORTANT**: CREATE NEW BRANCHES EVEN FOR CHANGES NOT RELATED TO GITHUB ISSUES.

1. **Branch**: Create a new branch with descriptive name (e.g., `feat/add-inventory`)
2. **Analyze and Research**: Use ast-grep and Rust LSP to understand the codebase. The `rust-codebase-researcher` agent is skilled at this.
    1. Use the sprites skill when working with sprites or UI. 
    2. Use the documentation index to quickly find relevant documentation to the issue at hand.
3. **Ask**: Clarify any ambiguity with the user before proceeding
4. **Compare**: Check similar functionality in the codebase for patterns
5. **Make Changes**: Execute your plan
6. **Test**: Run tests for changed modules only.
7. **Clean-Up**: Fix any compiler warnings related to your changes
8. **Verify**: Ask user to verify changes
9. **Merge**: Commit, merge, and push. No PR necessary.
10. **Close**: If working on a GitHub issue, close it
11. **Document**: Update documentation based on the documentation section below.
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

