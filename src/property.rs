
use random::Seed;
use rand;

pub fn property(name: &str) -> Property {
    let bullshit = rand::random::<Seed>(); // thread local secure randomness

    Property {
        name: name.to_string(),
        params: RunParams::default(),
        seed: bullshit,
    }
}

#[derive(Clone, Debug)]
pub struct Property {
    name: String,
    params: RunParams,
    seed: Seed,
}

impl Property {
    pub fn with_seed(&self, seed:Seed) -> Property {
        Property {
            name: self.name.clone(),
            params: self.params,
            seed,
        }
    }

    pub fn with_n(&self, n: u32) -> Property {
        Property {
            name: self.name.clone(),
            params: RunParams { n },
            seed: self.seed,
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct RunParams {
    n: u32,
}

impl Default for RunParams {
    fn default() -> Self {
        RunParams { n: 100 }
    }
}