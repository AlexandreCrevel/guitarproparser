use super::common::EnableTag;
use serde::Deserialize;

// ---------------------------------------------------------------------------
// MasterBars (measure headers)
// ---------------------------------------------------------------------------

#[derive(Debug, Deserialize)]
pub struct MasterBarsWrapper {
    #[serde(rename = "MasterBar", default)]
    pub master_bars: Vec<MasterBar>,
}

#[derive(Debug, Deserialize)]
pub struct MasterBar {
    #[serde(rename = "Key", default)]
    pub key: Option<Key>,
    #[serde(rename = "Time", default)]
    pub time: String,
    #[serde(rename = "Bars", default)]
    pub bars: String,
    #[serde(rename = "Repeat", default)]
    pub repeat: Option<Repeat>,
    #[serde(rename = "AlternateEndings", default)]
    pub alternate_endings: Option<String>,
    #[serde(rename = "DoubleBar", default)]
    pub double_bar: Option<String>,
    #[serde(rename = "Section", default)]
    pub section: Option<Section>,
    #[serde(rename = "Fermatas", default)]
    pub fermatas: Option<FermatasWrapper>,
    #[serde(rename = "FreeTime", default)]
    pub free_time: Option<String>,
    #[serde(rename = "Directions", default)]
    pub directions: Option<DirectionsWrapper>,
}

#[derive(Debug, Deserialize)]
pub struct Key {
    #[serde(rename = "AccidentalCount", default)]
    pub accidental_count: i32,
    #[serde(rename = "Mode", default)]
    pub mode: String,
}

impl Default for Key {
    fn default() -> Self {
        Key {
            accidental_count: 0,
            mode: "Major".to_string(),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct Repeat {
    #[serde(rename = "@start", default)]
    pub start: String,
    #[serde(rename = "@end", default)]
    pub end: String,
    #[serde(rename = "@count", default)]
    pub count: i32,
}

#[derive(Debug, Deserialize)]
pub struct Section {
    #[serde(rename = "Letter", default)]
    pub letter: Option<String>,
    #[serde(rename = "Text", default)]
    pub text: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct DirectionsWrapper {
    #[serde(rename = "Target", default)]
    pub target: Option<String>,
    #[serde(rename = "Jump", default)]
    pub jump: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct FermatasWrapper {
    #[serde(rename = "Fermata", default)]
    pub fermatas: Vec<Fermata>,
}

#[derive(Debug, Deserialize)]
pub struct Fermata {
    #[serde(rename = "Type", default)]
    pub fermata_type: Option<String>,
    #[serde(rename = "Offset", default)]
    pub offset: Option<String>,
    #[serde(rename = "Length", default)]
    pub length: Option<String>,
}

// ---------------------------------------------------------------------------
// Bars (per-track measures)
// ---------------------------------------------------------------------------

#[derive(Debug, Deserialize)]
pub struct BarsWrapper {
    #[serde(rename = "Bar", default)]
    pub bars: Vec<Bar>,
}

#[derive(Debug, Deserialize)]
pub struct Bar {
    #[serde(rename = "@id", default)]
    pub id: i32,
    #[serde(rename = "Voices", default)]
    pub voices: String,
    #[serde(rename = "Clef", default)]
    pub clef: Option<String>,
    #[serde(rename = "SimileMark", default)]
    pub simile_mark: Option<String>,
}

// ---------------------------------------------------------------------------
// Voices
// ---------------------------------------------------------------------------

#[derive(Debug, Deserialize)]
pub struct VoicesWrapper {
    #[serde(rename = "Voice", default)]
    pub voices: Vec<Voice>,
}

#[derive(Debug, Deserialize)]
pub struct Voice {
    #[serde(rename = "@id", default)]
    pub id: i32,
    #[serde(rename = "Beats", default)]
    pub beats: String,
}

// ---------------------------------------------------------------------------
// Beats
// ---------------------------------------------------------------------------

#[derive(Debug, Deserialize)]
pub struct BeatsWrapper {
    #[serde(rename = "Beat", default)]
    pub beats: Vec<Beat>,
}

#[derive(Debug, Deserialize)]
pub struct Beat {
    #[serde(rename = "@id", default)]
    pub id: i32,
    #[serde(rename = "Notes", default)]
    pub notes: Option<String>,
    #[serde(rename = "Rhythm", default)]
    pub rhythm: Option<RhythmRef>,
    #[serde(rename = "Dynamic", default)]
    pub dynamic: Option<String>,
    #[serde(rename = "GraceNotes", default)]
    pub grace_notes: Option<String>,
    /// Note: "Fadding" is a typo in the upstream GP6 XML format (should be "Fading").
    #[serde(rename = "Fadding", default)]
    pub fadding: Option<String>,
    #[serde(rename = "Tremolo", default)]
    pub tremolo: Option<String>,
    #[serde(rename = "Wah", default)]
    pub wah: Option<String>,
    #[serde(rename = "FreeText", default)]
    pub free_text: Option<String>,
    #[serde(rename = "Arpeggio", default)]
    pub arpeggio: Option<String>,
    #[serde(rename = "Ottavia", default)]
    pub ottavia: Option<String>,
    #[serde(rename = "Whammy", default)]
    pub whammy: Option<WhammyInfo>,
    #[serde(rename = "Properties", default)]
    pub properties: Option<BeatPropertiesWrapper>,
}

#[derive(Debug, Deserialize)]
pub struct WhammyInfo {
    #[serde(rename = "@originValue", default)]
    pub origin_value: Option<f64>,
    #[serde(rename = "@middleValue", default)]
    pub middle_value: Option<f64>,
    #[serde(rename = "@destinationValue", default)]
    pub destination_value: Option<f64>,
    #[serde(rename = "@originOffset", default)]
    pub origin_offset: Option<f64>,
    #[serde(rename = "@middleOffset1", default)]
    pub middle_offset1: Option<f64>,
    #[serde(rename = "@middleOffset2", default)]
    pub middle_offset2: Option<f64>,
    #[serde(rename = "@destinationOffset", default)]
    pub destination_offset: Option<f64>,
}

#[derive(Debug, Deserialize)]
pub struct BeatPropertiesWrapper {
    #[serde(rename = "Property", default)]
    pub properties: Vec<BeatProperty>,
}

#[derive(Debug, Deserialize)]
pub struct BeatProperty {
    #[serde(rename = "@name", default)]
    pub name: String,
    #[serde(rename = "Direction", default)]
    pub direction: Option<String>,
    #[serde(rename = "Enable", default)]
    pub enable: Option<EnableTag>,
    #[serde(rename = "Float", default)]
    pub float: Option<f64>,
    #[serde(rename = "Flags", default)]
    pub flags: Option<i32>,
    #[serde(rename = "Strength", default)]
    pub strength: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct RhythmRef {
    #[serde(rename = "@ref", default)]
    pub r#ref: i32,
}

// ---------------------------------------------------------------------------
// Rhythms
// ---------------------------------------------------------------------------

#[derive(Debug, Deserialize)]
pub struct RhythmsWrapper {
    #[serde(rename = "Rhythm", default)]
    pub rhythms: Vec<Rhythm>,
}

#[derive(Debug, Deserialize)]
pub struct Rhythm {
    #[serde(rename = "@id", default)]
    pub id: i32,
    #[serde(rename = "NoteValue", default)]
    pub note_value: String,
    #[serde(rename = "AugmentationDot", default)]
    pub augmentation_dot: Option<AugmentationDot>,
    #[serde(rename = "PrimaryTuplet", default)]
    pub primary_tuplet: Option<PrimaryTuplet>,
}

#[derive(Debug, Deserialize)]
pub struct AugmentationDot {
    #[serde(rename = "@count", default)]
    pub count: i32,
}

#[derive(Debug, Deserialize)]
pub struct PrimaryTuplet {
    #[serde(rename = "@num", default)]
    pub num: i32,
    #[serde(rename = "@den", default)]
    pub den: i32,
}
