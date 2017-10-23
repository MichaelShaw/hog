
use std::num::Wrapping as w;

#[allow(bad_style)]
pub type w32 = w<u32>;

#[derive(Copy, Clone, Debug)]
pub struct Seed { // taken from rand::XorShiftRng
    x: w32,
    y: w32,
    z: w32,
    w: w32,
}

impl Seed {
    pub fn from_seed(seed: [u32; 4]) -> Seed  {
        assert!(!seed.iter().all(|&x| x == 0),
                "Seed::from_seed called with an all zero seed.");

        Seed {
            x: w(seed[0]),
            y: w(seed[1]),
            z: w(seed[2]),
            w: w(seed[3]),
        }
    }

    pub fn new_unseeded() -> Seed {
        Seed {
            x: w(0x193a6754),
            y: w(0xa8a7d469),
            z: w(0x97830e05),
            w: w(0x113ba7bb),
        }
    }

    pub fn split(&self) -> (Seed, Seed) {
        let x = self.x;
        let t = x ^ (x << 11);
        let w_ = self.w;

        let next_seed = Seed {
            x: self.y,
            y: self.z,
            z: self.w,
            w: w_ ^ (w_ >> 19) ^ (t ^ (t >> 8)),
        };

        (*self, next_seed)
    }

    pub fn u32(&self) -> u32 { // could always make this consume it to be ""careful"" (would have to remove copying)
        self.w.0
    }
}
