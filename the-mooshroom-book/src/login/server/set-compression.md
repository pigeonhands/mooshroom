# 0x03 - SetCompression

All packets after recieving this will be in [compressed packet format](../../packet-structure.md#compressed).

Data should only be compressed if the body of the packet is greater than `threshold`.

| field_name | type     |
| ---------- | -------- |
| threshold  | `VarInt` |

## Rust 
---------

```rust,noplayground
pub struct SetCompression {
    pub threshold: VarInt,
}
```