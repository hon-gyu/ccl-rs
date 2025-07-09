use std::fmt::Display;

#[derive(Debug, PartialEq, Eq)]
pub struct KeyVal {
    pub key: String,
    pub value: String,
}

impl KeyVal {
    pub fn new(key: String, value: String) -> Self {
        Self {
            key: key.trim().to_string(),
            value: value.trim().to_string(),
        }
    }

    fn dedent<'a>(lines: &[&'a str]) -> Vec<&'a str> {
        let min_indent = lines
            .iter()
            .map(|line| line.len() - line.trim_start().len())
            .min()
            .unwrap();

        // Learning note:
        // Equivalent to `&(*line)[min_indent..]`
        // indexing is method calling, so it will do derefer coercion
        // indexing a slice gives a slice
        // line[min_indent..] is of type str
        // &line[min_indent..] is of type &str
        lines.iter().map(|line| &line[min_indent..]).collect()
    }

    // Parse one level
    pub fn parse_one_level(data: &str) -> Result<Vec<Self>, String> {
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

                key_vals.push(Self::new(
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
}

impl Display for KeyVal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} = {:?}", self.key, self.value)
    }
}

enum ValueEntry {
    String(String),
    Nested(Ccl),
}

struct Ccl {
    key: String,
    value: Vec<ValueEntry>,
}

impl Ccl {
    fn parse(data: &str) -> Result<Vec<Self>, String> {
        let key_vals = KeyVal::parse_one_level(data)?;

        let mut ccls = Vec::new();
        for key_val in key_vals {
            let key = key_val.key;
            let value = key_val.value;

            let parsed_ccl = Self::parse(&value);
            match parsed_ccl {
                Err(e) => {
                    let ccl = Ccl {
                        key,
                        value: vec![ValueEntry::String(value)],
                    };
                }
                Ok(ccls) => {
                    // ?
                    todo!()
                }
            }
            ccls.push(ccl);
        }
        Ok(ccls)
    }
}

// // Parse nested
// pub fn parse(data: &str) -> Result<Vec<Self>, String> {
//     // Learning note:
//     // extract or early return the error
//     let mut key_vals = Self::parse_one_level(data)?;

//     for key_val in key_vals.iter_mut() {
//         let val_parsed = Self::parse(&key_val.value);
//         match val_parsed {
//             Err(e) => {
//                 continue;
//             }
//             Ok(val_parsed) => {
//                 key_val.value = val_parsed;
//             }
//         }
//     }
//     Ok(key_vals)
// }

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
    fn test_parse() {
        let data = data();
        let key_vals = KeyVal::parse_one_level(&data).unwrap();
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
