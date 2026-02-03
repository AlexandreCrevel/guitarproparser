// Chord module - Chord structure and related types

pub mod barre;
mod core;
pub mod pitch_class;

// Re-export main types
pub use barre::Barre;
pub use core::Chord;
pub use pitch_class::{PitchClass, FLAT_NOTES, SHARP_NOTES};
