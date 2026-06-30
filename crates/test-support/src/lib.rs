//! Shared fake test utilities for no-hardware CI.

#[cfg(test)]
use std::path::{Path, PathBuf};

/// Deterministic fake clock.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct FakeClock {
    now_ms: u64,
}

impl FakeClock {
    /// Current fake timestamp in milliseconds.
    #[must_use]
    pub const fn now_ms(self) -> u64 {
        self.now_ms
    }

    /// Advance the fake clock.
    pub fn advance_ms(&mut self, ms: u64) {
        self.now_ms = self.now_ms.saturating_add(ms);
    }
}

#[cfg(test)]
fn fixture_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .join("fixtures")
}

#[cfg(test)]
fn read_fixture(path: &str) -> String {
    std::fs::read_to_string(fixture_root().join(path)).expect("fixture should be readable")
}

#[cfg(test)]
mod tests {
    use super::FakeClock;
    use super::{fixture_root, read_fixture};

    #[test]
    fn advances_deterministically() {
        let mut clock = FakeClock::default();
        clock.advance_ms(42);
        assert_eq!(clock.now_ms(), 42);
    }

    #[test]
    fn m0_reference_fixtures_exist() {
        let root = fixture_root();
        for path in [
            "manifest.json",
            "spectrum-csharp/dome_mapping.json",
            "spectrum-csharp/dome_geometry.json",
            "spectrum-csharp/bar_stage_topology.json",
            "spectrum-csharp/opc_packets/two_pixels_channel_2.bin",
            "config/spectrum_default_config.xml",
            "orientation/datagram_lengths.json",
            "madmom/valid-and-invalid.txt",
        ] {
            assert!(root.join(path).exists(), "missing fixture {path}");
        }
    }

    #[test]
    fn fixture_content_matches_inventory_smoke_counts() {
        let dome_mapping = read_fixture("spectrum-csharp/dome_mapping.json");
        assert!(dome_mapping.contains(r#""strut_count": 190"#));
        assert!(dome_mapping.contains(r#""bar_control_box": 5"#));

        let dome_geometry = read_fixture("spectrum-csharp/dome_geometry.json");
        assert!(dome_geometry.contains(r#""line_count": 190"#));
        assert!(dome_geometry.contains(r#""point_count": 71"#));

        let stage = read_fixture("spectrum-csharp/bar_stage_topology.json");
        assert!(stage.contains(r#""side_count": 48"#));
        assert!(stage.contains(r#""layer_count": 3"#));
    }
}
