# Hardware Readiness Checklist

Hardware validation gates release tags, not ordinary pull requests. Complete this
checklist after the no-hardware CI suite is green.

## Preflight

- Confirm `cargo test --workspace` passes.
- Confirm `cargo clippy --workspace --all-targets -- -D warnings` passes.
- Confirm UI smoke check passes.
- Confirm `no_hardware_server_migration_and_simulator_smoke` passes.
- Record commit SHA, operator laptop OS, and fixture capture date.

## Dome

- Run flash-by-strut diagnostic and verify physical strut order.
- Run strut iteration diagnostic and verify control-box mapping.
- Run strand test diagnostic and verify strand direction.
- Run full-color flash diagnostic and verify brightness limits.
- Confirm buffer-based simulator modes run without OPC enabled.

## Bar

- Verify bar pixels route through dome control box 5.
- Run bar corner/runner diagnostic and verify reversal rules.

## Stage

- Run stage side/layer diagnostic.
- Verify all 48 sides and 3 layers match physical layout.

## Inputs

- Verify MIDI board palette, flash, tap tempo, and knob bindings.
- Verify audio volume input with the show audio device.
- Verify Madmom sidecar beat parsing against the selected input.
- Verify orientation devices on UDP port 5005.

## Network And Recovery

- Verify OPC reconnect after controller restart or cable interruption.
- Verify server start/stop from browser UI.
- Verify simulator frame stream remains responsive during a 60-second run.

## Sign-Off

- Operator:
- Date:
- Notes:
