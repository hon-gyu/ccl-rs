/// Test cases for nested value parsing
/// Extracted from vendor/ccl/test/test_parser/test_nested.ml

use ccl_rs::key_val::KeyVal;
use ccl_rs::parser::CCL;

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

    #[test]
    fn test_extra_equation() {
        let config = r#"
        ports =
            = 8000
            = 8001
            = 8002
        "#;
        let result = KeyVal::parse(config).unwrap();
        insta::assert_debug_snapshot!(result, @r#"
        [
            KeyVal {
                key: "ports",
                value: "\n            = 8000\n            = 8001\n            = 8002",
            },
        ]
        "#);
        let key_val_tree = KeyVal::parse_flat_to_tree(&result);
        insta::assert_debug_snapshot!(key_val_tree, @r#"
        {
            "ports": [
                Tree(
                    {
                        "": [
                            Leaf(
                                "8000",
                            ),
                            Leaf(
                                "8001",
                            ),
                            Leaf(
                                "8002",
                            ),
                        ],
                    },
                ),
            ],
        }
        "#);
        let ccl = CCL::parse(result);
        insta::assert_debug_snapshot!(ccl, @r#"
        CCL(
            {
                "ports": CCL(
                    {
                        "": CCL(
                            {
                                "8000": CCL(
                                    {},
                                ),
                                "8001": CCL(
                                    {},
                                ),
                                "8002": CCL(
                                    {},
                                ),
                            },
                        ),
                    },
                ),
            },
        )
        "#);


        let config = r#"
        ports =
            =
            8000 =
            8001 =
            8002 =
        "#;
        let result = KeyVal::parse(config).unwrap();
        insta::assert_debug_snapshot!(result, @r#"
        [
            KeyVal {
                key: "ports",
                value: "\n            =\n            8000 =\n            8001 =\n            8002 =",
            },
        ]
        "#);
        let ccl = CCL::parse(result);
        insta::assert_debug_snapshot!(ccl, @r#"
        CCL(
            {
                "ports": CCL(
                    {
                        "": CCL(
                            {
                                "": CCL(
                                    {},
                                ),
                            },
                        ),
                        "8000": CCL(
                            {
                                "": CCL(
                                    {},
                                ),
                            },
                        ),
                        "8001": CCL(
                            {
                                "": CCL(
                                    {},
                                ),
                            },
                        ),
                        "8002": CCL(
                            {
                                "": CCL(
                                    {},
                                ),
                            },
                        ),
                    },
                ),
            },
        )
        "#);

        let config = r#"
        ports
            = 8000
            = 8001
            = 8002
        "#;
        let result = KeyVal::parse(config).unwrap();
        insta::assert_debug_snapshot!(result, @r#"
        [
            KeyVal {
                key: "ports",
                value: "8000\n            = 8001\n            = 8002",
            },
        ]
        "#);
    }

}
