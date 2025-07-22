use ccl_rs::key_val::{KeyVal, KeyVals};
use ccl_rs::parser::CCL;
use proptest::prelude::*;

prop_compose! {
    fn short_str()(s in "[a-c]{0,3}") -> String {
        s
    }
}

prop_compose! {
    fn key_strat()(key in short_str()) -> String {
        key
    }
}

fn value_strat() -> impl Strategy<Value = String> {
    let leaf = short_str();

    leaf.prop_recursive(
        8,   // max depth
        256, // max size hint
        10,  // max items per collection
        |inner| {
            (key_strat(), inner)
                .prop_map(|(key, value)| format!("{} = {}", key, value))
        },
    )
}

prop_compose! {
    fn key_val_strat()(
        key in key_strat(),
        value in value_strat()
    ) -> KeyVal {
        KeyVal::new(key, value)
    }
}

prop_compose! {
    fn key_vals_strat()(
        key_vals in prop::collection::vec(key_val_strat(), 0..100)
    ) -> KeyVals {
        key_vals
    }
}

prop_compose! {
    fn ccl_strat()(key_vals in key_vals_strat()) -> CCL {
        CCL::parse(key_vals)
    }
}

proptest! {
    #[test]
    fn test_roundtrip(ccl in ccl_strat()) {
        let ccl2 = CCL::parse(KeyVal::parse(&ccl.pretty()).unwrap());
        prop_assert_eq!(ccl, ccl2);
    }
}
