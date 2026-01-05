# Script Issues Encountered

## Date: 2025-01-05

### Issue 1: `create.py` argument parsing bug

**Script:** `.claude/scripts/issue/create.py`
**Command:**
```bash
python3 .claude/scripts/issue/create.py --title "Research: UI Framework Migration..." --body "..." --label "enhancement"
```

**Expected:** Issue created with correct title
**Actual:** Issue #124 created with title "--title" instead of the actual title

**Impact:** Issue content is correct but title is wrong, requires manual fix.

**Root Cause:** Likely argparse configuration issue - the `--title` flag name is being captured as the value instead of the argument following it.

---

### Issue 2: Missing `edit.py` script

**Context:** Hook blocks `gh issue edit` and suggests using `python3 .claude/scripts/issue/edit.py`
**Problem:** The script doesn't exist

**Hook message:**
```
Use helper scripts instead of raw gh commands.
Blocked command: gh issue edit 124 --title "..."
Use instead: python3 .claude/scripts/issue/edit.py
```

**Impact:** Cannot edit issues programmatically - must be done manually on GitHub.

---

### Issue 3: Missing `labels.py` script

**Context:** Hook suggests `python3 .claude/scripts/issue/labels.py` for listing labels
**Problem:** The script doesn't exist

**Impact:** Cannot query available labels before creating issues, leading to trial-and-error with label names.

---

## Recommendations

1. **Fix `create.py`:** Debug argparse configuration to correctly capture `--title` argument value
2. **Add `edit.py`:** Create script for editing issue title, body, labels, assignees
3. **Add `labels.py`:** Create script for listing available repository labels
4. **Update hook:** Don't suggest scripts that don't exist, or create all suggested scripts
