// Beat module - Beat structure and related types

mod core;
pub mod display;
pub mod effects;
pub mod stroke;
pub mod voice;

// Re-export for convenience
pub use core::Beat;
pub use display::BeatDisplay;
pub use effects::BeatEffects;
pub use stroke::BeatStroke;
pub use voice::Voice;
