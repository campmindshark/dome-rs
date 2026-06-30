# Hardware Mapping

Hardware mapping is treated as fixture data, not an implementation detail to rediscover.

Known constants from Spectrum inventory:

- Dome: 190 struts, 7,580 logical LEDs, 71 projection vertices.
- Bar: routed through dome OPC control box 5 in operator wiring.
- Stage: 48 sides, 3 layers.
- OPC: Spectrum firmware expects `[channel][0][len_hi][len_lo][RGB...]`, without the usual `0xff` prefix.

## Examples

Dome and bar routing example:

```text
dome strut pixel -> dome logical map -> OPC channel bytes
bar pixel        -> dome control box 5 -> same OPC connection
```

OPC packet example for two RGB pixels on channel 2:

```text
02 00 00 06 12 34 56 aa bb cc
```

That packet means:

- channel: `0x02`
- command: `0x00`
- payload length: `0x0006`
- RGB payload: `0x123456`, `0xaabbcc`

Fixture captures must include every logical-to-device mapping used by dome, bar, and stage outputs.
