# 0x02 - LoginSuccess

State is changed to [Play](../../play.md) after this packet.

| field_name       | type                                                           |
| ---------------- | -------------------------------------------------------------- |
| uuid_length      | `VarInt`                                                       |
| uuid             | `[u8;16]`                                                      |
| username_length  | `VarInt`                                                       |
| username         | `[u8;signature_length]` string                                 |
| properties_count | `VarInt`                                                       |
| properties       | `[`[property](./login-success.md#property)`;properties_count]` |

### Property
| field_name       | type                            | condition       |
| ---------------- | ------------------------------- | --------------- |
| name_length      | `VarInt`                        |
| name             | `[u8;name_length]`  string      |
| value_length     | `VarInt`                        |
| value            | `[u8;name_length]`  string      |
| has_signature    | `bool`                          |
| signature_length | `VarInt`                        | `has_signature` |
| signature        | `[u8;signature_length]`  string | `has_signature` |

## Rust 
---------

```rust,noplayground
pub struct Property {
    pub name: String,
    pub value: String,
    pub signature: Option<String>,
}

pub struct LoginSuccess {
    pub uuid: uuid::Uuid,
    pub username: String,
    pub properties: Vec<Properties>,
}
```