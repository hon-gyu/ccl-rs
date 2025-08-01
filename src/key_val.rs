use crate::monoid::Monoid;
use std::collections::BTreeMap;
use std::fmt::Display;

#[derive(Clone, Debug, PartialEq, Eq)]
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

fn get_indent(line: &str) -> usize {
    line.len() - line.trim_start().len()
}

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
    ///
    /// 4 variables to consider when parsing a line:
    /// 1. is line empty?
    /// 2. is key_buf empty?
    /// 3. indent > fst_indent?
    /// 4. line contains "="?
    pub fn parse(data: &str) -> Result<KeyVals, String> {
        let mut key_vals = Vec::new();

        let lines = data.lines().collect::<Vec<&str>>();

        // Delete empty lines at the beginning
        let mut i = 0;
        while i < lines.len() && lines[i].trim().is_empty() {
            i += 1;
        }
        let lines = &lines[i..];

        // If there are no lines, return an empty list
        if lines.len() == 0 {
            return Ok(key_vals);
        }

        let fst_indent = get_indent(lines[0]);

        let mut key_buf = String::new();

        fn continue_key_buf(key_buf: &mut String, line: &str) {
            key_buf.push_str(&format!("\n{}", line.trim_end()));
        }

        fn continue_last_value(key_vals: &mut KeyVals, line: &str) {
            let last_key_val = key_vals
                .last_mut()
                .expect("Never: empty line before any key-value pair");
            last_key_val
                .value
                .push_str(&format!("\n{}", line.trim_end()));
        }

        fn add_new_key_val(
            key_buf: &mut String,
            line: &str,
            key_vals: &mut KeyVals,
        ) {
            let (curr_key, curr_value) =
                line.split_once("=").expect("Never");

            key_buf.push_str(&format!("\n{}", curr_key.trim_end()));
            key_vals.push(KeyVal::new(
                key_buf.trim().to_string(),
                curr_value.trim().to_string(),
            ));
            *key_buf = String::new();
        }
        for line in lines.iter() {
            let indent = get_indent(line);

            if line.trim().is_empty() {
                if !key_buf.is_empty() {
                    continue_key_buf(&mut key_buf, line);
                } else {
                    continue_last_value(&mut key_vals, line);
                }
            } else {
                if !key_buf.is_empty() {
                    if !line.contains("=") {
                        continue_key_buf(&mut key_buf, line);
                    } else {
                        add_new_key_val(&mut key_buf, line, &mut key_vals);
                    }
                } else {
                    if indent > fst_indent {
                        continue_last_value(&mut key_vals, line);
                    } else {
                        if !line.contains("=") {
                            continue_key_buf(&mut key_buf, line);
                        } else {
                            add_new_key_val(
                                &mut key_buf,
                                line,
                                &mut key_vals,
                            );
                        }
                    }
                }
            }
        }

        // Check non-closed key-value pairs
        if !key_buf.is_empty() {
            return Err(format!(
                "No value found for key: {}",
                key_buf.trim()
            ));
        }

        // Trim all values again to avoid trailing newlines
        for key_val in key_vals.iter_mut() {
            key_val.value = key_val.value.trim_end().to_string();
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

    pub fn parse_flat_to_tree(key_vals: &KeyVals) -> KeyValTree {
        let mut tree = KeyValTree::new();

        for key_val in key_vals {
            let KeyVal { key, value } = key_val;
            let node = match KeyVal::parse(value) {
                Ok(new_key_vals) if !new_key_vals.is_empty() => {
                    KeyValNode::Tree(KeyVal::parse_flat_to_tree(
                        &new_key_vals,
                    ))
                }
                Err(_) | Ok(_) => KeyValNode::Leaf(value.to_string()),
            };

            insert_map(&mut tree, key, node);
        }

        tree
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

fn insert_map<'a, T>(
    map: &'a mut BTreeMap<String, Vec<T>>,
    key: &str,
    value: T,
) -> &'a mut BTreeMap<String, Vec<T>> {
    if let Some(nodes) = map.get_mut(key) {
        nodes.push(value);
    } else {
        map.insert(key.to_string(), vec![value]);
    }

    map
}

pub type KeyValTree = BTreeMap<String, Vec<KeyValNode>>;

#[derive(Clone, Debug)]
pub enum KeyValNode {
    Leaf(String),
    Tree(KeyValTree),
}

fn _leave_to_key_val(key: &str, node: KeyValNode) -> Option<KeyVal> {
    match node {
        KeyValNode::Leaf(value) => Some(KeyVal::new(key.to_string(), value)),
        _ => None,
    }
}

#[cfg(test)]
pub mod key_val_tests {
    use super::*;

    #[test]
    fn test_key_val_parse_4() {
        let input = "key \n= val\n";
        let key_vals = KeyVal::parse(input).unwrap();
        insta::assert_debug_snapshot!(key_vals, @r#"
        [
            KeyVal {
                key: "key",
                value: "val",
            },
        ]
        "#);
    }

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
        let pretty_str = KeyVal::pretty(&key_vals);

        insta::assert_snapshot!(pretty_str, @r#"
        a = "b"
        b = "\n  c = d\n  d =\n    e = f\n    f = g\n  g = h"
        h = "i"
        i = "j"
        j = "k\n  k = l"
        "#);
    }

    #[test]
    #[should_panic]
    fn test_key_val_parse_3() {
        let data = r#"
        c
        "#;
        let _ = KeyVal::parse(data).unwrap();
    }

    #[test]
    fn test_key_val_parse_2() {
        let data = r#"
        a = 
            b = c
            d = e
        "#;
        let key_vals = KeyVal::parse(data).unwrap();
        insta::assert_debug_snapshot!(key_vals, @r#"
        [
            KeyVal {
                key: "a",
                value: "\n            b = c\n            d = e",
            },
        ]
        "#);
    }

    #[test]
    fn test_key_val_parse_trailing_indent() {
        let data = r#"
  a = b
  c = d
"#;
        let key_vals = KeyVal::parse(data).unwrap();
        insta::assert_debug_snapshot!(key_vals, @r#"
        [
            KeyVal {
                key: "a",
                value: "b",
            },
            KeyVal {
                key: "c",
                value: "d",
            },
        ]
        "#);
        insta::assert_snapshot!(KeyVal::pretty(&key_vals), @r#"
        a = "b"
        c = "d"
        "#);
    }
}

#[cfg(test)]
mod key_val_tree_tests {
    use super::*;

    #[test]
    fn test_parse_flat_to_tree_1() {
        let data = r#"
a = 
  b = c
  d = e
"#;
        let key_vals = KeyVal::parse(data).unwrap();
        insta::assert_debug_snapshot!(key_vals, @r#"
        [
            KeyVal {
                key: "a",
                value: "\n  b = c\n  d = e",
            },
        ]
        "#);

        let tree = KeyVal::parse_flat_to_tree(&key_vals);

        insta::assert_debug_snapshot!(tree, @r#"
        {
            "a": [
                Tree(
                    {
                        "b": [
                            Leaf(
                                "c",
                            ),
                        ],
                        "d": [
                            Leaf(
                                "e",
                            ),
                        ],
                    },
                ),
            ],
        }
        "#);
    }

    #[test]
    fn test_parse_flat_to_tree_4() {
        let data = r#"
= d
= e
"#;
        let key_vals = KeyVal::parse(data).unwrap();
        insta::assert_debug_snapshot!(key_vals, @r#"
        [
            KeyVal {
                key: "",
                value: "d",
            },
            KeyVal {
                key: "",
                value: "e",
            },
        ]
        "#);
    }

    #[test]
    fn test_parse_flat_to_tree_3() {
        let data = r#"
abc =
  = d
  = e
"#;
        let key_vals = KeyVal::parse(data).unwrap();
        insta::assert_debug_snapshot!(key_vals, @r#"
        [
            KeyVal {
                key: "abc",
                value: "\n  = d\n  = e",
            },
        ]
        "#);
        let intermediate_tree = KeyVal::parse_flat_to_tree(
            &KeyVal::parse("\n  = d\n  = e").unwrap(),
        );
        insta::assert_debug_snapshot!(intermediate_tree, @r#"
        {
            "": [
                Leaf(
                    "d",
                ),
                Leaf(
                    "e",
                ),
            ],
        }
        "#);

        let tree = KeyVal::parse_flat_to_tree(&key_vals);
        insta::assert_debug_snapshot!(tree, @r#"
        {
            "abc": [
                Tree(
                    {
                        "": [
                            Leaf(
                                "d",
                            ),
                            Leaf(
                                "e",
                            ),
                        ],
                    },
                ),
            ],
        }
        "#);
    }

    #[test]
    fn test_parse_flat_to_tree_2() {
        let data = r#"
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
"#;

        let key_vals = KeyVal::parse(&data).unwrap();
        let tree = KeyVal::parse_flat_to_tree(&key_vals);

        insta::assert_debug_snapshot!(tree, @r#"
        {
            "a": [
                Leaf(
                    "b",
                ),
            ],
            "b": [
                Tree(
                    {
                        "c": [
                            Leaf(
                                "d",
                            ),
                        ],
                        "d": [
                            Tree(
                                {
                                    "e": [
                                        Leaf(
                                            "f",
                                        ),
                                    ],
                                    "f": [
                                        Leaf(
                                            "g",
                                        ),
                                    ],
                                },
                            ),
                        ],
                        "g": [
                            Leaf(
                                "h",
                            ),
                        ],
                    },
                ),
            ],
            "h": [
                Leaf(
                    "i",
                ),
            ],
            "i": [
                Leaf(
                    "j",
                ),
            ],
            "j": [
                Tree(
                    {
                        "k\n  k": [
                            Leaf(
                                "l",
                            ),
                        ],
                    },
                ),
            ],
        }
        "#);
    }
}
