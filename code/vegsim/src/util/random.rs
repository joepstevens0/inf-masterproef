use std::sync::Mutex;

use pcg_rand::Pcg32;
use rand::{ seq::SliceRandom, Rng, SeedableRng};

use crate::parameters;

lazy_static::lazy_static! {
    static ref RANDOM_DATA: Mutex<Random> = Mutex::new(Random::new());
    // static ref SEED: u64 = std::time::SystemTime::now().elapsed().unwrap().as_secs();
}

pub struct Random {
    rng: Pcg32,
}

impl Random {
    pub fn new() -> Self {
        let rng = Pcg32::seed_from_u64(parameters::SEED);
        Self { rng }
    }

    pub fn rand() -> f32 {
        // rand::thread_rng().gen_range(0f32..1f32)
        RANDOM_DATA.lock().unwrap().rng.gen_range(0f32..1f32)
    }

    pub fn choose<T>(list: &[T]) -> &T {
        list.choose(&mut RANDOM_DATA.lock().unwrap().rng).unwrap()
    }

    pub fn reset(){
        RANDOM_DATA.lock().unwrap().rng = Pcg32::seed_from_u64(parameters::SEED);
    }
}
