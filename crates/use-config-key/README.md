# use-config-key

Primitive configuration key, path, and section types.

`use-config-key` models names for configuration values. It supports plain keys such as `port` and dotted paths such as `server.port`, while rejecting empty keys and empty path segments.

```rust
use use_config_key::ConfigPath;

let path = ConfigPath::parse("server.port").unwrap();
let segments: Vec<_> = path.segments().collect();

assert_eq!(segments, vec!["server", "port"]);
assert_eq!(path.to_string(), "server.port");
```

This crate only models names. It does not read files, parse configuration formats, or resolve values.
