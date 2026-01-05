# Workflow Efficiency Goals

Priority order for all workflow improvements: **Stability > Token Usage > Speed**

## Goal Priority (Highest to Lowest)

### P1: Stability
**Definition:** Changes are consistent, use necessary tools, no reverts needed

| Requirement | Enforcement |
|-------------|-------------|
| Run LSP findReferences before ANY code removal | Hook blocks removal without prior check |
| Use LSP for all Rust navigation (not grep) | Hook blocks grep on .rs files |
| Consistent tool selection across sessions | Decision trees in CLAUDE.md |
| No compilation errors from blind deletions | Pre-removal enforcement |

**Metrics:**
- Revert count per session: **Target 0**
- Compilation errors from removals: **Target 0**

---

### P2: Token Usage
**Definition:** Efficient token use, less waste

| Requirement | Enforcement |
|-------------|-------------|
| Use ast-grep for 5+ similar changes | Edit pattern detection hook |
| Parallelize Read calls when reading 3+ files | Documentation in CLAUDE.md |
| Avoid repeated exploration of same code paths | Session state tracking |
| Track token usage per operation type | Feedback template metrics |

**Metrics:**
- Tokens per file edited
- Tokens per line changed
- Manual edit ratio: **Target <20%**

---

### P3: Speed
**Definition:** Changes happen faster via LSP/ast-grep

| Requirement | Enforcement |
|-------------|-------------|
| Prefer ast-grep batch operations over sequential edits | Skill documentation |
| Use LSP for instant navigation (vs grep scanning) | Hook enforcement |
| Parallel operations where possible | Decision trees |

**Metrics:**
- Time per file edited
- Batch operation usage rate

---

## Application to Agent Constraints

All agents must operate within these goal constraints:

| Agent | Stability Constraints | Token Constraints | Speed Constraints |
|-------|----------------------|-------------------|-------------------|
| code-change | Verify findReferences before delegating removals | Delegate to avoid redundant exploration | Use parallel reads |
| coder | Run findReferences before any removal | Use ast-grep for 5+ similar changes | Use LSP, not grep |
| reviewer | Check for removal-without-reference patterns | Flag inefficient edit patterns | - |
| test-writer | - | Test only changed modules | - |

## Issue Mapping

All workflow improvement issues support these goals:

| Issue | Stability | Token | Speed |
|-------|-----------|-------|-------|
| #98 Session state | Enables | Enables | Enables |
| #99 Edit detection | ✓ | ✓✓ | ✓✓ |
| #100 Pre-removal | ✓✓ | ✓ | ✓ |
| #101 Agent constraints | ✓✓ | ✓✓ | ✓✓ |
| #102 Token tracking | - | ✓✓ | - |
| #103 Decision trees | ✓ | ✓ | ✓ |
| #104 Code-nav update | ✓ | ✓ | ✓✓ |

## Quick Reference

Before making changes, check:
1. **Removing code?** → LSP findReferences FIRST (P1: Stability)
2. **5+ similar changes?** → ast-grep, not manual edits (P2: Token)
3. **Reading 3+ files?** → Parallel Read calls (P2: Token)
4. **Navigating Rust?** → LSP, not grep (P3: Speed)
