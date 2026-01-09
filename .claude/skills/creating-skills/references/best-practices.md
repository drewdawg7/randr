# Skill Best Practices

## Progressive Disclosure Framework

Structure skills in layers:

### Layer 1 - Core Identity (Always Visible)
- Expert persona in one sentence
- Primary responsibility
- Most important behavioral rule

### Layer 2 - Common Workflows
- Step-by-step processes
- Default behaviors
- Quality checks

### Layer 3 - Edge Cases (Revealed When Needed)
- Unusual situations
- Fallback strategies
- Escalation criteria

### Layer 4 - Advanced Features
- Sophisticated techniques
- Integrations
- Optimizations

## Anthropic Best Practices

1. **Concise is key** - Context window is shared. Challenge each piece.

2. **Set appropriate freedom**:
   - High freedom (text): Multiple approaches valid
   - Medium freedom (pseudocode): Preferred pattern, some variation
   - Low freedom (exact scripts): Fragile operations

3. **Third person** - "Processes..." not "I can..."

4. **Avoid time-sensitive info** - No dates that become outdated

5. **Specificity over generality** - Every instruction adds concrete value

6. **Examples clarify** - Include input/output for ambiguous behaviors

7. **Decision frameworks** - Clear if/then logic

8. **Self-verification** - Checklists for validating work

9. **Appropriate scope** - One thing done well

10. **Context awareness** - Acknowledge project context (CLAUDE.md)

## Before Creating New Skills

Ask: Can existing skills be improved instead?

- **Enhance first** - Better guidance beats new narrow skills
- **Progressive disclosure** - Layer complexity, don't separate
- **Guide, don't prescribe** - Help intelligent choices, not rigid trees

## Common Mistakes

- Too verbose (keep SKILL.md <500 lines)
- First person descriptions
- Missing trigger in description
- Too broad scope
- Duplicating what Claude already knows
- No self-verification checklist
