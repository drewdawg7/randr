#!/usr/bin/env python3
"""Run cargo check with JSON output parsing."""
import json
import subprocess
import sys

def run_cargo_check(package=None):
    """Run cargo check and parse results."""
    cmd = ["cargo", "check", "--message-format=json"]
    if package:
        cmd.extend(["-p", package])

    result = subprocess.run(cmd, capture_output=True, text=True)

    errors = []
    warnings = []

    for line in result.stdout.splitlines():
        try:
            msg = json.loads(line)
            if msg.get("reason") == "compiler-message":
                message = msg.get("message", {})
                level = message.get("level", "")
                rendered = message.get("rendered", "")
                spans = message.get("spans", [])

                location = None
                if spans:
                    span = spans[0]
                    location = {
                        "file": span.get("file_name", ""),
                        "line": span.get("line_start", 0),
                        "column": span.get("column_start", 0)
                    }

                entry = {
                    "message": message.get("message", ""),
                    "rendered": rendered,
                    "location": location
                }

                if level == "error":
                    errors.append(entry)
                elif level == "warning":
                    warnings.append(entry)
        except json.JSONDecodeError:
            continue

    return {
        "success": result.returncode == 0,
        "error_count": len(errors),
        "warning_count": len(warnings),
        "errors": errors,
        "warnings": warnings
    }

def main():
    package = sys.argv[1] if len(sys.argv) > 1 else None
    result = run_cargo_check(package)
    print(json.dumps(result, indent=2))

if __name__ == "__main__":
    main()
