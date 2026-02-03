// Traits module - All operation traits
// These traits define the interfaces for reading/writing Guitar Pro formats

pub mod beat_ops;
pub mod chord_ops;
pub mod effect_ops;
pub mod header_ops;
pub mod mix_table_ops;
pub mod note_ops;

// Re-export for convenience
pub use beat_ops::SongBeatOps;
pub use chord_ops::SongChordOps;
pub use effect_ops::SongEffectOps;
pub use header_ops::SongHeaderOps;
pub use mix_table_ops::SongMixTableOps;
pub use note_ops::SongNoteOps;
