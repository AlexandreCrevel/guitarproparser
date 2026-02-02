// Types module - Pure data structures
// This module contains all the data structures used in the Guitar Pro parser
// Organized by domain: song, track, measure, beat, note, effects, chord, mix_table, enums

pub mod song;
pub mod track;
pub mod measure;
pub mod beat;
pub mod note;
pub mod effects;
pub mod chord;
pub mod mix_table;
pub mod enums;

// Re-export key structures for convenience
pub use song::*;
pub use measure::*;
pub use enums::*;
pub use effects::*;
pub use beat::*;
pub use note::*;
pub use chord::*;
pub use mix_table::*;

// TODO: Uncomment as modules are implemented
// pub use track::*;
