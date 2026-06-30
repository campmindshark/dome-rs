//! Output-side protocols and fixture topology.

pub mod commands;
pub mod opc;
pub mod simulator;
pub mod topology;

pub use commands::{BarCommand, DomeCommand, StageCommand};
pub use opc::encode_frame;
pub use simulator::{DomeOutputSink, SimulatorColor};
