# 0x00 - LoginStart

Setting `has_signature_data` to false (and thus omitting dependant fields) will perform an offline login, only supported by offline servers. 

Server will **not** send an [EncryptionRequest](../server/encryption-request.md) if `has_signature_data` is `false`.


| field_name         | type                      | condition            |
| ------------------ | ------------------------- | -------------------- |
| name_length        | `varInt`                  |                      |
| name               | `[u8;name_length]` string |                      |
| has_signature_data | `bool`                    |                      |
| sig_timestamp      | `u64`                     | `has_signature_data` |
| public_key_length  | `VarInt`                  | `has_signature_data` |
| public_key         | `[u8;public_key_length]`  | `has_signature_data` |
| signature_length   | `VarInt`                  | `has_signature_data` |
| signature          | `[u8;signature_length]`   |                      |
| has_player_uuid    | `bool`                    |
| player_uuid        | `[8;16]`                  | `has_player_uuid`    |


## Rust 
---------

```rust,noplayground
pub struct SignatureData {
    pub timestamp: u64,
    pub public_key: Vec<u8>,
    pub signature: Vec<u8>,
}

pub struct LoginStart {
    pub name: String,
    pub sig_data: Option<SignatureData>,
    pub player_uuid: Option<uuid::Uuid>,
}
```