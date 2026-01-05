#!/usr/bin/env python3
"""Find symbols using ast-grep for Rust code."""
import json
import subprocess
import sys

def run_ast_grep(pattern, lang="rust"):
    """Run ast-grep to find pattern matches."""
    cmd = ["ast-grep", "--pattern", pattern, "--lang", lang, "--json"]

    result = subprocess.run(cmd, capture_output=True, text=True)

    if result.returncode != 0 and not result.stdout:
        return {"success": False, "error": result.stderr.strip() or "No matches found"}

    try:
        # ast-grep outputs a JSON array
        raw_matches = json.loads(result.stdout) if result.stdout.strip() else []
        matches = []
        for match in raw_matches:
            matches.append({
                "file": match.get("file", ""),
                "line": match.get("range", {}).get("start", {}).get("line", 0),
                "column": match.get("range", {}).get("start", {}).get("column", 0),
                "text": match.get("text", ""),
                "lines": match.get("lines", "")
            })

        return {
            "success": True,
            "count": len(matches),
            "matches": matches
        }
    except json.JSONDecodeError as e:
        return {"success": False, "error": f"Failed to parse output: {e}"}

def find_function(name):
    """Find a function definition."""
    # Try with pub first, then without
    result = run_ast_grep(f"pub fn {name}($$$) $$$")
    if not result.get("success") or result.get("count", 0) == 0:
        result = run_ast_grep(f"fn {name}($$$) $$$")
    return result

def find_struct(name):
    """Find a struct definition."""
    # Try with pub first, then without
    result = run_ast_grep(f"pub struct {name} {{ $$$ }}")
    if not result.get("success") or result.get("count", 0) == 0:
        result = run_ast_grep(f"struct {name} {{ $$$ }}")
    return result

def find_impl(name):
    """Find impl blocks for a type."""
    return run_ast_grep(f"impl $_ for {name} $$$")

def find_trait(name):
    """Find a trait definition."""
    result = run_ast_grep(f"pub trait {name} $$$")
    if not result.get("success") or result.get("count", 0) == 0:
        result = run_ast_grep(f"trait {name} $$$")
    return result

def main():
    if len(sys.argv) < 3:
        print(json.dumps({
            "success": False,
            "error": "Usage: find_symbol.py <type> <name>",
            "types": ["function", "struct", "impl", "trait", "pattern"]
        }))
        sys.exit(1)

    symbol_type = sys.argv[1]
    name = sys.argv[2]

    if symbol_type == "function":
        result = find_function(name)
    elif symbol_type == "struct":
        result = find_struct(name)
    elif symbol_type == "impl":
        result = find_impl(name)
    elif symbol_type == "trait":
        result = find_trait(name)
    elif symbol_type == "pattern":
        result = run_ast_grep(name)
    else:
        result = {"success": False, "error": f"Unknown type: {symbol_type}"}

    print(json.dumps(result, indent=2))

if __name__ == "__main__":
    main()
