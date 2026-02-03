use fraction::ToPrimitive;

use crate::error::GpResult;
use crate::io::primitive::*;
use crate::model::chord::SongChordOps;
use crate::model::key_signature::read_duration;
use crate::model::mix_table::SongMixTableOps;
use crate::model::note::{NoteEffect, SongNoteOps};
use crate::model::song::Song;
use crate::types::beat::{Beat, Voice};
use crate::types::enums::*;

use super::effects::*;

/// Read beat. The first byte is the beat flags. It lists the data present in the current beat:
/// - *0x01*: dotted notes
/// - *0x02*: presence of a chord diagram
/// - *0x04*: presence of a text
/// - *0x08*: presence of effects
/// - *0x10*: presence of a mix table change event
/// - *0x20*: the beat is a n-tuplet
/// - *0x40*: status: True if the beat is empty of if it is a rest
/// - *0x80*: *blank*
///
/// Flags are followed by:
/// - Status: `byte`. If flag at *0x40* is true, read one byte. If value of the byte is *0x00* then beat is empty, if value is *0x02* then the beat is rest.
/// - Beat duration: `byte`. See `Duration::read()`.
/// - Chord diagram. See `Chord::read()`.
/// - Text: `int-byte-size-string`.
/// - Beat effects. See `BeatEffects::read()`.
/// - Mix table change effect. See `MixTableChange::read()`.
pub fn read_beat(
    song: &mut Song,
    data: &[u8],
    seek: &mut usize,
    voice: &mut Voice,
    start: i64,
    track_index: usize,
) -> GpResult<i64> {
    let flags = read_byte(data, seek)?;
    //println!("read_beat(),    flags: {} \t seek: {}", flags, *seek);
    //get a beat
    let mut b = 0;
    let mut new_beat = true;
    for i in (0usize..voice.beats.len()).rev() {
        if voice.beats[i].start == Some(start) {
            b = i;
            new_beat = false;
            break;
        }
    }
    if new_beat {
        voice.beats.push(Beat {
            start: Some(start),
            ..Default::default()
        });
        b = voice.beats.len() - 1;
    }

    if (flags & 0x40) == 0x40 {
        voice.beats[b].status = get_beat_status(read_byte(data, seek)?);
    } //else { voice.beats[b].status = BeatStatus::Normal;}
    let duration = read_duration(data, seek, flags)?;
    let mut note_effect = NoteEffect::default();
    if (flags & 0x02) == 0x02 {
        voice.beats[b].effect.chord = Some(song.read_chord(
            data,
            seek,
            song.tracks[track_index].strings.len().to_u8().unwrap(),
        )?);
    }
    if (flags & 0x04) == 0x04 {
        voice.beats[b].text = read_int_byte_size_string(data, seek)?;
    }
    if (flags & 0x08) == 0x08 {
        let chord = voice.beats[b].effect.chord.clone();
        if song.version.number.0 == 3 {
            voice.beats[b].effect = read_beat_effects_v3(song, data, seek, &mut note_effect)?;
        } else {
            voice.beats[b].effect = read_beat_effects_v4(song, data, seek)?;
        }
        voice.beats[b].effect.chord = chord;
    }
    if (flags & 0x10) == 0x10 {
        let mtc = song.read_mix_table_change(data, seek)?;
        voice.beats[b].effect.mix_table_change = Some(mtc);
    }
    song.read_notes(
        data,
        seek,
        track_index,
        &mut voice.beats[b],
        &duration,
        note_effect,
    )?;
    Ok(if voice.beats[b].status == BeatStatus::Empty {
        0
    } else {
        duration.time().to_i64().unwrap()
    })
}

/// Read beat. First, beat is read is in Guitar Pro 3 `guitarpro.gp3.readBeat`. Then it is followed by set of flags stored in `short`.
/// - *0x0001*: break beams
/// - *0x0002*: direct beams down
/// - *0x0004*: force beams
/// - *0x0008*: direct beams up
/// - *0x0010*: ottava (8va)
/// - *0x0020*: ottava bassa (8vb)
/// - *0x0040*: quindicesima (15ma)
/// - *0x0100*: quindicesima bassa (15mb)
/// - *0x0200*: start tuplet bracket here
/// - *0x0400*: end tuplet bracket here
/// - *0x0800*: break secondary beams
/// - *0x1000*: break secondary tuplet
/// - *0x2000*: force tuplet bracket
/// - Break secondary beams: `byte`. Appears if flag at *0x0800* is set. Signifies how much beams should be broken.
pub fn read_beat_v5(
    song: &mut Song,
    data: &[u8],
    seek: &mut usize,
    voice: &mut Voice,
    start: &mut i64,
    track_index: usize,
) -> GpResult<i64> {
    let duration = read_beat(song, data, seek, voice, *start, track_index)?;
    //get the beat used in read_beat()
    let b = voice.beats.len() - 1;

    let flags2 = read_short(data, seek)?;
    //println!("read_beat_v5(), flags2: {} \t seek: {}", flags2, *seek);
    if (flags2 & 0x0010) == 0x0010 {
        voice.beats[b].octave = Octave::Ottava;
    }
    if (flags2 & 0x0020) == 0x0020 {
        voice.beats[b].octave = Octave::OttavaBassa;
    }
    if (flags2 & 0x0040) == 0x0040 {
        voice.beats[b].octave = Octave::Quindicesima;
    }
    if (flags2 & 0x0100) == 0x0100 {
        voice.beats[b].octave = Octave::QuindicesimaBassa;
    }

    voice.beats[b].display.break_beam = (flags2 & 0x0001) == 0x0001;
    voice.beats[b].display.force_beam = (flags2 & 0x0004) == 0x0004;
    voice.beats[b].display.force_bracket = (flags2 & 0x2000) == 0x2000;
    voice.beats[b].display.break_secondary_tuplet = (flags2 & 0x1000) == 0x1000;
    if (flags2 & 0x0002) == 0x0002 {
        voice.beats[b].display.beam_direction = VoiceDirection::Down;
    }
    if (flags2 & 0x0008) == 0x0008 {
        voice.beats[b].display.beam_direction = VoiceDirection::Up;
    }
    if (flags2 & 0x0200) == 0x0200 {
        voice.beats[b].display.tuplet_bracket = TupletBracket::Start;
    }
    if (flags2 & 0x0400) == 0x0400 {
        voice.beats[b].display.tuplet_bracket = TupletBracket::End;
    }
    if (flags2 & 0x0800) == 0x0800 {
        voice.beats[b].display.break_secondary = read_byte(data, seek)?;
    }

    Ok(duration)
}
