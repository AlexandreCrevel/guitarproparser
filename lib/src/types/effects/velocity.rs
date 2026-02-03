// Velocity / dynamics constants and functions
use fraction::ToPrimitive;

/// A collection of velocities / dynamics
pub const MIN_VELOCITY: i16 = 15;
pub const VELOCITY_INCREMENT: i16 = 16;

// Dynamic levels (commented out to reduce unused code)
//pub const PIANO_PIANISSIMO: i16 = MIN_VELOCITY * VELOCITY_INCREMENT;
//pub const PIANO: i16 = MIN_VELOCITY + VELOCITY_INCREMENT * 2;
//pub const MEZZO_PIANO: i16 = MIN_VELOCITY + VELOCITY_INCREMENT * 3;
//pub const MEZZO_FORTE: i16 = MIN_VELOCITY + VELOCITY_INCREMENT * 4;
pub const FORTE: i16 = MIN_VELOCITY + VELOCITY_INCREMENT * 5;
//pub const FORTISSIMO: i16 = MIN_VELOCITY + VELOCITY_INCREMENT * 6;
//pub const FORTE_FORTISSIMO: i16 = MIN_VELOCITY + VELOCITY_INCREMENT * 7;

pub const DEFAULT_VELOCITY: i16 = FORTE;

/// Convert Guitar Pro dynamic value to raw MIDI velocity
pub fn unpack_velocity(v: i16) -> i16 {
    //println!("unpack_velocity({})", v);
    MIN_VELOCITY + VELOCITY_INCREMENT * v - VELOCITY_INCREMENT
}

/// Convert raw MIDI velocity to Guitar Pro dynamic value
pub fn pack_velocity(velocity: i16) -> i8 {
    ((velocity + VELOCITY_INCREMENT - MIN_VELOCITY)
        .to_f32()
        .unwrap()
        / VELOCITY_INCREMENT.to_f32().unwrap())
    .ceil()
    .to_i8()
    .unwrap()
}
