use crate::model::song::Song;
use fraction::ToPrimitive;
use std::{fs, io::Read};

fn read_file(path: String) -> Vec<u8> {
    let test_path = if path.starts_with("test/") {
        format!("../{}", path)
    } else {
        format!("../test/{}", path)
    };
    let f = fs::OpenOptions::new()
        .read(true)
        .open(&test_path)
        .unwrap_or_else(|e| panic!("Cannot open file '{}': {}", test_path, e));
    let size: usize = fs::metadata(&test_path)
        .unwrap_or_else(|e| panic!("Unable to get file size for '{}': {}", test_path, e))
        .len()
        .to_usize()
        .unwrap();
    let mut data: Vec<u8> = Vec::with_capacity(size);
    f.take(size as u64)
        .read_to_end(&mut data)
        .unwrap_or_else(|e| panic!("Unable to read file contents from '{}': {}", test_path, e));
    data
}

#[test]
fn test_gp3_chord() {
    let mut song: Song = Song::default();
    song.read_gp3(&read_file(String::from("test/Chords.gp3")));
}
#[test]
fn test_gp4_chord() {
    let mut song: Song = Song::default();
    song.read_gp4(&read_file(String::from("test/Chords.gp4")));
}
#[test]
fn test_gp5_chord() {
    let mut song: Song = Song::default();
    song.read_gp5(&read_file(String::from("test/Chords.gp5")));
}
#[test]
fn test_gp5_unknown_chord_extension() {
    let mut song: Song = Song::default();
    song.read_gp5(&read_file(String::from("test/Unknown Chord Extension.gp5")));
}
#[test]
fn test_gp5_chord_without_notes() {
    let mut song: Song = Song::default();
    song.read_gp5(&read_file(String::from("test/chord_without_notes.gp5")));
    let mut song: Song = Song::default();
    song.read_gp5(&read_file(String::from("test/001_Funky_Guy.gp5")));
}

#[test]
fn test_gp3_duration() {
    let mut song: Song = Song::default();
    song.read_gp3(&read_file(String::from("test/Duration.gp3")));
}

#[test]
fn test_gp3_effects() {
    let mut song: Song = Song::default();
    song.read_gp3(&read_file(String::from("test/Effects.gp3")));
}
#[test]
fn test_gp4_effects() {
    let mut song: Song = Song::default();
    song.read_gp4(&read_file(String::from("test/Effects.gp4")));
}
#[test]
fn test_gp5_effects() {
    let mut song: Song = Song::default();
    song.read_gp5(&read_file(String::from("test/Effects.gp5")));
}

#[test]
fn test_gp3_harmonics() {
    let mut song: Song = Song::default();
    song.read_gp3(&read_file(String::from("test/Harmonics.gp3")));
}
#[test]
fn test_gp4_harmonics() {
    let mut song: Song = Song::default();
    song.read_gp4(&read_file(String::from("test/Harmonics.gp4")));
}
#[test]
fn test_gp5_harmonics() {
    let mut song: Song = Song::default();
    song.read_gp5(&read_file(String::from("test/Harmonics.gp5")));
}

#[test]
fn test_gp4_key() {
    let mut song: Song = Song::default();
    song.read_gp4(&read_file(String::from("test/Key.gp4")));
}
#[test]
fn test_gp5_key() {
    let mut song: Song = Song::default();
    song.read_gp5(&read_file(String::from("test/Key.gp5")));
}

#[test]
fn test_gp4_repeat() {
    let mut song: Song = Song::default();
    song.read_gp4(&read_file(String::from("test/Repeat.gp4")));
}
#[test]
fn test_gp5_repeat() {
    let mut song: Song = Song::default();
    song.read_gp5(&read_file(String::from("test/Repeat.gp5")));
}

#[test]
fn test_gp5_rse() {
    let mut song: Song = Song::default();
    song.read_gp5(&read_file(String::from("test/RSE.gp5")));
}

#[test]
fn test_gp4_slides() {
    let mut song: Song = Song::default();
    song.read_gp4(&read_file(String::from("test/Slides.gp4")));
}
#[test]
fn test_gp5_slides() {
    let mut song: Song = Song::default();
    song.read_gp5(&read_file(String::from("test/Slides.gp5")));
}

#[test]
fn test_gp4_strokes() {
    let mut song: Song = Song::default();
    song.read_gp4(&read_file(String::from("test/Strokes.gp4")));
}
#[test]
fn test_gp5_strokes() {
    let mut song: Song = Song::default();
    song.read_gp5(&read_file(String::from("test/Strokes.gp5")));
}

#[test]
fn test_gp4_vibrato() {
    let mut song: Song = Song::default();
    song.read_gp4(&read_file(String::from("test/Vibrato.gp4")));
}

#[test]
fn test_gp5_voices() {
    let mut song: Song = Song::default();
    song.read_gp5(&read_file(String::from("test/Voices.gp5")));
}

#[test]
fn test_gp5_no_wah() {
    let mut song: Song = Song::default();
    song.read_gp5(&read_file(String::from("test/No Wah.gp5")));
}
#[test]
fn test_gp5_wah() {
    let mut song: Song = Song::default();
    song.read_gp5(&read_file(String::from("test/Wah.gp5")));
}
#[test]
fn test_gp5_wah_m() {
    let mut song: Song = Song::default();
    song.read_gp5(&read_file(String::from("test/Wah-m.gp5")));
}

#[test]
fn test_gp5_all_percussion() {
    let mut song: Song = Song::default();
    song.read_gp5(&read_file(String::from("test/all-percussion.gp5")));
}
#[test]
fn test_gp5_basic_bend() {
    let mut song: Song = Song::default();
    song.read_gp5(&read_file(String::from("test/basic-bend.gp5")));
}
#[test]
fn test_gp5_beams_sterms_ledger_lines() {
    let mut song: Song = Song::default();
    song.read_gp5(&read_file(String::from(
        "test/beams-stems-ledger-lines.gp5",
    )));
}
#[test]
fn test_gp5_brush() {
    let mut song: Song = Song::default();
    song.read_gp5(&read_file(String::from("test/brush.gp5")));
}
#[test]
fn test_gp3_capo_fret() {
    let mut song: Song = Song::default();
    song.read_gp3(&read_file(String::from("test/capo-fret.gp3")));
}
#[test]
fn test_gp4_capo_fret() {
    let mut song: Song = Song::default();
    song.read_gp4(&read_file(String::from("test/capo-fret.gp4")));
}
#[test]
fn test_gp5_capo_fret() {
    let mut song: Song = Song::default();
    song.read_gp5(&read_file(String::from("test/capo-fret.gp5")));
}
#[test]
fn test_gp3_copyright() {
    let mut song: Song = Song::default();
    song.read_gp3(&read_file(String::from("test/copyright.gp3")));
}
#[test]
fn test_gp4_copyright() {
    let mut song: Song = Song::default();
    song.read_gp4(&read_file(String::from("test/copyright.gp4")));
}
#[test]
fn test_gp5_copyright() {
    let mut song: Song = Song::default();
    song.read_gp5(&read_file(String::from("test/copyright.gp5")));
}
#[test]
fn test_gp3_dotted_gliss() {
    let mut song: Song = Song::default();
    song.read_gp3(&read_file(String::from("test/dotted-gliss.gp3")));
}
#[test]
fn test_gp5_dotted_tuplets() {
    let mut song: Song = Song::default();
    song.read_gp5(&read_file(String::from("test/dotted-tuplets.gp5")));
}
#[test]
fn test_gp5_dynamic() {
    let mut song: Song = Song::default();
    song.read_gp5(&read_file(String::from("test/dynamic.gp5")));
}
#[test]
fn test_gp4_fade_in() {
    let mut song: Song = Song::default();
    song.read_gp4(&read_file(String::from("test/fade-in.gp4")));
}
#[test]
fn test_gp5_fade_in() {
    let mut song: Song = Song::default();
    song.read_gp5(&read_file(String::from("test/fade-in.gp5")));
}
#[test]
fn test_gp4_fingering() {
    let mut song: Song = Song::default();
    song.read_gp4(&read_file(String::from("test/fingering.gp4")));
}
#[test]
fn test_gp5_fingering() {
    let mut song: Song = Song::default();
    song.read_gp5(&read_file(String::from("test/fingering.gp5")));
}
#[test]
fn test_gp4_fret_diagram() {
    let mut song: Song = Song::default();
    song.read_gp4(&read_file(String::from("test/fret-diagram.gp4")));
}
#[test]
fn test_gp5_fret_diagram() {
    let mut song: Song = Song::default();
    song.read_gp5(&read_file(String::from("test/fret-diagram.gp5")));
}
#[test]
fn test_gp3_ghost_note() {
    let mut song: Song = Song::default();
    song.read_gp3(&read_file(String::from("test/ghost_note.gp3")));
}
#[test]
fn test_gp5_grace() {
    let mut song: Song = Song::default();
    song.read_gp5(&read_file(String::from("test/grace.gp5")));
}
#[test]
fn test_gp5_heavy_accent() {
    let mut song: Song = Song::default();
    song.read_gp5(&read_file(String::from("test/heavy-accent.gp5")));
}
#[test]
fn test_gp3_high_pitch() {
    let mut song: Song = Song::default();
    song.read_gp3(&read_file(String::from("test/high-pitch.gp3")));
}
#[test]
fn test_gp4_keysig() {
    let mut song: Song = Song::default();
    song.read_gp4(&read_file(String::from("test/keysig.gp4")));
}
#[test]
fn test_gp5_keysig() {
    let mut song: Song = Song::default();
    song.read_gp5(&read_file(String::from("test/keysig.gp5")));
}
#[test]
fn test_gp4_legato_slide() {
    let mut song: Song = Song::default();
    song.read_gp4(&read_file(String::from("test/legato-slide.gp4")));
}
#[test]
fn test_gp5_legato_slide() {
    let mut song: Song = Song::default();
    song.read_gp5(&read_file(String::from("test/legato-slide.gp5")));
}
#[test]
fn test_gp4_let_ring() {
    let mut song: Song = Song::default();
    song.read_gp4(&read_file(String::from("test/let-ring.gp4")));
}
#[test]
fn test_gp5_let_ring() {
    let mut song: Song = Song::default();
    song.read_gp5(&read_file(String::from("test/let-ring.gp5")));
}
#[test]
fn test_gp4_palm_mute() {
    let mut song: Song = Song::default();
    song.read_gp4(&read_file(String::from("test/palm-mute.gp4")));
}
#[test]
fn test_gp5_palm_mute() {
    let mut song: Song = Song::default();
    song.read_gp5(&read_file(String::from("test/palm-mute.gp5")));
}
#[test]
fn test_gp4_pick_up_down() {
    let mut song: Song = Song::default();
    song.read_gp4(&read_file(String::from("test/pick-up-down.gp4")));
}
#[test]
fn test_gp5_pick_up_down() {
    let mut song: Song = Song::default();
    song.read_gp5(&read_file(String::from("test/pick-up-down.gp5")));
}
#[test]
fn test_gp4_rest_centered() {
    let mut song: Song = Song::default();
    song.read_gp4(&read_file(String::from("test/rest-centered.gp4")));
}
#[test]
fn test_gp5_rest_centered() {
    let mut song: Song = Song::default();
    song.read_gp5(&read_file(String::from("test/rest-centered.gp5")));
}
#[test]
fn test_gp4_sforzato() {
    let mut song: Song = Song::default();
    song.read_gp4(&read_file(String::from("test/sforzato.gp4")));
}
#[test]
fn test_gp4_shift_slide() {
    let mut song: Song = Song::default();
    song.read_gp4(&read_file(String::from("test/shift-slide.gp4")));
}
#[test]
fn test_gp5_shift_slide() {
    let mut song: Song = Song::default();
    song.read_gp5(&read_file(String::from("test/shift-slide.gp5")));
}
#[test]
fn test_gp4_slide_in_above() {
    let mut song: Song = Song::default();
    song.read_gp4(&read_file(String::from("test/slide-in-above.gp4")));
}
#[test]
fn test_gp5_slide_in_above() {
    let mut song: Song = Song::default();
    song.read_gp5(&read_file(String::from("test/slide-in-above.gp5")));
}
#[test]
fn test_gp4_slide_in_below() {
    let mut song: Song = Song::default();
    song.read_gp4(&read_file(String::from("test/slide-in-below.gp4")));
}
#[test]
fn test_gp5_slide_in_below() {
    let mut song: Song = Song::default();
    song.read_gp5(&read_file(String::from("test/slide-in-below.gp5")));
}
#[test]
fn test_gp4_slide_out_down() {
    let mut song: Song = Song::default();
    song.read_gp4(&read_file(String::from("test/slide-out-down.gp4")));
}
#[test]
fn test_gp5_slide_out_down() {
    let mut song: Song = Song::default();
    song.read_gp5(&read_file(String::from("test/slide-out-down.gp5")));
}
#[test]
fn test_gp4_slide_out_up() {
    let mut song: Song = Song::default();
    song.read_gp4(&read_file(String::from("test/slide-out-up.gp4")));
}
#[test]
fn test_gp5_slide_out_up() {
    let mut song: Song = Song::default();
    song.read_gp5(&read_file(String::from("test/slide-out-up.gp5")));
}
#[test]
fn test_gp4_slur() {
    let mut song: Song = Song::default();
    song.read_gp4(&read_file(String::from("test/slur.gp4")));
}
#[test]
fn test_gp5_slur_notes_effect_mask() {
    let mut song: Song = Song::default();
    song.read_gp5(&read_file(String::from("test/slur-notes-effect-mask.gp5")));
}
#[test]
fn test_gp5_tap_slap_pop() {
    let mut song: Song = Song::default();
    song.read_gp5(&read_file(String::from("test/tap-slap-pop.gp5")));
}
#[test]
fn test_gp3_tempo() {
    let mut song: Song = Song::default();
    song.read_gp3(&read_file(String::from("test/tempo.gp3")));
}
#[test]
fn test_gp4_tempo() {
    let mut song: Song = Song::default();
    song.read_gp4(&read_file(String::from("test/tempo.gp4")));
}
#[test]
fn test_gp5_tempo() {
    let mut song: Song = Song::default();
    song.read_gp5(&read_file(String::from("test/tempo.gp5")));
}
#[test]
fn test_gp4_test_irr_tuplet() {
    let mut song: Song = Song::default();
    song.read_gp4(&read_file(String::from("test/testIrrTuplet.gp4")));
}
#[test]
fn test_gp5_tremolos() {
    let mut song: Song = Song::default();
    song.read_gp5(&read_file(String::from("test/tremolos.gp5")));
}
#[test]
fn test_gp4_trill() {
    let mut song: Song = Song::default();
    song.read_gp4(&read_file(String::from("test/trill.gp4")));
}
#[test]
fn test_gp4_tuplet_with_slur() {
    let mut song: Song = Song::default();
    song.read_gp4(&read_file(String::from("test/tuplet-with-slur.gp4")));
}
#[test]
fn test_gp5_vibrato() {
    let mut song: Song = Song::default();
    song.read_gp5(&read_file(String::from("test/vibrato.gp5")));
}
#[test]
fn test_gp3_volta() {
    let mut song: Song = Song::default();
    song.read_gp3(&read_file(String::from("test/volta.gp3")));
}
#[test]
fn test_gp4_volta() {
    let mut song: Song = Song::default();
    song.read_gp4(&read_file(String::from("test/volta.gp4")));
}
#[test]
fn test_gp5_volta() {
    let mut song: Song = Song::default();
    song.read_gp5(&read_file(String::from("test/volta.gp5")));
}

#[test]
fn test_gp7_read() {
    let mut song = Song::default();
    let data = read_file(String::from("test/keysig.gp"));
    song.read_gp(&data);

    println!("Version: {:?}", song.version);
    println!("Name: {}", song.name);
    println!("Tracks: {}", song.tracks.len());
    println!("Measures: {}", song.measure_headers.len());
    if !song.tracks.is_empty() {
        println!("Track 1 measures: {}", song.tracks[0].measures.len());
    }

    assert_eq!(song.tracks.len(), 1);
    assert_eq!(song.measure_headers.len(), 32);
    assert_eq!(song.tracks[0].measures.len(), 32);
}

// ==================== GPX (Guitar Pro 6) tests ====================

fn read_gpx(filename: &str) -> Song {
    let mut song = Song::default();
    song.read_gpx(&read_file(String::from(filename)));
    song
}

#[test]
fn test_gpx_keysig() {
    let song = read_gpx("test/keysig.gpx");
    assert_eq!(song.tracks.len(), 1);
    assert_eq!(song.measure_headers.len(), 32);
}
#[test]
fn test_gpx_copyright() {
    let song = read_gpx("test/copyright.gpx");
    assert!(!song.tracks.is_empty());
}
#[test]
fn test_gpx_tempo() {
    let song = read_gpx("test/tempo.gpx");
    assert!(!song.measure_headers.is_empty());
}
#[test]
fn test_gpx_rest_centered() {
    let song = read_gpx("test/rest-centered.gpx");
    assert!(!song.tracks.is_empty());
}
#[test]
fn test_gpx_dotted_tuplets() {
    let song = read_gpx("test/dotted-tuplets.gpx");
    assert!(!song.tracks.is_empty());
}
#[test]
fn test_gpx_tuplets() {
    let song = read_gpx("test/tuplets.gpx");
    assert!(!song.tracks.is_empty());
}
#[test]
fn test_gpx_tuplets2() {
    let song = read_gpx("test/tuplets2.gpx");
    assert!(!song.tracks.is_empty());
}
#[test]
fn test_gpx_test_irr_tuplet() {
    let song = read_gpx("test/testIrrTuplet.gpx");
    assert!(!song.tracks.is_empty());
}
#[test]
fn test_gpx_repeats() {
    let song = read_gpx("test/repeats.gpx");
    assert!(!song.measure_headers.is_empty());
}
#[test]
fn test_gpx_repeated_bars() {
    let song = read_gpx("test/repeated-bars.gpx");
    assert!(!song.tracks.is_empty());
}
#[test]
fn test_gpx_volta() {
    let song = read_gpx("test/volta.gpx");
    assert!(!song.measure_headers.is_empty());
}
#[test]
fn test_gpx_multivoices() {
    let song = read_gpx("test/multivoices.gpx");
    assert!(!song.tracks.is_empty());
}
#[test]
fn test_gpx_double_bar() {
    let song = read_gpx("test/double-bar.gpx");
    assert!(!song.measure_headers.is_empty());
}
#[test]
fn test_gpx_clefs() {
    let song = read_gpx("test/clefs.gpx");
    assert!(!song.tracks.is_empty());
}
#[test]
fn test_gpx_bend() {
    let song = read_gpx("test/bend.gpx");
    assert!(!song.tracks.is_empty());
}
#[test]
fn test_gpx_basic_bend() {
    let song = read_gpx("test/basic-bend.gpx");
    assert!(!song.tracks.is_empty());
}
#[test]
fn test_gpx_vibrato() {
    let song = read_gpx("test/vibrato.gpx");
    assert!(!song.tracks.is_empty());
}
#[test]
fn test_gpx_let_ring() {
    let song = read_gpx("test/let-ring.gpx");
    assert!(!song.tracks.is_empty());
}
#[test]
fn test_gpx_palm_mute() {
    let song = read_gpx("test/palm-mute.gpx");
    assert!(!song.tracks.is_empty());
}
#[test]
fn test_gpx_accent() {
    let song = read_gpx("test/accent.gpx");
    assert!(!song.tracks.is_empty());
}
#[test]
fn test_gpx_sforzato() {
    let song = read_gpx("test/sforzato.gpx");
    assert!(!song.tracks.is_empty());
}
#[test]
fn test_gpx_heavy_accent() {
    let song = read_gpx("test/heavy-accent.gpx");
    assert!(!song.tracks.is_empty());
}
#[test]
fn test_gpx_ghost_note() {
    let song = read_gpx("test/ghost-note.gpx");
    assert!(!song.tracks.is_empty());
}
#[test]
fn test_gpx_dead_note() {
    let song = read_gpx("test/dead-note.gpx");
    assert!(!song.tracks.is_empty());
}
#[test]
fn test_gpx_trill() {
    let song = read_gpx("test/trill.gpx");
    assert!(!song.tracks.is_empty());
}
#[test]
fn test_gpx_tremolos() {
    let song = read_gpx("test/tremolos.gpx");
    assert!(!song.tracks.is_empty());
}
#[test]
fn test_gpx_grace() {
    let song = read_gpx("test/grace.gpx");
    assert!(!song.tracks.is_empty());
}
#[test]
fn test_gpx_grace_before_beat() {
    let song = read_gpx("test/grace-before-beat.gpx");
    assert!(!song.tracks.is_empty());
}
#[test]
fn test_gpx_grace_on_beat() {
    let song = read_gpx("test/grace-on-beat.gpx");
    assert!(!song.tracks.is_empty());
}
#[test]
fn test_gpx_artificial_harmonic() {
    let song = read_gpx("test/artificial-harmonic.gpx");
    assert!(!song.tracks.is_empty());
}
#[test]
fn test_gpx_high_pitch() {
    let song = read_gpx("test/high-pitch.gpx");
    assert!(!song.tracks.is_empty());
}
#[test]
fn test_gpx_shift_slide() {
    let song = read_gpx("test/shift-slide.gpx");
    assert!(!song.tracks.is_empty());
}
#[test]
fn test_gpx_legato_slide() {
    let song = read_gpx("test/legato-slide.gpx");
    assert!(!song.tracks.is_empty());
}
#[test]
fn test_gpx_slide_out_down() {
    let song = read_gpx("test/slide-out-down.gpx");
    assert!(!song.tracks.is_empty());
}
#[test]
fn test_gpx_slide_out_up() {
    let song = read_gpx("test/slide-out-up.gpx");
    assert!(!song.tracks.is_empty());
}
#[test]
fn test_gpx_slide_in_below() {
    let song = read_gpx("test/slide-in-below.gpx");
    assert!(!song.tracks.is_empty());
}
#[test]
fn test_gpx_slide_in_above() {
    let song = read_gpx("test/slide-in-above.gpx");
    assert!(!song.tracks.is_empty());
}
#[test]
fn test_gpx_brush() {
    let song = read_gpx("test/brush.gpx");
    assert!(!song.tracks.is_empty());
}
#[test]
fn test_gpx_arpeggio() {
    let song = read_gpx("test/arpeggio.gpx");
    assert!(!song.tracks.is_empty());
}
#[test]
fn test_gpx_rasg() {
    let song = read_gpx("test/rasg.gpx");
    assert!(!song.tracks.is_empty());
}
#[test]
fn test_gpx_fade_in() {
    let song = read_gpx("test/fade-in.gpx");
    assert!(!song.tracks.is_empty());
}
#[test]
fn test_gpx_volume_swell() {
    let song = read_gpx("test/volume-swell.gpx");
    assert!(!song.tracks.is_empty());
}
#[test]
fn test_gpx_pick_up_down() {
    let song = read_gpx("test/pick-up-down.gpx");
    assert!(!song.tracks.is_empty());
}
#[test]
fn test_gpx_slur() {
    let song = read_gpx("test/slur.gpx");
    assert!(!song.tracks.is_empty());
}
#[test]
fn test_gpx_slur_hammer_slur() {
    let song = read_gpx("test/slur_hammer_slur.gpx");
    assert!(!song.tracks.is_empty());
}
#[test]
fn test_gpx_slur_slur_hammer() {
    let song = read_gpx("test/slur_slur_hammer.gpx");
    assert!(!song.tracks.is_empty());
}
#[test]
fn test_gpx_slur_over_3_measures() {
    let song = read_gpx("test/slur_over_3_measures.gpx");
    assert!(!song.tracks.is_empty());
}
#[test]
fn test_gpx_slur_voices() {
    let song = read_gpx("test/slur_voices.gpx");
    assert!(!song.tracks.is_empty());
}
#[test]
fn test_gpx_slur_notes_effect_mask() {
    let song = read_gpx("test/slur-notes-effect-mask.gpx");
    assert!(!song.tracks.is_empty());
}
#[test]
fn test_gpx_dotted_gliss() {
    let song = read_gpx("test/dotted-gliss.gpx");
    assert!(!song.tracks.is_empty());
}
#[test]
fn test_gpx_ottava1() {
    let song = read_gpx("test/ottava1.gpx");
    assert!(!song.tracks.is_empty());
}
#[test]
fn test_gpx_ottava2() {
    let song = read_gpx("test/ottava2.gpx");
    assert!(!song.tracks.is_empty());
}
#[test]
fn test_gpx_ottava3() {
    let song = read_gpx("test/ottava3.gpx");
    assert!(!song.tracks.is_empty());
}
#[test]
fn test_gpx_ottava4() {
    let song = read_gpx("test/ottava4.gpx");
    assert!(!song.tracks.is_empty());
}
#[test]
fn test_gpx_ottava5() {
    let song = read_gpx("test/ottava5.gpx");
    assert!(!song.tracks.is_empty());
}
#[test]
fn test_gpx_mordents() {
    let song = read_gpx("test/mordents.gpx");
    assert!(!song.tracks.is_empty());
}
#[test]
fn test_gpx_turn() {
    let song = read_gpx("test/turn.gpx");
    assert!(!song.tracks.is_empty());
}
#[test]
fn test_gpx_barre() {
    let song = read_gpx("test/barre.gpx");
    assert!(!song.tracks.is_empty());
}
#[test]
fn test_gpx_fingering() {
    let song = read_gpx("test/fingering.gpx");
    assert!(!song.tracks.is_empty());
}
#[test]
fn test_gpx_fret_diagram() {
    let song = read_gpx("test/fret-diagram.gpx");
    assert!(!song.tracks.is_empty());
}
#[test]
fn test_gpx_fret_diagram_2instruments() {
    let song = read_gpx("test/fret-diagram_2instruments.gpx");
    assert!(song.tracks.len() >= 2);
}
#[test]
fn test_gpx_text() {
    let song = read_gpx("test/text.gpx");
    assert!(!song.tracks.is_empty());
}
#[test]
fn test_gpx_timer() {
    let song = read_gpx("test/timer.gpx");
    assert!(!song.tracks.is_empty());
}
#[test]
fn test_gpx_directions() {
    let song = read_gpx("test/directions.gpx");
    assert!(!song.measure_headers.is_empty());
}
#[test]
fn test_gpx_fermata() {
    let song = read_gpx("test/fermata.gpx");
    assert!(!song.tracks.is_empty());
}
#[test]
fn test_gpx_free_time() {
    let song = read_gpx("test/free-time.gpx");
    assert!(!song.tracks.is_empty());
}
#[test]
fn test_gpx_dynamic() {
    let song = read_gpx("test/dynamic.gpx");
    assert!(!song.tracks.is_empty());
}
#[test]
fn test_gpx_crescendo_diminuendo() {
    let song = read_gpx("test/crescendo-diminuendo.gpx");
    assert!(!song.tracks.is_empty());
}
#[test]
fn test_gpx_wah() {
    let song = read_gpx("test/wah.gpx");
    assert!(!song.tracks.is_empty());
}
#[test]
fn test_gpx_all_percussion() {
    let song = read_gpx("test/all-percussion.gpx");
    assert!(!song.tracks.is_empty());
}
#[test]
fn test_gpx_beams_stems_ledger_lines() {
    let song = read_gpx("test/beams-stems-ledger-lines.gpx");
    assert!(!song.tracks.is_empty());
}
#[test]
fn test_gpx_chordnames_keyboard() {
    let song = read_gpx("test/chordnames_keyboard.gpx");
    assert!(!song.tracks.is_empty());
}
#[test]
fn test_gpx_tuplet_with_slur() {
    let song = read_gpx("test/tuplet-with-slur.gpx");
    assert!(!song.tracks.is_empty());
}

#[test]
fn test_gpx_all_files_parse() {
    use std::fs;
    let test_dir = "../test";
    let mut pass = 0;
    let mut failures: Vec<String> = Vec::new();
    for entry in fs::read_dir(test_dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.extension().map_or(false, |e| e == "gpx") {
            let fname = path.file_name().unwrap().to_str().unwrap().to_string();
            let data = fs::read(&path).unwrap();
            let mut song = Song::default();
            match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                song.read_gpx(&data);
            })) {
                Ok(_) => { pass += 1; }
                Err(e) => {
                    let msg = if let Some(s) = e.downcast_ref::<String>() {
                        s.clone()
                    } else if let Some(s) = e.downcast_ref::<&str>() {
                        s.to_string()
                    } else {
                        "unknown".to_string()
                    };
                    let short = &msg[..msg.len().min(100)];
                    failures.push(format!("{}: {}", fname, short));
                }
            }
        }
    }
    if !failures.is_empty() {
        for f in &failures {
            eprintln!("FAIL: {}", f);
        }
    }
    eprintln!("{} pass, {} fail out of {}", pass, failures.len(), pass + failures.len());
    assert!(failures.is_empty(), "{} files failed to parse", failures.len());
}
