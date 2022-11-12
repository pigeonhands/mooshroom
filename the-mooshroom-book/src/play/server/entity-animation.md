# 0x03 - EntityAnimation

| field_name                                   | type     |
| -------------------------------------------- | -------- |
| entity_id                                    | `VarInt` |
| [animation](./entity-animation.md#animation) | `u8`     |


## Fields
-----

### animation

| value | name               |
| ----- | ------------------ |
| 0     | SwingMainArm       |
| 1     | TakeDamage         |
| 2     | LeaveBed           |
| 3     | SwingOffhand       |
| 4     | CriticalEffect     |
| 5     | MagicCriticalEfect |

## Rust 
---------

```rust,noplayground
#[repr(u8)]
pub enum Animation {
    SwingMainArm = 0,
    TakeDamage = 1,
    LeaveBed = 2,
    SwingOffhand = 3,
    CriticalEffect=4,
    MagicCriticalEfect = 5
}

pub struct EntityAnimation {
    pub entity_id: VarInt,
    pub animation: Animation,
}
```

