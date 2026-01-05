#!/usr/bin/env python3
"""
PostToolUse hook: Track LSP operations for session state.

Records LSP operations, especially findReferences, so that the
pre-removal enforcement hook can verify symbols were checked.
"""
import json
import re
import sys
from pathlib import Path

# Import session state from same directory
sys.path.insert(0, str(Path(__file__).parent))
from session_state import get_state


def extract_symbol_from_result(tool_result: dict) -> list:
    """Try to extract symbol names from LSP result."""
    symbols = []

    # The result might contain symbol information in various formats
    content = str(tool_result)

    # Look for common symbol patterns in the result
    # This is best-effort since LSP result format varies
    patterns = [
        r'(\w+)\s*:\s*\w+',  # field: Type
        r'fn\s+(\w+)',       # fn name
        r'struct\s+(\w+)',   # struct Name
        r'enum\s+(\w+)',     # enum Name
    ]

    for pattern in patterns:
        matches = re.findall(pattern, content)
        symbols.extend(matches)

    return list(set(symbols))[:5]  # Limit to 5 unique symbols


def main():
    try:
        hook_input = json.load(sys.stdin)
    except json.JSONDecodeError:
        return

    tool_name = hook_input.get("tool_name", "")
    tool_input = hook_input.get("tool_input", {})
    tool_result = hook_input.get("tool_result", {})

    # Only track LSP operations
    if tool_name != "LSP":
        return

    operation = tool_input.get("operation", "")
    file_path = tool_input.get("filePath", "")
    line = tool_input.get("line", 0)
    character = tool_input.get("character", 0)

    state = get_state()

    # Record LSP call
    state.increment("lsp_calls")

    # For findReferences, try to record what symbol was checked
    if operation == "findReferences":
        # Try to extract symbol from result
        symbols = extract_symbol_from_result(tool_result)

        # Also try to get from the file context if we have it
        # The hook doesn't have easy access to file content, so this is best-effort

        for symbol in symbols:
            if symbol and len(symbol) > 1:
                state.record_symbol_check(symbol)

        # Record a generic marker with location info
        location_key = f"{file_path}:{line}:{character}"
        state.add_to_set("locations_checked", location_key)


if __name__ == "__main__":
    main()
