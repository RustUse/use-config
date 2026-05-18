#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use core::{cmp::Ordering, fmt};

/// A primitive kind for describing a configuration source.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum ConfigSourceKind {
    /// Built-in or caller-provided default values.
    Default,
    /// Values identified as coming from a file.
    File,
    /// Values identified as coming from an environment source.
    Environment,
    /// Values identified as coming from runtime input.
    Runtime,
    /// Values identified as explicit overrides.
    Override,
    /// Values identified as secret references.
    SecretReference,
    /// Caller-defined source kind.
    Custom(String),
}

impl fmt::Display for ConfigSourceKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Default => formatter.write_str("default"),
            Self::File => formatter.write_str("file"),
            Self::Environment => formatter.write_str("environment"),
            Self::Runtime => formatter.write_str("runtime"),
            Self::Override => formatter.write_str("override"),
            Self::SecretReference => formatter.write_str("secret-reference"),
            Self::Custom(value) => formatter.write_str(value),
        }
    }
}

/// Identity and precedence metadata for a configuration source.
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct ConfigSource {
    /// Source kind.
    pub kind: ConfigSourceKind,
    /// Optional source name.
    pub name: Option<String>,
    /// Source priority. Higher priority wins in layer merges.
    pub priority: i32,
}

impl ConfigSource {
    /// Creates a source from explicit parts.
    #[must_use]
    pub fn new(kind: ConfigSourceKind, name: Option<String>, priority: i32) -> Self {
        Self {
            kind,
            name: normalize_name(name),
            priority,
        }
    }

    /// Creates an unnamed source.
    #[must_use]
    pub fn unnamed(kind: ConfigSourceKind, priority: i32) -> Self {
        Self::new(kind, None, priority)
    }

    /// Creates a named source.
    #[must_use]
    pub fn named(kind: ConfigSourceKind, name: impl Into<String>, priority: i32) -> Self {
        Self::new(kind, Some(name.into()), priority)
    }

    /// Returns the source kind.
    #[must_use]
    pub const fn kind(&self) -> &ConfigSourceKind {
        &self.kind
    }

    /// Returns the optional source name.
    #[must_use]
    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    /// Returns the source priority.
    #[must_use]
    pub const fn priority(&self) -> i32 {
        self.priority
    }
}

impl fmt::Display for ConfigSource {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(name) = &self.name {
            write!(formatter, "{}:{}@{}", self.kind, name, self.priority)
        } else {
            write!(formatter, "{}@{}", self.kind, self.priority)
        }
    }
}

impl Ord for ConfigSource {
    fn cmp(&self, other: &Self) -> Ordering {
        self.priority
            .cmp(&other.priority)
            .then_with(|| self.kind.cmp(&other.kind))
            .then_with(|| self.name.cmp(&other.name))
    }
}

impl PartialOrd for ConfigSource {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn normalize_name(name: Option<String>) -> Option<String> {
    name.and_then(|value| {
        let trimmed = value.trim();

        if trimmed.is_empty() {
            None
        } else {
            Some(trimmed.to_owned())
        }
    })
}

#[cfg(test)]
mod tests {
    use super::{ConfigSource, ConfigSourceKind};

    #[test]
    fn source_creation() {
        let source = ConfigSource::named(ConfigSourceKind::File, " app.toml ", 10);

        assert_eq!(source.kind(), &ConfigSourceKind::File);
        assert_eq!(source.name(), Some("app.toml"));
        assert_eq!(source.priority(), 10);
    }

    #[test]
    fn source_display() {
        let named = ConfigSource::named(ConfigSourceKind::Override, "cli", 20);
        let unnamed = ConfigSource::unnamed(ConfigSourceKind::Default, 0);

        assert_eq!(named.to_string(), "override:cli@20");
        assert_eq!(unnamed.to_string(), "default@0");
    }

    #[test]
    fn priority_ordering() {
        let low = ConfigSource::unnamed(ConfigSourceKind::Default, 0);
        let high = ConfigSource::unnamed(ConfigSourceKind::Override, 10);
        let mut sources = vec![high.clone(), low.clone()];

        sources.sort();

        assert_eq!(sources, vec![low, high]);
    }

    #[test]
    fn custom_source_kind() {
        let source = ConfigSource::named(ConfigSourceKind::Custom("fixture".to_owned()), "base", 5);

        assert_eq!(source.kind().to_string(), "fixture");
        assert_eq!(source.to_string(), "fixture:base@5");
    }
}
