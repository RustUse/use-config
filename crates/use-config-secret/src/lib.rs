#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use core::{fmt, str::FromStr};
use std::error::Error;

/// Error returned when a secret reference is invalid.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SecretRefError;

impl fmt::Display for SecretRefError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("secret reference is empty")
    }
}

impl Error for SecretRefError {}

/// A safe reference to a secret value.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct SecretRef(String);

impl SecretRef {
    /// Creates a secret reference from a non-empty identifier.
    pub fn new(input: impl AsRef<str>) -> Result<Self, SecretRefError> {
        let trimmed = input.as_ref().trim();

        if trimmed.is_empty() {
            Err(SecretRefError)
        } else {
            Ok(Self(trimmed.to_owned()))
        }
    }

    /// Returns the safe secret reference identifier.
    #[must_use]
    pub fn id(&self) -> &str {
        &self.0
    }

    /// Returns the owned secret reference identifier.
    #[must_use]
    pub fn into_string(self) -> String {
        self.0
    }
}

impl fmt::Debug for SecretRef {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.debug_tuple("SecretRef").field(&self.0).finish()
    }
}

impl fmt::Display for SecretRef {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "secret:{}", self.0)
    }
}

impl FromStr for SecretRef {
    type Err = SecretRefError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Self::new(input)
    }
}

/// A value wrapper that redacts display and debug output.
pub struct Redacted<T> {
    value: T,
}

impl<T> Redacted<T> {
    /// Wraps a value in redacted display/debug behavior.
    #[must_use]
    pub const fn new(value: T) -> Self {
        Self { value }
    }

    /// Returns the wrapped value by reference.
    #[must_use]
    pub const fn expose(&self) -> &T {
        &self.value
    }

    /// Returns the wrapped value by value.
    #[must_use]
    pub fn into_inner(self) -> T {
        self.value
    }
}

impl<T: Clone> Clone for Redacted<T> {
    fn clone(&self) -> Self {
        Self::new(self.value.clone())
    }
}

impl<T: PartialEq> PartialEq for Redacted<T> {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl<T: Eq> Eq for Redacted<T> {}

impl<T> fmt::Debug for Redacted<T> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("[redacted]")
    }
}

impl<T> fmt::Display for Redacted<T> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("[redacted]")
    }
}

#[cfg(test)]
mod tests {
    use super::{Redacted, SecretRef, SecretRefError};

    #[test]
    fn debug_does_not_leak_wrapped_secret_value() {
        let value = Redacted::new("very-secret-token");

        assert_eq!(format!("{value:?}"), "[redacted]");
        assert!(!format!("{value:?}").contains("very-secret-token"));
    }

    #[test]
    fn display_does_not_leak_wrapped_secret_value() {
        let value = Redacted::new("very-secret-token");

        assert_eq!(value.to_string(), "[redacted]");
        assert!(!value.to_string().contains("very-secret-token"));
    }

    #[test]
    fn secret_reference_displays_safe_identifier() {
        let reference = SecretRef::new(" database/password ").expect("reference should parse");

        assert_eq!(reference.id(), "database/password");
        assert_eq!(reference.to_string(), "secret:database/password");
        assert_eq!(SecretRef::new(" "), Err(SecretRefError));
    }

    #[test]
    fn equality_works_where_appropriate() {
        let first = SecretRef::new("api/key").expect("reference should parse");
        let second = SecretRef::new("api/key").expect("reference should parse");
        let redacted_first = Redacted::new("same");
        let redacted_second = Redacted::new("same");

        assert_eq!(first, second);
        assert_eq!(redacted_first, redacted_second);
    }
}
