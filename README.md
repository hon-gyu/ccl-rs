# ccl-rs

A Rust implementation of [Categorical Configuration Language](https://github.com/chshersh/ccl) (OCaml).

## Performance

Benchmark comparison with the original OCaml implementation:

### Small file (61 bytes)
- Rust: ~3-4ms
- OCaml: ~4ms

### Large file (346KB)
- Rust: ~32-44ms
- OCaml: ~59-63ms