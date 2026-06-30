# Architecture

Domers is structured as a headless Rust engine with browser control and simulator views.

## Crates

- `domers-core`: shared colors, beat timing, config types.
- `domers-engine`: scheduler and frame orchestration.
- `domers-outputs`: dome/bar/stage commands, topology, OPC encoding.
- `domers-inputs`: Madmom, MIDI, audio, and orientation seams.
- `domers-visualizers`: visualizer inventory and eventual ports.
- `domers-server`: HTTP/WebSocket API.
- `domers-test-support`: fake clocks and no-hardware test utilities.

## Timing Contracts

- Engine target: Spectrum-compatible 400 Hz compute cap.
- OPC target: independent 200 Hz send cap.
- Browser simulator: throttled stream derived from engine frames, not hardware sockets.

## Concurrency Contract

The Rust implementation should prefer frame-local config snapshots and event channels over shared mutable UI state. Stress tests must cover concurrent config updates, MIDI replay, and visualizer frames.

## Configuration Contract

Domers-native configuration is TOML. XML is not a runtime config format for the
Rust app; it is only a legacy Spectrum import format handled by:

```sh
cargo run --bin domers-config -- import-spectrum-xml <spectrum.xml> <domers.toml>
```

The importer maps live Spectrum fields into nested Domers sections such as
`[dome]`, `[bar]`, `[stage]`, `[tempo]`, and `[madmom]`, and reports stale or
unsupported XML fields instead of silently preserving them.

## Madmom Contract

Domers keeps Madmom as a configurable sidecar protocol for v1 compatibility:
the sidecar emits `BEAT:{seconds}` lines that feed the beat engine. Unlike the
Windows WPF app, Domers does not assume a bundled `Madmom/env/Scripts/python.exe`
path. The TOML config owns the command/path, which leaves room for either a
Python sidecar, a Docker-sidecar test fake, or a future native Rust beat
detector behind the same beat-report interface.
