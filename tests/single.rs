/// Test cases for single key-value pair parsing
/// Extracted from vendor/ccl/test/test_parser/test_single.ml
use ccl_rs::key_val::KeyVal;

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("key=val")]
    #[case("key = val")]
    #[case("  key = val")]
    #[case("key = val  ")]
    #[case("  key  =  val  ")]
    #[case("\nkey = val\n")]
    #[case("key \n= val\n")]
    #[case("  \n key  \n=  val  \n")]
    fn test_single_key_val(#[case] config: &str) {
        let result = KeyVal::parse(config).unwrap();
        let expected = vec![KeyVal {
            key: "key".to_string(),
            value: "val".to_string(),
        }];
        assert_eq!(result, expected);
    }

    #[rstest]
    #[case("key =")]
    #[case("key =\n")]
    #[case("key =  ")]
    #[case("key =  \n")]
    fn test_empty_value(#[case] config: &str) {
        let result = KeyVal::parse(config).unwrap();
        let expected = vec![KeyVal {
            key: "key".to_string(),
            value: "".to_string(),
        }];
        assert_eq!(result, expected);
    }

    #[rstest]
    #[case("= val")]
    #[case("  = val")]
    #[case("\n  = val")]
    fn test_empty_key(#[case] config: &str) {
        let result = KeyVal::parse(config).unwrap();
        let expected = vec![KeyVal {
            key: "".to_string(),
            value: "val".to_string(),
        }];
        assert_eq!(result, expected);
    }

    #[rstest]
    #[case("=")]
    #[case("  =  ")]
    #[case("\n  =  \n")]
    fn test_empty_key_value(#[case] config: &str) {
        let result = KeyVal::parse(config).unwrap();
        let expected = vec![KeyVal {
            key: "".to_string(),
            value: "".to_string(),
        }];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_multiple_equality() {
        let config = "a=b=c";
        let result = KeyVal::parse(config).unwrap();
        insta::assert_debug_snapshot!(result, @r#"
        [
            KeyVal {
                key: "a",
                value: "b=c",
            },
        ]
        "#);
    }

    #[test]
    fn test_multiple_equality_2() {
        let config = "a = b = c";
        let result = KeyVal::parse(config).unwrap();
        insta::assert_debug_snapshot!(result, @r#"
        [
            KeyVal {
                key: "a",
                value: "b = c",
            },
        ]
        "#);
    }

    #[test]
    fn test_empty_equality() {
        let config = " = = ";
        let result = KeyVal::parse(config).unwrap();
        insta::assert_debug_snapshot!(result, @r#"
        [
            KeyVal {
                key: "",
                value: "=",
            },
        ]
        "#);
    }

    #[test]
    fn test_section() {
        let config = "== Section 2 ==";
        let result = KeyVal::parse(config).unwrap();
        insta::assert_debug_snapshot!(result, @r#"
        [
            KeyVal {
                key: "",
                value: "= Section 2 ==",
            },
        ]
        "#);
    }

    #[test]
    fn test_comment() {
        let config = "/= this is a comment";
        let result = KeyVal::parse(config).unwrap();
        insta::assert_debug_snapshot!(result, @r#"
        [
            KeyVal {
                key: "/",
                value: "this is a comment",
            },
        ]
        "#);
    }
}
