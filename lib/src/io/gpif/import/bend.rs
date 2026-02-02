// GPIF bend effect builders
use crate::io::gpif::WhammyInfo;
use crate::model::effects::{BendEffect, BendPoint, GP_BEND_SEMITONE, BEND_EFFECT_MAX_POSITION};
use crate::model::enums::BendType;

/// Build bend effect from GPIF origin/middle/destination values (float, in 1/100 semitone).
/// When middle value and offsets are provided, uses them for a more accurate 3-point curve.
///
/// Note: GPIF provides a `middle_offset2` parameter, but it's not used here because
/// the GP5 bend model uses a simple 3-point curve with a single middle position.
pub(crate) fn build_bend_effect_full(
    origin: f64,
    middle: Option<f64>,
    destination: f64,
    origin_offset: Option<f64>,
    middle_offset1: Option<f64>,
    destination_offset: Option<f64>,
) -> BendEffect {
    let mut bend = BendEffect::default();
    let origin_val = (origin / GP_BEND_SEMITONE as f64).round() as i8;
    let dest_val = (destination / GP_BEND_SEMITONE as f64).round() as i8;

    if origin == 0.0 && destination > 0.0 {
        bend.kind = BendType::Bend;
    } else if origin > 0.0 && destination == 0.0 {
        bend.kind = BendType::ReleaseUp;
    } else if origin > 0.0 && destination > 0.0 {
        if destination > origin {
            bend.kind = BendType::Bend;
        } else if destination < origin {
            bend.kind = BendType::ReleaseUp;
        } else {
            bend.kind = BendType::Bend;
        }
    }

    bend.value = (destination.max(origin) / GP_BEND_SEMITONE as f64 * 2.0).round() as i16;

    // Compute positions (0-12 range). GPIF offsets are percentages (0-100).
    let p0 = origin_offset
        .map(|o| (o / 100.0 * BEND_EFFECT_MAX_POSITION as f64).round() as u8)
        .unwrap_or(0);
    let p2 = destination_offset
        .map(|o| (o / 100.0 * BEND_EFFECT_MAX_POSITION as f64).round() as u8)
        .unwrap_or(BEND_EFFECT_MAX_POSITION);

    let mid_val = middle
        .map(|m| (m / GP_BEND_SEMITONE as f64).round() as i8)
        .unwrap_or(((origin_val as i16 + dest_val as i16) / 2) as i8);
    let p1 = middle_offset1
        .map(|o| (o / 100.0 * BEND_EFFECT_MAX_POSITION as f64).round() as u8)
        .unwrap_or(BEND_EFFECT_MAX_POSITION / 2);

    bend.points.push(BendPoint {
        position: p0,
        value: origin_val,
        vibrato: false,
    });
    bend.points.push(BendPoint {
        position: p1,
        value: mid_val,
        vibrato: false,
    });
    bend.points.push(BendPoint {
        position: p2,
        value: dest_val,
        vibrato: false,
    });
    bend
}

/// Build bend effect from simple origin/destination values (backwards compatible).
pub(crate) fn build_bend_effect(origin: f64, destination: f64) -> BendEffect {
    build_bend_effect_full(origin, None, destination, None, None, None)
}

/// Build a whammy bar bend effect from GPIF Whammy element attributes.
pub(crate) fn build_whammy_effect(w: &WhammyInfo) -> BendEffect {
    build_bend_effect_full(
        w.origin_value.unwrap_or(0.0),
        w.middle_value,
        w.destination_value.unwrap_or(0.0),
        w.origin_offset,
        w.middle_offset1,
        w.destination_offset,
    )
}
