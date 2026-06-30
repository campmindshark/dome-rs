# domers-visualizers

Deterministic visualizer and diagnostic renderers.

## Responsibilities

- Track the used Spectrum visualizer inventory.
- Render dome live visualizers into simulator/hardware command streams.
- Render dome, bar, and stage diagnostic patterns.
- Keep local stable frame hashes and compare against captured Spectrum visualizer goldens.
- Consume the active full eight-color palette bank.

## Key Files

- `src/lib.rs`: visualizer inventory, input model, dome/bar/stage renderers, diagnostics, and tests.
- `../../fixtures/spectrum-csharp/visualizer_frame_cases.json`: source-traceable manifest with captured Spectrum frame hashes.

## Tests

Run this crate with:

```sh
cargo test -p domers-visualizers
```

Exact Spectrum frame equivalence is gated by the captured Spectrum hashes in the
visualizer manifest and by the Rust frame-hash tests in this crate.
