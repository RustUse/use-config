#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use core::{fmt, str::FromStr};
use std::error::Error;

/// Error returned when a configuration key, section, or path is invalid.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ConfigKeyError {
    /// The provided key or path was empty after trimming whitespace.
    Empty,
    /// A dotted path contained an empty segment.
    EmptySegment,
    /// A single key or section segment contained a dot.
    DottedSegment,
}

impl fmt::Display for ConfigKeyError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let message = match self {
            Self::Empty => "configuration key is empty",
            Self::EmptySegment => "configuration path contains an empty segment",
            Self::DottedSegment => "configuration segment must not contain dots",
        };

        formatter.write_str(message)
    }
}

impl Error for ConfigKeyError {}

/// A validated single configuration key segment.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ConfigKey(String);

impl ConfigKey {
    /// Creates a configuration key from one non-empty segment.
    ///
    /// # Errors
    ///
    /// Returns [`ConfigKeyError`] when the input is empty after trimming or contains a dot.
    pub fn new(input: impl AsRef<str>) -> Result<Self, ConfigKeyError> {
        validated_single_segment(input.as_ref()).map(|segment| Self(segment.to_owned()))
    }

    /// Returns the key as a borrowed string slice.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Returns the owned key string.
    #[must_use]
    pub fn into_string(self) -> String {
        self.0
    }
}

impl fmt::Display for ConfigKey {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

impl FromStr for ConfigKey {
    type Err = ConfigKeyError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Self::new(input)
    }
}

/// A validated configuration section name.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ConfigSection(String);

impl ConfigSection {
    /// Creates a configuration section from one non-empty segment.
    ///
    /// # Errors
    ///
    /// Returns [`ConfigKeyError`] when the input is empty after trimming or contains a dot.
    pub fn new(input: impl AsRef<str>) -> Result<Self, ConfigKeyError> {
        validated_single_segment(input.as_ref()).map(|segment| Self(segment.to_owned()))
    }

    /// Returns the section as a borrowed string slice.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Returns the owned section string.
    #[must_use]
    pub fn into_string(self) -> String {
        self.0
    }
}

impl fmt::Display for ConfigSection {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

impl FromStr for ConfigSection {
    type Err = ConfigKeyError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Self::new(input)
    }
}

/// A validated dotted configuration path.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ConfigPath {
    segments: Vec<String>,
}

impl ConfigPath {
    /// Parses a dotted configuration path.
    ///
    /// # Errors
    ///
    /// Returns [`ConfigKeyError`] when the path is empty or contains an invalid segment.
    pub fn parse(input: &str) -> Result<Self, ConfigKeyError> {
        input.parse()
    }

    /// Creates a path from already separated segments.
    ///
    /// # Errors
    ///
    /// Returns [`ConfigKeyError`] when no segments are provided or any segment is invalid.
    pub fn from_segments<I, S>(segments: I) -> Result<Self, ConfigKeyError>
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let mut validated = Vec::new();

        for segment in segments {
            validated.push(validated_path_segment(segment.as_ref())?.to_owned());
        }

        if validated.is_empty() {
            return Err(ConfigKeyError::Empty);
        }

        Ok(Self {
            segments: validated,
        })
    }

    /// Returns the number of path segments.
    #[must_use]
    pub const fn len(&self) -> usize {
        self.segments.len()
    }

    /// Returns `true` when the path has no segments.
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.segments.is_empty()
    }

    /// Iterates over path segments in deterministic order.
    pub fn segments(&self) -> impl Iterator<Item = &str> {
        self.segments.iter().map(String::as_str)
    }

    /// Returns a path segment by index.
    #[must_use]
    pub fn get(&self, index: usize) -> Option<&str> {
        self.segments.get(index).map(String::as_str)
    }

    /// Returns the first path segment.
    #[must_use]
    pub fn first(&self) -> Option<&str> {
        self.get(0)
    }

    /// Returns the final path segment.
    #[must_use]
    pub fn last(&self) -> Option<&str> {
        self.segments.last().map(String::as_str)
    }
}

impl fmt::Display for ConfigPath {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (index, segment) in self.segments.iter().enumerate() {
            if index > 0 {
                formatter.write_str(".")?;
            }

            formatter.write_str(segment)?;
        }

        Ok(())
    }
}

impl From<ConfigKey> for ConfigPath {
    fn from(key: ConfigKey) -> Self {
        Self {
            segments: vec![key.into_string()],
        }
    }
}

impl FromStr for ConfigPath {
    type Err = ConfigKeyError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let trimmed = input.trim();

        if trimmed.is_empty() {
            return Err(ConfigKeyError::Empty);
        }

        let mut segments = Vec::new();

        for segment in trimmed.split('.') {
            segments.push(validated_path_segment(segment)?.to_owned());
        }

        Ok(Self { segments })
    }
}

fn validated_single_segment(input: &str) -> Result<&str, ConfigKeyError> {
    let trimmed = input.trim();

    if trimmed.is_empty() {
        return Err(ConfigKeyError::Empty);
    }

    if trimmed.contains('.') {
        return Err(ConfigKeyError::DottedSegment);
    }

    Ok(trimmed)
}

fn validated_path_segment(input: &str) -> Result<&str, ConfigKeyError> {
    let trimmed = input.trim();

    if trimmed.is_empty() {
        return Err(ConfigKeyError::EmptySegment);
    }

    if trimmed.contains('.') {
        return Err(ConfigKeyError::DottedSegment);
    }

    Ok(trimmed)
}

#[cfg(test)]
mod tests {
    use super::{ConfigKey, ConfigKeyError, ConfigPath, ConfigSection};
    use std::str::FromStr;

    #[test]
    fn valid_single_key() {
        let key = ConfigKey::from_str(" port ").expect("key should parse");

        assert_eq!(key.as_str(), "port");
        assert_eq!(key.to_string(), "port");
    }

    #[test]
    fn valid_dotted_path() {
        let path = ConfigPath::parse("server.port").expect("path should parse");

        assert_eq!(path.len(), 2);
        assert_eq!(path.first(), Some("server"));
        assert_eq!(path.last(), Some("port"));
    }

    #[test]
    fn invalid_empty_key() {
        assert_eq!(ConfigKey::new(" "), Err(ConfigKeyError::Empty));
        assert_eq!(ConfigSection::new(""), Err(ConfigKeyError::Empty));
        assert_eq!(ConfigPath::parse(""), Err(ConfigKeyError::Empty));
    }

    #[test]
    fn invalid_empty_segment() {
        assert_eq!(
            ConfigPath::parse("server..port"),
            Err(ConfigKeyError::EmptySegment)
        );
    }

    #[test]
    fn segment_iteration_and_access_preserve_order() {
        let path = ConfigPath::parse("server.http.port").expect("path should parse");
        let segments: Vec<_> = path.segments().collect();

        assert_eq!(segments, vec!["server", "http", "port"]);
        assert_eq!(path.get(1), Some("http"));
    }

    #[test]
    fn display_round_trip() {
        let path = ConfigPath::parse("server.port").expect("path should parse");
        let round_trip = ConfigPath::parse(&path.to_string()).expect("display should parse");

        assert_eq!(path, round_trip);
    }
}
