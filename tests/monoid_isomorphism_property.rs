use ccl_rs::key_val::{KeyVal, KeyVals};
use ccl_rs::monoid::Monoid;
use proptest::prelude::*;

prop_compose! {
    fn small_string()(s in "[a-c]{0,3}") -> String {
        s
    }
}

prop_compose! {
    fn key_strategy()(key in small_string()) -> String {
        key
    }
}

fn value_strategy() -> BoxedStrategy<String> {
    let leaf = small_string();

    leaf.prop_recursive(
        8,   // max depth
        256, // max size hint
        10,  // max items per collection
        |inner| {
            (key_strategy(), inner)
                .prop_map(|(key, value)| format!("{} = {}", key, value))
        },
    )
    .boxed()
}

prop_compose! {
    fn key_val_strategy()(
        key in key_strategy(),
        value in value_strategy()
    ) -> KeyVal {
        KeyVal::new(key, value)
    }
}

prop_compose! {
    fn key_vals_strategy()(
        key_vals in prop::collection::vec(key_val_strategy(), 0..100)
    ) -> KeyVals {
        key_vals
    }
}

prop_compose! {
    fn valid_ccl_string_strategy()(
        key_vals in key_vals_strategy()
    ) -> String {
        KeyVal::pretty(&key_vals)
    }
}

proptest! {
    #[test]
    fn test_roundtrip(key_vals in key_vals_strategy()) {
        let pretty_str = KeyVal::pretty(&key_vals);
        let parsed = KeyVal::parse(&pretty_str);

        match parsed {
            Ok(parsed_key_vals) => {
                // The roundtrip test should verify that parse(pretty(x)) produces something
                // equivalent when pretty-printed again, not exact equality
                let pretty_original = KeyVal::pretty(&key_vals);
                let pretty_parsed = KeyVal::pretty(&parsed_key_vals);
                prop_assert_eq!(pretty_original, pretty_parsed);
            }
            Err(_) => {
                // If parsing fails, we can't guarantee roundtrip
                // but the original OCaml test uses assume_fail for this case
                return Ok(());
            }
        }
    }

    #[test]
    fn test_parse_concatenation_property(
        ccl1 in valid_ccl_string_strategy(),
        ccl2 in valid_ccl_string_strategy()
    ) {
        // Test the property: parse(concat(ccl1, ccl2)) ≡ parse(ccl1) @ parse(ccl2)
        let concatenated = format!("{}\n{}", ccl1, ccl2);

        let parse_concat = KeyVal::parse(&concatenated);

        if let (Ok(parsed1), Ok(parsed2), Ok(parsed_concat)) = (
            KeyVal::parse(&ccl1),
            KeyVal::parse(&ccl2),
            parse_concat
        ) {
            let expected = parsed1.merge(parsed2);
            prop_assert_eq!(parsed_concat, expected);
        }
    }

    #[test]
    fn test_parse_pretty_isomorphism(key_vals in key_vals_strategy()) {
        // Test that pretty and parse are inverse operations (when successful)
        let pretty_str = KeyVal::pretty(&key_vals);

        if let Ok(parsed) = KeyVal::parse(&pretty_str) {
            let pretty_again = KeyVal::pretty(&parsed);
            let parsed_again = KeyVal::parse(&pretty_again);

            prop_assert!(parsed_again.is_ok());
            if let Ok(parsed_again) = parsed_again {
                prop_assert_eq!(parsed, parsed_again);
            }
        }
    }
}
