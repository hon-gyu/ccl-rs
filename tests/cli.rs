use assert_cmd::Command;
use predicates::prelude::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_help() {
        let mut cmd = Command::cargo_bin("ccl-rs").unwrap();

        cmd.arg("--help")
            .assert()
            .success()
            .stdout(predicate::str::contains("Merge CCL files and query"));
    }
    #[test]
    fn test_load() {
        let mut cmd = Command::cargo_bin("ccl-rs").unwrap();

        let output = cmd
            .arg("--file")
            .arg("tests/fixtures/sample1.ccl")
            .output()
            .unwrap();

        let stdout = String::from_utf8(output.stdout).unwrap();

        insta::assert_snapshot!(stdout, @r"
        numbers =
          bar =
            19023135 =
          baz =
            12905843 =
          foo =
            12341234 =
        ");
    }

    #[test]
    fn test_merge() {
        let mut cmd = Command::cargo_bin("ccl-rs").unwrap();

        let output = cmd
            .arg("--file")
            .arg("tests/fixtures/sample1.ccl")
            .arg("tests/fixtures/sample2.ccl")
            .output()
            .unwrap();

        let stdout = String::from_utf8(output.stdout).unwrap();

        insta::assert_snapshot!(stdout, @r"
        numbers =
          bar =
            19023135 =
          baz =
            123 =
            12905843 =
          foo =
            1 =
            12341234 =
        somekey =
          someval =
        this =
          bar =
            baz =
          foo =
          that =
        ");
    }

    #[test]
    fn test_query_single() {
        let mut cmd = Command::cargo_bin("ccl-rs").unwrap();

        let output = cmd
            .arg("--file")
            .arg("tests/fixtures/sample1.ccl")
            .arg("tests/fixtures/sample2.ccl")
            .arg("--query")
            .arg("numbers")
            .output()
            .unwrap();

        let stdout = String::from_utf8(output.stdout).unwrap();

        insta::assert_snapshot!(stdout, @r"
        bar =
          19023135 =
        baz =
          123 =
          12905843 =
        foo =
          1 =
          12341234 =
        ");
    }

    #[test]
    fn test_query_single_nested() {
        let mut cmd = Command::cargo_bin("ccl-rs").unwrap();

        let output = cmd
            .arg("--file")
            .arg("tests/fixtures/sample1.ccl")
            .arg("tests/fixtures/sample2.ccl")
            .arg("--query")
            .arg("numbers=foo")
            .output()
            .unwrap();

        let stdout = String::from_utf8(output.stdout).unwrap();

        insta::assert_snapshot!(stdout, @r"
        1 =
        12341234 =
        ");
    }

    #[test]
    fn test_query_multiple() {
        let mut cmd = Command::cargo_bin("ccl-rs").unwrap();

        let output = cmd
            .arg("--file")
            .arg("tests/fixtures/sample1.ccl")
            .arg("tests/fixtures/sample2.ccl")
            .arg("--query")
            .arg("numbers=foo")
            .arg("somekey")
            .arg("this=bar")
            .output()
            .unwrap();

        let stdout = String::from_utf8(output.stdout).unwrap();

        insta::assert_snapshot!(stdout, @r"
        1 =
        12341234 =

        someval =

        baz =
        ");
    }
}
