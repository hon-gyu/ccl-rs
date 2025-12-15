# ccl-rs

A Rust implementation of [Categorical Configuration Language](https://github.com/chshersh/ccl) - a minimal configuration language where configs form a monoid under concatenation, parse/pretty-print operations are monoid homomorphisms (enabling provably correct parallel parsing), and together form an isomorphism (enabling lossless round-trips).

| Property                                               | Definition                                     | In CCL Terms                                                                                                                                                     | Implication                              |
| ------------------------------------------------------ | ---------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------- | ---------------------------------------- |
| Associativity<br>(Semigroup)<br>[prop-test](https://github.com/hon-gyu/ccl-rs/blob/1994603e6574944de657b5dca9ff39c693d3572f/tests/monoid_property.rs#L58-L67)                           | `(a ⊕ b) ⊕ c = a ⊕ (b ⊕ c)`                    | `(default + user) + project = default + (user + project)`                                                                                                        | Composing multiple configs is worry-free |
| Left/Right Identity <br>(Monoid)<br>[prop-test](https://github.com/hon-gyu/ccl-rs/blob/1994603e6574944de657b5dca9ff39c693d3572f/tests/monoid_property.rs#L69-L81)                       | `empty ⊕ a = a`<br>`a ⊕ empty = a`             | - `empty + config = config`<br>- `config + empty = config`                                                                                                       | Missing configs are valid                |
| Monoid Homomorphism<br>(structure-preserving map)      | `f(a ⊕ b) = f(a) ⊗ f(b)`<br>`f(empty) = empty` | - `parse(file1 ^ file2) = parse(file1) @ parse(file2)`<br>- `parse("") = []`<br>- `pretty(list1 @ list2) = pretty(list1) ^ pretty(list2)`<br>- `pretty([]) = ""` | Parallel processing; incremental updates |
| Monoid Isomorphism<br>(two-way structure preservation)<br>[prop-test](https://github.com/hon-gyu/ccl-rs/blob/1994603e6574944de657b5dca9ff39c693d3572f/tests/monoid_isomorphism_property.rs#L54-L60) | `f ∘ g = id`<br>`g ∘ f = id`                   | - `pretty(parse(config_text)) = config_text`<br>- `parse(pretty(key_vals)) = key_vals`                                                                           | Serialization is lossless                |

## Usage

See [CLI tests](https://github.com/hon-gyu/ccl-rs/tree/main/tests/cli.rs) for examples
