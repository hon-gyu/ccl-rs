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

    /// Parse a string into a vector of KeyVals by indentation
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

/// Indent a string by a given number of spaces for each line
fn indent(s: &str, indent: usize) -> String {
    let indent_str = " ".repeat(indent);
    s.lines()
        .map(|line| format!("{}{}", indent_str, line))
        .collect::<Vec<String>>()
        .join("\n")
}

impl Display for Ccl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();
        s.push_str(format!("{} = \n", self.key).as_str());

        for value_entry in self.value.iter() {
            match value_entry {
                ValueEntry::String(string) => {
                    s.push_str(indent(string, 2).as_str());
                    s.push_str("\n");
                }
                ValueEntry::Nested(ccl) => {
                    s.push_str(
                        indent(format!("{}", ccl).as_str(), 2).as_str(),
                    );
                    s.push_str("\n");
                }
            }
        }

        write!(f, "{}", s)
    }
}

impl Ccl {
    fn init_string(key: String, value: String) -> Self {
        Self {
            key: key,
            value: vec![ValueEntry::String(value)],
        }
    }

    fn init_nested(key: String, value: Vec<Ccl>) -> Self {
        let mut ccl_val = Vec::new();
        for ccl in value {
            ccl_val.push(ValueEntry::Nested(ccl));
        }
        Self {
            key: key,
            value: ccl_val,
        }
    }

    /// Recursively parse CCLs from a string
    /// # Arguments:
    /// - data: a string of CCLs
    /// # Returns:
    /// - A vector of CCLs
    fn parse(data: &str) -> Result<Vec<Self>, String> {
        let key_vals = KeyVal::parse_one_level(data)?;

        let mut ccls = Vec::new();
        for key_val in key_vals {
            let key = key_val.key;
            let value = key_val.value;

            let parsed_ccl = Self::parse(&value);
            let ccl = match parsed_ccl {
                Err(_) => {
                    // Value is a string, not nested CCL
                    Ccl::init_string(key, value)
                }
                Ok(nested_ccls) => {
                    // Value contains nested CCLs
                    Ccl::init_nested(key, nested_ccls)
                }
            };
            ccls.push(ccl);
        }
        Ok(ccls)
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

    #[test]
    fn test_ccl_init_string() {
        let ccl = Ccl::init_string("a".to_string(), "b".to_string());
        insta::assert_snapshot!(ccl, @r"
        a = 
          b
        ")
    }

    #[test]
    fn test_ccl_init_nested() {
        let ccls = (1..6)
            .map(|i| Ccl::init_string(format!("a{}", i), format!("b{}", i)))
            .collect::<Vec<Ccl>>();
        let ccl = Ccl::init_nested("root".to_string(), ccls);
        insta::assert_snapshot!(ccl, @r"
        root = 
          a1 = 
            b1
          a2 = 
            b2
          a3 = 
            b3
          a4 = 
            b4
          a5 = 
            b5
        ")
    }
}
