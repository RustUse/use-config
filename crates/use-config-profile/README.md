# use-config-profile

Primitive runtime and deployment profile names.

`use-config-profile` models profile names such as local, development, test, staging, production, and custom names. Common aliases like `dev` and `prod` normalize to stable variants.

```rust
use std::str::FromStr;
use use_config_profile::ConfigProfile;

assert_eq!(ConfigProfile::from_str("dev").unwrap(), ConfigProfile::Development);
assert_eq!(ConfigProfile::Custom("preview".to_owned()).to_string(), "preview");
```

Profiles carry no file, environment, secret, or runtime behavior.
