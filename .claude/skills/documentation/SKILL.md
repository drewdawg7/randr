---
name: documentation
description: Guidelines for creating and maintaining project documentation in .claude/skills/updating-code.
---

## Purpose

Documentation enables faster future changes by allowing quick scanning instead of codebase exploration. Be descriptive - include file names, module names, function names, and concepts.

## Location

All documentation goes in `.claude/skills/updating-code/`:
- Organized by subdirectories for related information
- Named by relevant area (e.g., `ui/`, `entities/`, `combat/`)

## What to Document

1. **Architecture** - High-level system structure and patterns
2. **Decisions** - Justifications for design choices
3. **Patterns** - Coding and architecture conventions
4. **Areas** - Explanations of specific code areas
5. **UI** - UI decisions and patterns (especially important)

## Format Guidelines

- Write for efficient scanning (minimize token/time waste)
- Use tables, bullet points, and code blocks
- Include file paths and function names for navigation
- Only use examples from the codebase or closely related

## Organization

- Break down files by relevant area
- Use subdirectories to group related information
- If files repeat similar content, extract to higher-level docs
- Re-organize freely: delete, rename, move, or edit as needed

## Documentation Checklist

When completing code changes:

1. Create/update `.md` files in `.claude/skills/updating-code`
2. Cover architectural information
3. Document UI patterns if applicable
4. Include file/module/function references
5. Extract repeated information to shared docs
6. Update skill.index.md if necessary
