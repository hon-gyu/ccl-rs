---
AI: true
---
# CCL Parser Logic

## Sequence Diagram

```mermaid
sequenceDiagram
    participant User
    participant parse
    participant kvs_p
    participant key_val
    participant value_p
    participant nested_kvs_p

    User->>parse: "key1=value1\n  nested=nested_value"
    parse->>kvs_p: parse root level
    kvs_p->>key_val: parse first key-value (prefix_len=0)
    
    key_val->>key_val: extract key "key1"
    key_val->>value_p: parse value (prefix_len=0)
    
    value_p->>value_p: read "value1"
    value_p->>value_p: peek next char '\n'
    value_p->>value_p: count spaces (2 spaces)
    value_p->>value_p: 2 > 0 (expected_prefix_len), continue
    value_p->>value_p: read "  nested=nested_value"
    value_p-->>key_val: return complete value
    
    key_val-->>kvs_p: return {key: "key1", value: "value1\n  nested=nested_value"}
    kvs_p-->>parse: return key-value pairs
    parse-->>User: Result with parsed data

    Note over User,nested_kvs_p: For nested parsing only
    User->>parse_value: "  nested=nested_value"
    parse_value->>nested_kvs_p: parse nested content
    nested_kvs_p->>nested_kvs_p: peek char, find '\n'
    nested_kvs_p->>nested_kvs_p: count spaces (2)
    nested_kvs_p->>key_val: parse with prefix_len=2
    key_val-->>nested_kvs_p: return nested key-value
    nested_kvs_p-->>parse_value: return nested pairs
    parse_value-->>User: Result with nested data
```

## Key Decision Points

1. **Indentation Detection**: `value_p` counts spaces after newlines to determine if continuation
2. **Prefix Comparison**: If `spaces_len <= expected_prefix_len`, stop parsing (end of value)
3. **Nested Parsing**: `nested_kvs_p` calculates indentation level for nested key-value pairs
4. **Recursive Value Parsing**: `value_p` uses `fix` for self-referential parsing of multi-line values