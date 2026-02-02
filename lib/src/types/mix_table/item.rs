// MixTableItem structure

/// A mix table item describes a mix parameter, e.g. volume or reverb
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct MixTableItem {
    pub value: u8,
    pub duration: u8,
    pub all_tracks: bool,
}
