use std::collections::BTreeMap;
use std::collections::HashMap;

#[derive(Debug, Clone)]
enum ValueEntry {
    String(String),
    Nested(Ccl),
}

#[derive(Debug, Clone)]
struct Ccl {
    map: HashMap<String, Vec<ValueEntry>>,
}

impl Ccl {
    fn empty() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    fn key_val(key: String, value: String) -> Self {
        let mut map = HashMap::new();
        map.insert(key, vec![ValueEntry::String(value)]);
        Self { map }
    }

    fn merge(self, other: Self) -> Self {
        let mut map = self.map;
        for (key, values) in other.map {
            if let Some(existing) = map.get_mut(&key) {
                existing.extend(values);
            } else {
                map.insert(key, values);
            }
        }
        Self { map }
    }

    // Pretty print the map, determinstically
    fn pretty(&self) -> String {
        // _ is asking rust to infer the type of the key and value
        let sorted_map: BTreeMap<_, _> = self.map.iter().collect();
        format!("{:#?}", sorted_map)
    }

    // // Create a CCL with key k mapping to an empty CCL
    // fn key(key: String) -> Self {
    //     let mut map = HashMap::new();
    //     map.insert(key, vec![ValueEntry::Nested(Ccl::empty())]);
    //     Self { map }
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty() {
        let ccl = Ccl::empty();
        insta::assert_snapshot!(ccl.pretty(), @"{}");
    }

    #[test]
    fn test_key_val() {
        let ccl = Ccl::key_val("tt".to_string(), "ttt".to_string());
        insta::assert_snapshot!(ccl.pretty(), @r#"
        {
            "tt": [
                String(
                    "ttt",
                ),
            ],
        }
        "#)
    }

    #[test]
    fn test_merge_same_key() {
        let ccl1 = Ccl::key_val("tt".to_string(), "ttt".to_string());
        let ccl2 = Ccl::key_val("tt".to_string(), "ttt2".to_string());
        let ccl = ccl1.merge(ccl2);
        insta::assert_snapshot!(ccl.pretty(), @r#"
        {
            "tt": [
                String(
                    "ttt",
                ),
                String(
                    "ttt2",
                ),
            ],
        }
        "#)
    }

    #[test]
    fn test_merge_different_key() {
        let ccl1 = Ccl::key_val("tt".to_string(), "ttt".to_string());
        let ccl2 = Ccl::key_val("tt2".to_string(), "ttt2".to_string());
        let ccl = ccl1.merge(ccl2);
        insta::assert_snapshot!(ccl.pretty(), @r#"
        {
            "tt": [
                String(
                    "ttt",
                ),
            ],
            "tt2": [
                String(
                    "ttt2",
                ),
            ],
        }
        "#)
    }

    #[test]
    fn test_merge_same_key_same_value() {
        let ccl1 = Ccl::key_val("tt".to_string(), "ttt".to_string());
        let ccl2 = Ccl::key_val("tt".to_string(), "ttt".to_string());
        let ccl = ccl1.merge(ccl2);
        insta::assert_snapshot!(ccl.pretty(), @r#"
        {
            "tt": [
                String(
                    "ttt",
                ),
                String(
                    "ttt",
                ),
            ],
        }
        "#)
    }
}
