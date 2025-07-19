/// Test cases for empty inputs and error cases
/// Extracted from vendor/ccl/test/test_parser/test_empty.ml and test_error.ml

use ccl_rs::key_val::{KeyVal, KeyVals};

#[cfg(test)]
mod empty_and_error_tests {
    use super::*;

    #[test]
    fn test_empty_inputs() {
        let empty_cases = vec![
            "",
            " ",
            "   ",
            "\n",
            "  \n",
            "\n\n",
            "  \n  \n  ",
        ];

        for input in empty_cases {
            let result = KeyVal::parse(input).unwrap();
            assert_eq!(result, Vec::<KeyVal>::new(), "Failed for empty input: {:?}", input);
        }
    }

    #[test]
    fn test_parse_errors() {
        let error_cases = vec![
            "key",  // Missing equals sign and value
        ];

        for input in error_cases {
            let result = KeyVal::parse(input);
            assert!(result.is_err(), "Expected error for input: {:?}, but got: {:?}", input, result);
        }
    }
}