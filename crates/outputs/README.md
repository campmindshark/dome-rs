# domers-outputs

Output command types, simulator sinks, topology helpers, and OPC transport.

## Responsibilities

- Define dome, bar, and stage command streams.
- Encode Spectrum-compatible OPC frames.
- Map dome, bar, and stage logical pixels to hardware device frames.
- Maintain persistent sparse channel buffers matching Spectrum flush semantics.
- Provide simulator output sinks for no-hardware tests.

## Key Files

- `src/commands.rs`: `DomeCommand`, `BarCommand`, and `StageCommand`.
- `src/opc.rs`: OPC address parsing, frame encoding, persistent channels, and TCP client.
- `src/simulator.rs`: simulator color compensation and output sink behavior.
- `src/topology.rs`: extracted dome/bar/stage topology fixture helpers.

## Tests

Run this crate with:

```sh
cargo test -p domers-outputs
```
