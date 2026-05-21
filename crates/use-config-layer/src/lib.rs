#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use std::collections::BTreeMap;

use use_config_key::ConfigPath;
use use_config_source::ConfigSource;
use use_config_value::ConfigValue;

/// A deterministic map of configuration values paired with source metadata.
#[derive(Clone, Debug, PartialEq)]
pub struct ConfigLayer {
    /// Source metadata for this layer.
    pub source: ConfigSource,
    values: BTreeMap<ConfigPath, ConfigValue>,
}

impl ConfigLayer {
    /// Creates an empty layer for a source.
    #[must_use]
    pub const fn new(source: ConfigSource) -> Self {
        Self {
            source,
            values: BTreeMap::new(),
        }
    }

    /// Creates a layer from source metadata and values.
    #[must_use]
    pub const fn from_values(
        source: ConfigSource,
        values: BTreeMap<ConfigPath, ConfigValue>,
    ) -> Self {
        Self { source, values }
    }

    /// Returns the layer source.
    #[must_use]
    pub const fn source(&self) -> &ConfigSource {
        &self.source
    }

    /// Returns the deterministic value map.
    #[must_use]
    pub const fn values(&self) -> &BTreeMap<ConfigPath, ConfigValue> {
        &self.values
    }

    /// Returns the number of values in the layer.
    #[must_use]
    pub fn len(&self) -> usize {
        self.values.len()
    }

    /// Returns `true` when the layer has no values.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    /// Inserts a path value into the layer.
    pub fn insert(&mut self, path: ConfigPath, value: ConfigValue) -> Option<ConfigValue> {
        self.values.insert(path, value)
    }

    /// Returns a value for a path.
    #[must_use]
    pub fn get(&self, path: &ConfigPath) -> Option<&ConfigValue> {
        self.values.get(path)
    }

    /// Iterates over values in deterministic path order.
    pub fn iter(&self) -> impl Iterator<Item = (&ConfigPath, &ConfigValue)> {
        self.values.iter()
    }
}

/// Merges two layers using shallow layer semantics.
///
/// Layers are applied in ascending priority order. If priorities match, the
/// second layer wins for duplicate paths.
#[must_use]
pub fn merge_two_layers(
    first: &ConfigLayer,
    second: &ConfigLayer,
) -> BTreeMap<ConfigPath, ConfigValue> {
    merge_layers([first, second])
}

/// Merges many layers using shallow layer semantics.
///
/// Layers are applied in ascending priority order. Caller order is preserved
/// for equal priorities, so later equal-priority layers replace earlier values.
/// Map values are replaced as whole values; nested map merging is intentionally
/// not part of this first version.
#[must_use]
pub fn merge_layers<'a, I>(layers: I) -> BTreeMap<ConfigPath, ConfigValue>
where
    I: IntoIterator<Item = &'a ConfigLayer>,
{
    let mut ordered_layers: Vec<_> = layers.into_iter().enumerate().collect();

    ordered_layers.sort_by(|(left_index, left), (right_index, right)| {
        left.source
            .priority()
            .cmp(&right.source.priority())
            .then_with(|| left_index.cmp(right_index))
    });

    let mut merged = BTreeMap::new();

    for (_, layer) in ordered_layers {
        for (path, value) in layer.iter() {
            merged.insert(path.clone(), value.clone());
        }
    }

    merged
}

#[cfg(test)]
mod tests {
    use super::{ConfigLayer, merge_layers, merge_two_layers};
    use use_config_key::ConfigPath;
    use use_config_source::{ConfigSource, ConfigSourceKind};
    use use_config_value::ConfigValue;

    #[test]
    fn lower_priority_default_is_overridden_by_higher_priority_override() {
        let path = ConfigPath::parse("server.port").expect("path should parse");
        let mut defaults = ConfigLayer::new(ConfigSource::unnamed(ConfigSourceKind::Default, 0));
        let mut overrides = ConfigLayer::new(ConfigSource::unnamed(ConfigSourceKind::Override, 10));

        defaults.insert(path.clone(), ConfigValue::from(8080_i64));
        overrides.insert(path.clone(), ConfigValue::from(9090_i64));

        let merged = merge_two_layers(&defaults, &overrides);

        assert_eq!(merged.get(&path).and_then(ConfigValue::as_i64), Some(9090));
    }

    #[test]
    fn later_layer_wins_when_priority_is_equal() {
        let path = ConfigPath::parse("mode").expect("path should parse");
        let source = ConfigSource::unnamed(ConfigSourceKind::Runtime, 5);
        let mut first = ConfigLayer::new(source.clone());
        let mut second = ConfigLayer::new(source);

        first.insert(path.clone(), ConfigValue::from("first"));
        second.insert(path.clone(), ConfigValue::from("second"));

        let merged = merge_layers([&first, &second]);

        assert_eq!(
            merged.get(&path).and_then(ConfigValue::as_str),
            Some("second")
        );
    }

    #[test]
    fn unrelated_keys_are_preserved() {
        let host = ConfigPath::parse("server.host").expect("path should parse");
        let port = ConfigPath::parse("server.port").expect("path should parse");
        let mut defaults = ConfigLayer::new(ConfigSource::unnamed(ConfigSourceKind::Default, 0));
        let mut overrides = ConfigLayer::new(ConfigSource::unnamed(ConfigSourceKind::Override, 10));

        defaults.insert(host.clone(), ConfigValue::from("localhost"));
        overrides.insert(port.clone(), ConfigValue::from(9090_i64));

        let merged = merge_layers([&defaults, &overrides]);

        assert_eq!(
            merged.get(&host).and_then(ConfigValue::as_str),
            Some("localhost")
        );
        assert_eq!(merged.get(&port).and_then(ConfigValue::as_i64), Some(9090));
    }

    #[test]
    fn merge_order_is_deterministic() {
        let path = ConfigPath::parse("answer").expect("path should parse");
        let mut low = ConfigLayer::new(ConfigSource::unnamed(ConfigSourceKind::Default, 0));
        let mut middle = ConfigLayer::new(ConfigSource::unnamed(ConfigSourceKind::Runtime, 5));
        let mut high = ConfigLayer::new(ConfigSource::unnamed(ConfigSourceKind::Override, 10));

        high.insert(path.clone(), ConfigValue::from(3_i64));
        low.insert(path.clone(), ConfigValue::from(1_i64));
        middle.insert(path.clone(), ConfigValue::from(2_i64));

        let merged = merge_layers([&high, &low, &middle]);

        assert_eq!(merged.get(&path).and_then(ConfigValue::as_i64), Some(3));
    }
}
