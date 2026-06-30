# Domers

Rust rewrite of Camp Mindshark Spectrum lighting control.

## Status

Domers runs without lighting hardware:

- TOML config loading from `examples/domers.toml` or an imported config
- HTTP API for health, state, start, stop, and dome visualizer config
- WebSocket simulator frame stream
- browser operator shell served by the Rust binary
- tested scheduler, OPC, input, simulator, visualizer, migration, and server paths

Run locally:

```sh
cargo run --bin domers -- --config examples/domers.toml --bind 127.0.0.1:3000
```

Then open `http://127.0.0.1:3000`.

## Development

```sh
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
node ui/check.mjs
```

## Configuration

Domers uses TOML for native configuration. Start from the checked example:

```sh
cp examples/domers.toml domers.toml
```

Spectrum XML is supported only as a legacy import source:

```sh
cargo run --bin domers-config -- import-spectrum-xml \
  fixtures/config/spectrum_default_config.xml \
  domers.toml
```

The importer writes a new Domers TOML file and prints warnings for stale Spectrum fields, inert v1 cuts, and invalid MIDI binding targets.

Example TOML output shape:

```toml
[dome]
enabled = true
simulation_enabled = false
opc_address = "192.168.1.69:7890"
active_visualizer = 0
test_pattern = 0
brightness = 0.356915762888129

[tempo]
source = "human"
flash_speed = 0.0

[madmom]
command = "DBNBeatTracker"
audio_input_index = 0
```

See [`docs/configuration.md`](docs/configuration.md) for the full config/import contract.
Intentional differences from Spectrum are tracked in
[`docs/intentional-deviations.md`](docs/intentional-deviations.md).

## Madmom

The old Windows app searched for a bundled Python 3.7 virtualenv at `Madmom/env/Scripts/python.exe` and spawned `DBNBeatTracker`, then parsed `BEAT:{seconds}` lines from stdout. Domers preserves that sidecar protocol for compatibility, but the command/path is config-driven in TOML instead of being hard-coded to the Windows bundle. A native Rust beat tracker can use the same beat input contract later.

## UI

The browser shell exposes engine start/stop, dome visualizer selection, flash speed, metrics, and a WebSocket-backed dome simulator canvas. See [`docs/ui-expectations.md`](docs/ui-expectations.md) for expected UI states and image placeholders.

TODO: Add image of the Domers operator page here.

- Capture: full browser window at desktop size.
- Expected: title, start/stop buttons, dome visualizer selector, flash speed slider, metrics, stream status, and simulator canvas are visible.
- Suggested file: `docs/images/readme-operator-shell.png`.

Local Docker/Rust may not be installed on every workstation; GitHub Actions is the merge-blocking source of truth.
