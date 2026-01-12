# Rust Idioms

Preferred Rust patterns used throughout the codebase.

## Option Handling

### Use `map_or` for Option Defaults

When extracting a value from an `Option` with a default fallback, prefer `map_or` over `match`:

```rust
// Preferred
self.stat(t).map_or(0, |si| si.current_value)

// Avoid
match self.stat(t) {
    Some(si) => si.current_value,
    None     => 0
}
```

**Examples in codebase:**
- `StatSheet::value()` in `src/stats/definition.rs:41`
- `StatSheet::max_value()` in `src/stats/definition.rs:45`
