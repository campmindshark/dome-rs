use domers_core::{WarningKind, analyze_spectrum_xml};
use domers_outputs::DomeCommand;
use domers_server::ServerState;

#[test]
fn no_hardware_server_migration_and_simulator_smoke() {
    let xml = include_str!("../../fixtures/config/spectrum_default_config.xml");
    let report = analyze_spectrum_xml(xml);
    assert!(report.contains(WarningKind::StaleField, "kickT"));
    assert!(report.contains(WarningKind::InvalidMidiBindingTarget, "snareT"));

    let mut server = ServerState::default();
    server.start();
    server.patch_dome_active_vis(1);

    for _ in 0..60 {
        let frame = server.simulator_frame();
        assert!(frame.iter().any(|command| matches!(command, DomeCommand::Flush)));
    }

    assert_eq!(server.metrics().frames, 60);
    assert_eq!(server.metrics().simulator_frames, 60);
}
