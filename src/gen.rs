
use random::Seed;


pub trait Gen {
    type Item;

    fn produce(&self, seed:Seed) -> Self::Item;

    fn map<B, F>(self, f: F) -> Map<Self, F> where
        Self: Sized, F: Fn(Self::Item) -> B {
        Map { gen: self, f: f }
    }

    fn flat_map<B, F, OG>(self, f: F) -> FlatMap<Self, F> where
        Self: Sized,
        OG : Gen,
        F: Fn(Self::Item) -> OG {
        FlatMap { gen: self, f: f }
    }
}

pub fn ret<A>(a:A) -> PureGen<A> {
    PureGen {
        a: a
    }
}

pub struct PureGen<A> {
    pub a: A
}

impl<A> Gen for PureGen<A> where A: Clone {
    type Item = A;

    #[allow(unused_variables)]
    fn produce(&self, seed:Seed) -> A {
        self.a.clone()
    }
}

pub struct Map<G, F> {
    gen: G,
    f: F,
}

impl<B, G: Gen, F> Gen for Map<G, F> where F: Fn(G::Item) -> B {
    type Item = B;

    fn produce(&self, seed:Seed) -> B {
        (self.f)(self.gen.produce(seed))
    }
}

pub struct FlatMap<G, F> {
    gen: G,
    f: F,
}

impl<G: Gen, F, OG: Gen> Gen for FlatMap<G, F> where F: Fn(G::Item) -> OG {
    type Item = OG::Item;

    fn produce(&self, seed:Seed) -> OG::Item {
        let (sa, sb) = seed.split();
        let a = self.gen.produce(sa);
        let b = (self.f)(a);
        b.produce(sb)
    }
}
