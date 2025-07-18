use crate::monoid::Monoid;
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::fmt::Display;

#[derive(Debug, PartialEq, Eq)]
pub struct KeyVal {
    pub key: String,
    pub value: String,
}

/// Vec<KeyVal> is a monoid with
///    - the empty list as the identity element
///    - the concatenation of two lists as the merge operation
impl KeyVal {
    pub fn new(key: String, value: String) -> Self {
        Self {
            key: key.trim().to_string(),
            value: value.trim().to_string(),
        }
    }

    /// Parse a string into a vector of KeyVals by indentation
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

/// pretty and parse are monoid isomorphisms
fn pretty(key_vals: &Vec<KeyVal>) -> String {
    key_vals
        .iter()
        .map(|key_val| key_val.to_string())
        .collect::<Vec<String>>()
        .join("\n")
}

type KeyMap<T> = BTreeMap<String, T>;

#[derive(Clone)]
enum Entry {
    Leaf(String),
    Nested(EntryMap),
}

type EntryMap = KeyMap<Vec<Entry>>;

/// The only way to stop the recursion is to bind a key to an empty map.
/// And therefore, final level is values mapped to empty maps.
#[derive(Clone)]
struct CCL(KeyMap<CCL>);

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

#[cfg(test)]
mod test_ccl {
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
}

// fn fmt_ccl(ccl: &Ccl, indent: usize, boxed: bool) -> String {
//     let mut s = String::new();
//     for (key, value) in ccl.0.iter() {

//         // one value and it's a string -> one line
//         if value.len() == 1 {
//             if let ValueEntry::String(string) = value.first().unwrap() {
//                 let mut new = format!("{} = {}", key, string);
//                 if boxed {
//                     new = add_box(&new);
//                 }
//                 s.push_str(new.as_str());
//                 s.push_str("\n");
//             }
//         }
//         else {
//             let new_key_line = format!("{} =", key);
//             let new_value_line =
//         }
//     }
//     s
// }

// /// Format a single key-value in CCL
// fn fmt_one(key: &str, value: &Vec<ValueEntry>, boxed: bool) -> String {
//     let mut s = String::new();
//     s.push_str(format!("{} = ", key).as_str());

//     if value.len() == 1 {
//         if let ValueEntry::String(string) = value.first().unwrap() {
//             s.push_str(string);
//             if boxed {
//                 // Add padding to the string
//                 s = format!(" {} ", s);
//             }
//             return s;
//         }
//     }

//     s.push_str("\n");

//     for value_entry in value.iter() {
//         match value_entry {
//             ValueEntry::String(string) => {
//                 s.push_str(indent(string, 2).as_str());
//                 s.push_str("\n");
//             }
//             ValueEntry::Nested(ccl) => {
//                 s.push_str(indent)
//                 // s.push_str(indent(format!("{}", ccl).as_str(), 2).as_str());
//                 s.push_str("\n");
//                 if boxed {
//                     s = add_box(&s);
//                 }
//             }
//         }
//     }
//     s
// }

// fn fmt_ccl(ccl: &Ccl, boxed: bool) -> String {
//     ccl.0
//         .iter()
//         .map(|(key, value)| fmt_one(key, value, boxed))
//         .collect::<Vec<String>>()
//         .join("\n")
// }

// impl Display for Ccl {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "{}", fmt_ccl(self, false))
//     }
// }

// impl Ccl {
//     pub fn empty() -> Self {
//         Self(HashMap::new())
//     }

//     pub fn key_val(key: String, value: String) -> Self {
//         // TODO: should we check if the value can be parsed as a CCL?
//         let value = vec![ValueEntry::String(value)];
//         let mut ccl = HashMap::new();
//         ccl.insert(key, value);
//         Ccl(ccl)
//     }

//     pub fn nested(key: String, value: Vec<Ccl>) -> Self {
//         let mut ccl_val = Vec::new();
//         for ccl in value {
//             ccl_val.push(ValueEntry::Nested(ccl));
//         }
//         let mut ccl = HashMap::new();
//         ccl.insert(key, ccl_val);
//         Ccl(ccl)
//     }

//     pub fn merge(self, other: Self) -> Self {
//         let mut map = self.0;
//         for (rkey, rvalues) in other.0 {
//             if let Some(lvalues) = map.get_mut(&rkey) {
//                 lvalues.extend(rvalues);
//             } else {
//                 map.insert(rkey, rvalues);
//             }
//         }
//         Self(map)
//     }

//     pub fn of_list(ccls: Vec<Self>) -> Self {
//         ccls.iter()
//             .fold(Self::empty(), |acc, ccl| acc.merge(ccl.clone()))
//     }

//     /// Recursively parse CCLs from a string
//     /// # Arguments:
//     /// - data: a string of CCLs
//     /// # Returns:
//     /// - A CCL
//     pub fn parse(data: &str) -> Result<Self, String> {
//         let key_vals = KeyVal::parse(data)?;

//         let mut ccls = Vec::new();
//         for key_val in key_vals {
//             let key = key_val.key;
//             let value = key_val.value;

//             let parsed_ccl = Self::parse(&value);
//             let ccl = match parsed_ccl {
//                 Err(_) => {
//                     // Value is a string, not nested CCL
//                     Ccl::key_val(key, value)
//                 }
//                 Ok(nested_ccl) => {
//                     // Value contains nested CCLs
//                     Ccl::nested(key, vec![nested_ccl])
//                 }
//             };
//             ccls.push(ccl);
//         }
//         Ok(Ccl::of_list(ccls))
//     }

//     fn pretty(&self) -> String {
//         fmt_ccl(self, true)
//     }
// }

// impl From<KeyVal> for Ccl {
//     fn from(key_val: KeyVal) -> Self {
//         Ccl::key_val(key_val.key, key_val.value)
//     }
// }

#[cfg(test)]
mod test_key_val {
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

    // #[test]
    // fn test_ccl_init_string() {
    //     let ccl = Ccl::key_val("a".to_string(), "b".to_string());
    //     insta::assert_snapshot!(ccl, @"a = b")
    // }

    // #[test]
    // fn test_ccl_init_nested() {
    //     let ccls = (1..6)
    //         .map(|i| Ccl::key_val(format!("a{}", i), format!("b{}", i)))
    //         .collect::<Vec<Ccl>>();
    //     let ccl = Ccl::nested("root".to_string(), ccls);
    //     insta::assert_snapshot!(ccl, @r"
    //     root =
    //       a1 = b1
    //       a2 = b2
    //       a3 = b3
    //       a4 = b4
    //       a5 = b5
    //     ");

    //     let ccl2 = Ccl::nested(
    //         "root's root".to_string(),
    //         vec![ccl.clone(), ccl.clone()],
    //     );
    //     insta::assert_snapshot!(ccl2, @r"
    //     root's root =
    //       root =
    //         a1 = b1
    //         a2 = b2
    //         a3 = b3
    //         a4 = b4
    //         a5 = b5
    //       root =
    //         a1 = b1
    //         a2 = b2
    //         a3 = b3
    //         a4 = b4
    //         a5 = b5
    //     ");
    // }
}
