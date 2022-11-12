# 0x01 - PingResponse

In response to [PingResponse](../client/ping-request.md).

`ping_id` will be the same as `ping_id` in the request.

| field_name    |  type        |
|---------------|--------------|
| ping_id       | `u64`        |


## Rust 
---------

```rust,noplayground
pub struct PingResponse(u64);
```