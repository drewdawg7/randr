#!/usr/bin/env python3
"""PreToolUse hook: Block Grep on .rs files, suggest LSP instead."""
import json
import sys

def main():
    hook_input = json.load(sys.stdin)
    tool_name = hook_input.get("tool_name", "")
    tool_input = hook_input.get("tool_input", {})

    if tool_name != "Grep":
        print(json.dumps({"decision": "allow"}))
        return

    # Check if searching .rs files
    pattern = tool_input.get("pattern", "")
    glob_filter = tool_input.get("glob", "")
    type_filter = tool_input.get("type", "")
    path = tool_input.get("path", "")

    is_rust_search = (
        "*.rs" in glob_filter or
        type_filter == "rust" or
        type_filter == "rs" or
        path.endswith(".rs")
    )

    if is_rust_search:
        print(json.dumps({
            "decision": "block",
            "reason": f"""Use LSP instead of Grep for Rust code navigation.

LSP operations available:
- goToDefinition: Find where '{pattern}' is defined
- findReferences: Find all usages of '{pattern}'
- goToImplementation: Find trait implementations
- workspaceSymbol: Search for symbols matching '{pattern}'

Example:
  LSP tool with operation="workspaceSymbol", filePath="src/main.rs", line=1, character=1

Grep is only for: .md files, string literals, comments, or when LSP fails."""
        }))
    else:
        print(json.dumps({"decision": "allow"}))

if __name__ == "__main__":
    main()
