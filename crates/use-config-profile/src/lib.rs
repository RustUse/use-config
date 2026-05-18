#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use core::{fmt, str::FromStr};
use std::error::Error;

/// A primitive runtime or deployment profile name.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum ConfigProfile {
    /// Local developer profile.
    Local,
    /// Development profile.
    Development,
    /// Test profile.
    Test,
    /// Staging profile.
    Staging,
    /// Production profile.
    Production,
    /// Caller-defined profile name.
    Custom(String),
}

/// Error returned when a profile name is invalid.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ConfigProfileError;

impl fmt::Display for ConfigProfileError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("configuration profile is empty")
    }
}

impl Error for ConfigProfileError {}

impl fmt::Display for ConfigProfile {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Local => formatter.write_str("local"),
            Self::Development => formatter.write_str("development"),
            Self::Test => formatter.write_str("test"),
            Self::Staging => formatter.write_str("staging"),
            Self::Production => formatter.write_str("production"),
            Self::Custom(value) => formatter.write_str(value),
        }
    }
}

impl FromStr for ConfigProfile {
    type Err = ConfigProfileError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let trimmed = input.trim();

        if trimmed.is_empty() {
            return Err(ConfigProfileError);
        }

        let profile = match trimmed.to_ascii_lowercase().as_str() {
            "local" => Self::Local,
            "development" | "dev" => Self::Development,
            "test" | "testing" => Self::Test,
            "staging" | "stage" => Self::Staging,
            "production" | "prod" => Self::Production,
            _ => Self::Custom(trimmed.to_owned()),
        };

        Ok(profile)
    }
}

#[cfg(test)]
mod tests {
    use super::{ConfigProfile, ConfigProfileError};
    use std::str::FromStr;

    #[test]
    fn parse_known_profiles() {
        assert_eq!(ConfigProfile::from_str("local"), Ok(ConfigProfile::Local));
        assert_eq!(
            ConfigProfile::from_str("development"),
            Ok(ConfigProfile::Development)
        );
        assert_eq!(ConfigProfile::from_str("test"), Ok(ConfigProfile::Test));
        assert_eq!(
            ConfigProfile::from_str("staging"),
            Ok(ConfigProfile::Staging)
        );
        assert_eq!(
            ConfigProfile::from_str("production"),
            Ok(ConfigProfile::Production)
        );
    }

    #[test]
    fn parse_aliases() {
        assert_eq!(
            ConfigProfile::from_str("dev"),
            Ok(ConfigProfile::Development)
        );
        assert_eq!(
            ConfigProfile::from_str("prod"),
            Ok(ConfigProfile::Production)
        );
        assert_eq!(ConfigProfile::from_str("stage"), Ok(ConfigProfile::Staging));
    }

    #[test]
    fn parse_custom_profile() {
        assert_eq!(
            ConfigProfile::from_str("preview"),
            Ok(ConfigProfile::Custom("preview".to_owned()))
        );
        assert_eq!(ConfigProfile::from_str(" "), Err(ConfigProfileError));
    }

    #[test]
    fn display_known_profiles() {
        assert_eq!(ConfigProfile::Local.to_string(), "local");
        assert_eq!(ConfigProfile::Development.to_string(), "development");
        assert_eq!(ConfigProfile::Production.to_string(), "production");
    }

    #[test]
    fn display_custom_profile() {
        assert_eq!(
            ConfigProfile::Custom("preview".to_owned()).to_string(),
            "preview"
        );
    }
}
