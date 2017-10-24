extern crate hog;

use hog::property::*;
use hog::gen::*;
use hog::check::*;

#[derive(Eq, Clone, Copy, PartialEq, Debug)]
enum Color {
    Red, Green, Blue
}

fn main() {

    let gen : Uniform<u32> = uniform::<u32>(0, 10);
    property("reverse").forall(gen.map(|n| eq(n, n * 2)));

    let vec_gen = uniform(0, 10).in_vec_with_max_size(5);
    property("vec").with_n(5).forall(vec_gen.map(|v| {
        println!("v -> {:?}", v);
        true
    }));

    let red = ret(Color::Red);
    let green = ret(Color::Green);
    let blue = ret(Color::Blue);

    let color = choice(vec![red, green, blue]);

    property("colors").with_n(5).forall(color.map(|c| {
        println!("color -> {:?}", c);
        true
    }))
}

