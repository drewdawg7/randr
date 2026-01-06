#!/usr/bin/env python3
"""
PostToolUse hook: Track test execution results.

Monitors Bash commands for cargo test invocations and records results.
"""
import json
import re
import sys
from pathlib import Path

# Import session state from same directory
sys.path.insert(0, str(Path(__file__).parent))
from session_state import get_state


def is_test_command(command: str) -> bool:
    """Check if this is a test command."""
    return bool(re.search(r'\bcargo\s+test\b', command))


def parse_test_result(result: str) -> bool | None:
    """Parse test output to determine pass/fail."""
    if not result:
        return None

    # Look for test result patterns
    if re.search(r'test result: ok\.', result):
        return True
    if re.search(r'test result: FAILED', result):
        return False
    if re.search(r'FAILED', result) and 'error' in result.lower():
        return False
    if re.search(r'passed', result) and not re.search(r'failed', result):
        return True

    return None


def main():
    try:
        hook_input = json.load(sys.stdin)
    except json.JSONDecodeError:
        return

    tool_name = hook_input.get("tool_name", "")
    tool_input = hook_input.get("tool_input", {})
    tool_result = hook_input.get("tool_result", {})

    # Only track Bash operations that are test commands
    if tool_name != "Bash":
        return

    command = tool_input.get("command", "")
    if not is_test_command(command):
        return

    # Parse the result
    result_str = str(tool_result.get("stdout", "")) + str(tool_result.get("stderr", ""))
    passed = parse_test_result(result_str)

    if passed is not None:
        state = get_state()
        state.record_test_result(passed)


if __name__ == "__main__":
    main()
