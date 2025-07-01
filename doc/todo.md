# CCL-RS Implementation TODO

## Model Implementation (src/model.rs)

### Core Data Structures
- [x] `ValueEntry` enum (String/Nested variants)
- [x] `Ccl` struct with HashMap<String, Vec<ValueEntry>>

### Basic Functions
- [x] `empty()` - create empty CCL
- [x] `key_val(key, value)` - create CCL with single key-value pair
- [x] `merge(self, other)` - merge two CCLs, combining values for same keys

### Missing Core Functions
- [ ] `key(k)` - create CCL with key mapping to empty CCL (commented out)
- [ ] `nested(key, values)` - create CCL with key mapping to merged list of CCLs
- [ ] `of_list(ccls)` - merge list of CCLs into one
- [ ] `compare()` - comparison function for CCLs

### Advanced Functions
- [ ] `fix_entry_map()` - normalize entry map structure
- [ ] `add_key_val()` - add key-value pair to existing map
- [ ] `of_key_vals()` - convert parsed key-values to CCL
- [ ] `fix()` - main processing function

### Utility Functions
- [x] `pretty()` - debug formatting (basic version)
- [ ] Improve `pretty()` to match OCaml indented output format

### eDSL (Embedded Domain Specific Language)
- [ ] `=:` operator (alias for key_val)
- [ ] Builder pattern methods for ergonomic CCL construction

### Testing
- [x] Basic tests for empty, key_val, merge
- [ ] Tests for key, nested, of_list functions
- [ ] Tests for complex nested structures
- [ ] Property-based tests for merge associativity

## Parser Implementation (src/parser.rs)

### Core Parsing
- [ ] Define `KeyVal` struct (key-value pair from parsing)
- [ ] `parse_value()` - parse string value, detect if nested config
- [ ] Main parser for CCL syntax
- [ ] Error handling and recovery

### Integration
- [ ] Connect parser output to model's `of_key_vals()`
- [ ] Handle parsing errors gracefully

## CLI/Library Interface
- [ ] Public API design
- [ ] Configuration file loading
- [ ] Command-line interface (if needed)

## Documentation & Examples
- [ ] API documentation
- [ ] Usage examples
- [ ] Comparison with OCaml version