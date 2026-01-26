---
name: feature-planning
description: Gathers detailed requirements through structured questions before creating GitHub issues. Use when planning new features, discussing enhancements, or creating issues from user ideas.
---

# Feature Planning

Guides through a structured workflow to gather requirements and create well-formed GitHub issues.

## Workflow

### 1. Explore Codebase

Launch Explore agents to understand relevant areas:

```
Task(subagent_type=Explore): "Find how [relevant system] works. Look for:
- Current implementation patterns
- Related UI components
- Data structures involved"
```

### 2. Summarize Current State

Present findings to user before asking questions:

```markdown
## Current State

**[System Name]**: Brief description of how it works today
- Key component: `path/to/file.rs`
- Data flow: A → B → C
- Relevant patterns: [pattern name]
```

### 3. Ask Structured Questions

Use AskUserQuestion with up to 4 questions per call. Progress through categories logically:

**Round 1 - Core Design**
- Layout/Position: Where should this appear?
- Display Format: How should data be shown?

**Round 2 - Data & Interaction**
- Data Display: What information to show?
- Interaction: Click/select behavior?

**Round 3 - Navigation & States**
- Navigation: Keyboard controls?
- Empty/Edge States: What if no data?

**Round 4 - Polish**
- Styling: Colors, formatting?
- Priority: Urgency level?

Build on previous answers - reference user choices in follow-up questions.

### 4. Create GitHub Issue

Synthesize all gathered information into a structured issue:

```bash
gh issue create --title "feat: [Feature Name]" --body "$(cat <<'EOF'
## Summary
[One-sentence description]

## Background
[Current state and why this is needed]

## Requirements

### Layout
- [ ] Position: [where]
- [ ] Size: [dimensions]

### Display
- [ ] Data shown: [what]
- [ ] Format: [how]

### Interaction
- [ ] [behavior 1]
- [ ] [behavior 2]

### Navigation
- [ ] [keyboard controls]

### Edge Cases
- [ ] Empty state: [behavior]
- [ ] Error state: [behavior]

## Acceptance Criteria
- [ ] [criterion 1]
- [ ] [criterion 2]

## Priority
[Low/Medium/High] - [reason]
EOF
)"
```

## Question Guidelines

- **Max 4 questions per AskUserQuestion call**
- **Explore first** - Ask informed questions based on codebase knowledge
- **Build on answers** - Reference previous responses
- **Offer defaults** - Suggest reasonable options based on existing patterns
- **Include "Other"** - Users can always provide custom input

See [references/question-templates.md](references/question-templates.md) for category-specific examples.

## Verification Checklist

- [ ] Explored relevant codebase areas before asking questions
- [ ] Presented current state summary to user
- [ ] Asked questions across relevant categories
- [ ] Built on user answers in follow-up questions
- [ ] Created issue with clear acceptance criteria
- [ ] Issue includes all gathered requirements
