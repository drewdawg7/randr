#!/usr/bin/env python3
"""Run cargo tests with JSON output parsing."""
import json
import subprocess
import sys
import re

def run_tests(filter_pattern=None, package=None):
    """Run cargo test and parse results."""
    cmd = ["cargo", "test"]
    if package:
        cmd.extend(["-p", package])
    if filter_pattern:
        cmd.append(filter_pattern)
    cmd.append("--")
    cmd.append("--nocapture")

    result = subprocess.run(cmd, capture_output=True, text=True)

    # Parse test output
    output = result.stdout + result.stderr
    passed = []
    failed = []
    ignored = []

    # Match test results
    for line in output.splitlines():
        if " ... ok" in line:
            test_name = line.split(" ... ")[0].strip()
            if test_name.startswith("test "):
                test_name = test_name[5:]
            passed.append(test_name)
        elif " ... FAILED" in line:
            test_name = line.split(" ... ")[0].strip()
            if test_name.startswith("test "):
                test_name = test_name[5:]
            failed.append(test_name)
        elif " ... ignored" in line:
            test_name = line.split(" ... ")[0].strip()
            if test_name.startswith("test "):
                test_name = test_name[5:]
            ignored.append(test_name)

    # Extract failure details
    failure_details = {}
    if failed:
        # Parse failure messages
        failure_section = False
        current_test = None
        current_output = []

        for line in output.splitlines():
            if line.startswith("---- ") and " stdout ----" in line:
                if current_test and current_output:
                    failure_details[current_test] = "\n".join(current_output)
                current_test = line.replace("---- ", "").replace(" stdout ----", "").strip()
                current_output = []
            elif current_test:
                current_output.append(line)

        if current_test and current_output:
            failure_details[current_test] = "\n".join(current_output)

    return {
        "success": result.returncode == 0,
        "passed": len(passed),
        "failed": len(failed),
        "ignored": len(ignored),
        "passed_tests": passed,
        "failed_tests": failed,
        "ignored_tests": ignored,
        "failure_details": failure_details
    }

def main():
    filter_pattern = None
    package = None

    i = 1
    while i < len(sys.argv):
        arg = sys.argv[i]
        if arg == "-p" and i + 1 < len(sys.argv):
            package = sys.argv[i + 1]
            i += 2
        else:
            filter_pattern = arg
            i += 1

    result = run_tests(filter_pattern, package)
    print(json.dumps(result, indent=2))

if __name__ == "__main__":
    main()
