# use-config-value

Primitive deterministic configuration value representation.

`use-config-value` provides a small `ConfigValue` enum for nulls, booleans, integers, floats, strings, lists, and maps. Maps use `BTreeMap` so key order is deterministic.

```rust
use std::collections::BTreeMap;
use use_config_value::ConfigValue;

let mut values = BTreeMap::new();
values.insert("port".to_owned(), ConfigValue::from(8080_i64));

let value = ConfigValue::from(values);

assert_eq!(value.as_map().unwrap()["port"].as_i64(), Some(8080));
```

This crate does not serialize, deserialize, or depend on `serde`.
