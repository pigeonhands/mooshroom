# 0x00 - StatusResponse

In response to [SatusRequest](../client/status-request.md).

| field_name            |  type        |
|-----------------------|------------------------|
| response_length       | `VarInt`               |
| response              | `[u8;response_length]` json string |


## Rust 
---------

```rust,noplayground
pub struct StatusResponse {
    pub response: String,
}
```

### With json deseralization
--------
```rust,noplayground
#[derive(Deserialize)]
pub struct ServerVersion {
    pub name: String,
    pub protocol: i32,
}

#[derive(Deserialize)]
pub struct ServerPlayer {
    pub name: String,
    pub id: uuid::Uuid,
}

#[derive(Deserialize)]
pub struct ServerPlayers {
    pub max: usize,
    pub online: usize,
    pub sample: Vec<ServerPlayers>,
}

#[derive(Deserialize)]
pub struct StatusBody {
    pub version: ServerVersion,
    pub favicon: String,
    #[serde(rename = "previewsChat")]
    pub previews_chat: Option<bool>,
    #[serde(rename = "enforcesSecureChat")]
    pub enforces_secure_chat: Option<bool>,
}

pub struct StatusResponse {
    pub response: Json<StatusBody>,
}
```