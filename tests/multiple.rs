/// Test cases for multiple key-value pair parsing
/// Extracted from vendor/ccl/test/test_parser/test_multiple.ml
use ccl_rs::key_val::KeyVal;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_two_key_value_pairs() {
        let input = "key1 = val1\nkey2 = val2";
        let expected = vec![
            KeyVal {
                key: "key1".to_string(),
                value: "val1".to_string(),
            },
            KeyVal {
                key: "key2".to_string(),
                value: "val2".to_string(),
            },
        ];

        let result = KeyVal::parse(input).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_config_with_trailing_and_leading_whitespace() {
        let input = "\nkey1 = val1\nkey2 = val2\n";
        let expected = vec![
            KeyVal {
                key: "key1".to_string(),
                value: "val1".to_string(),
            },
            KeyVal {
                key: "key2".to_string(),
                value: "val2".to_string(),
            },
        ];

        let result = KeyVal::parse(input).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_real_life_like_config() {
        let input = "\nname = Dmitrii Kovanikov\nlogin = chshersh\nlanguage = OCaml\ndate = 2024-05-25\n";
        let expected = vec![
            KeyVal {
                key: "name".to_string(),
                value: "Dmitrii Kovanikov".to_string(),
            },
            KeyVal {
                key: "login".to_string(),
                value: "chshersh".to_string(),
            },
            KeyVal {
                key: "language".to_string(),
                value: "OCaml".to_string(),
            },
            KeyVal {
                key: "date".to_string(),
                value: "2024-05-25".to_string(),
            },
        ];

        let result = KeyVal::parse(input).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_list_like_config() {
        let input = "\n= 3\n= 1\n= 2\n";
        let expected = vec![
            KeyVal {
                key: "".to_string(),
                value: "3".to_string(),
            },
            KeyVal {
                key: "".to_string(),
                value: "1".to_string(),
            },
            KeyVal {
                key: "".to_string(),
                value: "2".to_string(),
            },
        ];

        let result = KeyVal::parse(input).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_array_like_config() {
        let input = "\n1 =\n2 =\n3 =\n";
        let expected = vec![
            KeyVal {
                key: "1".to_string(),
                value: "".to_string(),
            },
            KeyVal {
                key: "2".to_string(),
                value: "".to_string(),
            },
            KeyVal {
                key: "3".to_string(),
                value: "".to_string(),
            },
        ];

        let result = KeyVal::parse(input).unwrap();
        assert_eq!(result, expected);
    }
}
