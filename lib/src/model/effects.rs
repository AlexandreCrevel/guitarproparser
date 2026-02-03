// Re-export effects from the new types and traits modules for backward compatibility
#![deprecated(since = "0.2.0", note = "Use crate::types::effects instead")]
// This file is deprecated and will be removed in a future version.
// Please use `crate::types::effects::*` and `crate::traits::effect_ops::*` instead.

pub use crate::traits::effect_ops::SongEffectOps;
pub use crate::types::effects::*;
