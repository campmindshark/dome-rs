//! Spectrum XML migration analysis.

/// Warning category emitted by the migration analyzer.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WarningKind {
    /// Field is known stale and has no Rust v1 model.
    StaleField,
    /// Field exists but is intentionally inert/cut in v1.
    InertField,
    /// MIDI binding references a missing config property.
    InvalidMidiBindingTarget,
}

/// One migration warning.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MigrationWarning {
    /// Warning kind.
    pub kind: WarningKind,
    /// XML field or binding target.
    pub field: String,
}

/// Migration analysis result.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct MigrationReport {
    /// Warnings found while analyzing XML.
    pub warnings: Vec<MigrationWarning>,
}

impl MigrationReport {
    /// Whether this report contains a warning for a field.
    #[must_use]
    pub fn contains(&self, kind: WarningKind, field: &str) -> bool {
        self.warnings
            .iter()
            .any(|warning| warning.kind == kind && warning.field == field)
    }
}

const STALE_FIELDS: &[&str] = &[
    "audioDeviceIndex",
    "huesEnabled",
    "ledBoardEnabled",
    "huesOutputInSeparateThread",
    "ledBoardOutputInSeparateThread",
    "boardBeagleboneOPCAddress",
    "boardRowLength",
    "boardRowsPerStrip",
    "boardBrightness",
    "hueURL",
    "hueIndices",
    "kickT",
    "snareT",
];

const INERT_FIELDS: &[&str] = &["domeAutoFlashDelay"];

const VALID_BINDING_TARGETS: &[&str] = &[
    "domeBrightness",
    "domeVolumeAnimationSize",
    "domeVolumeRotationSpeed",
    "domeGradientSpeed",
    "domeGlobalFadeSpeed",
    "domeGlobalHueSpeed",
    "domeTwinkleDensity",
    "domeRippleCDStep",
    "domeRippleStep",
    "domeRadialSize",
    "domeRadialFrequency",
    "domeRadialCenterAngle",
    "domeRadialCenterDistance",
    "domeRadialCenterSpeed",
    "flashSpeed",
    "stageTracerSpeed",
    "colorPaletteIndex",
];

/// Analyze a Spectrum XML config and emit migration warnings.
#[must_use]
pub fn analyze_spectrum_xml(xml: &str) -> MigrationReport {
    let mut report = MigrationReport::default();
    for field in STALE_FIELDS {
        if xml.contains(&format!("<{field}>")) {
            report.warnings.push(MigrationWarning {
                kind: WarningKind::StaleField,
                field: (*field).to_string(),
            });
        }
    }
    for field in INERT_FIELDS {
        if xml.contains(&format!("<{field}>")) {
            report.warnings.push(MigrationWarning {
                kind: WarningKind::InertField,
                field: (*field).to_string(),
            });
        }
    }
    for target in config_property_names(xml) {
        if !VALID_BINDING_TARGETS.contains(&target.as_str()) {
            report.warnings.push(MigrationWarning {
                kind: WarningKind::InvalidMidiBindingTarget,
                field: target,
            });
        }
    }
    report
}

fn config_property_names(xml: &str) -> Vec<String> {
    let mut values = Vec::new();
    let mut remaining = xml;
    while let Some(start) = remaining.find("<configPropertyName>") {
        remaining = &remaining[start + "<configPropertyName>".len()..];
        let Some(end) = remaining.find("</configPropertyName>") else {
            break;
        };
        values.push(remaining[..end].trim().to_string());
        remaining = &remaining[end + "</configPropertyName>".len()..];
    }
    values
}

#[cfg(test)]
mod tests {
    use super::{analyze_spectrum_xml, WarningKind};

    #[test]
    fn warns_for_default_config_stale_fields_and_broken_bindings() {
        let xml = include_str!("../../../fixtures/config/spectrum_default_config.xml");
        let report = analyze_spectrum_xml(xml);

        assert!(report.contains(WarningKind::StaleField, "huesEnabled"));
        assert!(report.contains(WarningKind::StaleField, "kickT"));
        assert!(report.contains(WarningKind::StaleField, "snareT"));
        assert!(report.contains(WarningKind::InertField, "domeAutoFlashDelay"));
        assert!(!report.contains(WarningKind::InertField, "humanLinkOutput"));
        assert!(!report.contains(WarningKind::InertField, "madmomLinkOutput"));
        assert!(report.contains(WarningKind::InvalidMidiBindingTarget, "kickT"));
        assert!(report.contains(WarningKind::InvalidMidiBindingTarget, "snareT"));
    }

    #[test]
    fn accepts_known_live_binding_targets() {
        let xml = "<configPropertyName>domeBrightness</configPropertyName>";
        let report = analyze_spectrum_xml(xml);

        assert!(!report.contains(WarningKind::InvalidMidiBindingTarget, "domeBrightness"));
    }
}
