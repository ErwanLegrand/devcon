use serde::Deserialize;
use serde::de::{self, Deserializer, SeqAccess, Visitor};
use std::fmt;

/// A lifecycle hook value that accepts either a plain string or an array of strings.
///
/// String form: `"npm install"` → executed as `sh -c "npm install"`
/// Array form:  `["npm", "install"]` → executed as `npm install` (no shell)
#[derive(Debug, Clone, PartialEq)]
pub enum OneOrMany {
    One(String),
    Many(Vec<String>),
}

impl OneOrMany {
    /// Return the program and arguments for executing this hook.
    ///
    /// - `One(cmd)` → `("sh", ["-c", cmd])`
    /// - `Many(parts)` → `(parts[0], parts[1..])`
    ///
    /// Returns `None` if `Many` is empty.
    #[must_use]
    pub fn to_exec_parts(&self) -> Option<(String, Vec<String>)> {
        match self {
            Self::One(cmd) => Some(("sh".to_string(), vec!["-c".to_string(), cmd.clone()])),
            Self::Many(parts) if parts.is_empty() => None,
            Self::Many(parts) => Some((parts[0].clone(), parts[1..].to_vec())),
        }
    }
}

struct OneOrManyVisitor;

impl<'de> Visitor<'de> for OneOrManyVisitor {
    type Value = OneOrMany;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a string or an array of strings")
    }

    fn visit_str<E: de::Error>(self, value: &str) -> Result<OneOrMany, E> {
        Ok(OneOrMany::One(value.to_string()))
    }

    fn visit_string<E: de::Error>(self, value: String) -> Result<OneOrMany, E> {
        Ok(OneOrMany::One(value))
    }

    fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<OneOrMany, A::Error> {
        let mut parts = Vec::new();
        while let Some(item) = seq.next_element::<String>()? {
            parts.push(item);
        }
        Ok(OneOrMany::Many(parts))
    }
}

impl<'de> Deserialize<'de> for OneOrMany {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_any(OneOrManyVisitor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_string_form() {
        let json = r#""npm install""#;
        let v: OneOrMany = json5::from_str(json).expect("parse string form");
        assert_eq!(v, OneOrMany::One("npm install".to_string()));
    }

    #[test]
    fn deserialize_array_form() {
        let json = r#"["npm", "install"]"#;
        let v: OneOrMany = json5::from_str(json).expect("parse array form");
        assert_eq!(
            v,
            OneOrMany::Many(vec!["npm".to_string(), "install".to_string()])
        );
    }

    #[test]
    fn to_exec_parts_string() {
        let v = OneOrMany::One("echo hello".to_string());
        let (prog, args) = v.to_exec_parts().unwrap();
        assert_eq!(prog, "sh");
        assert_eq!(args, vec!["-c", "echo hello"]);
    }

    #[test]
    fn to_exec_parts_array() {
        let v = OneOrMany::Many(vec!["npm".to_string(), "install".to_string()]);
        let (prog, args) = v.to_exec_parts().unwrap();
        assert_eq!(prog, "npm");
        assert_eq!(args, vec!["install"]);
    }

    #[test]
    fn to_exec_parts_empty_array() {
        let v = OneOrMany::Many(vec![]);
        assert!(v.to_exec_parts().is_none());
    }
}
