// Re-export note from the new types and traits modules for backward compatibility
#![deprecated(since = "0.2.0", note = "Use crate::types::note instead")]
// This file is deprecated and will be removed in a future version.
// Please use `crate::types::note::*` and `crate::traits::note_ops::*` instead.

pub use crate::traits::note_ops::SongNoteOps;
pub use crate::types::note::*;
