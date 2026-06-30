//! Input parsers and fakeable input seams.

pub mod audio;
pub mod madmom;
pub mod midi;
pub mod orientation;

pub use audio::{
    capture_devices_from_all_endpoints, current_audio_device_index, parse_volume_payload,
    AudioDevice, AudioDeviceFlow, EnumeratedAudioEndpoint, VolumeReplay,
};
pub use madmom::{parse_beat_line, MadmomLaunchConfig, MadmomSidecar};
pub use midi::{
    continuous_knob, discrete_knob, logarithmic_knob, parse_midi_payload, MidiCommand,
    MidiCommandKind, MidiReplay, MidiState,
};
pub use orientation::{
    classify_datagram, parse_datagram, DatagramKind, OrientationDevice, OrientationInputState,
    OrientationQuaternion, ParsedOrientationDatagram,
};
