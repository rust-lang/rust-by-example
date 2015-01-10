use std::rand::Rng;
use std::rand;

fn main() {
    // create a task-local Random Number Generator
    let mut rng = rand::thread_rng();

    // the `gen` methods generates values in the full range of each type using
    // a uniform distribution
    println!("randomly generate some values for different primitive types");
    println!("u8: {}", rng.gen::<u8>());
    println!("i8: {}", rng.gen::<i8>());
    println!("u16: {}", rng.gen::<u16>());
    println!("i16: {}", rng.gen::<i16>());
    // except for floats which get generated in the range [0, 1>
    println!("f32: {}", rng.gen::<f32>());
    println!("f64: {}", rng.gen::<f64>());

    // `gen_iter` returns an iterator that yields a infinite number of randomly
    // generated numbers
    let mut v: Vec<u8> = rng.gen_iter::<u8>().take(10).collect();

    println!("10 randomly generated u8 values");
    println!("{:?}", v.as_slice());

    // `shuffle` shuffles a mutable slice in place
    rng.shuffle(v.as_mut_slice());
    println!("shuffle previous slice");
    println!("{:?}", v.as_slice());

    // `choose` will sample an slice *with* replacement
    // i.e. the same element can be chosen more than one time
    println!("sample previous slice *with* replacement 10 times");
    for _ in range(0u32, 10) {
        match rng.choose(v.as_slice()) {
            None => panic!("slice was empty"),
            Some(x) => println!("{:?}", x),
        }
    }
}
