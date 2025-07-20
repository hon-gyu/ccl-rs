/// Test cases for nested value parsing
/// Extracted from vendor/ccl/test/test_parser/test_nested.ml

use ccl_rs::key_val::KeyVal;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_line_nested_value() {
        let config = r#"
key =
  val
"#;
        let expected = vec![
            KeyVal { key: "key".to_string(), value: "\n  val".to_string() },
        ];

        let result = KeyVal::parse(config).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_multi_line_nested_value() {
        let config = "\nkey =\n  line1\n  line2\n";
        let expected = vec![
            KeyVal { key: "key".to_string(), value: "\n  line1\n  line2".to_string() },
        ];

        let result = KeyVal::parse(config).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_multi_line_with_empty_line() {
        let config = r#"
key =
  line1

  line2
"#;
        let expected = vec![
            KeyVal { key: "key".to_string(), value: "\n  line1\n\n  line2".to_string() },
        ];

        let result = KeyVal::parse(config).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_nested_key_value_pairs() {
        let config = "\nkey =\n  field1 = value1\n  field2 = value2\n";
        let expected = vec![
            KeyVal { 
                key: "key".to_string(), 
                value: "\n  field1 = value1\n  field2 = value2".to_string() 
            },
        ];

        let result = KeyVal::parse(config).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_deep_nested_key_value_pairs() {
        let config = "\nkey =\n  field1 = value1\n  field2 =\n    subfield = x\n    another = y\n";
        let expected = vec![
            KeyVal { 
                key: "key".to_string(), 
                value: "\n  field1 = value1\n  field2 =\n    subfield = x\n    another = y".to_string() 
            },
        ];

        let result = KeyVal::parse(config).unwrap();
        assert_eq!(result, expected);
    }
}