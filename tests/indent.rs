use ccl_rs::key_val::KeyVal;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_smaller_indent_than_first_key() {
        let config = r#"
  key
val = 1
"#;
        let result = KeyVal::parse(config).unwrap();
        insta::assert_debug_snapshot!(result, @r#"
        [
            KeyVal {
                key: "key\nval",
                value: "1",
            },
        ]
        "#);
    }

    #[test]
    fn test_two_line_key() {
        let config = r#"
key1
key2
  = val
"#;
        let result = KeyVal::parse(config).unwrap();
        insta::assert_debug_snapshot!(result, @r#"
        [
            KeyVal {
                key: "key1\nkey2",
                value: "val",
            },
        ]
        "#);
    }

    #[test]
    fn test_multiline_key_with_variable_indent() {
        let config = r#"
  asd
 cvabaccc
qwfs = 1
"#;
        let result = KeyVal::parse(config).unwrap();
        insta::assert_debug_snapshot!(result, @r#"
        [
            KeyVal {
                key: "asd\n cvabaccc\nqwfs",
                value: "1",
            },
        ]
        "#);
    }

    #[test]
    fn test_multiline_key_with_variable_indent_2() {
        let config = r#"
    gwadgalk
asdasd
      asdasd
   = 1
"#;
        let result = KeyVal::parse(config).unwrap();
        insta::assert_debug_snapshot!(result, @r#"
        [
            KeyVal {
                key: "gwadgalk\nasdasd\n      asdasd",
                value: "1",
            },
        ]
        "#);
    }

    #[test]
    fn test_ambiguous_key_or_value() {
        let config = r#"
  ad
ck
      wd
   = gw
  ag
wd
  = qs
"#;
        let result = KeyVal::parse(config).unwrap();
        insta::assert_debug_snapshot!(result, @r#"
        [
            KeyVal {
                key: "ad\nck\n      wd",
                value: "gw",
            },
            KeyVal {
                key: "ag\nwd",
                value: "qs",
            },
        ]
        "#);
    }

    #[test]
    fn test_ambiguous_key_or_value_2() {
        let config = r#"
  ad
ck
      wd
   = gw
   ag
wd
  = qs
"#;
        let result = KeyVal::parse(config).unwrap();
        insta::assert_debug_snapshot!(result, @r#"
        [
            KeyVal {
                key: "ad\nck\n      wd",
                value: "gw\n   ag",
            },
            KeyVal {
                key: "wd",
                value: "qs",
            },
        ]
        "#);
    }

    #[test]
    fn test_ambiguous_key_or_value_3() {
        let config = r#"
  bf = wd
    sd = de
 sdfg
    sge
  we = 1
"#;
        let result = KeyVal::parse(config).unwrap();
        insta::assert_debug_snapshot!(result, @r#"
        [
            KeyVal {
                key: "bf",
                value: "wd\n    sd = de",
            },
            KeyVal {
                key: "sdfg\n    sge\n  we",
                value: "1",
            },
        ]
        "#);
    }

    #[test]
    fn test_indent_causing_unclosed_key() {
        let config = r#"
  bf = wd
    sd = de
 sdfg
    sge
  we
"#;
        let result = KeyVal::parse(config).unwrap_err();
        insta::assert_snapshot!(result, @r"
        No value found for key: sdfg
            sge
          we
        ");
    }
}
