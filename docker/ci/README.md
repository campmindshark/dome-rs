# CI Docker Harness

This directory documents Docker loopback services for multi-process CI. The Rust
suite covers the loopback gates without Docker:

```sh
cargo test -p domers-server hardware_outputs_send_mapped_dome_frame_to_loopback_opc
cargo test -p domers-server hardware_outputs_reconnect_after_loopback_opc_returns
cargo test -p domers-server runtime_udp_input_adapters_feed_live_state
```

Those tests verify OPC write visibility, OPC reconnect after a missing listener
returns, and live UDP input ingestion. Docker services can replace the in-process
listeners when the suite needs multi-process failure injection.

Docker service roles:

- OPC TCP listener
- fake Madmom sidecar
- fake orientation UDP emitter
- integration runner

Docker Compose services wrap the same gates when multi-process failure injection
is needed before physical hardware sign-off.

## Example

```sh
docker compose -f docker/ci/docker-compose.yml up --abort-on-container-exit
```

Target services:

```text
server-under-test -> opc-listener
server-under-test -> fake-madmom
fake-orientation  -> server-under-test:5005/udp
integration-runner -> server-under-test
```
