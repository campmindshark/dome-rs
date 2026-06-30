//! Fakeable audio volume input.

/// Spectrum audio endpoint flow.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AudioDeviceFlow {
    /// Capture/recording endpoint.
    Capture,
    /// Render/playback endpoint.
    Render,
}

/// Audio endpoint as Spectrum exposes it to config/UI.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AudioDevice {
    /// Stable endpoint id.
    pub id: String,
    /// Friendly display name.
    pub name: String,
    /// All-endpoint index used by PortAudio/Madmom.
    pub index: i32,
}

/// Raw endpoint emitted by a fakeable enumerator.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EnumeratedAudioEndpoint {
    /// Stable endpoint id.
    pub id: String,
    /// Friendly display name.
    pub name: String,
    /// Endpoint flow.
    pub flow: AudioDeviceFlow,
}

/// Return Spectrum's capture-device list while preserving all-endpoint indexes.
#[must_use]
#[allow(
    clippy::cast_possible_truncation,
    reason = "Audio endpoint indexes are bounded by host device enumeration"
)]
pub fn capture_devices_from_all_endpoints(
    endpoints: &[EnumeratedAudioEndpoint],
) -> Vec<AudioDevice> {
    endpoints
        .iter()
        .enumerate()
        .filter(|(_, endpoint)| endpoint.flow == AudioDeviceFlow::Capture)
        .map(|(index, endpoint)| AudioDevice {
            id: endpoint.id.clone(),
            name: endpoint.name.clone(),
            index: i32::try_from(index).unwrap_or(i32::MAX),
        })
        .collect()
}

/// Find the current Madmom/PortAudio index for a configured audio device id.
#[must_use]
pub fn current_audio_device_index(
    configured_device_id: Option<&str>,
    devices: &[AudioDevice],
) -> i32 {
    let Some(configured_device_id) = configured_device_id else {
        return -1;
    };
    devices
        .iter()
        .find(|device| device.id == configured_device_id)
        .map_or(-1, |device| device.index)
}

/// Deterministic volume replay for no-hardware tests.
#[derive(Clone, Debug)]
pub struct VolumeReplay {
    samples: Vec<f32>,
    cursor: usize,
}

impl VolumeReplay {
    /// Create a volume replay stream.
    #[must_use]
    pub fn new(samples: Vec<f32>) -> Self {
        Self { samples, cursor: 0 }
    }

    /// Return the next volume sample, clamped to `[0.0, 1.0]`.
    pub fn next_volume(&mut self) -> Option<f32> {
        let sample = *self.samples.get(self.cursor)?;
        self.cursor += 1;
        Some(sample.clamp(0.0, 1.0))
    }
}

/// Parse one live audio volume payload.
#[must_use]
pub fn parse_volume_payload(payload: &[u8]) -> Option<f32> {
    let text = std::str::from_utf8(payload).ok()?.trim();
    let volume = text.parse::<f32>().ok()?;
    Some(volume.clamp(0.0, 1.0))
}

#[cfg(test)]
mod tests {
    use super::{
        capture_devices_from_all_endpoints, current_audio_device_index, parse_volume_payload,
        AudioDeviceFlow, EnumeratedAudioEndpoint, VolumeReplay,
    };

    #[test]
    fn replays_clamped_volume_samples() {
        let mut replay = VolumeReplay::new(vec![-1.0, 0.25, 2.0]);
        assert_eq!(replay.next_volume(), Some(0.0));
        assert_eq!(replay.next_volume(), Some(0.25));
        assert_eq!(replay.next_volume(), Some(1.0));
        assert_eq!(replay.next_volume(), None);
    }

    #[test]
    fn parses_live_volume_payloads() {
        assert_eq!(parse_volume_payload(b"0.25\n"), Some(0.25));
        assert_eq!(parse_volume_payload(b"2.0"), Some(1.0));
        assert_eq!(parse_volume_payload(b"noise"), None);
    }

    #[test]
    fn capture_devices_preserve_all_endpoint_indexes() {
        let endpoints = vec![
            endpoint("speaker", "Speaker", AudioDeviceFlow::Render),
            endpoint("mic-a", "Mic A", AudioDeviceFlow::Capture),
            endpoint("loopback", "Loopback", AudioDeviceFlow::Render),
            endpoint("mic-b", "Mic B", AudioDeviceFlow::Capture),
        ];

        let devices = capture_devices_from_all_endpoints(&endpoints);

        assert_eq!(devices.len(), 2);
        assert_eq!(devices[0].id, "mic-a");
        assert_eq!(devices[0].index, 1);
        assert_eq!(devices[1].id, "mic-b");
        assert_eq!(devices[1].index, 3);
        assert_eq!(current_audio_device_index(Some("mic-b"), &devices), 3);
        assert_eq!(current_audio_device_index(Some("missing"), &devices), -1);
        assert_eq!(current_audio_device_index(None, &devices), -1);
    }

    fn endpoint(id: &str, name: &str, flow: AudioDeviceFlow) -> EnumeratedAudioEndpoint {
        EnumeratedAudioEndpoint {
            id: id.to_string(),
            name: name.to_string(),
            flow,
        }
    }
}
