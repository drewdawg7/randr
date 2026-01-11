---
name: updating-code
description: Required workflow for ALL code changes - invoke FIRST before any implementation. Use when adding features, fixing bugs, refactoring, editing files, modifying code, updating functions, changing behavior, writing new code, deleting code, working on issues, or making any changes to the codebase. Covers git branching, LSP navigation, testing, and merge process.
---

## Workflow
Follow this workflow for ALL code changes:
**IMPORTANT**: CREATE NEW BRANCHES EVEN FOR CHANGES NOT RELATED TO GITHUB ISSUES.

1. **Branch**: Create a new branch with descriptive name (e.g., `feat/add-inventory`)
2. **Analyze**: Use ast-grep and Rust LSP to understand the codebase. The `rust-codebase-researcher` agent is skilled at this.
3. **Ask**: Clarify any ambiguity with the user before proceeding
4. **Compare**: Check similar functionality in the codebase for patterns
5. **Make Changes**: Execute your plan
6. **Test**: Run tests for changed modules only. If the test requires the developer to verify, pause and tell the developer to run the code.
7. **Clean-Up**: Fix any compiler warnings related to your changes
8. **Merge**: Commit, merge, and push. No PR necessary.
9. **Close**: If working on a GitHub issue, close it
10. **Stats**: Use the `session-stats` skill to aggregate previous session stats


## LSP Quick Reference (Use Instead of Grep)

| Task | Operation | When to Use |
|------|-----------|-------------|
| Find definition | `LSP goToDefinition` | Locate where symbol is defined |
| Find all usages | `LSP findReferences` | **REQUIRED** before any removal/rename |
| Find implementations | `LSP goToImplementation` | Find trait implementations |
| Get type info | `LSP hover` | Check types, docs, signatures |
| List symbols | `LSP documentSymbol` | Overview of file structure |
| Search workspace | `LSP workspaceSymbol` | Find symbols by name |

## ast-grep for Pattern Matching

```bash
# Find patterns across files
ast-grep --pattern '$EXPR.unwrap()' --lang rust src/

# Batch replacement (>5 similar changes)
ast-grep --pattern 'OLD' --rewrite 'NEW' --lang rust src/ --update-all
```

Common patterns:
- `$EXPR.unwrap()` - Find unwrap calls
- `impl $TRAIT for $TYPE { $$$ }` - Find trait implementations
- `pub fn $NAME($$$) $$$` - Find public functions

## Bevy UI Patterns

Use **bundles** for complex UI spawning instead of inline component tuples:

```rust
// Good: Use bundles for reusable UI patterns
bar.spawn((PlayerHealthBar, HealthBarBundle::new(AlignItems::FlexStart)))
    .with_children(|bar| {
        bar.spawn(HealthBarNameBundle::new(player_name));
        bar.spawn(SpriteHealthBarBundle::new(AlignSelf::FlexStart));
        bar.spawn(HealthBarTextBundle::new(health, max_health));
    });

// Avoid: Inline component tuples for repeated patterns
player_side.spawn((
    Text::new("PLAYER"),
    TextFont { font_size: 24.0, ..default() },
    TextColor(Color::srgb(0.5, 0.8, 0.5)),
    Node { margin: UiRect::bottom(Val::Px(10.0)), ..default() },
));
```

Available bundles in `src/ui/screens/health_bar.rs`:
- `HealthBarBundle` - Health bar container
- `HealthBarNameBundle`, `HealthBarTextBundle`, `SpriteHealthBarBundle`
