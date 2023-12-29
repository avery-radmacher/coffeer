use once_cell::sync::Lazy;
use rand_simple::{self, Normal, Uniform};

static SEEDS: Lazy<[u32; 3]> = Lazy::new(|| rand_simple::generate_seeds!(3));

static mut UNIFORM_DISTRIBUTION: Lazy<Uniform> = Lazy::new(|| {
    let mut distribution = Uniform::new(SEEDS[0]);
    for _ in 0..1024 {
        distribution.sample();
    }
    distribution
});

static mut NORMAL_DISTRIBUTION: Lazy<Normal> = Lazy::new(|| {
    let mut distribution = Normal::new([SEEDS[1], SEEDS[2]]);
    for _ in 0..1024 {
        distribution.sample();
    }
    distribution
});

fn uniform(range: u32) -> u32 {
    // TODO make thread-safe
    let sample = (unsafe { UNIFORM_DISTRIBUTION.sample() } * range as f64).trunc() as u32;
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
    // TODO make thread-safe
    unsafe { NORMAL_DISTRIBUTION.sample() }
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
