// Traits module - All operation traits
// These traits define the interfaces for reading/writing Guitar Pro formats

pub mod header_ops;
pub mod effect_ops;
pub mod beat_ops;
pub mod note_ops;
pub mod chord_ops;
pub mod mix_table_ops;

// Re-export for convenience
pub use header_ops::SongHeaderOps;
pub use effect_ops::SongEffectOps;
pub use beat_ops::SongBeatOps;
pub use note_ops::SongNoteOps;
pub use chord_ops::SongChordOps;
pub use mix_table_ops::SongMixTableOps;

// TODO: Phases 7-8, 11 - Create remaining trait files
// pub mod song_ops;
// pub mod track_ops;
// pub mod measure_ops;
// pub mod beat_ops;
// pub mod note_ops;
// pub mod effect_ops;
// pub mod chord_ops;
// pub mod mix_table_ops;
// pub mod midi_ops;
// pub mod page_ops;
// pub mod lyric_ops;
// pub mod rse_ops;
// pub mod gpif_ops;
