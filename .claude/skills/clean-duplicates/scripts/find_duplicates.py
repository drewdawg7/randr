#!/usr/bin/env python3
"""
find_duplicates.py - Identify potential duplicate GitHub issues

Usage: python find_duplicates.py [--threshold 0.4]

Compares open issues and identifies potential duplicates based on:
- Title similarity (keyword overlap, 50% weight)
- Body keyword overlap (35% weight)
- Label overlap (15% weight)

Output: JSON with duplicate candidates sorted by confidence
"""

import argparse
import json
import re
import subprocess
from typing import Any


def run_cmd(cmd: list[str]) -> tuple[bool, str]:
    """Run a command and return (success, output)."""
    try:
        result = subprocess.run(cmd, capture_output=True, text=True, check=True)
        return True, result.stdout.strip()
    except subprocess.CalledProcessError as e:
        return False, e.stderr.strip()


def get_open_issues() -> list[dict[str, Any]]:
    """Fetch all open issues."""
    success, output = run_cmd([
        "gh", "issue", "list",
        "--state", "open",
        "--json", "number,title,body,labels,createdAt,comments",
        "--limit", "500"
    ])
    if not success or not output:
        return []
    return json.loads(output)


def normalize_text(text: str) -> str:
    """Lowercase, remove punctuation, normalize whitespace."""
    if not text:
        return ""
    text = text.lower()
    text = re.sub(r'[^\w\s]', ' ', text)
    text = re.sub(r'\s+', ' ', text).strip()
    return text


def get_keywords(text: str) -> set[str]:
    """Extract meaningful keywords (3+ chars) from text."""
    words = normalize_text(text).split()
    # Filter short words and common stop words
    stop_words = {
        'the', 'and', 'for', 'that', 'this', 'with', 'from', 'are', 'was',
        'will', 'can', 'has', 'have', 'been', 'being', 'would', 'could',
        'should', 'into', 'also', 'when', 'where', 'which', 'while', 'about'
    }
    return {w for w in words if len(w) >= 3 and w not in stop_words}


def similarity_score(title1: str, title2: str, body1: str, body2: str,
                     labels1: list, labels2: list) -> float:
    """Calculate similarity score between two issues (0.0 to 1.0)."""
    scores = []

    # Title similarity (word overlap) - 50% weight
    t1_words = get_keywords(title1)
    t2_words = get_keywords(title2)
    if t1_words and t2_words:
        title_overlap = len(t1_words & t2_words) / max(len(t1_words), len(t2_words))
        scores.append(title_overlap * 0.5)

    # Body keyword overlap - 35% weight
    b1_words = get_keywords(body1)
    b2_words = get_keywords(body2)
    if b1_words and b2_words:
        body_overlap = len(b1_words & b2_words) / max(len(b1_words), len(b2_words))
        scores.append(body_overlap * 0.35)

    # Label overlap - 15% weight
    if labels1 and labels2:
        label_overlap = len(set(labels1) & set(labels2)) / max(len(labels1), len(labels2))
        scores.append(label_overlap * 0.15)

    return sum(scores) if scores else 0.0


def find_duplicates(issues: list[dict], threshold: float) -> list[dict]:
    """Find potential duplicate pairs above threshold."""
    duplicates = []

    for i, issue1 in enumerate(issues):
        for issue2 in issues[i+1:]:
            labels1 = [l.get("name", "") for l in issue1.get("labels", [])]
            labels2 = [l.get("name", "") for l in issue2.get("labels", [])]

            # Skip if either is already marked as duplicate
            if "duplicate" in labels1 or "duplicate" in labels2:
                continue

            score = similarity_score(
                issue1.get("title", ""),
                issue2.get("title", ""),
                issue1.get("body", "") or "",
                issue2.get("body", "") or "",
                labels1,
                labels2
            )

            if score >= threshold:
                # Determine which should be the "original" (more comments wins)
                i1_comments = len(issue1.get("comments", []))
                i2_comments = len(issue2.get("comments", []))

                if i1_comments >= i2_comments:
                    original, duplicate = issue1, issue2
                else:
                    original, duplicate = issue2, issue1

                duplicates.append({
                    "duplicate": {
                        "number": duplicate.get("number"),
                        "title": duplicate.get("title"),
                    },
                    "original": {
                        "number": original.get("number"),
                        "title": original.get("title"),
                    },
                    "similarity": round(score, 3),
                    "reason": f"Title/body similarity: {score:.0%}"
                })

    # Sort by similarity (highest first)
    duplicates.sort(key=lambda x: x["similarity"], reverse=True)
    return duplicates


def main():
    parser = argparse.ArgumentParser(description="Find potential duplicate GitHub issues")
    parser.add_argument("--threshold", type=float, default=0.4,
                        help="Similarity threshold 0.0-1.0 (default: 0.4)")
    args = parser.parse_args()

    issues = get_open_issues()
    if not issues:
        print(json.dumps({"duplicates": [], "count": 0, "error": "No issues found"}))
        return

    duplicates = find_duplicates(issues, args.threshold)

    print(json.dumps({
        "duplicates": duplicates,
        "count": len(duplicates),
        "threshold": args.threshold,
        "issues_analyzed": len(issues)
    }, indent=2))


if __name__ == "__main__":
    main()
