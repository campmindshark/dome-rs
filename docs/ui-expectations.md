# UI Expectations

This document records the browser UI states worth checking. Add screenshots when a state is stable enough for comparison.

## Operator Shell

The shell lives in `ui/index.html` and is checked by `node ui/check.mjs`.

Expected elements:

- `Domers Operator` heading
- `Start` and `Stop` engine buttons
- engine status text
- WebSocket stream status text
- `domeActiveVis` selector with Volume, Radial, Race, Snakes, Quaternion Test, Quaternion Multi Test, Quaternion Paintbrush, and Splat
- `flashSpeed` slider
- frame and simulator-frame metrics
- `dome-simulator` canvas using the WebSocket frame source

TODO: Add image of the operator shell.

- Capture: browser window at desktop size.
- Expected: all controls above are visible, simulator canvas is black/empty before frames arrive.
- Suggested file: `docs/images/ui-operator-shell.png`.

## Running Engine State

Expected behavior after clicking `Start`:

- engine status changes to `running`
- controls remain interactive
- stream status reads `stream connected` once the WebSocket connects
- frame counters advance while the engine is running
- simulator canvas remains visible and receives frame data

TODO: Add image of running state.

- Capture: after clicking `Start`.
- Expected: status reads `running`.
- Suggested file: `docs/images/ui-running-state.png`.

## Dome Visualizer Selection

Expected behavior when selecting each dome visualizer:

- selected value matches the server config field `dome.active_visualizer`
- simulator frame stream updates after the selection is applied
- invalid values are rejected after API config validation is tightened

TODO: Add image sequence of the visualizer selector.

- Capture: dropdown open and at least one selected non-default mode.
- Expected: labels match the Spectrum active visualizer map.
- Suggested file: `docs/images/ui-dome-visualizer-selector.png`.

## Simulator Frame View

Expected simulator behavior:

- dome canvas uses frame data from the server, not direct hardware state
- buffer-based visualizers can render with OPC disabled
- display color compensation is applied only to the UI view, never to OPC bytes

TODO: Add image of a non-empty dome simulator frame.

- Capture: canvas after a deterministic simulator frame is rendered.
- Expected: visible colored points/struts, with the selected visualizer noted in the caption.
- Suggested file: `docs/images/ui-dome-simulator-frame.png`.

## Planned UI Sections

Add screenshots and notes as these sections land:

- config editor
- MIDI log
- color palette editor
- tempo/Madmom controls
- orientation calibration panel
- bar simulator
- stage simulator
- diagnostics panel
