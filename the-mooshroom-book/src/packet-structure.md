# Packet Structure
---------

The packet structure is depentant on if the server has set compression via the [SetCompression](./login/server/set-compression.md) packet.

## Uncompressed
---------

| field_name    | type                 |
| ------------- | -------------------- |
| packet_length | `VarInt`             |
| packet_id     | `VarInt`             |
| packet        | `[u8;packet_length]` |

`packet_length` is the length of `packet_id` + `packet`.

```rust,noplayground
pub enum StatePackets {
    // Avalible packets in current state
}
fn read_next_packet(reader: &mut impl std::io::Read) -> Result<StatePackets> {
    let mut body = {
        let length = VarInt::read(reader)?;
        let mut buffer = vec![0;length.0 as usize];
        reader.read_exact(&mut buffer)?;
        std::io::Cursor::new(buffer)
    };

    let packet_id = VarInt::read(&mut body)?;
    StatePackets::read_packet(packet_id, &mut body)
}
```

## Compressed
---------

`data` is compressed with `zlib` if `decompressed_length > 0`.

| field_name                  | type                 |
| --------------------------- | -------------------- |
| packet_length               | `VarInt`             |
| decompressed_length         | `VarInt`             |
| data (`packet_id` and `packet`) | `[u8;packet_length - sizeof(decompressed_length)]` |

`packet_length` is the length of `sizeof(decompressed_length)` + `data`.


```rust,noplayground
pub enum StatePackets {
    // Avalible packets in current state
}
fn read_next_packet(reader: &mut impl std::io::Read) -> Result<StatePackets> {
    let mut body = {
        let length = VarInt::read(reader)?;
        let mut buffer = vec![0;length.0 as usize];
        reader.read_exact(&mut buffer)?;
        std::io::Cursor::new(buffer)
    };
    let decompressed_length = VarInt::read(&mut body);

    let decompressed_body = if decompressed_length > 0 {
        let mut decompress_buffer = vec![0;decompressed_length.0 as usize];
        decompress_with_zlib(&mut body, &mut decompress_buffer)?;
        std::io::Cursor::new(decompress_buffer)
    } else{
        body
    };

    let packet_id = VarInt::read(reader)?;
    StatePackets::read_packet(packet_id, &mut body)
}
```