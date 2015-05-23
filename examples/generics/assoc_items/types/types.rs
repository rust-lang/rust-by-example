struct Container(i32, i32);

// A trait which will check to see if two items are stored inside of container.
// Also retrieves first or last value.
trait Contains {
    // Define generic types here which methods will be able utilize.
    type A;
    type B;

    fn contains(&self, &Self::A, &Self::B) -> bool;
    fn first(&self) -> i32;
    fn last(&self) -> i32;
}

impl Contains for Container {
    // Specify what types `A` and `B` are. If the `input` type
    // is `Container(i32, i32)`, the `output` types are determined
    // as `i32` and `i32`.
    type A = i32;
    type B = i32;

    // `&Self::A` and `&self::B` are also valid here.
    fn contains(&self, number: &i32, digit: &i32) -> bool {
        (&self.0 == number) && (&self.1 == digit)
    }
    // Grab the first number.
    fn first(&self) -> i32 { self.0 }

    // Grab the last number.
    fn last(&self) -> i32 { self.1 }
}

fn difference<C: Contains>(container: &C) -> i32 {
    container.last() - container.first()
}

fn main() {
    let number = 3;
    let digit = 10;

    let container = Container(number, digit);

    println!("Does container contain {} and {}: {}",
        &number, &digit,
        container.contains(&number, &digit));
    println!("First number: {}", container.first());
    println!("Last number: {}", container.last());
    
    println!("The difference is: {}", difference(&container));
}
