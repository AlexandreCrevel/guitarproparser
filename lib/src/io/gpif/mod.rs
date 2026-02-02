// GPIF (GP6/7 XML format) module

pub mod structures;
pub mod import;

// Re-export structures for convenience
pub use structures::*;
pub use import::SongGpifOps;
