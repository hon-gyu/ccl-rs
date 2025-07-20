use ccl_rs::key_val::KeyVal;

#[cfg(test)]
mod stress_tests {
    use super::*;

    #[test]
    fn test() {
        let config = r#"
/= This is a CCL document
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
    createdAt = 2024-12-31
"#;

        let result = KeyVal::parse(config).unwrap();
        insta::assert_debug_snapshot!(result, @r#"
        [
            KeyVal {
                key: "/",
                value: "This is a CCL document",
            },
            KeyVal {
                key: "title",
                value: "CCL Example",
            },
            KeyVal {
                key: "database",
                value: "\n    enabled = true\n    ports =\n    = 8000\n    = 8001\n    = 8002\n    limits =\n    cpu = 1500mi\n    memory = 10Gb",
            },
            KeyVal {
                key: "user",
                value: "\n    guestId = 42",
            },
            KeyVal {
                key: "user",
                value: "\n    login = chshersh\n    createdAt = 2024-12-31",
            },
        ]
        "#);
    }
}
