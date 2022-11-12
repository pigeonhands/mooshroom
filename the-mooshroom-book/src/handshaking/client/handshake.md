# 0x00 - Handshake
-------

Switches the server state to state specified in [next_state](./handshake.md#next_state).

| field_name                              | type                                |
| --------------------------------------- | ----------------------------------- |
| protocol_version                        | `u16`                               |
| server_address_length                   | `VarInt`                            |
| server_address                          | `[u8;server_address_length]` string |
| server_port                             | `u16`                               |
| [next_state](./handshake.md#next_state) | `VarInt`                            |

## Fields
-----

### next_state
| value | name   |
| ----- | ------ |
| 1     | Status |
| 2     | Login  |


`Status` will switch to [`Status`](/status.md) state.

`Login` will switch to [`Login`](/login.md) state.

## Rust 
---------

```rust,noplayground
#[value_type(VarInt)]
pub enum HandshakeState {
    Status = 1,
    Login = 2,
}

pub struct Handshake {
    pub protocol_version: VarInt,
    pub server_address: String,
    pub server_port: u16,
    pub next_state: HandshakeState,
}
```



