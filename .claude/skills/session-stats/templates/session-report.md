# Session Report: {{session_id}}

**Project:** {{project}}
**Date:** {{timing.start_time}} - {{timing.end_time}}
**Duration:** {{timing.duration_formatted}}

---

## Summary

| Metric | Value |
|--------|-------|
| Messages | {{messages.total}} ({{messages.user}} user / {{messages.assistant}} assistant) |
| Tool Calls | {{tools.total_calls}} |
| Files Modified | {{files.modified|length}} |
| Cost | ${{tokens.cost_usd}} |

---

## Token Usage

**Model:** {{tokens.model}}

| Type | Count |
|------|-------|
| Input | {{tokens.input|number}} |
| Output | {{tokens.output|number}} |
| Cache Read | {{tokens.cache_read|number}} |
| Cache Creation | {{tokens.cache_creation|number}} |
| **Total** | **{{tokens.total|number}}** |

---

## Tool Usage

| Tool | Calls |
|------|-------|
{{#each tools.by_type}}
| {{@key}} | {{this}} |
{{/each}}

**Success Rate:** {{tools.success_rate|percent}}

---

## Files

{{#if files.modified}}
### Modified
{{#each files.modified}}
- `{{this}}`
{{/each}}
{{/if}}

{{#if files.created}}
### Created
{{#each files.created}}
- `{{this}}`
{{/each}}
{{/if}}

---

## Performance

{{#if performance.slow_operations}}
### Slow Operations
{{#each performance.slow_operations}}
- **{{operation}}**: {{duration_ms}}ms at {{timestamp}}
{{/each}}
{{/if}}

**Average Tool Response:** {{performance.average_tool_response_ms}}ms

---

## Source Files

These are the actual files that were read to generate this report:

| Source | Path |
|--------|------|
| Session Data | `{{sources.session_jsonl}}` |
| Debug Log | `{{sources.debug_log}}` |
| File History | `{{sources.file_history}}` |
| History | `{{sources.history_jsonl}}` |
| Stats Cache | `{{sources.stats_cache}}` |

---

*Generated: {{metadata.generated_at}} | Generator v{{metadata.generator_version}}*
