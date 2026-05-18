#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use std::collections::BTreeMap;

/// A primitive deterministic configuration value.
#[derive(Clone, Debug, PartialEq)]
pub enum ConfigValue {
    /// No configured value.
    Null,
    /// Boolean value.
    Bool(bool),
    /// Signed integer value.
    Integer(i64),
    /// Floating-point value.
    Float(f64),
    /// Owned string value.
    String(String),
    /// Ordered list of configuration values.
    List(Vec<ConfigValue>),
    /// Deterministically ordered string-keyed map.
    Map(BTreeMap<String, ConfigValue>),
}

impl ConfigValue {
    /// Returns the boolean value, if this is a boolean.
    #[must_use]
    pub const fn as_bool(&self) -> Option<bool> {
        match self {
            Self::Bool(value) => Some(*value),
            _ => None,
        }
    }

    /// Returns the integer value, if this is an integer.
    #[must_use]
    pub const fn as_i64(&self) -> Option<i64> {
        match self {
            Self::Integer(value) => Some(*value),
            _ => None,
        }
    }

    /// Returns the floating-point value, if this is a float.
    #[must_use]
    pub const fn as_f64(&self) -> Option<f64> {
        match self {
            Self::Float(value) => Some(*value),
            _ => None,
        }
    }

    /// Returns the string value, if this is a string.
    #[must_use]
    pub fn as_str(&self) -> Option<&str> {
        match self {
            Self::String(value) => Some(value),
            _ => None,
        }
    }

    /// Returns the list value, if this is a list.
    #[must_use]
    pub fn as_list(&self) -> Option<&[ConfigValue]> {
        match self {
            Self::List(value) => Some(value),
            _ => None,
        }
    }

    /// Returns the map value, if this is a map.
    #[must_use]
    pub fn as_map(&self) -> Option<&BTreeMap<String, ConfigValue>> {
        match self {
            Self::Map(value) => Some(value),
            _ => None,
        }
    }
}

impl From<()> for ConfigValue {
    fn from((): ()) -> Self {
        Self::Null
    }
}

impl From<bool> for ConfigValue {
    fn from(value: bool) -> Self {
        Self::Bool(value)
    }
}

impl From<i8> for ConfigValue {
    fn from(value: i8) -> Self {
        Self::Integer(i64::from(value))
    }
}

impl From<i16> for ConfigValue {
    fn from(value: i16) -> Self {
        Self::Integer(i64::from(value))
    }
}

impl From<i32> for ConfigValue {
    fn from(value: i32) -> Self {
        Self::Integer(i64::from(value))
    }
}

impl From<i64> for ConfigValue {
    fn from(value: i64) -> Self {
        Self::Integer(value)
    }
}

impl From<u8> for ConfigValue {
    fn from(value: u8) -> Self {
        Self::Integer(i64::from(value))
    }
}

impl From<u16> for ConfigValue {
    fn from(value: u16) -> Self {
        Self::Integer(i64::from(value))
    }
}

impl From<u32> for ConfigValue {
    fn from(value: u32) -> Self {
        Self::Integer(i64::from(value))
    }
}

impl From<f32> for ConfigValue {
    fn from(value: f32) -> Self {
        Self::Float(f64::from(value))
    }
}

impl From<f64> for ConfigValue {
    fn from(value: f64) -> Self {
        Self::Float(value)
    }
}

impl From<String> for ConfigValue {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

impl From<&str> for ConfigValue {
    fn from(value: &str) -> Self {
        Self::String(value.to_owned())
    }
}

impl From<Vec<ConfigValue>> for ConfigValue {
    fn from(value: Vec<ConfigValue>) -> Self {
        Self::List(value)
    }
}

impl From<BTreeMap<String, ConfigValue>> for ConfigValue {
    fn from(value: BTreeMap<String, ConfigValue>) -> Self {
        Self::Map(value)
    }
}

#[cfg(test)]
mod tests {
    use super::ConfigValue;
    use std::collections::BTreeMap;

    #[test]
    fn primitive_conversions() {
        assert_eq!(ConfigValue::from(()), ConfigValue::Null);
        assert_eq!(ConfigValue::from(true), ConfigValue::Bool(true));
        assert_eq!(ConfigValue::from(42_i64), ConfigValue::Integer(42));
        assert_eq!(
            ConfigValue::from("hello"),
            ConfigValue::String("hello".to_owned())
        );
    }

    #[test]
    fn accessors_return_expected_values() {
        let list = ConfigValue::from(vec![ConfigValue::from("a"), ConfigValue::from("b")]);
        let mut map = BTreeMap::new();
        map.insert("enabled".to_owned(), ConfigValue::from(true));
        let map = ConfigValue::from(map);

        assert_eq!(ConfigValue::from(false).as_bool(), Some(false));
        assert_eq!(ConfigValue::from(12_i64).as_i64(), Some(12));
        assert_eq!(ConfigValue::from(1.5_f64).as_f64(), Some(1.5));
        assert_eq!(ConfigValue::from("text").as_str(), Some("text"));
        assert_eq!(list.as_list().map(<[ConfigValue]>::len), Some(2));
        assert_eq!(
            map.as_map()
                .and_then(|value| value.get("enabled"))
                .and_then(ConfigValue::as_bool),
            Some(true)
        );
    }

    #[test]
    fn wrong_type_accessors_return_none() {
        let value = ConfigValue::from("8080");

        assert_eq!(value.as_bool(), None);
        assert_eq!(value.as_i64(), None);
        assert_eq!(value.as_f64(), None);
        assert_eq!(ConfigValue::from(8080_i64).as_str(), None);
    }

    #[test]
    fn map_ordering_is_deterministic() {
        let mut map = BTreeMap::new();
        map.insert("z".to_owned(), ConfigValue::from(1_i64));
        map.insert("a".to_owned(), ConfigValue::from(2_i64));
        let value = ConfigValue::from(map);
        let keys: Vec<_> = value
            .as_map()
            .expect("map expected")
            .keys()
            .cloned()
            .collect();

        assert_eq!(keys, vec!["a".to_owned(), "z".to_owned()]);
    }
}
