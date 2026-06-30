//! Core shared types for the Domers Spectrum rewrite.

pub mod beat;
pub mod color;
pub mod config;
pub mod migration;

pub use beat::{BeatBroadcaster, BeatClock};
pub use color::Rgb;
pub use config::EngineConfig;
pub use migration::{analyze_spectrum_xml, MigrationReport, WarningKind};
