# use-config-layer

Primitive shallow configuration map layering and merging.

`use-config-layer` pairs a `ConfigSource` with a deterministic map of `ConfigPath` to `ConfigValue`. Merge helpers apply layers in ascending priority order, preserving caller order for equal priorities, so higher-priority or later layers replace earlier values.

Merge behavior is shallow in this first version. If two layers contain the same path, the winning `ConfigValue` replaces the previous value as a whole, including map values.

```rust
use use_config_key::ConfigPath;
use use_config_layer::{merge_layers, ConfigLayer};
use use_config_source::{ConfigSource, ConfigSourceKind};
use use_config_value::ConfigValue;

let path = ConfigPath::parse("server.port").unwrap();
let mut defaults = ConfigLayer::new(ConfigSource::unnamed(ConfigSourceKind::Default, 0));
let mut overrides = ConfigLayer::new(ConfigSource::unnamed(ConfigSourceKind::Override, 10));

defaults.insert(path.clone(), ConfigValue::from(8080_i64));
overrides.insert(path.clone(), ConfigValue::from(9090_i64));

let merged = merge_layers([&defaults, &overrides]);

assert_eq!(merged.get(&path).and_then(ConfigValue::as_i64), Some(9090));
```

This crate does not load, parse, watch, or manage application settings.
