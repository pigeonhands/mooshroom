# 0x00 - Disconnect

The client should disconnect after recieving this packet.

| field_name    | type                               |
| ------------- | ---------------------------------- |
| reason_length | `VarInt`                           |
| reason        | `[u8;response_length]` chat string |

## Rust 
---------

```rust,noplayground
pub struct Disconnect(Chat);
```