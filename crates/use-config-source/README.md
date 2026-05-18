# use-config-source

Primitive configuration source identity and precedence metadata.

`use-config-source` models where a configuration value came from and how sources compare by priority. It describes source identity only.

```rust
use use_config_source::{ConfigSource, ConfigSourceKind};

let source = ConfigSource::named(ConfigSourceKind::Custom("fixture".to_owned()), "baseline", 5);

assert_eq!(source.priority(), 5);
assert_eq!(source.to_string(), "fixture:baseline@5");
```

This crate does not read files, environment variables, secret managers, or runtime state.
