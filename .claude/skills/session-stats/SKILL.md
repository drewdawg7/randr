---
name: session-stats
description: Aggregate per-session statistics from ~/.claude data. Use when analyzing session metrics, tracking token usage, optimizing workflows, or generating usage reports.
---

# Session Stats Skill

Aggregates per-session statistics from `~/.claude` data sources and outputs structured reports to `.claude/session_stats/`.

## Quick Start

Generate a report for the most recent session:
```bash
python3 .claude/skills/session-stats/scripts/aggregate_session.py --latest
```

Generate a report for a specific session:
```bash
python3 .claude/skills/session-stats/scripts/aggregate_session.py --session-id <UUID>
```

## Available Commands

| Script | Purpose |
|--------|---------|
| `aggregate_session.py` | Main entry point - aggregates all data sources |
| `parse_session_jsonl.py` | Parses session JSONL for tokens and messages |
| `parse_debug_logs.py` | Parses debug logs for performance data |
| `generate_report.py` | Renders JSON through markdown template |

## Command Arguments

### `aggregate_session.py`

```
--session-id UUID   Specific session to aggregate
--latest            Aggregate most recent session
--date YYYY-MM-DD   Aggregate all sessions from date
--all               Aggregate all sessions (batch mode)
--output-dir PATH   Override output directory (default: .claude/session_stats/)
```

## Output Files

For each session, two files are generated:

1. **JSON data**: `.claude/session_stats/{session_id}.json`
   - Structured data for programmatic access
   - Contains exact token counts, tool usage, timing, files modified

2. **Markdown report**: `.claude/session_stats/{session_id}.md`
   - Human-readable report
   - Rendered from template

## What This Skill Answers

- How many tokens did this session use?
- What was the cost of this session?
- Which tools were used most frequently?
- What files were modified?
- Were there any slow operations?
- How long did the session last?

## Data Sources

The skill aggregates data from these `~/.claude` locations:

| Source | Contains |
|--------|----------|
| `projects/{path}/{session_id}.jsonl` | Token usage, messages, tool results |
| `debug/{session_id}.txt` | Performance logs, slow operations |
| `file-history/{session_id}/` | Files modified during session |
| `history.jsonl` | Session metadata, timestamps |

See `references/data-sources.md` for detailed format documentation.

## Output Schema

See `references/output-schema.md` for the complete JSON schema.

Key sections:
- `timing`: Start/end times, duration
- `tokens`: Input, output, cache read/creation, cost
- `tools`: Call counts by type, success rate
- `files`: Modified, read, created
- `performance`: Slow operations, average response time
- `sources`: Actual file paths used to generate the report
