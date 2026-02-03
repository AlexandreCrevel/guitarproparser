// Re-export beat from the new types and traits modules for backward compatibility
#![deprecated(since = "0.2.0", note = "Use crate::types::beat instead")]
// This file is deprecated and will be removed in a future version.
// Please use `crate::types::beat::*` and `crate::traits::beat_ops::*` instead.

pub use crate::traits::beat_ops::SongBeatOps;
pub use crate::types::beat::*;
