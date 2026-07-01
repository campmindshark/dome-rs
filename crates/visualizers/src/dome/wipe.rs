use domers_core::Rgb;
use domers_outputs::{dome_strut_length, topology::DOME_STRUTS, DomeCommand};

/// Black out every mapped dome LED using sparse pixel commands plus flush.
///
/// Matches Spectrum volume enable-wipe semantics and works on OPC paths that
/// only realize `current` pixels after `Flush` (unlike bare `Frame` commands).
#[must_use]
pub(crate) fn dome_blackout_commands() -> Vec<DomeCommand> {
    let mut commands = Vec::new();
    for strut_index in 0..DOME_STRUTS {
        let Some(length) = dome_strut_length(strut_index) else {
            continue;
        };
        for led_index in 0..length {
            commands.push(DomeCommand::Pixel {
                strut_index,
                led_index,
                color: Rgb::BLACK,
            });
        }
    }
    commands.push(DomeCommand::Flush);
    commands
}
