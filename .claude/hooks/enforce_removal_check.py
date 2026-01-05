#!/usr/bin/env python3
"""
PreToolUse hook: Enforce findReferences before code removal.

Blocks Edit operations that remove significant code constructs without
prior LSP findReferences check. This prevents compilation errors from
blind deletions.

Detects removal of:
- Struct fields
- Functions/methods
- Type definitions
- Enum variants
- Trait implementations
"""
import json
import re
import sys
from pathlib import Path

# Import session state from same directory
sys.path.insert(0, str(Path(__file__).parent))
from session_state import get_state

# Patterns for detecting significant code removal in Rust
REMOVAL_PATTERNS = [
    # Struct field: "pub field_name: Type" or "field_name: Type"
    (r'^\s*(pub\s+)?(\w+)\s*:\s*\w+', 'field'),
    # Function: "fn function_name" or "pub fn function_name"
    (r'^\s*(pub\s+)?(async\s+)?fn\s+(\w+)', 'function'),
    # Struct definition: "struct Name" or "pub struct Name"
    (r'^\s*(pub\s+)?struct\s+(\w+)', 'struct'),
    # Enum definition: "enum Name"
    (r'^\s*(pub\s+)?enum\s+(\w+)', 'enum'),
    # Enum variant: "VariantName" or "VariantName { ... }"
    (r'^\s*(\w+)\s*(\{|,|\()', 'variant'),
    # Type alias: "type Name"
    (r'^\s*(pub\s+)?type\s+(\w+)', 'type'),
    # Impl block: "impl Trait for Type" or "impl Type"
    (r'^\s*impl\s+(\w+)', 'impl'),
    # Const: "const NAME"
    (r'^\s*(pub\s+)?const\s+(\w+)', 'const'),
    # Static: "static NAME"
    (r'^\s*(pub\s+)?static\s+(\w+)', 'static'),
]


def is_deletion(old_string: str, new_string: str) -> bool:
    """Check if this edit is primarily a deletion."""
    old_len = len(old_string.strip())
    new_len = len(new_string.strip())

    # Complete removal
    if new_len == 0 and old_len > 0:
        return True

    # Significant reduction (>50% removed)
    if old_len > 20 and new_len < old_len * 0.5:
        return True

    return False


def extract_removed_symbols(old_string: str, new_string: str) -> list:
    """Extract symbols being removed from the edit."""
    symbols = []

    # Get lines being removed
    old_lines = set(old_string.strip().split('\n'))
    new_lines = set(new_string.strip().split('\n'))
    removed_lines = old_lines - new_lines

    for line in removed_lines:
        for pattern, symbol_type in REMOVAL_PATTERNS:
            match = re.search(pattern, line)
            if match:
                # Extract the symbol name (usually last group)
                groups = [g for g in match.groups() if g and g.strip() not in ('pub', 'async')]
                if groups:
                    symbol_name = groups[-1].strip()
                    if symbol_name and len(symbol_name) > 1:
                        symbols.append({
                            'name': symbol_name,
                            'type': symbol_type,
                            'line': line.strip()[:50]
                        })
                        break

    return symbols


def main():
    try:
        hook_input = json.load(sys.stdin)
    except json.JSONDecodeError:
        print(json.dumps({"decision": "allow"}))
        return

    tool_name = hook_input.get("tool_name", "")
    tool_input = hook_input.get("tool_input", {})

    # Only check Edit operations on .rs files
    if tool_name != "Edit":
        print(json.dumps({"decision": "allow"}))
        return

    file_path = tool_input.get("file_path", "")
    if not file_path.endswith(".rs"):
        print(json.dumps({"decision": "allow"}))
        return

    old_string = tool_input.get("old_string", "")
    new_string = tool_input.get("new_string", "")

    # Check if this is a deletion
    if not is_deletion(old_string, new_string):
        print(json.dumps({"decision": "allow"}))
        return

    # Extract symbols being removed
    removed_symbols = extract_removed_symbols(old_string, new_string)
    if not removed_symbols:
        print(json.dumps({"decision": "allow"}))
        return

    # Check session state for prior findReferences calls
    state = get_state()
    checked_symbols = state.get_checked_symbols()

    unchecked = []
    for sym in removed_symbols:
        if sym['name'] not in checked_symbols:
            unchecked.append(sym)

    if not unchecked:
        # All symbols were checked, allow the removal
        print(json.dumps({"decision": "allow"}))
        return

    # Block the removal - symbols weren't checked
    symbol_list = "\n".join([
        f"  - {s['name']} ({s['type']}): {s['line']}..."
        for s in unchecked[:3]  # Limit to first 3
    ])

    reason = f"""Cannot remove code without checking references first.

Symbols being removed without prior findReferences check:
{symbol_list}

Before removing, run LSP findReferences on each symbol:
  LSP operation="findReferences" filePath="{file_path}" line=<LINE> character=<CHAR>

This prevents compilation errors from removing code that's still in use.
See: .claude/docs/workflow-goals.md (P1: Stability)"""

    print(json.dumps({
        "decision": "block",
        "reason": reason
    }))


if __name__ == "__main__":
    main()
