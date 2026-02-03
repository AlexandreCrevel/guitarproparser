use serde::Deserialize;

use super::measure::{BarsWrapper, BeatsWrapper, MasterBarsWrapper, RhythmsWrapper, VoicesWrapper};
use super::note::NotesWrapper;
use super::track::TracksWrapper;

#[derive(Debug, Deserialize)]
pub struct Gpif {
    /// GP7 uses "GPVersion", GP6 uses "GPRevision"
    #[serde(rename = "GPVersion", default)]
    pub version: Option<String>,
    #[serde(rename = "GPRevision", default)]
    pub revision: Option<String>,
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

// ---------------------------------------------------------------------------
// Score metadata
// ---------------------------------------------------------------------------

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Score {
    #[serde(default)]
    pub title: String,
    #[serde(default)]
    pub sub_title: String,
    #[serde(default)]
    pub artist: String,
    #[serde(default)]
    pub album: String,
    #[serde(rename = "Words", default)]
    pub words: String,
    #[serde(rename = "Music", default)]
    pub music: String,
    #[serde(default)]
    pub copyright: String,
    #[serde(default)]
    pub tabber: String,
    #[serde(default)]
    pub instructions: String,
    #[serde(default)]
    pub notices: String,
}

// ---------------------------------------------------------------------------
// MasterTrack (tempo automations)
// ---------------------------------------------------------------------------

#[derive(Debug, Deserialize)]
pub struct MasterTrack {
    #[serde(rename = "Tracks", default)]
    pub tracks_count: String,
    #[serde(rename = "Automations", default)]
    pub automations: Option<AutomationsWrapper>,
    #[serde(rename = "RSE", default)]
    pub rse: Option<RseMasterWrapper>,
}

#[derive(Debug, Deserialize)]
pub struct RseMasterWrapper {
    #[serde(rename = "Master", default)]
    pub master: Option<RseMaster>,
}

#[derive(Debug, Deserialize)]
pub struct RseMaster {
    #[serde(rename = "Volume", default)]
    pub volume: Option<f32>,
    #[serde(rename = "Effect", default)]
    pub effects: Vec<RseEffect>,
}

// We need RseEffect here? It is used in Track as well.
// Let's check where RseEffect is defined. It was in structures.rs.
// It's used in Rail (Track) and RseMaster (Score).
// I should probably put RseEffect in a shared place or just duplicate/export it.
// `track` module seems to use it more heavily (EffectChain etc).
// I'll import it from `track` module or create `common.rs`.
// For now, I'll put it in `track` and import it, or `rse.rs`.
// Let's stick to `track` for now as `RseEffect` is often track related, but `MasterTrack` uses it too.
// Actually, I'll check `structures.rs` again. `RseEffect` is generic.
// I'll put `RseEffect` in `track.rs` and import it here.

use super::track::RseEffect;

#[derive(Debug, Deserialize)]
pub struct AutomationsWrapper {
    #[serde(rename = "Automation", default)]
    pub automations: Vec<Automation>,
}

#[derive(Debug, Deserialize)]
pub struct Automation {
    #[serde(rename = "Type", default)]
    pub automation_type: String,
    #[serde(rename = "Value", default)]
    pub value: String,
    #[serde(rename = "Bar", default)]
    pub bar: i32,
    #[serde(rename = "Position", default)]
    pub position: i32,
}
