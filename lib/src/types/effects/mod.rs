// Effects module - All effect types

pub mod bend;
pub mod grace;
pub mod harmonic;
pub mod tremolo;
pub mod trill;
pub mod velocity;

// Re-export for convenience
pub use bend::{BendEffect, BendPoint, BEND_EFFECT_MAX_POSITION, GP_BEND_SEMITONE, GP_BEND_POSITION, GP_BEND_SEMITONE_LENGTH};
pub use grace::GraceEffect;
pub use harmonic::HarmonicEffect;
pub use tremolo::TremoloPickingEffect;
pub use trill::TrillEffect;
pub use velocity::{MIN_VELOCITY, VELOCITY_INCREMENT, FORTE, DEFAULT_VELOCITY, unpack_velocity, pack_velocity};
