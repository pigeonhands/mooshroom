# 0x00 - SpawnEntity

| field_name  | type                                              |
| ----------- | ------------------------------------------------- |
| entity_id   | `VarInt`                                          |
| entity_uuid | `[u8;16]`                                         |
| entity_type | `VarInt`                                          |
| position    | `[u8;8]` [position](../../data-types.md#position) |
| pitch       | `u8` [angle](../../data-types.md#angle)           |
| yaw         | `u8` [angle](../../data-types.md#angle)           |
| data        | `VarInt`                                         |
| velocity_x  | `i16`                                             |
| velocity_y  | `i16`                                             |
| velocity_z  | `i16`                                             |

## Rust 
---------

```rust,noplayground
pub struct SpawnEntity {
    pub entity_id: VarInt,
    pub entity_uuid: uuid::Uuid,
    pub entity_type: VarInt,
    pub position: Position,
    pub pitch: Angle,
    pub yaw: Angle,
    pub data: VarInt,
    pub velocity: Vec3<i16>,
}
```