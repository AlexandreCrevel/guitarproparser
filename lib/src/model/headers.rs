// Re-export headers from the new types and traits modules for backward compatibility
#![deprecated(
    since = "0.2.0",
    note = "Use crate::types::song, crate::types::measure instead"
)]
// This file is deprecated and will be removed in a future version.
// Please use `crate::types::song::*`, `crate::types::measure::*` and `crate::traits::header_ops::*` instead.

pub use crate::traits::header_ops::SongHeaderOps;
pub use crate::types::measure::{FermataType, Marker, MeasureFermata, MeasureHeader, RepeatGroup};
pub use crate::types::song::{Clipboard, Version};
