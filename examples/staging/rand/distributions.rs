use std::rand::distributions::{IndependentSample,Normal,StudentT};
use std::rand;

fn main() {
    let mut rng = rand::thread_rng();

    let normal = Normal::new(0.0, 1.0);
    println!("10 samples from a normal distribution with mean 0.0 and SD 1.0");
    for _ in range(0u32, 10) {
        println!("{}", normal.ind_sample(&mut rng));
    }

    let student = StudentT::new(5.0);
    println!("10 samples from a T distribution with 5 degrees of freedom");
    for _ in range(0u32, 10) {
        println!("{}", student.ind_sample(&mut rng));
    }
}
