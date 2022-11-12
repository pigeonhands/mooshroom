# 0x02 - SpawnPlayer

| field_name  | type                                              |
| ----------- | ------------------------------------------------- |
| entity_id   | `VarInt`                                          |
| player_uuid | `[u8;16]`                                         |
| position    | `[u8;8]` [position](../../data-types.md#position) |
| pitch       | `u8` [angle](../../data-types.md#angle)           |
| yaw         | `u8` [angle](../../data-types.md#angle)           |

## Rust 
---------

```rust,noplayground
pub struct SpawnPlayer {
    pub entity_id: VarInt,
    pub player_uuid: uuid::Uuid,
    pub position: Position,
    pub yaw: Angle,
    pub pitch: Angle,
}
```