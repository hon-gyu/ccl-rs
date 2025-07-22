use crate::key_val::{KeyVal, KeyValNode, KeyValTree, KeyVals};
use crate::monoid::Monoid;
use std::collections::BTreeMap;

type KeyMap<T> = BTreeMap<String, T>;

/// The only way to stop the recursion is to bind a key to an empty map.
/// And therefore, final level is values mapped to empty maps.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CCL(pub KeyMap<CCL>);

impl Monoid for CCL {
    fn empty() -> Self {
        Self(KeyMap::new())
    }

    fn merge(self, other: Self) -> Self {
        let CCL(mut lmap) = self;
        let CCL(rmap) = other;

        for (rkey, rvalues) in rmap {
            match lmap.get_mut(&rkey) {
                Some(lvalues) => {
                    let merged = lvalues.clone().merge(rvalues);
                    *lvalues = merged;
                }
                None => {
                    lmap.insert(rkey, rvalues);
                }
            }
        }

        CCL(lmap)
    }
}

/// Helpers
impl CCL {
    /// ```text
    /// key =
    /// ```
    pub fn key(key: &str) -> Self {
        let mut map = KeyMap::new();
        map.insert(key.to_string(), CCL::empty());
        CCL(map)
    }

    /// ```text
    /// key =
    ///     value =
    /// ```
    pub fn key_val(key: &str, value: &str) -> Self {
        let mut map = KeyMap::new();
        map.insert(key.to_string(), CCL::key(value));
        CCL(map)
    }

    /// ```text
    /// key =
    ///     value1 =
    ///     value2 =
    /// ```
    pub fn nested(key: &str, values: Vec<CCL>) -> Self {
        let mut map = KeyMap::new();
        map.insert(key.to_string(), CCL::aggregate(values));
        CCL(map)
    }

    pub fn pretty(&self) -> String {
        let mut buf = String::new();
        self.pretty_impl(0, &mut buf);
        buf
    }

    /// terminal case is empty map (no key-value pairs to iterate over)
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
}

impl CCL {
    fn parse_tree_to_fix(tree: KeyValTree) -> CCL {
        let mut ccl = CCL::empty();

        for (key, values) in tree {
            let ccls = values
                .iter()
                .map(|value| match value {
                    KeyValNode::Leaf(leaf) => {
                        if leaf.is_empty() {
                            CCL::key(&key)
                        } else {
                            CCL::key_val(&key, leaf)
                        }
                    }
                    KeyValNode::Tree(tree) => CCL::nested(
                        &key,
                        vec![CCL::parse_tree_to_fix(tree.clone())],
                    ),
                })
                .collect::<Vec<CCL>>();

            ccl = ccl.merge(CCL::aggregate(ccls));
        }
        ccl
    }

    pub fn parse(key_vals: KeyVals) -> CCL {
        CCL::parse_tree_to_fix(KeyVal::parse_flat_to_tree(&key_vals))
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ccl_pretty() {
        let ccl = CCL::nested(
            "a",
            vec![
                CCL::key_val("b", "c"),
                CCL::key_val("d", "e"),
                CCL::key("h"),
            ],
        );
        insta::assert_snapshot!(ccl.pretty(), @r"
        a =
          b =
            c =
          d =
            e =
          h =
        ");
    }

    #[test]
    fn test_merge() {
        let ccl1 = CCL::nested("a", vec![CCL::key_val("b", "c")]);
        let ccl2 = CCL::nested("a", vec![CCL::key_val("b", "d")]);
        insta::assert_snapshot!(ccl1.merge(ccl2).pretty(), @r"
        a =
          b =
            c =
            d =
        ");
    }

    #[test]
    fn test_merge_associativity() {
        let ccl1 = CCL::nested("a", vec![CCL::key_val("b", "c")]);
        let ccl2 = CCL::nested("a", vec![CCL::key_val("b", "d")]);
        let ccl3 = CCL::nested("a", vec![CCL::key_val("b", "e")]);
        assert_eq!(
            ccl1.clone().merge(ccl2.clone()).merge(ccl3.clone()),
            ccl1.clone().merge(ccl2.clone().merge(ccl3.clone()))
        );
    }

    #[test]
    fn test_parse_ccl() {
        // 1. raw text
        let data = r#"
        1 = 2
        a = 
            b = c
            d = e
        "#;
        // 2. key-value list
        let key_vals = KeyVal::parse(data).unwrap();
        // 3, 4. key-value tree, CCL
        let ccl = CCL::parse(key_vals);
        insta::assert_debug_snapshot!(ccl, @r#"
        CCL(
            {
                "1": CCL(
                    {
                        "2": CCL(
                            {},
                        ),
                    },
                ),
                "a": CCL(
                    {
                        "b": CCL(
                            {
                                "c": CCL(
                                    {},
                                ),
                            },
                        ),
                        "d": CCL(
                            {
                                "e": CCL(
                                    {},
                                ),
                            },
                        ),
                    },
                ),
            },
        )
        "#);
        // 5. CCL-pretty
        insta::assert_snapshot!(ccl.pretty(), @r"
        1 =
          2 =
        a =
          b =
            c =
          d =
            e =
        ");
    }

    #[test]
    fn test_parse_ccl_2() {
        // 1. raw text
        let data = r#"
        =
            a = b
            c = d
        "#;
        // 2. key-value list
        let key_vals = KeyVal::parse(data).unwrap();
        // 3. key-value tree,
        let key_vals_tree = KeyVal::parse_flat_to_tree(&key_vals);
        insta::assert_debug_snapshot!(key_vals_tree, @r#"
        {
            "": [
                Tree(
                    {
                        "a": [
                            Leaf(
                                "b",
                            ),
                        ],
                        "c": [
                            Leaf(
                                "d",
                            ),
                        ],
                    },
                ),
            ],
        }
        "#);
        // 4. CCL
        let ccl = CCL::parse_tree_to_fix(key_vals_tree);
        insta::assert_debug_snapshot!(ccl, @r#"
        CCL(
            {
                "": CCL(
                    {
                        "a": CCL(
                            {
                                "b": CCL(
                                    {},
                                ),
                            },
                        ),
                        "c": CCL(
                            {
                                "d": CCL(
                                    {},
                                ),
                            },
                        ),
                    },
                ),
            },
        )
        "#);
        // 5. CCL-pretty
        insta::assert_snapshot!(ccl.pretty(), @r"
        =
         a =
           b =
         c =
           d =
        ");
    }

    #[test]
    fn test_parse_ccl_3() {
        // 1. raw text
        let data = r#"
        =
            a =
            c =
        "#;
        // 2. key-value list
        let key_vals = KeyVal::parse(data).unwrap();
        // 3. key-value tree,
        let key_vals_tree = KeyVal::parse_flat_to_tree(&key_vals);
        insta::assert_debug_snapshot!(key_vals_tree, @r#"
        {
            "": [
                Tree(
                    {
                        "a": [
                            Leaf(
                                "",
                            ),
                        ],
                        "c": [
                            Leaf(
                                "",
                            ),
                        ],
                    },
                ),
            ],
        }
        "#);
        // 4. CCL
        let ccl = CCL::parse_tree_to_fix(key_vals_tree);
        insta::assert_debug_snapshot!(ccl, @r#"
        CCL(
            {
                "": CCL(
                    {
                        "a": CCL(
                            {},
                        ),
                        "c": CCL(
                            {},
                        ),
                    },
                ),
            },
        )
        "#);
        // 5. CCL-pretty
        insta::assert_snapshot!(ccl.pretty(), @r"
        =
         a =
         c =
        ");
    }

    #[test]
    fn test_from_prop_test_roundtrip_failure() {
        let ccl = CCL::nested("", vec![CCL::nested("", vec![CCL::empty()])]);
        insta::assert_debug_snapshot!(ccl, @r#"
        CCL(
            {
                "": CCL(
                    {
                        "": CCL(
                            {},
                        ),
                    },
                ),
            },
        )
        "#);
        insta::assert_snapshot!(&ccl.pretty(), @r"
        =
          =
        ");

        let key_vals_2 = KeyVal::parse(&ccl.pretty()).unwrap();
        insta::assert_debug_snapshot!(key_vals_2, @r#"
        [
            KeyVal {
                key: "",
                value: "\n   =",
            },
        ]
        "#);

        let key_vals_tree_2 = KeyVal::parse_flat_to_tree(&key_vals_2);
        insta::assert_debug_snapshot!(key_vals_tree_2, @r#"
        {
            "": [
                Tree(
                    {
                        "": [
                            Leaf(
                                "",
                            ),
                        ],
                    },
                ),
            ],
        }
        "#);

        let ccl2 = CCL::parse_tree_to_fix(key_vals_tree_2);
        insta::assert_debug_snapshot!(ccl2, @r#"
        CCL(
            {
                "": CCL(
                    {
                        "": CCL(
                            {},
                        ),
                    },
                ),
            },
        )
        "#);
        insta::assert_snapshot!(ccl2.pretty(), @r"
        =
          =
        ");
    }
}
