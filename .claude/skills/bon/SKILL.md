---
name: bon
description: bon crate builder patterns. Use when creating builders, migrating manual builders to bon, or reviewing bon usage.
---

# bon Builder Patterns

## Quick Reference

| Pattern | bon Attribute |
|---------|---------------|
| Required constructor param | `#[builder(start_fn)]` |
| Optional with default | `#[builder(default = value)]` |
| Accept `impl Into<T>` | `#[builder(into)]` or `#[builder(on(String, into))]` |
| Vec accumulator | `#[builder(field)]` + custom method |
| Multi-param setter | Custom method with typestate |

## Simple Builder Pattern

```rust
use bon::Builder;

#[derive(Builder)]
#[builder(on(String, into))]  // All String fields accept impl Into<String>
pub struct Widget {
    #[builder(start_fn)]      // Required in constructor
    pub name: String,

    #[builder(default = 16.0)]
    pub size: f32,

    #[builder(default)]       // Uses Default::default()
    pub enabled: Option<bool>,
}

// Usage: Widget::builder("name").size(20.0).build()
```

## Accumulator Pattern (Vec fields)

Use `#[builder(field)]` for `Command::arg()`-style APIs:

```rust
#[derive(Builder)]
pub struct Config {
    #[builder(field)]  // Lives on builder, not struct
    items: Vec<String>,

    #[builder(default)]
    limit: Option<u32>,
}

use config_builder::State;

impl<S: State> ConfigBuilder<S> {
    pub fn item(mut self, item: impl Into<String>) -> Self {
        self.items.push(item.into());
        self
    }
}

// Usage: Config::builder().item("a").item("b").limit(10).build()
```

## Custom Multi-Param Setter

```rust
use widget_builder::State;

impl<S: State> WidgetBuilder<S> {
    pub fn with_position(self, x: f32, y: f32) -> Self {
        self.position(Some((x, y)))
    }
}
```

## Migration Checklist

When adding `#[derive(Builder)]`:
- [ ] Add `use bon::Builder;`
- [ ] Mark required params with `#[builder(start_fn)]`
- [ ] Add `#[builder(default = ...)]` for optional fields
- [ ] Use `#[builder(on(String, into))]` for String fields
- [ ] Use `#[builder(field)]` for Vec accumulators
- [ ] Delete manual `impl` block with `new()` and `with_*()` methods
- [ ] Update call sites: `.new()` → `.builder().build()`

## API Changes

| Before | After |
|--------|-------|
| `Foo::new(x)` | `Foo::builder(x).build()` |
| `.with_bar(y)` | `.bar(y)` |
| `Option<T>` field | Also generates `.maybe_bar(Option<T>)` |

## References

- [bon documentation](https://docs.rs/bon/latest/bon/)
- [bon `#[builder(start_fn)]`](https://bon-rs.com/reference/builder/member/start-fn)
- [bon `#[builder(default)]`](https://bon-rs.com/reference/builder/member/default)
- [bon `#[builder(field)]`](https://bon-rs.com/reference/builder/member/field)
- [bon custom methods](https://bon-rs.com/guide/typestate-api/custom-methods)

See [references/migration-examples.md](references/migration-examples.md) for codebase-specific examples.
