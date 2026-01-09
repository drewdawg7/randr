#!/usr/bin/env python3
"""
Parse debug log files to extract timing and performance data.

This script reads debug logs from ~/.claude/debug/{session_id}.txt
and extracts slow operations, errors, and timing information.
"""
import json
import re
import sys
from pathlib import Path
from typing import Optional
from datetime import datetime


def find_debug_log(session_id: str) -> Optional[Path]:
    """Find the debug log file for a given session ID."""
    debug_dir = Path.home() / ".claude" / "debug"

    if not debug_dir.exists():
        return None

    log_file = debug_dir / f"{session_id}.txt"
    if log_file.exists():
        return log_file

    return None


def parse_timestamp(ts_str: str) -> Optional[str]:
    """Parse timestamp from log line."""
    # Try ISO format: [2026-01-09T10:15:30.123Z]
    match = re.match(r'\[(\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}(?:\.\d+)?Z?)\]', ts_str)
    if match:
        return match.group(1)
    return None


def parse_debug_logs(session_id: str) -> dict:
    """
    Parse a debug log file and extract performance data.

    Returns:
        dict with:
            - slow_operations: List of slow operation entries
            - errors: List of error messages
            - timing: Tool execution timing data
            - source_path: Actual absolute path to the file read
    """
    log_file = find_debug_log(session_id)

    if not log_file:
        return {
            "slow_operations": [],
            "errors": [],
            "timing": {
                "tool_durations": [],
                "average_ms": 0
            },
            "source_path": None
        }

    slow_operations = []
    errors = []
    tool_durations = []

    # Patterns to match
    slow_op_pattern = re.compile(
        r'\[([^\]]+)\].*\[SLOW OPERATION DETECTED\]\s*(.+?)\s+took\s+(\d+)ms',
        re.IGNORECASE
    )
    error_pattern = re.compile(
        r'\[([^\]]+)\]\s*\[ERROR\]\s*(.+)',
        re.IGNORECASE
    )
    tool_timing_pattern = re.compile(
        r'\[([^\]]+)\].*Tool\s+(\w+)\s+completed\s+in\s+(\d+)ms',
        re.IGNORECASE
    )

    with open(log_file, "r") as f:
        for line in f:
            line = line.strip()
            if not line:
                continue

            # Check for slow operations
            slow_match = slow_op_pattern.search(line)
            if slow_match:
                timestamp, operation, duration = slow_match.groups()
                slow_operations.append({
                    "operation": operation.strip(),
                    "duration_ms": int(duration),
                    "timestamp": timestamp
                })
                continue

            # Check for errors
            error_match = error_pattern.search(line)
            if error_match:
                timestamp, message = error_match.groups()
                errors.append({
                    "message": message.strip(),
                    "timestamp": timestamp
                })
                continue

            # Check for tool timing
            timing_match = tool_timing_pattern.search(line)
            if timing_match:
                timestamp, tool_name, duration = timing_match.groups()
                tool_durations.append({
                    "tool": tool_name,
                    "duration_ms": int(duration),
                    "timestamp": timestamp
                })

    # Calculate average tool duration
    avg_duration = 0
    if tool_durations:
        avg_duration = sum(d["duration_ms"] for d in tool_durations) // len(tool_durations)

    return {
        "slow_operations": slow_operations,
        "errors": errors,
        "timing": {
            "tool_durations": tool_durations,
            "average_ms": avg_duration
        },
        "source_path": str(log_file.resolve())
    }


def main():
    """CLI entry point."""
    import argparse

    parser = argparse.ArgumentParser(
        description="Parse debug logs for performance data"
    )
    parser.add_argument("session_id", help="Session ID (UUID)")
    parser.add_argument("--json", action="store_true", help="Output as JSON")

    args = parser.parse_args()

    result = parse_debug_logs(args.session_id)

    if args.json:
        print(json.dumps(result, indent=2))
    else:
        print(f"Session: {args.session_id}")
        print(f"Source: {result['source_path']}")
        print(f"\nSlow Operations: {len(result['slow_operations'])}")
        for op in result["slow_operations"]:
            print(f"  - {op['operation']}: {op['duration_ms']}ms at {op['timestamp']}")
        print(f"\nErrors: {len(result['errors'])}")
        for err in result["errors"]:
            print(f"  - {err['message']}")
        print(f"\nAverage Tool Response: {result['timing']['average_ms']}ms")


if __name__ == "__main__":
    main()
