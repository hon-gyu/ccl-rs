```rust
use std::collections::BTreeMap;
use std::fmt;

/// A helper type for a map from keys to values where the key is String.
/// Using BTreeMap for consistent ordering like OCaml's Map.
pub type KeyMap<T> = BTreeMap<String, T>;

/// A single parsed value - either a terminal string or a nested structure
#[derive(Debug, Clone, PartialEq, Eq)]
enum ParsedValue {
    Terminal(String),
    Nested(ParsedTree),
}

/// Intermediate representation of the entire parsed structure
/// Maps keys to potentially multiple values (before merging)
type ParsedTree = KeyMap<Vec<ParsedValue>>;

/// The actual type of the configuration. It's represented as a dictionary from string to itself.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct CCL(KeyMap<CCL>);

impl CCL {
    /// Self-explanatory.
    pub fn empty() -> Self {
        CCL(KeyMap::new())
    }

    /// Merge two maps recursively. Keys from both Maps are preserved
    pub fn merge(self, other: CCL) -> CCL {
        let CCL(mut map1) = self;
        let CCL(map2) = other;
        
        for (key, value2) in map2 {
            match map1.get_mut(&key) {
                None => {
                    map1.insert(key, value2);
                }
                Some(value1) => {
                    let merged = value1.clone().merge(value2);
                    *value1 = merged;
                }
            }
        }
        
        CCL(map1)
    }

    /// Creates a singleton CCL from just a key (equivalent to key with empty value)
    pub fn key(k: impl Into<String>) -> Self {
        let mut map = KeyMap::new();
        map.insert(k.into(), CCL::empty());
        CCL(map)
    }

    /// `key_val key value` creates a singleton CCL from the given key
    /// associated with the value.
    pub fn key_val(k: impl Into<String>, v: impl Into<String>) -> Self {
        let mut map = KeyMap::new();
        map.insert(k.into(), CCL::key(v));
        CCL(map)
    }

    /// `nested key entries` creates CCL with a key associated to multiple nested
    /// values.
    pub fn nested(k: impl Into<String>, vals: Vec<CCL>) -> Self {
        let mut map = KeyMap::new();
        map.insert(k.into(), CCL::of_list(vals));
        CCL(map)
    }

    /// `of_list maps` creates CCL by applying merge to all entries.
    pub fn of_list(maps: Vec<CCL>) -> Self {
        maps.into_iter().fold(CCL::empty(), |acc, map| acc.merge(map))
    }

    /// Pretty-print the configuration.
    pub fn pretty(&self) -> String {
        let mut buf = String::new();
        self.pretty_impl(0, &mut buf);
        buf
    }

    fn pretty_impl(&self, indent: usize, buf: &mut String) {
        let CCL(map) = self;
        for (key, value) in map {
            let prefix = " ".repeat(indent);
            buf.push_str(&prefix);
            buf.push_str(key);
            buf.push_str(" =\n");
            value.pretty_impl(indent + 2, buf);
        }
    }

    /// Parse a flat list of key-value pairs into a nested CCL structure.
    /// Values that can be parsed as key-value pairs themselves are recursively parsed.
    pub fn from_key_value_pairs(key_vals: Vec<KeyVal>) -> Self {
        let parsed_structure = parse_to_intermediate(key_vals);
        convert_to_ccl(parsed_structure)
    }

    /// Alias for from_key_value_pairs to match OCaml interface
    pub fn fix(key_vals: Vec<KeyVal>) -> Self {
        Self::from_key_value_pairs(key_vals)
    }
}

/// Convenience trait for the =: operator
pub trait IntoKeyVal {
    fn key_val(self, v: impl Into<String>) -> CCL;
}

impl<T: Into<String>> IntoKeyVal for T {
    fn key_val(self, v: impl Into<String>) -> CCL {
        CCL::key_val(self, v)
    }
}

/// Parser module types
pub mod parser {
    #[derive(Debug, Clone)]
    pub struct KeyVal {
        pub key: String,
        pub value: String,
    }

    /// Placeholder for the actual parser
    /// In real implementation, this would parse the value string into key-value pairs
    pub fn parse_value(_value: &str) -> Result<Vec<KeyVal>, String> {
        // This is a placeholder - actual implementation would parse the string
        Err("Not implemented".to_string())
    }
}

use parser::{KeyVal, parse_value};

/// Phase 1: Recursively parse flat key-value pairs into an intermediate tree structure
/// This handles values that might themselves be parseable as key-value pairs
fn parse_to_intermediate(key_vals: Vec<KeyVal>) -> ParsedTree {
    let mut result: ParsedTree = KeyMap::new();
    
    for kv in key_vals {
        let parsed_value = match parse_value(&kv.value) {
            // Value couldn't be parsed - it's a terminal string
            Err(_) => ParsedValue::Terminal(kv.value),
            // Value was successfully parsed - recursively parse the result
            Ok(nested_kvs) => ParsedValue::Nested(parse_to_intermediate(nested_kvs)),
        };
        
        // Accumulate multiple values for the same key
        result.entry(kv.key)
            .or_insert_with(Vec::new)
            .push(parsed_value);
    }
    
    result
}

/// Phase 2: Convert the parsed tree to final CCL form
/// This merges multiple values for the same key and creates the proper structure
fn convert_to_ccl(parsed: ParsedTree) -> CCL {
    let mut ccl_map = KeyMap::new();
    
    for (key, values) in parsed {
        // Convert each value and merge them together
        let merged_ccl = values.into_iter()
            .map(|value| convert_value_to_ccl(value))
            .fold(CCL::empty(), |acc, ccl| acc.merge(ccl));
            
        ccl_map.insert(key, merged_ccl);
    }
    
    CCL(ccl_map)
}

/// Convert a single parsed value to CCL
fn convert_value_to_ccl(value: ParsedValue) -> CCL {
    match value {
        // Terminal string becomes a key pointing to empty CCL
        ParsedValue::Terminal(s) => CCL::key(s),
        // Nested tree is recursively converted
        ParsedValue::Nested(parsed_tree) => convert_to_ccl(parsed_tree),
    }
}

impl fmt::Display for CCL {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.pretty())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty() {
        let empty = CCL::empty();
        assert_eq!(empty.0.len(), 0);
    }

    #[test]
    fn test_key_val() {
        let kv = CCL::key_val("foo", "bar");
        assert!(kv.0.contains_key("foo"));
    }

    #[test]
    fn test_merge() {
        let kv1 = CCL::key_val("foo", "bar");
        let kv2 = CCL::key_val("baz", "qux");
        let merged = kv1.merge(kv2);
        assert!(merged.0.contains_key("foo"));
        assert!(merged.0.contains_key("baz"));
    }

    #[test]
    fn test_nested() {
        let inner1 = CCL::key_val("a", "1");
        let inner2 = CCL::key_val("b", "2");
        let nested = CCL::nested("outer", vec![inner1, inner2]);
        assert!(nested.0.contains_key("outer"));
    }

    #[test]
    fn test_of_list() {
        let kv1 = CCL::key_val("foo", "bar");
        let kv2 = CCL::key_val("baz", "qux");
        let list = CCL::of_list(vec![kv1, kv2]);
        assert!(list.0.contains_key("foo"));
        assert!(list.0.contains_key("baz"));
    }

    #[test]
    fn test_infix_operator() {
        // Using the trait method directly since Rust doesn't support custom operators
        let kv = "foo".key_val("bar");
        assert!(kv.0.contains_key("foo"));
    }
}
```