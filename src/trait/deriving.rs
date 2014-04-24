// a tuple struct that can be compared
#[deriving(Eq, Ord)]
struct Centimeters(f64);

// a tuple struct that can be printed
#[deriving(Show)]
struct Inches(int);

impl Inches {
    fn to_centimeters(&self) -> Centimeters {
        let &Inches(inches) = self;

        Centimeters(inches as f64 * 2.54)
    }
}

// a vanilla tuple struct
struct Seconds(int);

fn main() {
    let one_second = Seconds(1);
    let another_second = Seconds(1);

    // Error: Seconds can't be printed (doesn't implement the Show trait)
    //println!("{}", one_second);

    // Error: Seconds can't be compared
    //let this_is_true = one_second == another_second;

    let foot = Inches(12);

    println!("{}", foot);

    let meter = Centimeters(100.0);

    if foot.to_centimeters() < meter {
        println!("a foot is smaller than a meter")
    } else {
        println!("a foot is bigger than a meter")
    }
}
