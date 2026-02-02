// Measure module - Measure structure and related types

pub mod header;
pub mod marker;
pub mod fermata;

// Re-export for convenience
pub use header::{MeasureHeader, RepeatGroup};
pub use marker::Marker;
pub use fermata::{FermataType, MeasureFermata};

// TODO: Phase 11 - Move Measure structure here
// pub mod measure;
