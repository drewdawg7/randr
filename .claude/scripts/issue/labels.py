#!/usr/bin/env python3
"""List GitHub repository labels."""
import json
import subprocess
import sys

def run_gh(*args):
    """Run a gh command and return result."""
    result = subprocess.run(
        ["gh"] + list(args),
        capture_output=True,
        text=True
    )
    return result

def list_labels():
    """List all repository labels."""
    result = run_gh("label", "list", "--json", "name,description,color")

    if result.returncode != 0:
        return {"success": False, "error": result.stderr.strip()}

    try:
        labels = json.loads(result.stdout)
    except json.JSONDecodeError:
        return {"success": False, "error": "Failed to parse label data"}

    return {
        "success": True,
        "count": len(labels),
        "labels": labels
    }

def main():
    result = list_labels()
    print(json.dumps(result, indent=2))

if __name__ == "__main__":
    main()
