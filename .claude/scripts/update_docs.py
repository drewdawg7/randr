#!/usr/bin/env python3
"""Update documentation files without per-file permission prompts.

Usage:
    python3 .claude/scripts/update_docs.py write <file> <content>
    python3 .claude/scripts/update_docs.py replace <file> <old> <new>
    python3 .claude/scripts/update_docs.py replace_all <file> <old> <new>
    python3 .claude/scripts/update_docs.py append <file> <content>
    python3 .claude/scripts/update_docs.py insert_after <file> <marker> <content>

All paths are relative to the repo root (.claude/skills/updating-code/).
"""

import sys
import os

REPO_ROOT = os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
DOCS_DIR = os.path.join(REPO_ROOT, ".claude", "skills", "updating-code")


def resolve_path(path):
    """Resolve path relative to docs dir, or use absolute if given."""
    if os.path.isabs(path):
        return path
    return os.path.join(DOCS_DIR, path)


def cmd_write(path, content):
    os.makedirs(os.path.dirname(path), exist_ok=True)
    with open(path, "w") as f:
        f.write(content)
    print(f"Wrote {path}")


def cmd_replace(path, old, new, replace_all=False):
    with open(path, "r") as f:
        text = f.read()
    if old not in text:
        print(f"ERROR: old_string not found in {path}", file=sys.stderr)
        sys.exit(1)
    if not replace_all and text.count(old) > 1:
        print(f"ERROR: old_string has {text.count(old)} occurrences in {path} (use replace_all)", file=sys.stderr)
        sys.exit(1)
    text = text.replace(old, new) if replace_all else text.replace(old, new, 1)
    with open(path, "w") as f:
        f.write(text)
    print(f"Replaced in {path}")


def cmd_append(path, content):
    with open(path, "a") as f:
        f.write(content)
    print(f"Appended to {path}")


def cmd_insert_after(path, marker, content):
    with open(path, "r") as f:
        text = f.read()
    if marker not in text:
        print(f"ERROR: marker not found in {path}", file=sys.stderr)
        sys.exit(1)
    idx = text.index(marker) + len(marker)
    text = text[:idx] + content + text[idx:]
    with open(path, "w") as f:
        f.write(text)
    print(f"Inserted after marker in {path}")


def main():
    if len(sys.argv) < 3:
        print(__doc__)
        sys.exit(1)

    cmd = sys.argv[1]
    path = resolve_path(sys.argv[2])

    if cmd == "write":
        cmd_write(path, sys.argv[3])
    elif cmd == "replace":
        cmd_replace(path, sys.argv[3], sys.argv[4])
    elif cmd == "replace_all":
        cmd_replace(path, sys.argv[3], sys.argv[4], replace_all=True)
    elif cmd == "append":
        cmd_append(path, sys.argv[3])
    elif cmd == "insert_after":
        cmd_insert_after(path, sys.argv[3], sys.argv[4])
    else:
        print(f"Unknown command: {cmd}", file=sys.stderr)
        sys.exit(1)


if __name__ == "__main__":
    main()
