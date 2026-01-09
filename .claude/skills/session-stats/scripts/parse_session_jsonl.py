#!/usr/bin/env python3
"""
Parse session JSONL files to extract exact token usage and message data.

This script reads the session JSONL file from ~/.claude/projects/{encoded-path}/{session_id}.jsonl
and extracts token usage, message counts, and tool call data.
"""
import json
import sys
from pathlib import Path
from typing import Optional
from collections import Counter


def encode_project_path(project_path: str) -> str:
    """Encode project path for ~/.claude/projects directory naming."""
    return project_path.replace("/", "-")


def find_session_jsonl(session_id: str, project_path: Optional[str] = None) -> Optional[Path]:
    """Find the session JSONL file for a given session ID."""
    claude_dir = Path.home() / ".claude"
    projects_dir = claude_dir / "projects"

    if not projects_dir.exists():
        return None

    # If project path provided, look in specific directory
    if project_path:
        encoded = encode_project_path(project_path)
        session_file = projects_dir / encoded / f"{session_id}.jsonl"
        if session_file.exists():
            return session_file

    # Otherwise search all project directories
    for project_dir in projects_dir.iterdir():
        if project_dir.is_dir():
            session_file = project_dir / f"{session_id}.jsonl"
            if session_file.exists():
                return session_file

    return None


def parse_session_jsonl(session_id: str, project_path: Optional[str] = None) -> dict:
    """
    Parse a session JSONL file and extract stats.

    Returns:
        dict with:
            - tokens: Token usage breakdown
            - messages: Message counts
            - tools: Tool usage data
            - timing: First/last timestamps
            - model: Primary model used
            - source_path: Actual absolute path to the file read
    """
    session_file = find_session_jsonl(session_id, project_path)

    if not session_file:
        return {
            "tokens": {
                "input": 0,
                "output": 0,
                "cache_read": 0,
                "cache_creation": 0,
                "total": 0
            },
            "messages": {"total": 0, "user": 0, "assistant": 0},
            "tools": {"total_calls": 0, "by_type": {}, "durations": []},
            "timing": {"first": None, "last": None},
            "model": None,
            "source_path": None
        }

    # Track stats
    tokens = {
        "input": 0,
        "output": 0,
        "cache_read": 0,
        "cache_creation": 0
    }
    messages = {"user": 0, "assistant": 0}
    tool_counts = Counter()
    tool_durations = []
    timestamps = []
    models = Counter()

    # Stream parse the JSONL file
    with open(session_file, "r") as f:
        for line in f:
            line = line.strip()
            if not line:
                continue

            try:
                entry = json.loads(line)
            except json.JSONDecodeError:
                continue

            # Extract timestamp
            ts = entry.get("timestamp")
            if ts:
                timestamps.append(ts)

            # Count message types
            msg_type = entry.get("type")
            if msg_type == "user":
                messages["user"] += 1
            elif msg_type == "assistant":
                messages["assistant"] += 1

            # Extract message data
            message = entry.get("message", {})

            # Track model usage
            model = message.get("model")
            if model:
                models[model] += 1

            # Extract token usage from message.usage
            usage = message.get("usage", {})
            if usage:
                tokens["input"] += usage.get("input_tokens", 0)
                tokens["output"] += usage.get("output_tokens", 0)
                tokens["cache_read"] += usage.get("cache_read_input_tokens", 0)
                tokens["cache_creation"] += usage.get("cache_creation_input_tokens", 0)

            # Extract tool use from content blocks
            content = message.get("content", [])
            if isinstance(content, list):
                for block in content:
                    if isinstance(block, dict) and block.get("type") == "tool_use":
                        tool_name = block.get("name", "unknown")
                        tool_counts[tool_name] += 1

            # Extract tool use result data
            tool_result = entry.get("toolUseResult")
            if tool_result and isinstance(tool_result, dict):
                duration = tool_result.get("totalDurationMs")
                if duration:
                    tool_durations.append(duration)

    # Calculate totals
    tokens["total"] = (
        tokens["input"] +
        tokens["output"] +
        tokens["cache_read"] +
        tokens["cache_creation"]
    )

    messages["total"] = messages["user"] + messages["assistant"]

    # Determine primary model
    primary_model = models.most_common(1)[0][0] if models else None

    # Sort timestamps
    timestamps.sort()

    return {
        "tokens": tokens,
        "messages": messages,
        "tools": {
            "total_calls": sum(tool_counts.values()),
            "by_type": dict(tool_counts),
            "durations": tool_durations
        },
        "timing": {
            "first": timestamps[0] if timestamps else None,
            "last": timestamps[-1] if timestamps else None
        },
        "model": primary_model,
        "source_path": str(session_file.resolve())
    }


def main():
    """CLI entry point."""
    import argparse

    parser = argparse.ArgumentParser(
        description="Parse session JSONL for token usage and message data"
    )
    parser.add_argument("session_id", help="Session ID (UUID)")
    parser.add_argument("--project", help="Project path (optional)")
    parser.add_argument("--json", action="store_true", help="Output as JSON")

    args = parser.parse_args()

    result = parse_session_jsonl(args.session_id, args.project)

    if args.json:
        print(json.dumps(result, indent=2))
    else:
        print(f"Session: {args.session_id}")
        print(f"Source: {result['source_path']}")
        print(f"\nTokens:")
        print(f"  Input:          {result['tokens']['input']:,}")
        print(f"  Output:         {result['tokens']['output']:,}")
        print(f"  Cache Read:     {result['tokens']['cache_read']:,}")
        print(f"  Cache Creation: {result['tokens']['cache_creation']:,}")
        print(f"  Total:          {result['tokens']['total']:,}")
        print(f"\nMessages:")
        print(f"  User:      {result['messages']['user']}")
        print(f"  Assistant: {result['messages']['assistant']}")
        print(f"  Total:     {result['messages']['total']}")
        print(f"\nTools: {result['tools']['total_calls']} calls")
        for tool, count in sorted(result['tools']['by_type'].items(), key=lambda x: -x[1]):
            print(f"  {tool}: {count}")
        print(f"\nModel: {result['model']}")


if __name__ == "__main__":
    main()
