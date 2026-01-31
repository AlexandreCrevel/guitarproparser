use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Gpif {
    #[serde(rename = "GPVersion")]
    pub version: String,
    #[serde(rename = "Score")]
    pub score: Score,
    #[serde(rename = "MasterTrack")]
    pub master_track: MasterTrack,
    #[serde(rename = "Tracks")]
    pub tracks: TracksWrapper,
    #[serde(rename = "MasterBars")]
    pub master_bars: MasterBarsWrapper,
    #[serde(rename = "Bars")]
    pub bars: BarsWrapper,
    #[serde(rename = "Voices")]
    pub voices: VoicesWrapper,
    #[serde(rename = "Beats")]
    pub beats: BeatsWrapper,
    #[serde(rename = "Notes")]
    pub notes: NotesWrapper,
    #[serde(rename = "Rhythms")]
    pub rhythms: RhythmsWrapper,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Score {
    pub title: String,
    pub sub_title: String,
    pub artist: String,
    pub album: String,
    #[serde(rename = "Words")]
    pub words: String,
    #[serde(rename = "Music")]
    pub music: String,
    pub copyright: String,
    pub tabber: String,
    pub instructions: String,
    pub notices: String,
}

#[derive(Debug, Deserialize)]
pub struct MasterTrack {
    #[serde(rename = "Tracks", default)]
    pub tracks_count: i32,
    // Automations, RSE...
}

#[derive(Debug, Deserialize)]
pub struct TracksWrapper {
    #[serde(rename = "Track", default)]
    pub tracks: Vec<Track>,
}

#[derive(Debug, Deserialize)]
pub struct Track {
    #[serde(rename = "@id", default)]
    pub id: i32,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "ShortName")]
    pub short_name: String,
    #[serde(rename = "Color")]
    pub color: Option<String>,
    // Properties...
}

#[derive(Debug, Deserialize)]
pub struct MasterBarsWrapper {
    #[serde(rename = "MasterBar", default)]
    pub master_bars: Vec<MasterBar>,
}

#[derive(Debug, Deserialize)]
pub struct MasterBar {
    #[serde(rename = "Key")]
    pub key: Key,
    #[serde(rename = "Time")]
    pub time: String, // "4/4"
    #[serde(rename = "Bars")]
    pub bars: String, // Seems to be an index or count string?
}

#[derive(Debug, Deserialize)]
pub struct Key {
    #[serde(rename = "AccidentalCount", default)]
    pub accidental_count: i32,
    #[serde(rename = "Mode")]
    pub mode: String, // "Major"
}

#[derive(Debug, Deserialize)]
pub struct BarsWrapper {
    #[serde(rename = "Bar", default)]
    pub bars: Vec<Bar>,
}

#[derive(Debug, Deserialize)]
pub struct Bar {
    #[serde(rename = "@id", default)]
    pub id: i32,
    #[serde(rename = "Voices")]
    pub voices: String, // "0 -1 -1 -1"
    #[serde(rename = "Clef")]
    pub clef: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct VoicesWrapper {
    #[serde(rename = "Voice", default)]
    pub voices: Vec<Voice>,
}

#[derive(Debug, Deserialize)]
pub struct Voice {
    #[serde(rename = "@id", default)]
    pub id: i32,
    #[serde(rename = "Beats")]
    pub beats: String, // "0 1 2 3"
}

#[derive(Debug, Deserialize)]
pub struct BeatsWrapper {
    #[serde(rename = "Beat", default)]
    pub beats: Vec<Beat>,
}

#[derive(Debug, Deserialize)]
pub struct Beat {
    #[serde(rename = "@id", default)]
    pub id: i32,
    #[serde(rename = "Notes")]
    pub notes: Option<String>, 
    #[serde(rename = "Rhythm")]
    pub rhythm: Option<RhythmRef>, 
}

#[derive(Debug, Deserialize)]
pub struct RhythmRef {
    #[serde(rename = "@ref", default)]
    pub r#ref: i32,
}

#[derive(Debug, Deserialize)]
pub struct NotesWrapper {
    #[serde(rename = "Note", default)]
    pub notes: Vec<Note>,
}

#[derive(Debug, Deserialize)]
pub struct Note {
    #[serde(rename = "@id", default)]
    pub id: i32,
    #[serde(rename = "Properties")]
    pub properties: NoteProperties,
}

#[derive(Debug, Deserialize)]
pub struct NoteProperties {
    #[serde(rename = "Property", default)]
    pub properties: Vec<Property>,
}

#[derive(Debug, Deserialize)]
pub struct Property {
    #[serde(rename = "@name")]
    pub name: String,
    
    #[serde(rename = "Fret")]
    pub fret: Option<i32>,
    #[serde(rename = "String")]
    pub string: Option<i32>,
    #[serde(rename = "Pitch")]
    pub pitch: Option<Pitch>,
    #[serde(rename = "Number")]
    pub number: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct Pitch {
    #[serde(rename = "Step")]
    pub step: String,
    #[serde(rename = "Octave", default)]
    pub octave: i32,
    #[serde(rename = "Accidental")]
    pub accidental: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Fret {
    #[serde(rename = "Fret", default)]
    pub fret: i32,
}

#[derive(Debug, Deserialize)]
pub struct GpString {
    #[serde(rename = "String", default)]
    pub string: i32,
}

#[derive(Debug, Deserialize)]
pub struct Midi {
    #[serde(rename = "Number", default)]
    pub number: i32,
}

#[derive(Debug, Deserialize)]
pub struct RhythmsWrapper {
    #[serde(rename = "Rhythm", default)]
    pub rhythms: Vec<Rhythm>,
}

#[derive(Debug, Deserialize)]
pub struct Rhythm {
    #[serde(rename = "@id", default)]
    pub id: i32,
    #[serde(rename = "NoteValue")]
    pub note_value: String, // "Quarter"
    // AugmentationDot, etc.
}
