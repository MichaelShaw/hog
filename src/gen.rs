
use random::Random;
use rand::{Rand};

use num_integer::Integer;
use num_traits::NumCast;
use num_traits::cast::ToPrimitive;

pub trait Gen {
    type Item;

    fn produce(&self, rand:&mut Random) -> Self::Item;

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

    fn uniform<N>(min: N, max: N) -> Uniform<N> where N : Integer + ToPrimitive + NumCast + Copy {
        assert!(max >= min);
        Uniform {
            min,
            max,
        }
    }

    fn in_vec_with_max_size(self, size:usize) -> VecGen<Self, Uniform<usize>> where Self: Sized {
        VecGen {
            eg: self,
            lg: Uniform { min: 0, max: size },
        }
    }
}

pub struct Uniform<N> {
    min: N,
    max: N,
}

impl<N> Gen for Uniform<N> where N : Integer + ToPrimitive + NumCast + Copy {
    type Item = N;

    fn produce(&self, rand: &mut Random) -> Self::Item {
        let length = self.max - self.min;
        if length == N::zero() {
            self.min
        } else {
            let add = u64::rand(rand) % length.to_u64().unwrap(); // extremely unsafe atm.
            self.min + N::from(add).unwrap()
        }
    }
}

pub struct VecGen<ElementGen, LengthGen> {
    eg: ElementGen,
    lg: LengthGen,

}

impl<ElementGen, LengthGen> Gen for VecGen<ElementGen, LengthGen> where LengthGen : Gen<Item = usize>, ElementGen : Gen {
    type Item = Vec<ElementGen::Item>;

    fn produce(&self, rand: &mut Random) -> Self::Item {
        let length = self.lg.produce(rand);
        let mut out = vec![];
        for _ in 0..length {
            out.push(self.eg.produce(rand));
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

    fn produce(&self, rand: &mut Random) -> A {
        (self.f)(rand)
    }
}


pub struct PureGen<A> {
    a: A
}

impl<A> Gen for PureGen<A> where A: Clone {
    type Item = A;

    #[allow(unused_variables)]
    fn produce(&self, rand: &mut Random) -> A {
        self.a.clone()
    }
}

pub struct Map<G, F> {
    gen: G,
    f: F,
}

impl<B, G: Gen, F> Gen for Map<G, F> where F: Fn(G::Item) -> B {
    type Item = B;

    fn produce(&self, rand: &mut Random) -> B {
        (self.f)(self.gen.produce(rand))
    }
}

pub struct FlatMap<G, F> {
    gen: G,
    f: F,
}

impl<G: Gen, F, OG: Gen> Gen for FlatMap<G, F> where F: Fn(G::Item) -> OG {
    type Item = OG::Item;

    fn produce(&self, rand: &mut Random) -> OG::Item {
        let a = self.gen.produce(rand);
        let b = (self.f)(a);
        b.produce(rand)
    }
}
