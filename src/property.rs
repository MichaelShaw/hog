

use colored::*;
use random::Seed;
use rand;

use rand::XorShiftRng;
use rand::SeedableRng;

use gen::Gen;
use test_result::Testable;

pub fn property(name: &str) -> Property {
    Property {
        name: name.to_string(),
        params: RunParams::default(),
        seed: None,
    }
}

#[derive(Clone, Debug)]
pub struct Property {
    name: String,
    params: RunParams,
    seed: Option<Seed>, // seed override
}

impl Property {
    pub fn with_seed(&self, seed:Seed) -> Property {
        Property {
            name: self.name.clone(),
            params: self.params,
            seed: Some(seed),
        }
    }

    pub fn with_n(&self, n: u32) -> Property {
        Property {
            name: self.name.clone(),
            params: RunParams { n },
            seed: self.seed,
        }
    }

    pub fn forall<G, T>(&self, gen: G) where G : Gen<Item = T>, T : Testable {
        print_descirption(&self.name, self.params, self.seed);
        match self.seed {
            Some(seed) => {
                let mut test_rng = XorShiftRng::from_seed(seed);
                let test_result = gen.run(&mut test_rng).result();
                if test_result.is_failure() {
                    print_failure(&self.name, &test_result.description, seed);
                    return;
                } else {
                    print_success(&self.name, &test_result.description, seed);
                }
            },
            None => (),
        }

        for _ in 0..self.params.n {
            let seed = rand::random::<Seed>();
            let mut test_rng = XorShiftRng::from_seed(seed);
            let test_result = gen.run(&mut test_rng).result();

            if test_result.is_failure() {
                print_failure(&self.name, &test_result.description, seed);
                return;
            }
        }

        print_summary(&self.name, self.params);
    }
}

pub fn print_descirption(name: &str, run_params:RunParams, seed: Option<Seed>) {
    let out : String = if let Some(s) = seed {
        format!("Running {} with seed {:?} and {} test cases", name, s, run_params.n)
    } else {
        format!("Running {} with {} test cases", name, run_params.n)
    };

    println!("{}", out.cyan());
}

pub fn print_success(name: &str, description: &str, seed: Seed) {
    let out = format!("{} succeeded: {} (with seed: {:?})", name, description, seed);
    println!("{}", out.green());
}

pub fn print_failure(name: &str, description: &str, seed: Seed) {
    let out = format!("{} failed: {} (with seed: {:?})", name, description, seed);
    println!("{}", out.red());
}

pub fn print_summary(name: &str, run_params: RunParams) {
    let out = format!("{} suceeded through {} test cases", name, run_params.n);
    println!("{}", out.green())
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