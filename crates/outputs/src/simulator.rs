//! Simulator output sinks decoupled from hardware OPC.

use domers_core::Rgb;

use crate::commands::DomeCommand;

/// Display-adjusted simulator color.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct SimulatorColor {
    /// Red channel.
    pub r: u8,
    /// Green channel.
    pub g: u8,
    /// Blue channel.
    pub b: u8,
}

impl SimulatorColor {
    /// Apply Spectrum's simulator boost so dim LED colors remain visible.
    #[must_use]
    #[allow(
        clippy::cast_possible_truncation,
        clippy::cast_sign_loss,
        clippy::cast_precision_loss,
        reason = "Simulator display boost clamps before converting back to RGB channels"
    )]
    pub fn from_led_color(color: Rgb) -> Self {
        let max_channel = color.r.max(color.g).max(color.b);
        if max_channel == 0 {
            return Self::default();
        }

        let multiplier = (255.0 / f64::from(max_channel)).sqrt();
        let boost = |channel: u8| (f64::from(channel) * multiplier).clamp(0.0, 255.0) as u8;
        Self {
            r: boost(color.r),
            g: boost(color.g),
            b: boost(color.b),
        }
    }
}

/// Command sink for dome simulator frames.
#[derive(Clone, Debug, Default)]
pub struct DomeOutputSink {
    hardware_enabled: bool,
    simulation_enabled: bool,
    commands: Vec<DomeCommand>,
}

impl DomeOutputSink {
    /// Create a sink with independent hardware and simulator enablement.
    #[must_use]
    pub const fn new(hardware_enabled: bool, simulation_enabled: bool) -> Self {
        Self {
            hardware_enabled,
            simulation_enabled,
            commands: Vec::new(),
        }
    }

    /// Whether this output should participate in scheduling.
    #[must_use]
    pub const fn enabled(&self) -> bool {
        self.hardware_enabled || self.simulation_enabled
    }

    /// Write a single logical LED.
    pub fn set_pixel(&mut self, strut_index: usize, led_index: usize, color: Rgb) {
        if self.simulation_enabled {
            self.commands.push(DomeCommand::Pixel {
                strut_index,
                led_index,
                color,
            });
        }
    }

    /// Write a full frame, even when hardware/OPC is disabled.
    pub fn write_buffer(&mut self, frame: Vec<Rgb>) {
        if self.simulation_enabled {
            self.commands.push(DomeCommand::Frame(frame));
        }
    }

    /// Flush the simulator frame.
    pub fn flush(&mut self) {
        if self.simulation_enabled {
            self.commands.push(DomeCommand::Flush);
        }
    }

    /// Drain queued simulator commands.
    pub fn drain_commands(&mut self) -> Vec<DomeCommand> {
        std::mem::take(&mut self.commands)
    }
}

#[cfg(test)]
mod tests {
    use domers_core::Rgb;

    use super::{DomeOutputSink, SimulatorColor};
    use crate::commands::DomeCommand;

    #[test]
    fn simulation_only_sink_is_schedulable_and_emits_write_buffer_frames() {
        let mut sink = DomeOutputSink::new(false, true);
        assert!(sink.enabled());

        sink.write_buffer(vec![Rgb::from_u24(0x12_34_56)]);
        sink.flush();

        assert_eq!(
            sink.drain_commands(),
            vec![
                DomeCommand::Frame(vec![Rgb::from_u24(0x12_34_56)]),
                DomeCommand::Flush
            ]
        );
    }

    #[test]
    fn hardware_only_sink_does_not_emit_simulator_commands() {
        let mut sink = DomeOutputSink::new(true, false);
        assert!(sink.enabled());

        sink.set_pixel(1, 2, Rgb::from_u24(0xff_00_00));
        sink.write_buffer(vec![Rgb::from_u24(0x00_ff_00)]);
        sink.flush();

        assert!(sink.drain_commands().is_empty());
    }

    #[test]
    fn simulator_color_boosts_dim_channels() {
        let boosted = SimulatorColor::from_led_color(Rgb::from_u24(0x04_02_00));
        assert!(boosted.r > 4);
        assert!(boosted.g > 2);
        assert_eq!(boosted.b, 0);
    }
}
