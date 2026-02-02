// Re-export beat from the new types and traits modules for backward compatibility
// This file is deprecated and will be removed in a future version.
// Please use `crate::types::beat::*` and `crate::traits::beat_ops::*` instead.

pub use crate::types::beat::*;
pub use crate::traits::beat_ops::SongBeatOps;
