# Domers

Rust rewrite of Camp Mindshark Spectrum lighting control.

This repository is being built fixture-first: no hardware should be required to validate ordinary pull requests. Hardware checks are release gates only.

## Current Status

Initial scaffold only. The first implementation increments establish:

- documented porting inventory
- C# fixture capture layout
- Rust workspace and CI gates
- scheduler, OPC, input, simulator, and migration test seams

## Development

```sh
cargo fmt --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
node ui/check.mjs
```

## Configuration

Domers uses TOML for native configuration. Spectrum XML is supported only as a
legacy import source:

```sh
cargo run --bin domers-config -- import-spectrum-xml \
  fixtures/config/spectrum_default_config.xml \
  domers.toml
```

The importer writes a new Domers TOML file and prints warnings for stale Spectrum
fields, inert v1 cuts, and invalid MIDI binding targets.

## Madmom

The old Windows app searched for a bundled Python 3.7 virtualenv at
`Madmom/env/Scripts/python.exe` and spawned `DBNBeatTracker`, then parsed
`BEAT:{seconds}` lines from stdout. Domers preserves that sidecar protocol for
compatibility, but the command/path is config-driven in TOML instead of being
hard-coded to the Windows bundle. A future native Rust beat tracker can replace
the sidecar behind the same beat input contract.

Local Docker/Rust may not be installed on every workstation; GitHub Actions is the merge-blocking source of truth.
