use std::collections::HashMap;

struct KeyMap<V>(HashMap<String, V>);

impl<V> KeyMap<V> {
    fn init_singleton(key: String, value: V) -> Self {
        let mut map = HashMap::new();
        map.insert(key, value);
        KeyMap(map)
    }

    fn init_nested(key: String, values: Vec<KeyMap<V>>) -> Self {
        todo!()
    }

    fn merge(lst: Vec<KeyMap<V>>) -> Self {
        todo!()
    }

    fn pretty(&self) -> String {
        todo!()
    }
}
