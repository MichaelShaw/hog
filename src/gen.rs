
use random::Random;
use rand::{Rand};

use num_integer::Integer;
use num_traits::NumCast;
use num_traits::cast::ToPrimitive;

use std::collections::HashSet;

pub trait Gen {
    type Item;

    fn run(&self, rand:&mut Random) -> Self::Item;

    fn map<B, F>(self, f: F) -> Map<Self, F> where
        Self: Sized, F: Fn(Self::Item) -> B {
        Map { gen: self, f }
    }

    fn flat_map<B, F, OG>(self, f: F) -> FlatMap<Self, F> where
        Self: Sized,
        OG : Gen,
        F: Fn(Self::Item) -> OG {
        FlatMap { gen: self, f }
    }

    fn in_vec_with_max_size(self, size:usize) -> VecGen<Self, Uniform<usize>> where Self: Sized {
        VecGen {
            eg: self,
            lg: Uniform { min: 0, max: size },
        }
    }
}

pub fn characters_from(chars: &str, min_size: usize, max_size: usize) -> Characters {
    assert!(!chars.is_empty());
    assert!(max_size >= min_size);
    Characters {
        chars: chars.chars().collect(),
        min_size,
        max_size,
    }
}

#[derive(Clone, Debug)]
pub struct Characters {
    chars: Vec<char>,
    min_size: usize,
    max_size: usize,
}

impl Gen for Characters where  {
    type Item = String;

    fn run(&self, rand: &mut Random) -> Self::Item {
        let size = self.min_size + usize_less_than(rand, self.max_size - self.min_size);

        (0..size).map(|_| {
            self.chars[usize_less_than(rand, self.chars.len())]
        }).collect::<String>()
    }
}

#[derive(Clone, Debug)]
pub struct Choice<G> {
    gens: Vec<G>
}

pub fn choice<G>(gens: Vec<G>) -> Choice<G> {
    Choice {
        gens
    }
}

impl<G> Gen for Choice<G> where G : Gen {
    type Item = G::Item;

    fn run(&self, rand: &mut Random) -> Self::Item {
        let ig = &self.gens[usize_less_than(rand, self.gens.len())];
        ig.run(rand)
    }
}

pub fn usize_less_than(rand: &mut Random, max:usize) -> usize {
    usize::rand(rand) % max
}


pub fn uniform<N>(min: N, max: N) -> Uniform<N> where N : Integer + ToPrimitive + NumCast + Copy {
    assert!(max >= min);
    Uniform {
        min,
        max,
    }
}

#[derive(Clone, Debug)]
pub struct Uniform<N> where N : Clone {
    min: N,
    max: N,
}

impl<N> Gen for Uniform<N> where N : Integer + ToPrimitive + NumCast + Copy {
    type Item = N;

    fn run(&self, rand: &mut Random) -> Self::Item {
        let length = self.max - self.min;
        if length == N::zero() {
            self.min
        } else {
            let add = u64::rand(rand) % length.to_u64().unwrap(); // extremely unsafe atm.
            self.min + N::from(add).unwrap()
        }
    }
}

#[derive(Clone, Debug)]
pub struct VecGen<ElementGen, LengthGen> {
    eg: ElementGen,
    lg: LengthGen,
}

impl<ElementGen, LengthGen> Gen for VecGen<ElementGen, LengthGen> where LengthGen : Gen<Item = usize>, ElementGen : Gen {
    type Item = Vec<ElementGen::Item>;

    fn run(&self, rand: &mut Random) -> Self::Item {
        let length = self.lg.run(rand);
        let mut out = vec![];
        for _ in 0..length {
            out.push(self.eg.run(rand));
        }
        out
    }
}

pub fn ret<A>(a:A) -> PureGen<A> where A: Clone {
    PureGen { a }
}

pub struct FGen<A> {
    f: Box<Fn(&mut Random) -> A>,
}

impl<A> Gen for FGen<A> {
    type Item = A;

    fn run(&self, rand: &mut Random) -> A {
        (self.f)(rand)
    }
}

#[derive(Clone, Debug)]
pub struct PureGen<A> where A: Clone {
    a: A
}

impl<A> Gen for PureGen<A> where A: Clone {
    type Item = A;

    #[allow(unused_variables)]
    fn run(&self, rand: &mut Random) -> A {
        self.a.clone()
    }
}

pub struct Map<G, F> {
    gen: G,
    f: F,
}

impl<B, G: Gen, F> Gen for Map<G, F> where F: Fn(G::Item) -> B {
    type Item = B;

    fn run(&self, rand: &mut Random) -> B {
        (self.f)(self.gen.run(rand))
    }
}

pub struct FlatMap<G, F> {
    gen: G,
    f: F,
}

impl<G: Gen, F, OG: Gen> Gen for FlatMap<G, F> where F: Fn(G::Item) -> OG {
    type Item = OG::Item;

    fn run(&self, rand: &mut Random) -> OG::Item {
        let a = self.gen.run(rand);
        let b = (self.f)(a);
        b.run(rand)
    }
}
