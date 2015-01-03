use std::rand;
use std::rand::Rng;

#[deriving(Rand,Show)]
struct Point {
    x: f64,
    y: f64,
}

#[deriving(Rand,Show)]
struct Rectangle {
    p1: Point,
    p2: Point,
}

fn main() {
    let mut rng = rand::thread_rng();

    println!("random point\n{}", rng.gen::<Point>());
    println!("random rectangle\n{}", rng.gen::<Rectangle>());
}
