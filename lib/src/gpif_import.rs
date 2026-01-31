use crate::gp::Song;
use crate::track::{Track as SongTrack};
use crate::gpif::{Gpif, Bar, Voice, Beat, Note};
use crate::measure::Measure;
use crate::headers::MeasureHeader;
use crate::beat::Beat as SongBeat;
use crate::note::Note as SongNote;
use std::collections::HashMap;

impl Song {
    pub fn read_gpif(&mut self, gpif: &Gpif) {
        // 1. Metadata
        self.name = gpif.score.title.clone();
        self.artist = gpif.score.artist.clone();
        self.album = gpif.score.album.clone();
        self.words = gpif.score.words.clone();
        self.author = gpif.score.music.clone();
        self.writer = gpif.score.music.clone();
        self.transcriber = gpif.score.tabber.clone();
        self.copyright = gpif.score.copyright.clone();
        self.comments = gpif.score.instructions.clone();

        // 2. Measure Headers (MasterBars)
        self.measure_headers.clear();
        for mb in &gpif.master_bars.master_bars {
            let mut mh = MeasureHeader::default();
            // Simple parsing of 4/4
            let time_parts: Vec<&str> = mb.time.split('/').collect();
            if time_parts.len() == 2 {
                mh.time_signature.numerator = time_parts[0].parse().unwrap_or(4) as i8;
                mh.time_signature.denominator.value = time_parts[1].parse().unwrap_or(4) as u16;
            }
            // TODO: parse Key Signature
            self.measure_headers.push(mh);
        }

        // 3. Build Lookup Maps
        let bars_map: HashMap<i32, &Bar> = gpif.bars.bars.iter().map(|b| (b.id, b)).collect();
        let voices_map: HashMap<i32, &Voice> = gpif.voices.voices.iter().map(|v| (v.id, v)).collect();
        let beats_map: HashMap<i32, &Beat> = gpif.beats.beats.iter().map(|b| (b.id, b)).collect();
        let notes_map: HashMap<i32, &Note> = gpif.notes.notes.iter().map(|n| (n.id, n)).collect();

        // 4. Tracks
        self.tracks.clear();
        let num_measures = self.measure_headers.len();
        
        let mut bar_idx_counter = 0;

        for g_track in &gpif.tracks.tracks {
            let mut track = SongTrack::default();
            track.name = g_track.name.clone();
            
            // Simple color parsing "R G B"
            if let Some(color_str) = &g_track.color {
                let rgb: Vec<i32> = color_str.split_whitespace().filter_map(|s| s.parse().ok()).collect();
                if rgb.len() == 3 {
                    track.color = rgb[0] * 65536 + rgb[1] * 256 + rgb[2];
                } else {
                    track.color = 0;
                }
            } else {
                track.color = 0;
            }
            
            // TODO: Strings parsing

            for _m_idx in 0..num_measures {
                let bar_id = bar_idx_counter;
                bar_idx_counter += 1;

                let mut measure = Measure::default();
                if let Some(bar) = bars_map.get(&bar_id) {
                    // Parse Voices: "0 -1 -1 -1"
                    let voice_ids: Vec<i32> = bar.voices.split_whitespace()
                        .filter_map(|s| s.parse().ok())
                        .collect();
                    
                    measure.voices.clear(); // Clear default

                    for &vid in &voice_ids {
                         if vid < 0 { continue; } // -1 means no voice
                         let mut s_voice = crate::beat::Voice::default();
                        
                         if let Some(g_voice) = voices_map.get(&vid) {
                             let beat_ids: Vec<i32> = g_voice.beats.split_whitespace()
                                .filter_map(|s| s.parse().ok())
                                .collect();

                             for &bid in &beat_ids {
                                 if let Some(g_beat) = beats_map.get(&bid) {
                                     let mut s_beat = SongBeat::default();
                                     // Beat Duration/Rhythm?
                                     
                                     // Notes
                                     if let Some(notes_str) = &g_beat.notes {
                                         let note_ids: Vec<i32> = notes_str.split_whitespace()
                                            .filter_map(|s| s.parse().ok())
                                            .collect();
                                         for &nid in &note_ids {
                                             if let Some(g_note) = notes_map.get(&nid) {
                                                 let mut s_note = SongNote::default();
                                                 
                                                 // Iterate over properties to find Fret, String, etc.
                                                 for prop in &g_note.properties.properties {
                                                     match prop.name.as_str() {
                                                         "Fret" => {
                                                             if let Some(f) = prop.fret { s_note.value = f as i16; }
                                                         },
                                                         "String" => {
                                                             if let Some(s) = prop.string { s_note.string = s as i8; } 
                                                         },
                                                         "Midi" => {
                                                             // if let Some(m) = prop.number { s_note.velocity = m as i16; }
                                                         },
                                                         _ => {}
                                                     }
                                                 }
                                                 s_beat.notes.push(s_note);
                                             }
                                         }
                                     }
                                     s_voice.beats.push(s_beat);
                                 }
                             }
                         }
                         measure.voices.push(s_voice);
                    }
                }
                track.measures.push(measure);
            }
            self.tracks.push(track);
        }
    }
}
