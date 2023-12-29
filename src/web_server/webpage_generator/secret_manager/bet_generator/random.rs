use rand_simple;

fn uniform(range: u32) -> u32 {
    let seed = rand_simple::generate_seeds!(1)[0]; // TODO static generator?
    let mut distribution = rand_simple::Uniform::new(seed);
    distribution
        .try_set_params(0.0, range as f64)
        .expect("Error setting distribution parameters");
    for _ in 0..100 {
        // TODO initialize elsewhere
        distribution.sample();
    }
    let sample = distribution.sample().trunc() as u32;
    if sample == range {
        0
    } else {
        sample
    }
}

pub fn uniform_bool() -> bool {
    uniform(2) == 1
}

/// Returns a random number sampled from the normal distribution.
fn normal() -> f64 {
    let seed = rand_simple::generate_seeds!(2); // TODO static generator?
    let mut distribution = rand_simple::Normal::new(seed);
    for _ in 0..100 {
        // TODO initialize elsewhere
        distribution.sample();
    }
    distribution.sample()
}

/// Generates a random number between `min` (inclusive) and `max` (inclusive). The relative likelihood of numbers within
/// the range is equivalent to the relative likelihood of samples from the normal distribution transformed to the
/// supplied range according to the supplied `z_range`.
pub fn psuedo_normal(min: i32, max: i32, z_range: f64) -> i32 {
    if max <= min {
        panic!("Invalid inputs");
    }

    let scaling = (max - min) as f64 / (z_range * 2.0);
    let shift = (max + min) as f64 / 2.0;

    loop {
        let sample = normal() * scaling + shift;
        let rounded = sample.round() as i32;
        if rounded >= min && rounded < max {
            return rounded;
        }
    }
}
