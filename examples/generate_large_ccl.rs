use ccl_rs::key_val::{KeyVal, KeyVals};
use ccl_rs::parser::CCL;
use proptest::prelude::*;
use proptest::strategy::ValueTree;
use proptest::test_runner::TestRunner;
use std::fs::File;
use std::io::Write;

prop_compose! {
    fn long_str()(s in "[a-zA-Z0-9_]{10,50}") -> String {
        s
    }
}

prop_compose! {
    fn key_strat()(key in long_str()) -> String {
        key
    }
}

fn value_strat() -> impl Strategy<Value = String> {
    let leaf = long_str();

    leaf.prop_recursive(30, 10000, 50, |inner| {
        (key_strat(), inner)
            .prop_map(|(key, value)| format!("{} = {}", key, value))
    })
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
    fn large_key_vals_strat()(
        key_vals in prop::collection::vec(key_val_strat(), 100..5000)
    ) -> KeyVals {
        key_vals
    }
}

prop_compose! {
    fn large_ccl_strat()(key_vals in large_key_vals_strat()) -> CCL {
        CCL::parse(key_vals)
    }
}

fn main() -> std::io::Result<()> {
    let mut runner = TestRunner::default();

    // Generate a large CCL structure
    let tree = large_ccl_strat().new_tree(&mut runner).map_err(|e| {
        std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Strategy error: {}", e),
        )
    })?;
    let ccl = tree.current();

    // Convert to string representation
    let ccl_content = ccl.pretty();

    // Write to file
    let mut file = File::create("large_generated.ccl")?;
    file.write_all(ccl_content.as_bytes())?;

    println!("Generated large CCL file: large_generated.ccl");
    println!("File size: {} bytes", ccl_content.len());

    Ok(())
}
