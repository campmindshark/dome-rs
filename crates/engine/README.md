# domers-engine

Scheduling primitives for the operator runtime.

## Responsibilities

- Select active inputs, visualizers, and outputs for each operator frame.
- Preserve Spectrum-style priority behavior, including diagnostic overrides.
- Keep scheduling deterministic and independent from HTTP, hardware, and UI code.

## Key Files

- `src/scheduler.rs`: input/output specs, visualizer specs, priorities, and frame scheduling.
- `src/lib.rs`: public scheduler exports.

## Tests

Run this crate with:

```sh
cargo test -p domers-engine
```
