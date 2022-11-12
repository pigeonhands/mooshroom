# The Mooshroom Book

Client folders contain `c -> s` packets

Server folders contain `s -> c` packets

Packet documentation will only contain the structure of the packets body. The outer packet structure can be found on the [Packet Structure](./packet-structure.md) page.

This is documentation on the minecraft protocal centered around rust development. Each page will have a table of the raw segments with data types, followed by a rust example.

| field_name |  type     | condition |
|------------|-----------|  |
| entity_id  |    `u16` ||
| entity_name_length| `VarInt` ||
| entity_name | `[char;entity_name_length]` string ||
| has_position | bool ||
| x | `f32` |`has_position`|
| y | `f32` |`has_position`|
| z | `f32` |`has_position`|

if `condition` is not met, the field is 0 bytes. If `condition` is not specified, it is always in the packet.


```rust,noplayground
struct EntityExample {
    pub entity_id: u16,
    pub entity_name: String,
    pub location: Option<Vec3<f32>>
}
```

These examples are simplified versions of the structs found in [mooshroom](https://github.com/pigeonhands/mooshroom), and thus make use of some of its macros.


### `#[read(fn, params...)]`
----
Uses the `fn` parameter to parse the field it is attached to, and optionally passing in some previously parsed fields. Normally used to read a field that is contitional on a bitflag.

This is usualy used with `Option<T>`, but because it is manually parsing the field, it bypasses the automatic logic of reading a `bool` before `T` specified in the [data-types](./data-types.md) unless it is used as `Option::read`.

```rust,noplayground
struct Example {
    pub packet_type: f32,
    pub bitflag: u8,
    #[read(read_conditional_field, bitflag)]
    pub if_first_bit_set: Option<VarInt>
}
fn read_conditional_field(
    reader: &mut std::io::Reader, 
    bitflag: &u8
) -> Result<Option<VarInt>> {
    if (bitflag & 1) == 0 {
        None
    }else{
        Some(VarInt::read(reader))
    }
}
```

### `#[value_type(T)] / #[repr(T)]`
----
Signifies that the numeric enum type is `T`.
so for example, in terms of protocal reading, the two packet `struct`s below are equivilent.

```rust,noplayground
#[value_type(VarInt)]
enum ExampleEnum {
    Example0 = 0,
    Example1 = 1
}

struct PacketVarInt {
    pub packet_type: VarInt
}


struct PacketEnum {
    pub packet_type: ExampleEnum
}
```
