use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// A wrapper type that distinguishes between "absent" and "explicitly null" in JSON.
///
/// In REST APIs (especially PATCH requests):
/// - `Nullable::Absent` → field is omitted from JSON (no change)
/// - `Nullable::Null` → field is set to `null` in JSON (clear the value)
/// - `Nullable::Value(T)` → field is set to the value
///
/// # Example
/// ```rust
/// use sumup_rs::Nullable;
///
/// #[derive(serde::Serialize)]
/// struct UpdateRequest {
///     #[serde(skip_serializing_if = "Nullable::is_absent")]
///     name: Nullable<String>,
/// }
///
/// // Omit the field entirely
/// let req = UpdateRequest { name: Nullable::Absent };
/// assert_eq!(serde_json::to_string(&req).unwrap(), "{}");
///
/// // Explicitly set to null
/// let req = UpdateRequest { name: Nullable::Null };
/// assert_eq!(serde_json::to_string(&req).unwrap(), r#"{"name":null}"#);
///
/// // Set to a value
/// let req = UpdateRequest { name: Nullable::Value("New Name".to_string()) };
/// assert_eq!(serde_json::to_string(&req).unwrap(), r#"{"name":"New Name"}"#);
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Nullable<T> {
    /// Field should be omitted from serialization (no change to existing value)
    Absent,
    /// Field should be serialized as `null` (clear the existing value)
    Null,
    /// Field should be serialized with this value
    Value(T),
}

impl<T> Default for Nullable<T> {
    fn default() -> Self {
        Nullable::Absent
    }
}

impl<T> Nullable<T> {
    /// Returns true if this is `Absent` (for use with `skip_serializing_if`)
    pub fn is_absent(&self) -> bool {
        matches!(self, Nullable::Absent)
    }

    /// Returns true if this contains a value
    pub fn is_value(&self) -> bool {
        matches!(self, Nullable::Value(_))
    }

    /// Returns true if this is explicitly null
    pub fn is_null(&self) -> bool {
        matches!(self, Nullable::Null)
    }

    /// Convert to Option, treating both Absent and Null as None
    pub fn into_option(self) -> Option<T> {
        match self {
            Nullable::Value(v) => Some(v),
            _ => None,
        }
    }

    /// Map the inner value
    pub fn map<U, F: FnOnce(T) -> U>(self, f: F) -> Nullable<U> {
        match self {
            Nullable::Absent => Nullable::Absent,
            Nullable::Null => Nullable::Null,
            Nullable::Value(v) => Nullable::Value(f(v)),
        }
    }
}

impl<T> From<T> for Nullable<T> {
    fn from(value: T) -> Self {
        Nullable::Value(value)
    }
}

impl<T> From<Option<T>> for Nullable<T> {
    fn from(opt: Option<T>) -> Self {
        match opt {
            Some(v) => Nullable::Value(v),
            None => Nullable::Null,
        }
    }
}

impl<T: Serialize> Serialize for Nullable<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Nullable::Absent => {
                // This should not be called if skip_serializing_if is used correctly
                serializer.serialize_none()
            }
            Nullable::Null => serializer.serialize_none(),
            Nullable::Value(v) => v.serialize(serializer),
        }
    }
}

impl<'de, T: Deserialize<'de>> Deserialize<'de> for Nullable<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        // When deserializing, if the field exists we get here
        // If it's null, we return Null; otherwise we return Value
        Option::<T>::deserialize(deserializer).map(|opt| match opt {
            Some(v) => Nullable::Value(v),
            None => Nullable::Null,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, Debug, PartialEq)]
    struct TestStruct {
        #[serde(skip_serializing_if = "Nullable::is_absent", default)]
        name: Nullable<String>,
    }

    #[test]
    fn test_absent_is_omitted() {
        let s = TestStruct { name: Nullable::Absent };
        let json = serde_json::to_string(&s).unwrap();
        assert_eq!(json, "{}");
    }

    #[test]
    fn test_null_is_serialized() {
        let s = TestStruct { name: Nullable::Null };
        let json = serde_json::to_string(&s).unwrap();
        assert_eq!(json, r#"{"name":null}"#);
    }

    #[test]
    fn test_value_is_serialized() {
        let s = TestStruct { name: Nullable::Value("test".to_string()) };
        let json = serde_json::to_string(&s).unwrap();
        assert_eq!(json, r#"{"name":"test"}"#);
    }

    #[test]
    fn test_deserialize_missing() {
        let s: TestStruct = serde_json::from_str("{}").unwrap();
        assert_eq!(s.name, Nullable::Absent);
    }

    #[test]
    fn test_deserialize_null() {
        let s: TestStruct = serde_json::from_str(r#"{"name":null}"#).unwrap();
        assert_eq!(s.name, Nullable::Null);
    }

    #[test]
    fn test_deserialize_value() {
        let s: TestStruct = serde_json::from_str(r#"{"name":"test"}"#).unwrap();
        assert_eq!(s.name, Nullable::Value("test".to_string()));
    }
}
