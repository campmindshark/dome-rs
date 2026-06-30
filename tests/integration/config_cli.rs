use std::{fs, process::Command};

#[test]
fn imports_spectrum_xml_to_domers_toml_file() {
    let output = std::env::temp_dir().join(format!(
        "domers-config-{}-{}.toml",
        std::process::id(),
        "spectrum-import"
    ));
    let input = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/fixtures/config/spectrum_default_config.xml"
    );

    let status = Command::new(env!("CARGO_BIN_EXE_domers-config"))
        .args(["import-spectrum-xml", input, output.to_str().expect("valid temp path")])
        .status()
        .expect("domers-config should run");

    assert!(status.success());

    let toml = fs::read_to_string(&output).expect("output toml should exist");
    assert!(toml.contains("[dome]"));
    assert!(toml.contains("[stage]"));
    assert!(toml.contains("[madmom]"));

    let _ = fs::remove_file(output);
}
