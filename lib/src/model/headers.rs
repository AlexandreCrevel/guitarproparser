// Re-export headers from the new types and traits modules for backward compatibility
// This file is deprecated and will be removed in a future version.
// Please use `crate::types::song::*`, `crate::types::measure::*` and `crate::traits::header_ops::*` instead.

pub use crate::types::song::{Version, Clipboard};
pub use crate::types::measure::{MeasureHeader, Marker, FermataType, MeasureFermata, RepeatGroup};
pub use crate::traits::header_ops::SongHeaderOps;
