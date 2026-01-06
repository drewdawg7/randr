#!/usr/bin/env python3
"""PreToolUse hook: Block Grep/Bash grep on .rs files, suggest LSP instead."""
import json
import re
import sys
from pathlib import Path

# Import session state from same directory
sys.path.insert(0, str(Path(__file__).parent))
from session_state import get_state

def check_bash_grep_rust(command: str) -> bool:
    """Check if a bash command is grep/rg searching Rust files."""
    if not command:
        return False

    # Patterns that indicate searching Rust files via bash
    rust_patterns = [
        r'\bgrep\b.*\.rs\b',           # grep ... .rs
        r'\bgrep\b.*--include.*\.rs',   # grep --include=*.rs
        r'\brg\b.*--type\s*rust',       # rg --type rust
        r'\brg\b.*--type\s*rs',         # rg --type rs
        r'\brg\b.*-t\s*rust',           # rg -t rust
        r'\brg\b.*\.rs\b',              # rg ... .rs
        r'\bgrep\b.*src/',              # grep ... src/ (likely Rust)
        r'\brg\b.*src/',                # rg ... src/ (likely Rust)
    ]

    for pattern in rust_patterns:
        if re.search(pattern, command, re.IGNORECASE):
            return True
    return False

def get_lsp_suggestion(pattern: str = "") -> str:
    """Return helpful LSP suggestions."""
    return f"""Use LSP instead of grep/rg for Rust code navigation.

LSP operations:
- findReferences: Find all usages of a symbol
- goToDefinition: Find where a symbol is defined
- goToImplementation: Find trait implementations
- workspaceSymbol: Search for symbols by name

Example usage:
  LSP operation="findReferences" filePath="src/main.rs" line=10 character=5

For pattern-based searches, use ast-grep:
  python3 .claude/scripts/code/find_symbol.py function {pattern or 'my_function'}
  python3 .claude/scripts/code/find_symbol.py struct {pattern or 'MyStruct'}

Grep is only allowed for: .md files, string literals, comments."""

def main():
    hook_input = json.load(sys.stdin)
    tool_name = hook_input.get("tool_name", "")
    tool_input = hook_input.get("tool_input", {})

    # Check Bash tool for grep/rg on Rust files
    if tool_name == "Bash":
        command = tool_input.get("command", "")
        if check_bash_grep_rust(command):
            # Track the blocked grep attempt
            state = get_state()
            state.record_grep_blocked()
            print(json.dumps({
                "decision": "block",
                "reason": get_lsp_suggestion()
            }))
            return
        print(json.dumps({"decision": "allow"}))
        return

    # Check Grep tool
    if tool_name != "Grep":
        print(json.dumps({"decision": "allow"}))
        return

    # Check if searching .rs files or Rust source directories
    pattern = tool_input.get("pattern", "")
    glob_filter = tool_input.get("glob", "")
    type_filter = tool_input.get("type", "")
    path = tool_input.get("path", "")

    # Detect Rust-related searches
    is_rust_search = (
        "*.rs" in glob_filter or
        type_filter == "rust" or
        type_filter == "rs" or
        path.endswith(".rs")
    )

    # Also detect searches in src/ directory (likely Rust code in this project)
    is_src_search = (
        path.startswith("src/") or
        path.startswith("./src/") or
        "/src/" in path or
        path == "src"
    )

    # Block if searching Rust files OR searching in src/ without explicit non-Rust filter
    if is_rust_search or (is_src_search and not glob_filter and not type_filter):
        # Track the blocked grep attempt
        state = get_state()
        state.record_grep_blocked()
        print(json.dumps({
            "decision": "block",
            "reason": get_lsp_suggestion(pattern)
        }))
    else:
        print(json.dumps({"decision": "allow"}))

if __name__ == "__main__":
    main()
