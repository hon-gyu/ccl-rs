# ccl-rs

A Rust port of [Categorical Configuration Language](https://github.com/chshersh/ccl) (OCaml).

## Performance

This is a toy comparison on my machine.

### Small file (61 bytes)
- Rust: ~3-4ms
- OCaml: ~4ms

### Large file (346KB)
- Rust: ~32-44ms
- OCaml: ~59-63ms