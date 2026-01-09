---
name: analyzing-issues
description: Analyze GitHub issues to extract requirements and scope work. Use when starting work on an issue, scoping a feature, or breaking down complex issues.
---

# Analyzing GitHub Issues

## Quick Start

When analyzing an issue:

1. **Read the issue** - `gh issue view #N` or paste content
2. **Extract requirements** - Use the checklist below
3. **Identify gaps** - What's missing or unclear?
4. **Plan next steps** - What agents/skills to invoke?

## Requirements Checklist

### Functional Requirements
- [ ] What must the system do?
- [ ] What are the inputs/outputs?
- [ ] What are the success criteria?

### Non-Functional Requirements
- [ ] Performance expectations?
- [ ] Security considerations?
- [ ] Compatibility requirements?

### Technical Constraints
- [ ] Specific technologies required?
- [ ] Patterns to follow?
- [ ] Dependencies?

## Issue Analysis Template

```markdown
## Summary
[2-3 sentence overview]

## Requirements
- **Must have**: [list]
- **Nice to have**: [list]
- **Out of scope**: [list]

## Acceptance Criteria
- [ ] [criterion 1]
- [ ] [criterion 2]

## Open Questions
- [question 1]
- [question 2]

## Complexity
Low / Medium / High

## Next Steps
1. [action]
2. [action]
```

## When to Use rust-codebase-researcher

After analyzing the issue, invoke `rust-codebase-researcher` agent to:
- Find integration points
- Understand existing patterns
- Trace code paths mentioned in issue

## Skip Analysis When

- Issue is one sentence with no comments
- You're the issue author
- Already analyzed in current session
