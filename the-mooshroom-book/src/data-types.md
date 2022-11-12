## Data Types

These will be used to construct packets

## Primitives
-------
The primitive data types are all read using big-endian byte format.

* bool (1 byte)
* i8
* u8
* i16
* u16
* i32
* u32
* i64
* u64
* f32
* f64


# Complex Types
Complex types are used used in the `rust` examples of packets.

* [`VarInt` / `VarLong`](./data-types.md#varint--varlong)
* [`Option<T>`](./data-types.md#optiont)
* [`Vec<T>`](./data-types.md#vect)
* [`[T;N]` array](./data-types.md#tn-array)
* [`String`](./data-types.md#string)
* [`Identifier`](./data-types.md#identifier)
* [`Chat`](./data-types.md#chat)
* [`Json<T>`](./data-types.md#jsont)
* [`Vec3<T>`](./data-types.md#vec3t)
* [`Position`](./data-types.md#position)
* [`Uuid`](./data-types.md#uuid)
* [`Angle`](./data-types.md#angle)


### `VarInt` / `VarLong`
--------
A variable length signed integer read from LSB to MSB. The most significant bit of each byte indicates if there is another byte in the `VarInt`.

| type    | max bytes |
| ------- | --------- |
| VarInt  | 5         |
| VarLong | 10        |

For example, encoding the integer `12345`

| format | data             |
| ------ | ---------------- |
| Hex    | 0x3039           |
| Binary | 0000110000111001 |
| VarInt | 1011100100011000 |

```        
           LSB         MSB
            |           |
VarInt [1 0111001] [0 0011000]
        ^           ^
        |           |
        |     Indicates this is the last byte
 Indicates there is another byte
```

```rust
// least siginificant bits of each part with the indicator discarded
let lsb = 0b_0111001;
let msb = 0b_0011000;
println!("0x{:x} == 0x{:x}", (msb << 7) | lsb, 0b0000110000111001)
```


<details>
<summary> reading </summary>

```rust,no_run
fn read(reader: &mut impl std::io::Read) -> Result<Self> {
    let mut num_read = 0;
    let mut result = 0;
    loop {
        let read = reader.read_u8()?;
        let value = i32::from(read & 0b0111_1111);
        result |= value.overflowing_shl(7 * num_read).0;
        num_read += 1;
        if read & 0b1000_0000 == 0 {
            break;
        }
        if num_read > 5 {
            return Err(MoshroomError::VarIntTooLong);
        }
    }
    Ok(Self(result))
}
```
</details>


<details>
<summary> writing </summary>

```rust
fn write(&self, writer: &mut impl io::Write) -> Result<()> {
    let mut x = self.0 as u64;
    // Ensure at least one byte is always written
    writer
        .write_u8(((x & 0b0111_1111) | (0b1000_0000 * ((x >> 7 != 0) as u64))) as u8)
        .map_err(MoshroomError::IoError)?;
    x >>= 7;
    loop {
        if x == 0 {
            break;
        }
        let mut temp = (x & 0b0111_1111) as u8;
        x >>= 7;
        if x != 0 {
            temp |= 0b1000_0000;
        }
        writer.write_u8(temp).map_err(MoshroomError::IoError)?;
    }
    Ok(())
}
```

</details>

[mooshroom implimentation](https://github.com/pigeonhands/mooshroom/blob/master/mooshroom-core/src/varint.rs)


### `Option<T>`
--------

A `bool`, and an optional `T` if `bool` is true.

| exists | value          |
| ------ | -------------- |
| `bool` | `T` or nothing |


<details>
<summary> reading </summary>

```rust,no_run
fn read(reader: &mut impl std::io::Read) -> Result<Self> {
    if bool::read(reader)? {
        Ok(Some(T::read(reader)?))
    } else {
        Ok(None)
    }
}
```
</details>


<details>
<summary> writing </summary>

```rust,no_run
fn write(reader: &mut impl std::io::Read) -> Result<Self> {
    if let Some(t) = &self {
        true.write(writer)?;
        t.write(writer)?;
    } else {
        false.write(writer)?;
    }
    Ok(())
}
```
</details>



### `Vec<T>`
--------

A variable length array of `T`.

| length                                    | 0..length |
| ----------------------------------------- | --------- |
| [varint](./data-types.md#varint--varlong) | `T`       |


<details>
<summary> reading </summary>

```rust,no_run
fn read(reader: &mut impl std::io::Read) -> Result<Self> {
    let len = VarInt::read(reader)?.0 as usize;

    let mut buffer = Vec::with_capacity(len);
    for _ in 0..len {
        buffer.push(T::read(reader)?);
    }
    Ok(buffer)
}
```
</details>


<details>
<summary> writing </summary>

```rust,no_run
fn write(reader: &mut impl std::io::Read) -> Result<Self> {
    VarInt(self.len() as i32).write(writer)?;
    writer.write_all(self.as_bytes())?;
    Ok(())
}
```
</details>


### `[T;N]` array
--------

A fxed length array of `T`.

| 0..N |
| ---- |
| `T`  |


<details>
<summary> reading </summary>
Uses `MaybeUninit` so `T` dosent need to impliment `Default`.

```rust,no_run
fn read(reader: &mut impl std::io::Read) -> Result<Self> {
    unsafe {
        let mut buffer: [MaybeUninit<T>; N] = MaybeUninit::uninit().assume_init();
        for i in buffer.iter_mut() {
            i.write(T::read(reader)?);
        }
        Ok(buffer.as_ptr().cast::<[T; N]>().read())
    }
}
```
</details>


<details>
<summary> writing </summary>

```rust,no_run
fn write(reader: &mut impl std::io::Read) -> Result<Self> {
    for i in self.iter() {
        i.write(writer)?;
    }
    Ok(())
}
```
</details>



### `String`
 -----------
A variable-length unicode string

| length                                    | string-body |
| ----------------------------------------- | ----------- |
| [varint](./data-types.md#varint--varlong) | variable    |


<details>
<summary> reading </summary>

```rust,no_run
fn read(reader: &mut impl std::io::Read) -> Result<Self> {
    let len = VarInt::read(reader)?;

    let s = {
        let mut buffer: Vec<u8> = vec![0; len.0 as usize];
        reader.read_exact(&mut buffer)?;
        String::from_utf8(buffer).map_err(MoshroomError::InvalidString)?
    };
    Ok(s)
}
```
</details>


<details>
<summary> writing </summary>

```rust,no_run
fn write(reader: &mut impl std::io::Read) -> Result<Self> {
    VarInt(self.len() as i32).write(writer)?;
    writer.write_all(self.as_bytes())?;
    Ok(())
}
```
</details>

### `Identifier`
-----------

A [string](./data-types.md#string) in `namespace:value` format. e.g. `minecraft:ambient.cave`.

An identifier is limited to a limited character set, and if no namespace is contained, `minecraft` is used. 

e.g. if `ambient.cave` is recieved, it is atcualy `minecraft:ambient.cave`.

| segment   | regex          |
| --------- | -------------- |
| namespace | `[a-z0-9.-_]`  |
| value     | `[a-z0-9.-_/]` |

### `Chat`
-----------

A [string](./data-types.md#string) in rich text format.

### `Json<T>`
-----------

A [string](./data-types.md#string) deseralized as `json` into `T`.

### `Vec3<T>`
---------

Vector of dimention 3

```rust,noplayground
struct Vec3<T> {
    pub x: T,
    pub y: T,
    pub z: T
}
```

### `Position`
-----------
 A packed `xzy` used for position in some packets.

| x   | z   | y   |
| --- | --- | --- |
| i26 | i26 | i12 |

```rust,noplayground
pub struct Position {
    pub x: i32, //i26
    pub z: i32, //i26
    pub y: i16, //i12
}
```

<details>
<summary> reading </summary>

```rust,noplayground
fn read(reader: &mut impl std::io::Read) -> Result<Self> {
    let base = u64::read(reader)?;
    Ok(Self {
        x: (base >> 38) as i32 & I26_MASK,
        z: (base >> 12) as i32 & I26_MASK,
        y: base as i16 & I12_MASK,
    })
}
```
</details>


<details>
<summary> writing </summary>

```rust,no_run
fn write(reader: &mut impl std::io::Read) -> Result<Self> {
    let mut buffer = [0; 8];
    buffer[..26].copy_from_slice(&(self.x & I26_MASK).to_be_bytes()[6..]);
    buffer[26..26 + 26].copy_from_slice(&(self.z & I26_MASK).to_be_bytes()[6..]);
    buffer[26 + 26..].copy_from_slice(&(self.y & I12_MASK).to_be_bytes()[4..]);
    writer.write_all(&buffer)?;
    Ok(())
}
```
</details>


### `Uuid`
---------

A 16 byte uuid. This can be represented by the [uuid](https://github.com/uuid-rs/uuid) crate.

![crates.io](https://img.shields.io/crates/v/uuid.svg)

| 0..16   |
| ------- |
| [16;u8] |


<details>
<summary> reading </summary>

```rust,noplayground
fn read(reader: &mut impl std::io::Read) -> Result<Self> {
    let b = <[u8; 16]>::read(reader)?;
    Ok(uuid::Uuid::from_bytes(b))
}
```
</details>


<details>
<summary> writing </summary>

```rust,no_run
fn write(reader: &mut impl std::io::Read) -> Result<Self> {
    let s = self.as_bytes();
    <[u8; 16]>::write(s, writer)
}
```
</details>




### `Angle`
---------

A `u8` representing a rotation angle in steps of 1/256 of a full turn.

```rust,noplayground
pub struct Angle(u8);
impl Angle {
    pub fn to_deg(&self) -> f32 {
        self.0 / (256.0 / 360.0)
    }
}
```

<details>
<summary> reading </summary>

```rust,noplayground
fn read(reader: &mut impl std::io::Read) -> Result<Self> {
    let b = <[u8; 16]>::read(reader)?;
    Ok(uuid::Uuid::from_bytes(b))
}
```
</details>


<details>
<summary> writing </summary>

```rust,no_run
fn write(reader: &mut impl std::io::Read) -> Result<Self> {
    let s = self.as_bytes();
    <[u8; 16]>::write(s, writer)
}
```
</details>