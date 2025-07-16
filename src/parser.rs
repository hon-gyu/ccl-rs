use std::collections::HashMap;
use std::collections::BTreeMap;
use std::fmt::Display;

mod string_utils {
    /// Indent a string by a given number of spaces for each line
    pub fn indent(s: &str, indent: usize) -> String {
        let indent_str = " ".repeat(indent);
        s.lines()
            .map(|line| format!("{}{}", indent_str, line))
            .collect::<Vec<String>>()
            .join("\n")
    }

    /// Dedent a vector of strings by the minimum indentation
    pub fn dedent<'a>(lines: &[&'a str]) -> Vec<&'a str> {
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

    pub const BOX_DRAWING_CHARS: (&str, &str, &str, &str, &str, &str) =
        ("┌", "┐", "┘", "└", "─", "│");

    /// Add a box around a string
    pub fn add_box(s: &str) -> String {
        let (
            top_left,
            top_right,
            bottom_right,
            bottom_left,
            _horizontal,
            vertical,
        ) = BOX_DRAWING_CHARS;
        let lines = s.lines().collect::<Vec<&str>>();
        let max_len = lines.iter().map(|line| line.len()).max().unwrap();
        let mut result = String::new();
        result.push_str(top_left);
        result.push_str(&"─".repeat(max_len));
        result.push_str(top_right);
        result.push_str("\n");
        for line in lines {
            result.push_str(format!("{}{}", vertical, line).as_str());
            let pad = " ".repeat(max_len - line.len());
            result.push_str(format!("{}{}", pad, vertical).as_str());
            result.push_str("\n");
        }
        result.push_str(bottom_left);
        result.push_str(&"─".repeat(max_len));
        result.push_str(bottom_right);
        result
    }
}

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


    /// Parse a string into a vector of KeyVals by indentation
    pub fn parse(data: &str) -> Result<Vec<Self>, String> {
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

type KeyMap<T> = BTreeMap<String, T>;

#[derive(Clone)]
enum ValueEntry {
    String(String),
    Nested(Ccl),
}

#[derive(Clone)]
struct Ccl(HashMap<String, Vec<ValueEntry>>);

/// Indent a string by a given number of spaces for each line
fn indent(s: &str, indent: usize) -> String {
    let indent_str = " ".repeat(indent);
    s.lines()
        .map(|line| format!("{}{}", indent_str, line))
        .collect::<Vec<String>>()
        .join("\n")
}


fn fmt_ccl(ccl: &Ccl, indent: usize, boxed: bool) -> String {
    let mut s = String::new();
    for (key, value) in ccl.0.iter() {

        // one value and it's a string -> one line
        if value.len() == 1 {
            if let ValueEntry::String(string) = value.first().unwrap() {
                let mut new = format!("{} = {}", key, string);
                if boxed {
                    new = add_box(&new);
                }
                s.push_str(new.as_str());
                s.push_str("\n");
            }
        }
        else {
            let new_key_line = format!("{} =", key);
            let new_value_line = 
        }
    }
    s

}

/// Format a single key-value in CCL
fn fmt_one(key: &str, value: &Vec<ValueEntry>, boxed: bool) -> String {
    let mut s = String::new();
    s.push_str(format!("{} = ", key).as_str());

    if value.len() == 1 {
        if let ValueEntry::String(string) = value.first().unwrap() {
            s.push_str(string);
            if boxed {
                // Add padding to the string
                s = format!(" {} ", s);
            }
            return s;
        }
    }

    s.push_str("\n");

    for value_entry in value.iter() {
        match value_entry {
            ValueEntry::String(string) => {
                s.push_str(indent(string, 2).as_str());
                s.push_str("\n");
            }
            ValueEntry::Nested(ccl) => {
                s.push_str(indent)
                // s.push_str(indent(format!("{}", ccl).as_str(), 2).as_str());
                s.push_str("\n");
                if boxed {
                    s = add_box(&s);
                }
            }
        }
    }
    s
}

fn fmt_ccl(ccl: &Ccl, boxed: bool) -> String {
    ccl.0
        .iter()
        .map(|(key, value)| fmt_one(key, value, boxed))
        .collect::<Vec<String>>()
        .join("\n")
}

impl Display for Ccl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", fmt_ccl(self, false))
    }
}

impl Ccl {
    pub fn empty() -> Self {
        Self(HashMap::new())
    }

    pub fn key_val(key: String, value: String) -> Self {
        // TODO: should we check if the value can be parsed as a CCL?
        let value = vec![ValueEntry::String(value)];
        let mut ccl = HashMap::new();
        ccl.insert(key, value);
        Ccl(ccl)
    }

    pub fn nested(key: String, value: Vec<Ccl>) -> Self {
        let mut ccl_val = Vec::new();
        for ccl in value {
            ccl_val.push(ValueEntry::Nested(ccl));
        }
        let mut ccl = HashMap::new();
        ccl.insert(key, ccl_val);
        Ccl(ccl)
    }

    pub fn merge(self, other: Self) -> Self {
        let mut map = self.0;
        for (rkey, rvalues) in other.0 {
            if let Some(lvalues) = map.get_mut(&rkey) {
                lvalues.extend(rvalues);
            } else {
                map.insert(rkey, rvalues);
            }
        }
        Self(map)
    }

    pub fn of_list(ccls: Vec<Self>) -> Self {
        ccls.iter()
            .fold(Self::empty(), |acc, ccl| acc.merge(ccl.clone()))
    }

    /// Recursively parse CCLs from a string
    /// # Arguments:
    /// - data: a string of CCLs
    /// # Returns:
    /// - A CCL
    pub fn parse(data: &str) -> Result<Self, String> {
        let key_vals = KeyVal::parse_top_level(data)?;

        let mut ccls = Vec::new();
        for key_val in key_vals {
            let key = key_val.key;
            let value = key_val.value;

            let parsed_ccl = Self::parse(&value);
            let ccl = match parsed_ccl {
                Err(_) => {
                    // Value is a string, not nested CCL
                    Ccl::key_val(key, value)
                }
                Ok(nested_ccl) => {
                    // Value contains nested CCLs
                    Ccl::nested(key, vec![nested_ccl])
                }
            };
            ccls.push(ccl);
        }
        Ok(Ccl::of_list(ccls))
    }

    fn pretty(&self) -> String {
        fmt_ccl(self, true)
    }
}

impl From<KeyVal> for Ccl {
    fn from(key_val: KeyVal) -> Self {
        Ccl::key_val(key_val.key, key_val.value)
    }
}

const BOX_DRAWING_CHARS: (&str, &str, &str, &str, &str, &str) =
    ("┌", "┐", "┘", "└", "─", "│");

fn add_box(s: &str) -> String {
    let (
        top_left,
        top_right,
        bottom_right,
        bottom_left,
        _horizontal,
        vertical,
    ) = BOX_DRAWING_CHARS;
    let lines = s.lines().collect::<Vec<&str>>();
    let max_len = lines.iter().map(|line| line.len()).max().unwrap();
    let mut result = String::new();
    result.push_str(top_left);
    result.push_str(&"─".repeat(max_len));
    result.push_str(top_right);
    result.push_str("\n");
    for line in lines {
        result.push_str(format!("{}{}", vertical, line).as_str());
        let pad = " ".repeat(max_len - line.len());
        result.push_str(format!("{}{}", pad, vertical).as_str());
        result.push_str("\n");
    }
    result.push_str(bottom_left);
    result.push_str(&"─".repeat(max_len));
    result.push_str(bottom_right);
    result
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
        let key_vals = KeyVal::parse_top_level(&data).unwrap();
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
        let ccl = Ccl::key_val("a".to_string(), "b".to_string());
        insta::assert_snapshot!(ccl, @"a = b")
    }

    #[test]
    fn test_ccl_init_nested() {
        let ccls = (1..6)
            .map(|i| Ccl::key_val(format!("a{}", i), format!("b{}", i)))
            .collect::<Vec<Ccl>>();
        let ccl = Ccl::nested("root".to_string(), ccls);
        insta::assert_snapshot!(ccl, @r"
        root = 
          a1 = b1
          a2 = b2
          a3 = b3
          a4 = b4
          a5 = b5
        ");

        let ccl2 = Ccl::nested(
            "root's root".to_string(),
            vec![ccl.clone(), ccl.clone()],
        );
        insta::assert_snapshot!(ccl2, @r"
        root's root = 
          root = 
            a1 = b1
            a2 = b2
            a3 = b3
            a4 = b4
            a5 = b5
          root = 
            a1 = b1
            a2 = b2
            a3 = b3
            a4 = b4
            a5 = b5
        ");
    }

    #[test]
    fn test_add_box() {
        let s = "a\nbb\nc";
        let boxed = add_box(s);
        insta::assert_snapshot!(boxed, @r"
        ┌──┐
        │a │
        │bb│
        │c │
        └──┘
        ");
    }

    #[test]
    fn test_ccl_pretty() {
        let ccl = Ccl::parse(data().as_str()).unwrap();
        insta::assert_snapshot!(ccl.pretty(), @"");
    }
}
