use ccl_rs::key_val::KeyVal;

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("")]
    #[case(" ")]
    #[case("   ")]
    #[case("\n")]
    #[case("  \n")]
    #[case("\n\n")]
    #[case("  \n  \n  ")]
    fn test_empty(#[case] config: &str) {
        let result = KeyVal::parse(config).unwrap();
        assert_eq!(result, vec![]);
    }
}
