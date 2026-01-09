# Data Sources

This document details the data sources used by the session-stats skill.

## 1. Session JSONL (Primary Token Source)

**Location:** `~/.claude/projects/{encoded-path}/{session_id}.jsonl`

**Format:** JSONL (one JSON object per message)

**Example path:** `~/.claude/projects/-Users-drewstewart-code-game/0725b4bc-82c2-41ed-a241-fd1b54bd9ef5.jsonl`

**Structure:**
```json
{
  "parentUuid": "uuid",
  "sessionId": "0725b4bc-82c2-41ed-a241-fd1b54bd9ef5",
  "type": "assistant",
  "message": {
    "model": "claude-opus-4-5-20251101",
    "role": "assistant",
    "content": [...],
    "usage": {
      "input_tokens": 10,
      "cache_creation_input_tokens": 3127,
      "cache_read_input_tokens": 21090,
      "output_tokens": 306,
      "service_tier": "standard"
    }
  },
  "timestamp": "2026-01-09T22:30:34.859Z",
  "toolUseResult": {
    "totalDurationMs": 113290,
    "totalTokens": 52402,
    "totalToolUseCount": 33,
    "usage": {...}
  }
}
```

**Data extracted:**
- Per-message token usage (input, output, cache read, cache creation) - EXACT counts
- Model used per message
- Tool use results with duration and token counts
- Message timestamps
- Tool call counts from `toolUseResult.totalToolUseCount`

## 2. History JSONL

**Location:** `~/.claude/history.jsonl`

**Format:** JSONL (one JSON object per line)

**Structure:**
```json
{
  "display": "user message or query text",
  "pastedContents": {},
  "timestamp": 1767997827493,
  "project": "/Users/drewstewart/code/game",
  "sessionId": "0725b4bc-82c2-41ed-a241-fd1b54bd9ef5"
}
```

**Data extracted:**
- Session ID to group entries
- First/last timestamp for session duration
- Message count per session
- Project path

## 3. Debug Logs

**Location:** `~/.claude/debug/{session_id}.txt`

**Format:** Plain text log files

**Structure:**
```
[2026-01-09T10:15:30.123Z] [INFO] Session started
[2026-01-09T10:16:45.456Z] [SLOW OPERATION DETECTED] cargo build took 45000ms
[2026-01-09T10:17:00.789Z] [DEBUG] LSP server initialized
```

**Data extracted:**
- Slow operation warnings with durations
- Tool execution timing
- Error messages
- Session start/end markers

## 4. Stats Cache (Reference Only)

**Location:** `~/.claude/stats-cache.json`

**Format:** JSON

**Use for:** Cross-referencing and validation only, not primary token source

**Contains:** Daily aggregates, model totals, session counts

## 5. Tool Results

**Location:** `~/.claude/projects/{encoded-path}/tool-results/`

**Format:** JSON files with UUID names

**Data extracted:**
- Tool call counts by type
- Tool success/failure rates
- Large result sizes (potential optimization targets)

## 6. File History

**Location:** `~/.claude/file-history/{session_id}/`

**Format:** Versioned file snapshots

**Data extracted:**
- List of files modified
- Number of edits per file

## Path Encoding

Project paths in `~/.claude/projects/` use dashes to replace slashes:

| Original Path | Encoded |
|---------------|---------|
| `/Users/drewstewart/code/game` | `-Users-drewstewart-code-game` |
| `/home/user/project` | `-home-user-project` |

## Session ID Linking

The session ID is the primary key linking all data sources:

```
Session ID: 0725b4bc-82c2-41ed-a241-fd1b54bd9ef5
           ↓
~/.claude/projects/{path}/0725b4bc-82c2-41ed-a241-fd1b54bd9ef5.jsonl
~/.claude/debug/0725b4bc-82c2-41ed-a241-fd1b54bd9ef5.txt
~/.claude/file-history/0725b4bc-82c2-41ed-a241-fd1b54bd9ef5/
```
