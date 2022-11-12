# 0x01 - EncryptionRequest

| field_name            |  type        |
|-----------------------|------------------------|
| server_id_length      | `VarInt`               |
| server_id              | `[u8;server_id_length]` string |
| public_key_length      | `VarInt`               |
| public_key              | `[u8;server_id_length]`  |
| verify_token_length      | `VarInt`               |
| verify_token              | `[u8;server_id_length]`  |

## Rust 
---------

```rust,noplayground
pub struct EncryptionRequest {
    pub server_id: String,
    pub public_key: Vec<u8>,
    pub verify_token: Vec<u8>,
}
```