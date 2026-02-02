// Re-export chord from the new types and traits modules for backward compatibility
// This file is deprecated and will be removed in a future version.
// Please use `crate::types::chord::*` and `crate::traits::chord_ops::*` instead.

pub use crate::types::chord::*;
pub use crate::traits::chord_ops::SongChordOps;
