//! Domers config utility.

use std::{env, error::Error, fs, path::PathBuf};

use domers_core::import_spectrum_xml;

fn main() {
    if let Err(error) = run() {
        eprintln!("error: {error}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    let mut args = env::args().skip(1);
    match args.next().as_deref() {
        Some("import-spectrum-xml") => {
            let input = required_path(args.next(), "missing Spectrum XML input path")?;
            let output = required_path(args.next(), "missing Domers TOML output path")?;
            if args.next().is_some() {
                return Err("unexpected extra arguments".into());
            }
            import_spectrum_xml_command(&input, &output)
        }
        _ => Err(usage().into()),
    }
}

fn import_spectrum_xml_command(input: &PathBuf, output: &PathBuf) -> Result<(), Box<dyn Error>> {
    let xml = fs::read_to_string(input)?;
    let imported = import_spectrum_xml(&xml);
    let toml = imported.config.to_toml_string()?;
    fs::write(output, toml)?;

    for warning in imported.report.warnings {
        eprintln!("warning: {:?}: {}", warning.kind, warning.field);
    }

    Ok(())
}

fn required_path(value: Option<String>, message: &'static str) -> Result<PathBuf, Box<dyn Error>> {
    value.map(PathBuf::from).ok_or_else(|| message.into())
}

fn usage() -> &'static str {
    "usage: domers-config import-spectrum-xml <spectrum.xml> <domers.toml>"
}
