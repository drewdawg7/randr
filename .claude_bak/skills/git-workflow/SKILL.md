---
name: git-workflow
description: Git branch management for code changes. Checkout, commit, merge, and push workflows.
---

## Branch Checkout

Before making code changes, create a descriptive branch from main:

```bash
git checkout main
git pull origin main
git checkout -b <type>/<short-description>
```

**Branch naming conventions:**
- `feat/` - New features
- `fix/` - Bug fixes
- `refactor/` - Code restructuring
- `docs/` - Documentation changes

## Commit Conventions

Commit logical chunks of work for easy rollback:

```bash
git add <files>
git commit -m "<type>: <short description>"
```

**When to commit:**
- After completing a discrete, working change
- Before moving to a different area of the codebase
- After fixing a failing test
- Before risky changes (allows easy revert)

**Commit message types:**
- `feat:` - New functionality
- `fix:` - Bug fix
- `refactor:` - Code restructure without behavior change
- `docs:` - Documentation only
- `test:` - Test additions/changes

## Merge Workflow

After all changes are complete and verified:

```bash
# Ensure all changes are committed
git status

# Push branch to remote
git push -u origin <branch-name>

# Merge into main
git checkout main
git merge <branch-name>

# Push main and delete feature branch
git push origin main
git branch -d <branch-name>
git push origin --delete <branch-name>
```

## Quick Reference

| Action | Command |
|--------|---------|
| Create branch | `git checkout -b type/name` |
| Commit changes | `git commit -m "type: message"` |
| Push branch | `git push -u origin branch` |
| Merge to main | `git checkout main && git merge branch` |
| Delete local | `git branch -d branch` |
| Delete remote | `git push origin --delete branch` |
