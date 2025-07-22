#!/bin/bash

SIZE=${1:-large}

case $SIZE in
    small)
        file_name="tests/fixtures/sample1.ccl"
        ;;
    large)
        file_name="large_generated.ccl"
        if [ ! -f "$file_name" ]; then
            echo "Generating large test file..."
            cargo run --example generate_large_ccl
        fi
        ;;
    *)
        echo "Usage: $0 [small|medium|large]"
        echo "  small  - uses tests/fixtures/sample1.ccl"
        echo "  large  - uses large_generated.ccl (default)"
        exit 1
        ;;
esac

echo "CCL Performance Benchmark - $SIZE file"
echo "======================================"

FILE_SIZE=$(wc -c < "$file_name")
echo "Test file: $file_name"
echo "File size: $FILE_SIZE bytes"
echo ""

echo "Rust version (3 runs):"
for i in {1..3}; do
    echo -n "Run $i: "
    time ./target/release/ccl-rs --file $file_name > /dev/null
done

echo ""
echo "OCaml version (3 runs):"
for i in {1..3}; do
    echo -n "Run $i: "
    (cd vendor/ccl && time ./_build/default/bin/cclq.exe ../../$file_name -- > /dev/null)
done