# domers-test-support

Shared test helpers and fixture smoke checks.

## Responsibilities

- Provide deterministic test-time helpers for workspace tests.
- Assert required fixture groups exist and match expected smoke-counts.
- Keep no-hardware test support separate from production crates.

## Key Files

- `src/lib.rs`: deterministic clock/test helpers and fixture inventory tests.
- `../../fixtures/`: extracted Spectrum and no-hardware reference data used by tests.

## Tests

Run this crate with:

```sh
cargo test -p domers-test-support
```
