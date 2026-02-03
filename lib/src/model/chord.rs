// Re-export chord from the new types and traits modules for backward compatibility
#![deprecated(since = "0.2.0", note = "Use crate::types::chord instead")]
// This file is deprecated and will be removed in a future version.
// Please use `crate::types::chord::*` and `crate::traits::chord_ops::*` instead.

pub use crate::traits::chord_ops::SongChordOps;
pub use crate::types::chord::*;
