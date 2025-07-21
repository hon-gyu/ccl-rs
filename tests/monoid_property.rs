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
    fn test_associativity(
        x in key_vals_strategy(),
        y in key_vals_strategy(),
        z in key_vals_strategy()
    ) {
        let left = x.clone().merge(y.clone()).merge(z.clone());
        let right = x.merge(y.merge(z));
        prop_assert_eq!(left, right);
    }

    #[test]
    fn test_left_empty(x in key_vals_strategy()) {
        let empty = KeyVals::empty();
        let result = empty.merge(x.clone());
        prop_assert_eq!(x, result);
    }

    #[test]
    fn test_right_empty(x in key_vals_strategy()) {
        let empty = KeyVals::empty();
        let result = x.clone().merge(empty);
        prop_assert_eq!(x, result);
    }

}
