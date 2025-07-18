use crate::monoid::Monoid;
use std::fmt::Display;

#[derive(Debug, PartialEq, Eq)]
pub struct KeyVal {
    pub key: String,
    pub value: String,
}

impl Display for KeyVal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} = {:?}", self.key, self.value)
    }
}

/// Vec<KeyVal> is a monoid with
///    - the empty list as the identity element
///    - the concatenation of two lists as the merge operation
pub type KeyVals = Vec<KeyVal>;

impl KeyVal {
    pub fn new(key: String, value: String) -> Self {
        Self {
            key: key.trim().to_string(),
            value: value.trim().to_string(),
        }
    }

    /// Parse a string into a vector of KeyVal by indentation
    /// This function satisfies a peculiar property:
    /// parse (cat ccl1 ccl2) ≡ parse ccl1 @ parse ccl2
    /// In English, concatenating two files and then parsing the result is the same as
    /// parsing two files separately and then appending the resulting lists of
    /// key-value pairs.
    /// parse is a monoid homomorphism from the monoid of strings to the monoid of
    /// key-value pairs.
    /// monoid of strings has
    ///     - the empty string as the identity element
    ///     - the 'concatenation of two strings as the merge operation
    /// Note: to handle intentation, cat will trim the leading whitespace (or
    /// indentation?).
    pub fn parse(data: &str) -> Result<KeyVals, String> {
        let mut key_vals = Vec::new();

        let lines = data.trim().lines().collect::<Vec<&str>>();

        let len = lines.len();
        if len == 0 {
            return Ok(key_vals);
        }

        for (i, line) in lines.iter().enumerate() {
            let indentation = line.len() - line.trim_start().len();

            if i == 0 || indentation == 0 {
                let Some((curr_key, curr_value)) = line.split_once("=")
                else {
                    return Err(format!("Invalid line: {}", line));
                };

                key_vals.push(KeyVal::new(
                    curr_key.to_string(),
                    curr_value.to_string(),
                ));
                continue;
            } else {
                let last_key_val = key_vals.last_mut().unwrap();
                last_key_val.value.push_str(&format!("\n{}", line));
            }
        }

        Ok(key_vals)
    }

    /// pretty and parse are monoid isomorphisms
    pub fn pretty(key_vals: &KeyVals) -> String {
        key_vals
            .iter()
            .map(|key_val| key_val.to_string())
            .collect::<Vec<String>>()
            .join("\n")
    }
}

impl Monoid for KeyVals {
    fn empty() -> Self {
        Vec::new()
    }

    fn merge(self, other: Self) -> Self {
        let mut merged = self;
        merged.extend(other);
        merged
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn data() -> String {
        r#"
a = b
b =
  c = d
  d =
    e = f
    f = g
  g = h
h = i
i = j
j = k
  k = l
"#
        .to_string()
    }

    #[test]
    fn test_key_val_parse() {
        let data = data();
        let key_vals = KeyVal::parse(&data).unwrap();
        let parsed_str = key_vals
            .iter()
            .map(|key_val| key_val.to_string())
            .collect::<Vec<String>>()
            .join("\n");

        insta::assert_snapshot!(parsed_str, @r#"
        a = "b"
        b = "\n  c = d\n  d =\n    e = f\n    f = g\n  g = h"
        h = "i"
        i = "j"
        j = "k\n  k = l"
        "#);
    }
}
