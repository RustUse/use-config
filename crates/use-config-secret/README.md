# use-config-secret

Primitive secret references and redacted value wrappers.

`use-config-secret` models references to secrets and values that should not leak through accidental display or debug formatting. It does not store secrets, encrypt secrets, or talk to secret managers.

```rust
use use_config_secret::{Redacted, SecretRef};

let reference = SecretRef::new("database/password").unwrap();
let value = Redacted::new("super-secret");

assert_eq!(reference.to_string(), "secret:database/password");
assert_eq!(value.to_string(), "[redacted]");
assert_eq!(format!("{value:?}"), "[redacted]");
```

`Redacted<T>` deliberately keeps `Debug` and `Display` output redacted.
