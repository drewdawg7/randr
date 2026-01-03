---
name: clean-duplicates
description: Identify and close duplicate GitHub issues. Use when cleaning up issues, finding duplicates, or when asked to deduplicate the issue tracker.
---

# Clean Duplicates

Identifies duplicate GitHub issues, marks them as duplicates, and closes them with a reference to the original issue.

## Workflow

1. **List issues**: Use `list_issues.py` to fetch all open issues with titles and bodies
2. **Find duplicates**: Use `find_duplicates.py` to identify potential duplicates based on title/body similarity
3. **Review**: Present duplicate candidates to user for confirmation
4. **Close duplicates**: Use `close_duplicate.py` to:
   - Add 'duplicate' label
   - Post comment referencing original issue
   - Close issue with reason "not planned"

## Important

- **Always confirm** with user before closing any issue
- **Preserve** the older/more detailed issue as the original
- **Link** to the original issue in the closing comment
- Issues with more comments/activity are usually the canonical one

## Helper Scripts

Python scripts in `scripts/` directory automate common tasks. All output JSON.

### list_issues.py - Fetch All Issues
```bash
python .claude/skills/clean-duplicates/scripts/list_issues.py [--state open|closed|all]
```
Fetches all issues with number, title, body, labels, and creation date.

### find_duplicates.py - Identify Potential Duplicates
```bash
python .claude/skills/clean-duplicates/scripts/find_duplicates.py [--threshold 0.4]
```
Compares all open issues and identifies potential duplicates based on:
- Title similarity (keyword overlap, 50% weight)
- Body keyword overlap (35% weight)
- Label overlap (15% weight)

### close_duplicate.py - Close as Duplicate
```bash
python .claude/skills/clean-duplicates/scripts/close_duplicate.py <duplicate_number> <original_number>
```
Marks issue as duplicate of another:
- Adds 'duplicate' label (creates if needed)
- Posts linking comment
- Closes with "not planned" reason
