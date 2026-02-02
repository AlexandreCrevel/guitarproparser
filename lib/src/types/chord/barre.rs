// Barre structure

/// A single barre
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Barre {
    pub fret: i8,
    /// First string from the bottom of the barre
    pub start: i8,
    /// Last string on the top of the barre
    pub end: i8,
}
