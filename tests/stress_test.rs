/// Comprehensive stress test with complex CCL document
/// Extracted from vendor/ccl/test/test_extra/stress.ml

use ccl_rs::key_val::{KeyVal, KeyVals};

#[cfg(test)]
mod stress_tests {
    use super::*;

    #[test]
    fn test_comprehensive_ccl_document() {
        let input = r#"/= This is a CCL document
title = CCL Example

database =
  enabled = true
  ports =
    = 8000
    = 8001
    = 8002
  limits =
    cpu = 1500mi
    memory = 10Gb

user =
  guestId = 42

user =
  login = chshersh
  createdAt = 2024-12-31"#;

        let expected = vec![
            KeyVal { 
                key: "/".to_string(), 
                value: "This is a CCL document".to_string() 
            },
            KeyVal { 
                key: "title".to_string(), 
                value: "CCL Example".to_string() 
            },
            KeyVal { 
                key: "database".to_string(), 
                value: "\n  enabled = true\n  ports =\n    = 8000\n    = 8001\n    = 8002\n  limits =\n    cpu = 1500mi\n    memory = 10Gb".to_string() 
            },
            KeyVal { 
                key: "user".to_string(), 
                value: "\n  guestId = 42".to_string() 
            },
            KeyVal { 
                key: "user".to_string(), 
                value: "\n  login = chshersh\n  createdAt = 2024-12-31".to_string() 
            },
        ];

        let result = KeyVal::parse(input).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_complex_nested_structure() {
        let input = r#"numbers =
  foo = 12341234
  bar = 12341233
  baz = 12341236"#;

        let expected = vec![
            KeyVal { 
                key: "numbers".to_string(), 
                value: "\n  foo = 12341234\n  bar = 12341233\n  baz = 12341236".to_string() 
            },
        ];

        let result = KeyVal::parse(input).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_mixed_structure() {
        let input = r#"numbers =
  baz = 123
  foo = 1
somekey = someval
this =
  that =
  foo =
  bar = baz"#;

        let expected = vec![
            KeyVal { 
                key: "numbers".to_string(), 
                value: "\n  baz = 123\n  foo = 1".to_string() 
            },
            KeyVal { 
                key: "somekey".to_string(), 
                value: "someval".to_string() 
            },
            KeyVal { 
                key: "this".to_string(), 
                value: "\n  that =\n  foo =\n  bar = baz".to_string() 
            },
        ];

        let result = KeyVal::parse(input).unwrap();
        assert_eq!(result, expected);
    }
}