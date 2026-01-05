# Git Workflow Skill

## Branch Naming Convention

Format: `<type>/<description>`

| Type | Use For |
|------|---------|
| `feat` | New features |
| `fix` | Bug fixes |
| `refactor` | Code restructuring |
| `docs` | Documentation only |
| `test` | Test additions/fixes |
| `chore` | Maintenance tasks |

## Workflow Steps

### 1. Create Branch
```bash
python3 .claude/scripts/git/branch.py feat/my-feature
```

### 2. Make Changes
Edit files, run `cargo check` after each change.

### 3. Commit
```bash
python3 .claude/scripts/git/commit.py "feat: add new feature"
```

Commit message format: `<type>[(scope)]: <description>`

### 4. Merge to Main
```bash
python3 .claude/scripts/git/merge.py
```

This will:
1. Switch to main
2. Pull latest
3. Merge your branch
4. Push to remote
5. Delete the feature branch

## Rules

1. **Never commit directly to main** - Always use feature branches
2. **Use conventional commits** - Enforced by scripts
3. **Run cargo check before commit** - Catches errors early
4. **One feature per branch** - Keep changes focused
