#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

pub use use_config_key;
pub use use_config_layer;
pub use use_config_profile;
pub use use_config_secret;
pub use use_config_source;
pub use use_config_value;

/// Common primitive configuration vocabulary reexports.
pub mod prelude {
    pub use use_config_key::{ConfigKey, ConfigKeyError, ConfigPath, ConfigSection};
    pub use use_config_layer::{ConfigLayer, merge_layers, merge_two_layers};
    pub use use_config_profile::{ConfigProfile, ConfigProfileError};
    pub use use_config_secret::{Redacted, SecretRef, SecretRefError};
    pub use use_config_source::{ConfigSource, ConfigSourceKind};
    pub use use_config_value::ConfigValue;
}

#[cfg(test)]
mod tests {
    use super::prelude::{
        ConfigLayer, ConfigPath, ConfigSource, ConfigSourceKind, ConfigValue, merge_layers,
    };

    #[test]
    fn facade_exposes_configuration_primitives() {
        let path = ConfigPath::parse("server.port").expect("path should parse");
        let mut defaults = ConfigLayer::new(ConfigSource::unnamed(ConfigSourceKind::Default, 0));
        let mut overrides = ConfigLayer::new(ConfigSource::unnamed(ConfigSourceKind::Override, 10));

        defaults.insert(path.clone(), ConfigValue::from(8080_i64));
        overrides.insert(path.clone(), ConfigValue::from(9090_i64));

        let merged = merge_layers([&defaults, &overrides]);

        assert_eq!(merged.get(&path).and_then(ConfigValue::as_i64), Some(9090));
    }
}
