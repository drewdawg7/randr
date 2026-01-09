---
name: creating-skills
description: Guidelines for creating Claude Code skills. Use when creating new skills, optimizing existing skills, or learning skill best practices.
---

# Creating Skills

## Quick Start

### SKILL.md Format

```yaml
---
name: skill-name          # ≤64 chars, lowercase-with-hyphens, gerund preferred
description: Does X. Use when Y.  # ≤1024 chars, third person, includes trigger
allowed-tools:            # Optional: restrict tools
  - Read
  - Bash(cargo:*)
---

# Skill Title

## Quick start
[Essential 80% use case]

## Advanced features
See [references/](references/) for details.
```

### Directory Structure

```
skill-name/
├── SKILL.md              # Required, <500 lines
├── references/           # Detailed docs loaded on demand
└── scripts/              # Executable utilities
```

## Key Principles

1. **Concise is key** - Only add what Claude doesn't already know
2. **Progressive disclosure** - Essential info upfront, details in references/
3. **Third person descriptions** - "Processes files..." not "I can..."
4. **Keyword-rich triggers** - Include when-to-use in description
5. **One thing done well** - Focused scope

## Checklist

### Format
- [ ] Name: gerund form, ≤64 chars, lowercase-hyphens
- [ ] Description: ≤1024 chars, third person, includes trigger
- [ ] SKILL.md: <500 lines

### Content
- [ ] Core workflow in Quick start
- [ ] Details in references/
- [ ] Concrete examples for ambiguous behaviors
- [ ] Self-verification checklist

For detailed best practices, see [references/best-practices.md](references/best-practices.md).
