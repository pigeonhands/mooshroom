# 0x01 - PingRequest

Server will respond with [PingResponse](../server/ping-response.md) with the same `ping_id`.

| field_name    |  type        |
|---------------|--------------|
| ping_id       | `u64`        |


## Rust 
---------

```rust,noplayground
pub struct PingRequest(u64);
```