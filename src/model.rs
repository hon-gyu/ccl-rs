use std::collections::HashMap;

#[derive(Debug)]
struct KeyMap<V>(HashMap<String, V>);

impl<V> KeyMap<V> {
    fn init_singleton(key: String, value: V) -> Self {
        let mut map = HashMap::new();
        map.insert(key, value);
        KeyMap(map)
    }

    fn init_nested(
        key: String,
        values: Vec<KeyMap<V>>,
    ) -> KeyMap<Vec<KeyMap<V>>> {
        let mut map = HashMap::new();
        map.insert(key, values);
        KeyMap(map)
    }

    fn merge(lst: Vec<KeyMap<V>>) -> Self {
        todo!()
    }

    fn pretty(&self) -> String {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_multiple_key_map() -> Vec<KeyMap<String>> {
        let mut key_maps = Vec::new();
        for i in (0..10).step_by(2) {
            for j in (1..11).step_by(2) {
                let elem = KeyMap::init_singleton(
                    format!("t{}", i),
                    format!("t{}", j),
                );
                key_maps.push(elem);
            }
        }
        key_maps
    }

    #[test]
    fn test_init_singleton() {
        let key_map =
            KeyMap::init_singleton("t1".to_string(), "t2".to_string());
        insta::assert_debug_snapshot!(key_map, @r#"
        KeyMap(
            {
                "t1": "t2",
            },
        )
        "#);
    }

    #[test]
    fn test_init_nested() {
        let key_maps = get_multiple_key_map();
        let key_map = KeyMap::init_nested("k".to_string(), key_maps);
        insta::assert_debug_snapshot!(key_map, @r#"
        KeyMap(
            {
                "k": [
                    KeyMap(
                        {
                            "t0": "t1",
                        },
                    ),
                    KeyMap(
                        {
                            "t0": "t3",
                        },
                    ),
                    KeyMap(
                        {
                            "t0": "t5",
                        },
                    ),
                    KeyMap(
                        {
                            "t0": "t7",
                        },
                    ),
                    KeyMap(
                        {
                            "t0": "t9",
                        },
                    ),
                    KeyMap(
                        {
                            "t2": "t1",
                        },
                    ),
                    KeyMap(
                        {
                            "t2": "t3",
                        },
                    ),
                    KeyMap(
                        {
                            "t2": "t5",
                        },
                    ),
                    KeyMap(
                        {
                            "t2": "t7",
                        },
                    ),
                    KeyMap(
                        {
                            "t2": "t9",
                        },
                    ),
                    KeyMap(
                        {
                            "t4": "t1",
                        },
                    ),
                    KeyMap(
                        {
                            "t4": "t3",
                        },
                    ),
                    KeyMap(
                        {
                            "t4": "t5",
                        },
                    ),
                    KeyMap(
                        {
                            "t4": "t7",
                        },
                    ),
                    KeyMap(
                        {
                            "t4": "t9",
                        },
                    ),
                    KeyMap(
                        {
                            "t6": "t1",
                        },
                    ),
                    KeyMap(
                        {
                            "t6": "t3",
                        },
                    ),
                    KeyMap(
                        {
                            "t6": "t5",
                        },
                    ),
                    KeyMap(
                        {
                            "t6": "t7",
                        },
                    ),
                    KeyMap(
                        {
                            "t6": "t9",
                        },
                    ),
                    KeyMap(
                        {
                            "t8": "t1",
                        },
                    ),
                    KeyMap(
                        {
                            "t8": "t3",
                        },
                    ),
                    KeyMap(
                        {
                            "t8": "t5",
                        },
                    ),
                    KeyMap(
                        {
                            "t8": "t7",
                        },
                    ),
                    KeyMap(
                        {
                            "t8": "t9",
                        },
                    ),
                ],
            },
        )
        "#);
    }
}
