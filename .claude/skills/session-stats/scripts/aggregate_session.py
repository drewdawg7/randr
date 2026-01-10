#!/usr/bin/env python3
"""
Main entry point for session stats aggregation.

Aggregates data from all sources and outputs structured JSON and markdown reports.
"""
import json
import sys
import os
from pathlib import Path
from datetime import datetime, timezone
from typing import Optional, List

# Add scripts directory to path for imports
SCRIPT_DIR = Path(__file__).parent
sys.path.insert(0, str(SCRIPT_DIR))

from parse_session_jsonl import parse_session_jsonl, encode_project_path
from parse_debug_logs import parse_debug_logs

GENERATOR_VERSION = "1.0.0"

# Claude Opus 4.5 pricing per 1M tokens
PRICING = {
    "input": 15.00,
    "output": 75.00,
    "cache_read": 1.50,
    "cache_creation": 18.75
}


def get_claude_dir() -> Path:
    """Get the ~/.claude directory path."""
    return Path.home() / ".claude"


def get_history_file() -> Path:
    """Get the history.jsonl file path."""
    return get_claude_dir() / "history.jsonl"


def get_stats_cache_file() -> Path:
    """Get the stats-cache.json file path."""
    return get_claude_dir() / "stats-cache.json"


def get_file_history_dir(session_id: str) -> Optional[Path]:
    """Get the file-history directory for a session."""
    dir_path = get_claude_dir() / "file-history" / session_id
    return dir_path if dir_path.exists() else None


def find_sessions_from_history(project_path: Optional[str] = None, date: Optional[str] = None) -> List[dict]:
    """
    Find sessions from history.jsonl.

    Args:
        project_path: Filter by project path
        date: Filter by date (YYYY-MM-DD)

    Returns:
        List of session dicts with id, project, first_timestamp, last_timestamp
    """
    history_file = get_history_file()
    if not history_file.exists():
        return []

    sessions = {}

    with open(history_file, "r") as f:
        for line in f:
            line = line.strip()
            if not line:
                continue
            try:
                entry = json.loads(line)
            except json.JSONDecodeError:
                continue

            sid = entry.get("sessionId")
            if not sid:
                continue

            project = entry.get("project", "")
            timestamp = entry.get("timestamp", 0)

            # Apply filters
            if project_path and project != project_path:
                continue

            if date:
                entry_date = datetime.fromtimestamp(timestamp / 1000).strftime("%Y-%m-%d")
                if entry_date != date:
                    continue

            if sid not in sessions:
                sessions[sid] = {
                    "id": sid,
                    "project": project,
                    "first_timestamp": timestamp,
                    "last_timestamp": timestamp
                }
            else:
                sessions[sid]["first_timestamp"] = min(sessions[sid]["first_timestamp"], timestamp)
                sessions[sid]["last_timestamp"] = max(sessions[sid]["last_timestamp"], timestamp)

    return sorted(sessions.values(), key=lambda x: x["last_timestamp"], reverse=True)


def get_latest_session(project_path: Optional[str] = None) -> Optional[str]:
    """Get the most recent session ID."""
    sessions = find_sessions_from_history(project_path)
    return sessions[0]["id"] if sessions else None


def get_files_from_history(session_id: str) -> dict:
    """Get list of files from file-history directory."""
    file_history_dir = get_file_history_dir(session_id)

    if not file_history_dir:
        return {
            "modified": [],
            "created": [],
            "read": [],
            "source_path": None
        }

    files = []
    for item in file_history_dir.iterdir():
        if item.is_file():
            files.append(item.name)

    return {
        "modified": files,
        "created": [],
        "read": [],
        "source_path": str(file_history_dir.resolve())
    }


def format_duration(minutes: float) -> str:
    """Format duration in minutes to human-readable string."""
    total_seconds = int(minutes * 60)
    hours = total_seconds // 3600
    minutes_part = (total_seconds % 3600) // 60
    seconds = total_seconds % 60

    parts = []
    if hours > 0:
        parts.append(f"{hours}h")
    if minutes_part > 0:
        parts.append(f"{minutes_part}m")
    if seconds > 0 or not parts:
        parts.append(f"{seconds}s")

    return " ".join(parts)


def calculate_cost(tokens: dict) -> float:
    """Calculate cost in USD based on token usage."""
    cost = 0.0
    cost += tokens.get("input", 0) * PRICING["input"] / 1_000_000
    cost += tokens.get("output", 0) * PRICING["output"] / 1_000_000
    cost += tokens.get("cache_read", 0) * PRICING["cache_read"] / 1_000_000
    cost += tokens.get("cache_creation", 0) * PRICING["cache_creation"] / 1_000_000
    return round(cost, 4)


def aggregate_session(session_id: str, project_path: Optional[str] = None) -> dict:
    """
    Aggregate all data for a single session.

    Args:
        session_id: The session UUID
        project_path: Optional project path for filtering

    Returns:
        Complete session stats dict matching the output schema
    """
    # Parse session JSONL
    session_data = parse_session_jsonl(session_id, project_path)

    # Parse debug logs
    debug_data = parse_debug_logs(session_id)

    # Get file history
    file_data = get_files_from_history(session_id)

    # Get session metadata from history
    sessions = find_sessions_from_history()
    session_meta = next((s for s in sessions if s["id"] == session_id), None)

    # Calculate timing
    start_time = session_data["timing"]["first"]
    end_time = session_data["timing"]["last"]
    duration_minutes = 0.0

    if start_time and end_time:
        try:
            # Parse ISO timestamps
            if isinstance(start_time, str):
                start_dt = datetime.fromisoformat(start_time.replace("Z", "+00:00"))
            else:
                start_dt = datetime.fromtimestamp(start_time / 1000)

            if isinstance(end_time, str):
                end_dt = datetime.fromisoformat(end_time.replace("Z", "+00:00"))
            else:
                end_dt = datetime.fromtimestamp(end_time / 1000)

            duration_minutes = (end_dt - start_dt).total_seconds() / 60
        except (ValueError, TypeError):
            pass

    # Calculate average tool response time
    avg_tool_response = debug_data["timing"]["average_ms"]
    if not avg_tool_response and session_data["tools"]["durations"]:
        durations = session_data["tools"]["durations"]
        avg_tool_response = sum(durations) // len(durations) if durations else 0

    # Build the output
    result = {
        "schema_version": "1.0",
        "session_id": session_id,
        "project": session_meta["project"] if session_meta else project_path or "",

        "timing": {
            "start_time": start_time,
            "end_time": end_time,
            "duration_minutes": round(duration_minutes, 2),
            "duration_formatted": format_duration(duration_minutes)
        },

        "messages": session_data["messages"],

        "tokens": {
            "model": session_data["model"],
            "input": session_data["tokens"]["input"],
            "output": session_data["tokens"]["output"],
            "cache_read": session_data["tokens"]["cache_read"],
            "cache_creation": session_data["tokens"]["cache_creation"],
            "total": session_data["tokens"]["total"],
            "cost_usd": calculate_cost(session_data["tokens"])
        },

        "tools": {
            "total_calls": session_data["tools"]["total_calls"],
            "by_type": session_data["tools"]["by_type"],
            "success_rate": 1.0  # Assume success unless we have failure data
        },

        "agents": {
            "total_calls": session_data["agents"]["total_calls"],
            "by_type": session_data["agents"]["by_type"]
        },

        "skills": {
            "total_calls": session_data["skills"]["total_calls"],
            "by_name": session_data["skills"]["by_name"]
        },

        "files": {
            "modified": file_data["modified"],
            "read": file_data.get("read", []),
            "created": file_data.get("created", [])
        },

        "performance": {
            "slow_operations": debug_data["slow_operations"],
            "average_tool_response_ms": avg_tool_response
        },

        "sources": {
            "session_jsonl": session_data["source_path"],
            "debug_log": debug_data["source_path"],
            "file_history": file_data["source_path"],
            "history_jsonl": str(get_history_file().resolve()) if get_history_file().exists() else None,
            "stats_cache": str(get_stats_cache_file().resolve()) if get_stats_cache_file().exists() else None
        },

        "metadata": {
            "generated_at": datetime.now(timezone.utc).isoformat().replace("+00:00", "Z"),
            "generator_version": GENERATOR_VERSION
        }
    }

    return result


def save_stats(stats: dict, output_dir: Path) -> Path:
    """Save stats to JSON file."""
    output_dir.mkdir(parents=True, exist_ok=True)
    output_file = output_dir / f"{stats['session_id']}.json"

    with open(output_file, "w") as f:
        json.dump(stats, f, indent=2)

    return output_file


def main():
    """CLI entry point."""
    import argparse

    parser = argparse.ArgumentParser(
        description="Aggregate session stats from ~/.claude data sources"
    )

    group = parser.add_mutually_exclusive_group(required=True)
    group.add_argument("--session-id", help="Specific session ID to aggregate")
    group.add_argument("--latest", action="store_true", help="Aggregate most recent session")
    group.add_argument("--date", help="Aggregate all sessions from date (YYYY-MM-DD)")
    group.add_argument("--all", action="store_true", help="Aggregate all sessions")

    parser.add_argument("--project", help="Filter by project path")
    parser.add_argument(
        "--output-dir",
        default=".claude/session_stats",
        help="Output directory (default: .claude/session_stats)"
    )
    parser.add_argument("--json", action="store_true", help="Output JSON to stdout")
    parser.add_argument("--no-report", action="store_true", help="Skip markdown report generation")

    args = parser.parse_args()

    output_dir = Path(args.output_dir)

    # Determine which sessions to process
    session_ids = []

    if args.session_id:
        session_ids = [args.session_id]
    elif args.latest:
        latest = get_latest_session(args.project)
        if latest:
            session_ids = [latest]
        else:
            print("No sessions found", file=sys.stderr)
            sys.exit(1)
    elif args.date:
        sessions = find_sessions_from_history(args.project, args.date)
        session_ids = [s["id"] for s in sessions]
    elif args.all:
        sessions = find_sessions_from_history(args.project)
        session_ids = [s["id"] for s in sessions]

    if not session_ids:
        print("No sessions found to process", file=sys.stderr)
        sys.exit(1)

    # Process each session
    results = []
    for session_id in session_ids:
        print(f"Processing session: {session_id}", file=sys.stderr)
        stats = aggregate_session(session_id, args.project)
        results.append(stats)

        if not args.json:
            output_file = save_stats(stats, output_dir)
            print(f"  Saved: {output_file}", file=sys.stderr)

            if not args.no_report:
                # Import and run report generator
                try:
                    from generate_report import generate_report
                    report_file = generate_report(stats, output_dir)
                    print(f"  Report: {report_file}", file=sys.stderr)
                except ImportError:
                    print("  Warning: Could not import generate_report", file=sys.stderr)

    if args.json:
        if len(results) == 1:
            print(json.dumps(results[0], indent=2))
        else:
            print(json.dumps(results, indent=2))

    print(f"\nProcessed {len(results)} session(s)", file=sys.stderr)


if __name__ == "__main__":
    main()
