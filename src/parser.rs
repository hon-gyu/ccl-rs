use std::fmt::Display;

#[derive(Debug, PartialEq, Eq)]
struct KeyVal {
    key: String,
    value: String,
}

impl KeyVal {
    fn new(key: String, value: String) -> Self {
        Self {
            key: key.trim().to_string(),
            value: value.trim().to_string(),
        }
    }

    fn parse(data: &str) -> Vec<Self> {
        let mut key_vals = Vec::new();

        let lines = data.trim().lines();

        for line in lines {
            let (key, value) = line.split_once("=").unwrap();

            key_vals.push(Self::new(key.to_string(), value.to_string()))
        }

        key_vals
    }
}

impl Display for KeyVal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} = {}", self.key, self.value)
    }
}

#[cfg(test)]
mod tests {
    use std::io::Stderr;

    use super::*;

    fn data() -> String {
        r#"
key1 = value1
key2 = value2
    key3 = value3
key4 = value4
"#
        .to_string()
    }

    #[test]
    fn test_parse() {
        let data = data();
        let key_vals = KeyVal::parse(&data);
        let parsed_str = key_vals
            .iter()
            .map(|key_val| key_val.to_string())
            .collect::<Vec<String>>()
            .join("\n");

        insta::assert_snapshot!(parsed_str, @r"
        key1 = value1
        key2 = value2
        key3 = value3
        key4 = value4
        ");
    }
}
