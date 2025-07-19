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
    fn test_single_key_val(#[case] input: &str) {
        let result = KeyVal::parse(input).unwrap();
        let expected = vec![KeyVal {
            key: "key".to_string(),
            value: "val".to_string(),
        }];
        assert_eq!(result, expected);
    }

    // #[test]
    // fn test_basic_key_value() {
    //     let cases = vec![
    //         (
    //             "key=val",
    //             vec![KeyVal {
    //                 key: "key".to_string(),
    //                 value: "val".to_string(),
    //             }],
    //         ),
    //         (
    //             "key = val",
    //             vec![KeyVal {
    //                 key: "key".to_string(),
    //                 value: "val".to_string(),
    //             }],
    //         ),
    //         (
    //             "  key = val",
    //             vec![KeyVal {
    //                 key: "key".to_string(),
    //                 value: "val".to_string(),
    //             }],
    //         ),
    //         (
    //             "key = val  ",
    //             vec![KeyVal {
    //                 key: "key".to_string(),
    //                 value: "val".to_string(),
    //             }],
    //         ),
    //         (
    //             "  key  =  val  ",
    //             vec![KeyVal {
    //                 key: "key".to_string(),
    //                 value: "val".to_string(),
    //             }],
    //         ),
    //         (
    //             "\nkey = val\n",
    //             vec![KeyVal {
    //                 key: "key".to_string(),
    //                 value: "val".to_string(),
    //             }],
    //         ),
    //         (
    //             "key \n= val\n",
    //             vec![KeyVal {
    //                 key: "key".to_string(),
    //                 value: "val".to_string(),
    //             }],
    //         ),
    //         (
    //             "  \n key  \n=  val  \n",
    //             vec![KeyVal {
    //                 key: "key".to_string(),
    //                 value: "val".to_string(),
    //             }],
    //         ),
    //     ];

    //     for (input, expected) in cases {
    //         dbg!(&input);
    //         let result = KeyVal::parse(input).unwrap();
    //         dbg!(&result);
    //         assert_eq!(result, expected, "Failed for input: {:?}", input);
    //     }
    // }

    #[test]
    fn test_empty_values() {
        let cases = vec![
            (
                "key =",
                vec![KeyVal {
                    key: "key".to_string(),
                    value: "".to_string(),
                }],
            ),
            (
                "key =\n",
                vec![KeyVal {
                    key: "key".to_string(),
                    value: "".to_string(),
                }],
            ),
            (
                "key =  ",
                vec![KeyVal {
                    key: "key".to_string(),
                    value: "".to_string(),
                }],
            ),
            (
                "key =  \n",
                vec![KeyVal {
                    key: "key".to_string(),
                    value: "".to_string(),
                }],
            ),
        ];

        for (input, expected) in cases {
            let result = KeyVal::parse(input).unwrap();
            assert_eq!(result, expected, "Failed for input: {:?}", input);
        }
    }

    #[test]
    fn test_empty_keys() {
        let cases = vec![
            (
                "= val",
                vec![KeyVal {
                    key: "".to_string(),
                    value: "val".to_string(),
                }],
            ),
            (
                "  = val",
                vec![KeyVal {
                    key: "".to_string(),
                    value: "val".to_string(),
                }],
            ),
            (
                "\n  = val",
                vec![KeyVal {
                    key: "".to_string(),
                    value: "val".to_string(),
                }],
            ),
        ];

        for (input, expected) in cases {
            let result = KeyVal::parse(input).unwrap();
            assert_eq!(result, expected, "Failed for input: {:?}", input);
        }
    }

    #[test]
    fn test_empty_key_value() {
        let cases = vec![
            (
                "=",
                vec![KeyVal {
                    key: "".to_string(),
                    value: "".to_string(),
                }],
            ),
            (
                "  =  ",
                vec![KeyVal {
                    key: "".to_string(),
                    value: "".to_string(),
                }],
            ),
            (
                "\n  =  \n",
                vec![KeyVal {
                    key: "".to_string(),
                    value: "".to_string(),
                }],
            ),
        ];

        for (input, expected) in cases {
            let result = KeyVal::parse(input).unwrap();
            assert_eq!(result, expected, "Failed for input: {:?}", input);
        }
    }

    #[test]
    fn test_special_cases() {
        let cases = vec![
            (
                "a=b=c",
                vec![KeyVal {
                    key: "a".to_string(),
                    value: "b=c".to_string(),
                }],
            ),
            (
                "a = b = c",
                vec![KeyVal {
                    key: "a".to_string(),
                    value: "b = c".to_string(),
                }],
            ),
            (
                " =  = ",
                vec![KeyVal {
                    key: "".to_string(),
                    value: "=".to_string(),
                }],
            ),
            (
                "== Section 2 ==",
                vec![KeyVal {
                    key: "".to_string(),
                    value: "= Section 2 ==".to_string(),
                }],
            ),
            (
                "/= this is a comment",
                vec![KeyVal {
                    key: "/".to_string(),
                    value: "this is a comment".to_string(),
                }],
            ),
        ];

        for (input, expected) in cases {
            let result = KeyVal::parse(input).unwrap();
            assert_eq!(result, expected, "Failed for input: {:?}", input);
        }
    }
}
