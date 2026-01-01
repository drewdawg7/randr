# Adding New Locations

## Quick Checklist

1. [ ] Add `LocationId` variant to `enums.rs`
2. [ ] Add subtype to relevant category in `enums.rs`
3. [ ] Update `LocationId::location_type()` match
4. [ ] Add `LocationData` variant to `spec/definition.rs`
5. [ ] Create location submodule (`src/location/<name>/`)
6. [ ] Add spec to `spec/specs.rs`
7. [ ] Export from `location/mod.rs`
8. [ ] Add field to Town in `town/definition.rs`
9. [ ] Update Town helper methods
10. [ ] Update `system.rs` to use `from_spec()` for the new location

## Reference: Existing Locations

Use these as templates when adding new locations:

| Location | Type | Files |
|----------|------|-------|
| Store | Commerce | `src/location/store/` |
| Blacksmith | Crafting | `src/location/blacksmith/` |
| Field | Combat | `src/location/field/` |
| Mine | Resource | `src/location/mine/` |

## Key Files to Modify

### enums.rs
Add `LocationId` variant and update `location_type()` match.

### spec/definition.rs
Add `LocationData` variant with location-specific config struct.

### spec/specs.rs
Add static `Lazy<LocationSpec>` constant.

### location/mod.rs
Export the new submodule and struct.

### town/definition.rs
Add field and update helper methods.

### system.rs
Add `from_spec()` call for the new location:
```rust
let new_loc = match &NEW_LOCATION_SPEC.data {
    LocationData::NewType(data) => NewLocation::from_spec(&NEW_LOCATION_SPEC, data),
    _ => unreachable!(),
};
```

## Location Submodule Structure

```
<location>/
├── mod.rs          # Exports
├── definition.rs   # Struct + from_spec() + methods
├── traits.rs       # Location trait impl + Default
└── enums.rs        # Errors (if needed)
```

## Testing

Always run after changes:
```bash
cargo check
cargo test
```
