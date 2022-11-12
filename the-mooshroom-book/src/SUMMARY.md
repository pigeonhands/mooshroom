# Summary

[Introduction](README.md)

- [Data Types](./data-types.md)
- [Packet Structure](./packet-structure.md)

# Stages
- [Handshaking](./handshaking/handshaking.md)
    - [Client](./handshaking/client.md)
        - [0x00 - Handshake](./handshaking/client/handshake.md)
------
- [Status](./status/status.md)
    - [Client](./status/client.md)
        - [0x00 - StatusRequest](./status/client/status-request.md)
        - [0x01 - PingRequest](./status/client/ping-request.md)
    - [Server](./status/server.md)
        - [0x00 - StatusResponse](./status/server/status-response.md)
        - [0x01 - PingResponse](./status/server/ping-response.md)
-----
- [Login](./login/login.md)
    - [Client](./login/client.md)
        - [0x00 - LoginStart](./login/client/login-start.md)
        - [0x01 - EncryptionResponse](./login/client/encryption-response.md)
    - [Server](./login/server.md)
        - [0x00 - Disconnect](./login/server/disconnect.md)
        - [0x01 - EncryptionRequest](./login/server/encryption-request.md)
        - [0x02 - LoginSuccess](./login/server/login-success.md)
        - [0x03 - SetCompression](./login/server/set-compression.md)
----
- [Play](./play/play.md)
    - [Client](./play/client.md)
    - [Server](./play/server.md)
        - [0x00 - SpawnEntity](./play/server/spawn-entity.md)
        - [0x02 - SpawnPlayer](./play/server/spawn-player.md)
        - [0x03 - EntityAnimation](./play/server/entity-animation.md)