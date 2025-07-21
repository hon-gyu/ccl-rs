use ccl_rs::key_val::KeyVal;
use ccl_rs::parser::CCL;

#[cfg(test)]
mod tests {
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

        let key_vals = KeyVal::parse(config).unwrap();
        insta::assert_debug_snapshot!(key_vals, @r#"
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
                value: "\n    enabled = true\n    ports =\n        = 8000\n        = 8001\n        = 8002\n    limits =\n        cpu = 1500mi\n        memory = 10Gb",
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

        let key_val_tree = KeyVal::parse_flat_to_tree(&key_vals);
        insta::assert_debug_snapshot!(key_val_tree, @r#"
        {
            "/": [
                Leaf(
                    "This is a CCL document",
                ),
            ],
            "database": [
                Tree(
                    {
                        "enabled": [
                            Leaf(
                                "true",
                            ),
                        ],
                        "limits": [
                            Tree(
                                {
                                    "cpu": [
                                        Leaf(
                                            "1500mi",
                                        ),
                                    ],
                                    "memory": [
                                        Leaf(
                                            "10Gb",
                                        ),
                                    ],
                                },
                            ),
                        ],
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
                    },
                ),
            ],
            "title": [
                Leaf(
                    "CCL Example",
                ),
            ],
            "user": [
                Tree(
                    {
                        "guestId": [
                            Leaf(
                                "42",
                            ),
                        ],
                    },
                ),
                Tree(
                    {
                        "createdAt": [
                            Leaf(
                                "2024-12-31",
                            ),
                        ],
                        "login": [
                            Leaf(
                                "chshersh",
                            ),
                        ],
                    },
                ),
            ],
        }
        "#);

        let ccl = CCL::parse(key_vals);
        insta::assert_debug_snapshot!(ccl, @r#"
        CCL(
            {
                "/": CCL(
                    {
                        "This is a CCL document": CCL(
                            {},
                        ),
                    },
                ),
                "database": CCL(
                    {
                        "enabled": CCL(
                            {
                                "true": CCL(
                                    {},
                                ),
                            },
                        ),
                        "limits": CCL(
                            {
                                "cpu": CCL(
                                    {
                                        "1500mi": CCL(
                                            {},
                                        ),
                                    },
                                ),
                                "memory": CCL(
                                    {
                                        "10Gb": CCL(
                                            {},
                                        ),
                                    },
                                ),
                            },
                        ),
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
                ),
                "title": CCL(
                    {
                        "CCL Example": CCL(
                            {},
                        ),
                    },
                ),
                "user": CCL(
                    {
                        "createdAt": CCL(
                            {
                                "2024-12-31": CCL(
                                    {},
                                ),
                            },
                        ),
                        "guestId": CCL(
                            {
                                "42": CCL(
                                    {},
                                ),
                            },
                        ),
                        "login": CCL(
                            {
                                "chshersh": CCL(
                                    {},
                                ),
                            },
                        ),
                    },
                ),
            },
        )
        "#);
        insta::assert_snapshot!(ccl.pretty(), @r"
        / =
          This is a CCL document =
        database =
          enabled =
            true =
          limits =
            cpu =
              1500mi =
            memory =
              10Gb =
          ports =
             =
              8000 =
              8001 =
              8002 =
        title =
          CCL Example =
        user =
          createdAt =
            2024-12-31 =
          guestId =
            42 =
          login =
            chshersh =
        ");
    }
}
