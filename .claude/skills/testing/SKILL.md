# Testing Skill

## Running Tests

```bash
# Run all tests
python3 .claude/scripts/check/run_tests.py

# Run specific test
python3 .claude/scripts/check/run_tests.py test_name

# Run tests for a package
python3 .claude/scripts/check/run_tests.py -p package_name
```

## Test Strategy

**Only test changed modules** - Don't write tests for unchanged code.

### What to Test
1. Public API functions
2. Edge cases in logic
3. Error conditions
4. State transitions

### What NOT to Test
1. Private implementation details
2. Simple getters/setters
3. Third-party library behavior
4. UI rendering

## Test Structure

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_feature_does_thing() {
        // Arrange
        let input = setup_test_data();

        // Act
        let result = function_under_test(input);

        // Assert
        assert_eq!(result, expected);
    }
}
```

## Assertions

```rust
assert!(condition);           // Boolean check
assert_eq!(a, b);            // Equality
assert_ne!(a, b);            // Inequality
assert!(result.is_ok());     // Result success
assert!(result.is_err());    // Result failure
```

## Test Workflow

1. Run existing tests first: `python3 .claude/scripts/check/run_tests.py`
2. Write tests for new/changed code
3. Run tests again to verify
4. Fix any failures before committing
