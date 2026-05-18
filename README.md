# use-config

RustUse is “Composable sets of primitive Rust utility crates for fellow crustaceans.”

`use-config` is a primitive configuration vocabulary set. It provides small, composable Rust primitives for configuration keys, values, sources, layers, profiles, and safe secret references.

`use-config` is not a framework. It does not load files, parse formats, read environment variables, watch runtime state, or manage application settings. The crates in this set define stable vocabulary and deterministic helpers that higher-level crates can build on.

Configuration is a general systems primitive, not a Rust-specific concern, so `use-config` is a top-level RustUse set. Rust-specific configuration helpers for Cargo, rust-toolchain files, rustfmt, or clippy can live later in `use-rust` without coupling this set to the Rust toolchain.

## Crates

- `use-config`: facade crate for the full primitive vocabulary set
- `use-config-key`: configuration keys, dotted paths, and sections
- `use-config-value`: deterministic primitive configuration values
- `use-config-source`: source identity and precedence metadata
- `use-config-layer`: shallow configuration map layering and merging
- `use-config-profile`: runtime and deployment profile names
- `use-config-secret`: secret references and redacted value wrappers

## Scope

- primitive configuration names, values, sources, layers, profiles, and secret references
- deterministic data types and shallow merge helpers
- explicit behavior suitable for tests, examples, and higher-level tools
- no file or environment I/O
- no TOML, JSON, YAML, or dotenv parsing
- no async, global state, loaders, watchers, or app settings manager

## Example

```rust
use use_config::prelude::{merge_layers, ConfigLayer, ConfigPath, ConfigSource, ConfigSourceKind, ConfigValue};

let port = ConfigPath::parse("server.port").unwrap();

let mut defaults = ConfigLayer::new(ConfigSource::unnamed(ConfigSourceKind::Default, 0));
defaults.insert(port.clone(), ConfigValue::from(8080_i64));

let mut override_layer = ConfigLayer::new(ConfigSource::unnamed(ConfigSourceKind::Override, 10));
override_layer.insert(port.clone(), ConfigValue::from(9090_i64));

let merged = merge_layers([&defaults, &override_layer]);

assert_eq!(merged.get(&port).and_then(ConfigValue::as_i64), Some(9090));
```

This example only models values and precedence. It does not imply a loader, parser, runtime, or application framework.

## Related Sets

- `use-data`
- `use-fs`
- `use-encoding`
- `use-validate`
- `use-net`
- `use-web`
- `use-rust`

## License

Licensed under either of the following, at your option:

- MIT License
- Apache License, Version 2.0
