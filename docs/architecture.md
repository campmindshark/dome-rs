# Architecture

Domers is structured as a headless Rust engine with browser control and simulator views.

## Crates

- `domers-core`: shared colors, beat timing, config types, TOML config import, and migration warnings.
- `domers-engine`: scheduler and frame orchestration.
- `domers-outputs`: dome/bar/stage commands, topology, simulator sinks, and OPC encoding.
- `domers-inputs`: Madmom, MIDI, audio, and orientation seams.
- `domers-visualizers`: visualizer inventory and deterministic simulator frame harness.
- `domers-server`: HTTP/WebSocket contract surface and state semantics.
- `domers-test-support`: fake clocks and no-hardware test utilities.

## Runtime Shape

```text
Browser UI -> Server contract -> Engine scheduler -> Inputs + Visualizers -> Outputs
                                                        |                  |
                                                        |                  +-> OPC hardware
                                                        +--------------------> Simulator frames
```

The browser simulator is driven by engine frame data, not by hardware sockets. This keeps UI validation possible without a BeagleBone or LED fixtures connected.

## Timing Contracts

- Engine target: Spectrum-compatible 400 Hz compute cap.
- OPC target: independent 200 Hz send cap.
- Browser simulator: throttled stream derived from engine frames, not hardware sockets.

Example future timing test shape:

```text
fake clock -> scheduler frame -> visualizer render -> simulator frame -> metrics update
```

## Concurrency Contract

The Rust implementation should prefer frame-local config snapshots and event channels over shared mutable UI state. Stress tests must cover concurrent config updates, MIDI replay, and visualizer frames.

## Configuration Contract

Domers-native configuration is TOML. XML is not a runtime config format for the Rust app; it is only a legacy Spectrum import format handled by:

```sh
cargo run --bin domers-config -- import-spectrum-xml <spectrum.xml> <domers.toml>
```

The importer maps live Spectrum fields into nested Domers sections such as `[dome]`, `[bar]`, `[stage]`, `[tempo]`, and `[madmom]`, and reports stale or unsupported XML fields instead of silently preserving them. See [`configuration.md`](configuration.md).

## Madmom Contract

Domers keeps Madmom as a configurable sidecar protocol for v1 compatibility: the sidecar emits `BEAT:{seconds}` lines that feed the beat engine. Unlike the Windows WPF app, Domers does not assume a bundled `Madmom/env/Scripts/python.exe` path. The TOML config owns the command/path, which leaves room for either a Python sidecar, a Docker-sidecar test fake, or a future native Rust beat detector behind the same beat-report interface.

## TODO Images

TODO: Add architecture diagram image.

- Capture: rendered diagram of browser UI, server, engine, inputs, visualizers, outputs, OPC, and simulator frame stream.
- Expected: clearly shows simulator frames are separate from OPC hardware output.
- Suggested file: `docs/images/architecture-runtime-flow.png`.

TODO: Add screenshot of server metrics once the HTTP/WebSocket adapter exists.

- Capture: browser/devtool/API output showing frame counters and simulator frame counters.
- Expected: `frames` and `simulator_frames` are visible.
- Suggested file: `docs/images/architecture-server-metrics.png`.
