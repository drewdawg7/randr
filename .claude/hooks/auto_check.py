#!/usr/bin/env python3
"""PostToolUse hook: Auto cargo check after editing .rs files."""
import json
import sys
import subprocess

def run_cargo_check():
    """Run cargo check and return results."""
    try:
        result = subprocess.run(
            ["cargo", "check", "--message-format=json"],
            capture_output=True,
            text=True,
            timeout=60
        )

        errors = []
        warnings = []

        for line in result.stdout.splitlines():
            try:
                msg = json.loads(line)
                if msg.get("reason") == "compiler-message":
                    message = msg.get("message", {})
                    level = message.get("level", "")
                    rendered = message.get("rendered", "")

                    if level == "error":
                        errors.append(rendered)
                    elif level == "warning":
                        warnings.append(rendered)
            except json.JSONDecodeError:
                continue

        return {
            "success": result.returncode == 0,
            "errors": errors[:5],  # Limit to first 5
            "warnings": warnings[:3],  # Limit to first 3
            "error_count": len(errors),
            "warning_count": len(warnings)
        }
    except subprocess.TimeoutExpired:
        return {"success": False, "errors": ["cargo check timed out"], "warnings": []}
    except Exception as e:
        return {"success": False, "errors": [str(e)], "warnings": []}

def main():
    hook_input = json.load(sys.stdin)
    tool_name = hook_input.get("tool_name", "")
    tool_input = hook_input.get("tool_input", {})
    tool_result = hook_input.get("tool_result", {})

    # Only run after successful Edit/Write on .rs files
    if tool_name not in ("Edit", "Write"):
        return

    file_path = tool_input.get("file_path", "")
    if not file_path.endswith(".rs"):
        return

    # Run cargo check
    result = run_cargo_check()

    if not result["success"]:
        output = f"⚠️ cargo check failed after editing {file_path}\n\n"
        if result["errors"]:
            output += "Errors:\n" + "\n".join(result["errors"])
        print(output, file=sys.stderr)
    elif result["warning_count"] > 0:
        output = f"✓ cargo check passed ({result['warning_count']} warnings)\n"
        print(output, file=sys.stderr)

if __name__ == "__main__":
    main()
