# domers-visualizers

Deterministic visualizer and diagnostic renderers.

## Responsibilities

- Track the used Spectrum visualizer inventory.
- Render dome live visualizers into simulator/hardware command streams.
- Render dome, bar, and stage diagnostic patterns.
- Provide deterministic approximations and stable frame hashes until old Spectrum visualizer goldens are captured.
- Consume the active full eight-color palette bank.

## Key Files

- `src/lib.rs`: visualizer inventory, input model, dome/bar/stage renderers, diagnostics, and tests.
- `../../fixtures/spectrum-csharp/visualizer_frame_cases.json`: source-traceable manifest for future Spectrum frame golden capture.

## Tests

Run this crate with:

```sh
cargo test -p domers-visualizers
```

Exact Spectrum frame equivalence remains gated by the visualizer golden capture workflow.
