// Chord module - Chord structure and related types

pub mod chord;
pub mod pitch_class;
pub mod barre;

// Re-export main types
pub use chord::Chord;
pub use pitch_class::{PitchClass, SHARP_NOTES, FLAT_NOTES};
pub use barre::Barre;
