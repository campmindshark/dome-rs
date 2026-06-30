//! Fakeable MIDI command replay.

/// MIDI command category.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MidiCommandKind {
    /// Note on/off command.
    Note,
    /// Continuous controller command.
    ControlChange,
    /// Program change command.
    Program,
}

/// Normalized MIDI command.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MidiCommand {
    /// MIDI device index.
    pub device_index: u8,
    /// Command kind.
    pub kind: MidiCommandKind,
    /// Note/controller/program index.
    pub index: u8,
    /// Normalized value in `[0.0, 1.0]`.
    pub value: f32,
}

/// Deterministic MIDI replay stream.
#[derive(Clone, Debug)]
pub struct MidiReplay {
    commands: Vec<MidiCommand>,
    cursor: usize,
}

/// Stateful MIDI command processor mirroring Spectrum's no-hardware semantics.
#[derive(Clone, Debug, Default)]
pub struct MidiState {
    commands: Vec<MidiCommand>,
    drained: usize,
    knob_values: std::collections::BTreeMap<(u8, u8), f32>,
    note_velocities: std::collections::BTreeMap<(u8, u8), f32>,
    log: Vec<String>,
}

impl MidiState {
    /// Apply a MIDI command, updating note/knob state and the bounded log.
    pub fn push(&mut self, command: MidiCommand) {
        match command.kind {
            MidiCommandKind::ControlChange => {
                self.knob_values
                    .insert((command.device_index, command.index), command.value);
                self.push_log(format!(
                    "device {} knob {} -> {}",
                    command.device_index, command.index, command.value
                ));
            }
            MidiCommandKind::Note => {
                self.note_velocities
                    .insert((command.device_index, command.index), command.value);
                self.push_log(format!(
                    "device {} note {} -> {}",
                    command.device_index, command.index, command.value
                ));
            }
            MidiCommandKind::Program => {
                self.push_log(format!(
                    "device {} program {}",
                    command.device_index, command.index
                ));
            }
        }
        self.commands.push(command);
    }

    /// Drain commands exactly once per operator tick.
    pub fn drain_since_last_tick(&mut self) -> Vec<MidiCommand> {
        let commands = self.commands[self.drained..].to_vec();
        self.drained = self.commands.len();
        commands
    }

    /// Spectrum returns `-1.0` for untouched knobs.
    #[must_use]
    pub fn knob_value(&self, device_index: u8, index: u8) -> f32 {
        self.knob_values
            .get(&(device_index, index))
            .copied()
            .unwrap_or(-1.0)
    }

    /// Spectrum returns `0.0` for untouched notes.
    #[must_use]
    pub fn note_velocity(&self, device_index: u8, index: u8) -> f32 {
        self.note_velocities
            .get(&(device_index, index))
            .copied()
            .unwrap_or(0.0)
    }

    /// Recent log messages.
    #[must_use]
    pub fn log(&self) -> &[String] {
        &self.log
    }

    fn push_log(&mut self, message: String) {
        const MAX_LOG: usize = 64;
        if self.log.len() == MAX_LOG {
            self.log.remove(0);
        }
        self.log.push(message);
    }
}

impl MidiReplay {
    /// Create a MIDI replay stream.
    #[must_use]
    pub fn new(commands: Vec<MidiCommand>) -> Self {
        Self {
            commands,
            cursor: 0,
        }
    }

    /// Drain commands since the last tick.
    pub fn commands_since_last_tick(&mut self) -> Vec<MidiCommand> {
        let remaining = self.commands[self.cursor..].to_vec();
        self.cursor = self.commands.len();
        remaining
    }
}

/// Parse one live MIDI command payload.
///
/// Accepted shape: `note,64,1.0`, `cc,1,0.5`, `program,3,1.0`, or
/// `device,2,note,64,1.0`.
#[must_use]
pub fn parse_midi_payload(payload: &[u8]) -> Option<MidiCommand> {
    let text = std::str::from_utf8(payload).ok()?.trim();
    let mut parts = text.split(',').map(str::trim);
    let mut device_index = 0;
    let first = parts.next()?;
    let kind_text = if first == "device" {
        device_index = parts.next()?.parse::<u8>().ok()?;
        parts.next()?
    } else {
        first
    };
    let kind = match kind_text {
        "note" => MidiCommandKind::Note,
        "cc" | "control_change" => MidiCommandKind::ControlChange,
        "program" => MidiCommandKind::Program,
        _ => return None,
    };
    let index = parts.next()?.parse::<u8>().ok()?;
    let value = parts.next()?.parse::<f32>().ok()?.clamp(0.0, 1.0);
    if parts.next().is_some() {
        return None;
    }
    Some(MidiCommand {
        device_index,
        kind,
        index,
        value,
    })
}

/// Spectrum continuous knob interpolation.
#[must_use]
pub fn continuous_knob(value: f32, from: f32, to: f32) -> f32 {
    from + (to - from) * value.clamp(0.0, 1.0)
}

/// Spectrum discrete knob mapping.
#[must_use]
#[allow(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    reason = "MIDI values map to small UI option counts"
)]
pub fn discrete_knob(value: f32, num_possible_values: u8) -> u8 {
    if num_possible_values == 0 {
        return 0;
    }
    let step = 1.0 / f32::from(num_possible_values + 2);
    let mut num_steps = (value.clamp(0.0, 1.0) / step) as u8;
    if num_steps == 0 {
        return 0;
    }
    num_steps = num_steps.saturating_sub(1);
    num_steps.min(num_possible_values - 1)
}

/// Spectrum discrete logarithmic knob mapping with the zero slot enabled.
#[must_use]
pub fn logarithmic_knob(value: f32, num_possible_values: u8, start_value: f32) -> f32 {
    let index = discrete_knob(value, num_possible_values.saturating_add(1));
    if index == 0 {
        0.0
    } else {
        2_f32.powi(i32::from(index - 1)) * start_value
    }
}

#[cfg(test)]
mod tests {
    use super::{
        continuous_knob, discrete_knob, logarithmic_knob, parse_midi_payload, MidiCommand,
        MidiCommandKind, MidiReplay, MidiState,
    };

    #[test]
    fn drains_midi_commands_once() {
        let command = MidiCommand {
            device_index: 0,
            kind: MidiCommandKind::Note,
            index: 64,
            value: 1.0,
        };
        let mut replay = MidiReplay::new(vec![command]);

        assert_eq!(replay.commands_since_last_tick(), vec![command]);
        assert!(replay.commands_since_last_tick().is_empty());
    }

    fn assert_close(left: f32, right: f32) {
        assert!((left - right).abs() < f32::EPSILON, "{left} != {right}");
    }

    #[test]
    fn parses_live_midi_payloads() {
        assert_eq!(
            parse_midi_payload(b"note,64,1.0"),
            Some(MidiCommand {
                device_index: 0,
                kind: MidiCommandKind::Note,
                index: 64,
                value: 1.0
            })
        );
        assert_eq!(
            parse_midi_payload(b"cc,1,0.5"),
            Some(MidiCommand {
                device_index: 0,
                kind: MidiCommandKind::ControlChange,
                index: 1,
                value: 0.5
            })
        );
        assert_eq!(
            parse_midi_payload(b"device,2,note,64,1.0"),
            Some(MidiCommand {
                device_index: 2,
                kind: MidiCommandKind::Note,
                index: 64,
                value: 1.0
            })
        );
        assert_eq!(parse_midi_payload(b"bad,1,1"), None);
    }

    #[test]
    fn tracks_device_scoped_knobs_notes_and_drain() {
        let mut state = MidiState::default();
        assert_close(state.knob_value(2, 1), -1.0);
        assert_close(state.note_velocity(2, 64), 0.0);

        let knob = MidiCommand {
            device_index: 2,
            kind: MidiCommandKind::ControlChange,
            index: 1,
            value: 0.5,
        };
        let note = MidiCommand {
            device_index: 2,
            kind: MidiCommandKind::Note,
            index: 64,
            value: 1.0,
        };
        state.push(knob);
        state.push(note);

        assert_close(state.knob_value(2, 1), 0.5);
        assert_close(state.note_velocity(2, 64), 1.0);
        assert_eq!(state.drain_since_last_tick(), vec![knob, note]);
        assert!(state.drain_since_last_tick().is_empty());
        assert_eq!(state.log().len(), 2);
    }

    #[test]
    fn maps_spectrum_knob_values() {
        assert_close(continuous_knob(0.25, 10.0, 20.0), 12.5);
        assert_eq!(discrete_knob(0.0, 4), 0);
        assert_eq!(discrete_knob(0.5, 4), 2);
        assert_eq!(discrete_knob(1.0, 4), 3);
        assert_close(logarithmic_knob(0.0, 4, 0.25), 0.0);
        assert_close(logarithmic_knob(0.5, 4, 0.25), 0.5);
    }
}
