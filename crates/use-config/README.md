# use-config

Facade crate for the `use-config` primitive configuration vocabulary set.

`use-config` reexports focused crates for configuration keys, values, sources, layers, profiles, and safe secret references. It is not a configuration framework and does not load files, parse formats, read environment variables, or manage application settings.

```rust
use use_config::prelude::{ConfigPath, ConfigValue};

let path = ConfigPath::parse("server.port").unwrap();
let value = ConfigValue::from(8080_i64);

assert_eq!(path.to_string(), "server.port");
assert_eq!(value.as_i64(), Some(8080));
```

Use the focused crates directly when you only need one part of the vocabulary.
