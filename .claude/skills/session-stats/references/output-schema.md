# Output Schema

This document defines the JSON schema for session stats output.

## Full Schema

```json
{
  "schema_version": "1.0",
  "session_id": "0725b4bc-82c2-41ed-a241-fd1b54bd9ef5",
  "project": "/Users/drewstewart/code/game",

  "timing": {
    "start_time": "2026-01-08T10:15:30.000Z",
    "end_time": "2026-01-08T11:45:22.000Z",
    "duration_minutes": 89.87,
    "duration_formatted": "1h 29m 52s"
  },

  "messages": {
    "total": 127,
    "user": 45,
    "assistant": 82
  },

  "tokens": {
    "model": "claude-opus-4-5-20251101",
    "input": 50000,
    "output": 12000,
    "cache_read": 450000,
    "cache_creation": 8000,
    "total": 520000,
    "cost_usd": 0.15
  },

  "tools": {
    "total_calls": 156,
    "by_type": {
      "Read": 42,
      "Edit": 15,
      "Bash": 28,
      "Grep": 18,
      "Glob": 12,
      "Write": 8,
      "Task": 5,
      "LSP": 28
    },
    "success_rate": 0.98
  },

  "files": {
    "modified": ["src/ui/store.rs", "src/systems/player.rs"],
    "read": ["src/main.rs", "Cargo.toml"],
    "created": ["src/ui/gold_display.rs"]
  },

  "performance": {
    "slow_operations": [
      {
        "operation": "cargo build",
        "duration_ms": 45000,
        "timestamp": "2026-01-08T10:32:15.000Z"
      }
    ],
    "average_tool_response_ms": 250
  },

  "sources": {
    "session_jsonl": "/Users/drewstewart/.claude/projects/-Users-drewstewart-code-game/0725b4bc.jsonl",
    "debug_log": "/Users/drewstewart/.claude/debug/0725b4bc.txt",
    "file_history": "/Users/drewstewart/.claude/file-history/0725b4bc/",
    "history_jsonl": "/Users/drewstewart/.claude/history.jsonl",
    "stats_cache": "/Users/drewstewart/.claude/stats-cache.json"
  },

  "metadata": {
    "generated_at": "2026-01-08T12:00:00.000Z",
    "generator_version": "1.0.0"
  }
}
```

## Field Descriptions

### Root Fields

| Field | Type | Description |
|-------|------|-------------|
| `schema_version` | string | Schema version for backwards compatibility |
| `session_id` | string | UUID of the session |
| `project` | string | Absolute path to the project |

### Timing Object

| Field | Type | Description |
|-------|------|-------------|
| `start_time` | ISO 8601 | Session start timestamp |
| `end_time` | ISO 8601 | Session end timestamp |
| `duration_minutes` | number | Duration in decimal minutes |
| `duration_formatted` | string | Human-readable duration |

### Messages Object

| Field | Type | Description |
|-------|------|-------------|
| `total` | integer | Total messages in session |
| `user` | integer | User messages count |
| `assistant` | integer | Assistant messages count |

### Tokens Object

| Field | Type | Description |
|-------|------|-------------|
| `model` | string | Primary model used |
| `input` | integer | Total input tokens |
| `output` | integer | Total output tokens |
| `cache_read` | integer | Tokens read from cache |
| `cache_creation` | integer | Tokens used to create cache |
| `total` | integer | Sum of all token types |
| `cost_usd` | number | Estimated cost in USD |

### Tools Object

| Field | Type | Description |
|-------|------|-------------|
| `total_calls` | integer | Total tool invocations |
| `by_type` | object | Map of tool name to call count |
| `success_rate` | number | Ratio of successful calls (0-1) |

### Files Object

| Field | Type | Description |
|-------|------|-------------|
| `modified` | array | Files that were edited |
| `read` | array | Files that were read |
| `created` | array | New files created |

### Performance Object

| Field | Type | Description |
|-------|------|-------------|
| `slow_operations` | array | Operations exceeding threshold |
| `average_tool_response_ms` | number | Average tool response time |

### Sources Object

Contains the actual absolute file paths used to generate this report. This enables:
- Debugging data issues
- Verifying data provenance
- Re-running if source data changes

| Field | Type | Description |
|-------|------|-------------|
| `session_jsonl` | string or null | Path to session JSONL file |
| `debug_log` | string or null | Path to debug log file |
| `file_history` | string or null | Path to file history directory |
| `history_jsonl` | string | Path to history.jsonl |
| `stats_cache` | string or null | Path to stats-cache.json |

### Metadata Object

| Field | Type | Description |
|-------|------|-------------|
| `generated_at` | ISO 8601 | When the report was generated |
| `generator_version` | string | Version of the aggregation scripts |

## Cost Calculation

Token costs are calculated using current Claude pricing:

| Token Type | Price per 1M tokens |
|------------|---------------------|
| Input | $15.00 |
| Output | $75.00 |
| Cache Read | $1.50 |
| Cache Creation | $18.75 |

```python
cost = (
    (input_tokens * 15 / 1_000_000) +
    (output_tokens * 75 / 1_000_000) +
    (cache_read_tokens * 1.5 / 1_000_000) +
    (cache_creation_tokens * 18.75 / 1_000_000)
)
```
