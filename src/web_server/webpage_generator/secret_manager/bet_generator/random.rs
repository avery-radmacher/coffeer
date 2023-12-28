fn uniform(range: u32) -> u32 {
    todo!()
}

pub fn uniform_bool() -> bool {
    uniform(2) == 1
}

const normal_z: f64 = 1.0; // TODO verify (and elide?)

/// Returns a random number sampled from the normal distribution.
fn normal() -> f64 {
    todo!()
}

/// Generates a random number between `min` (inclusive) and `max` (exclusive). The relative likelihood of numbers within
/// the range is equivalent to the relative likelihood of samples from the normal distribution transformed to the
/// supplied range according to the supplied `z_range`.
pub fn psuedo_normal(min: i32, max: i32, z_range: f64) -> i32 {
    if max <= min {
        panic!("Invalid inputs");
    }

    let scaling = (max - min) as f64 / (z_range * 2.0 * normal_z);
    let shift = (max + min) as f64 / 2.0;

    loop {
        let sample = normal() * scaling + shift;
        let rounded = sample.round() as i32;
        if rounded >= min && rounded < max {
            return rounded;
        }
    }
}
