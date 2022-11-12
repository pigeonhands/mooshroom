# 0x01 - EncryptionResponse

if `has_verify_token` then verify token is supplied, otherwise salt and signature data is supplied.

| field_name | type | condition |
| ---------- | ---- ||
| shared_secret_length | `VarInt`                    ||
| shared_secret_length | `[u8;shared_secret_length]` ||
| has_verify_token     | `bool`                      ||
| verify_token_length  | `VarInt`                    | `has_verify_token`  |
| verify_token         | `[u8;verify_token_length]`  | `has_verify_token`  |
| salt                 | `i64`                       | `!has_verify_token` |
| signature_length     | `VarInt`                    | `!has_verify_token` |
| signature            | `[u8;signature_length]`     | `!has_verify_token` |



## Rust 
---------

```rust,noplayground
pub struct SignatureInfo {
    pub salt: i64,
    pub signature: Vec<u8>,
}

pub struct EncryptionResponse {
    pub shared_secret: Vec<u8>,
    pub has_verify_token: bool,
    #[read(read_verify_token, has_verify_token)]
    pub verify_token: Option<Vec<u8>>,
    #[read(read_signature, has_verify_token)]
    pub signature: Option<SignatureInfo>,
}

fn read_verify_token<const PV: usize>(
    reader: &mut impl std::io::Read,
    has_verify_token: &bool,
) -> Result<Option<Vec<u8>>> {
    if *has_verify_token {
        Ok(Some(Vec::read(reader)?))
    } else {
        Ok(None)
    }
}
fn read_signature<const PV: usize>(
    reader: &mut impl std::io::Read,
    has_verify_token: &bool,
) -> Result<Option<SignatureInfo>> {
    let r = if *has_verify_token {
        None
    } else {
        Some(SignatureInfo {
            salt: i64::read(reader)?,
            signature: Vec::read(reader)?,
        })
    };
    Ok(r)
}
```