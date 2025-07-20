use ccl_rs::key_val::KeyVal;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error() {
        let config = "key";
        let result = KeyVal::parse(config).unwrap_err();
        insta::assert_snapshot!(result, @"No value found for key: key");
    }

    #[test]
    fn test_error_unclosed_new_key() {
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
