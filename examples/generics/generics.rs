// A generic struct
struct Pair<T> {
    first: T,
    second: T,
}

// A generic function
fn swap<T>(pair: Pair<T>) -> Pair<T> {
    let Pair { first, second } = pair;

    Pair { first: second, second: first }
}

// Reimplementing a 2-element tuple as a tuple struct
struct Tuple2<T, U>(T, U);

fn main() {
    // Explicitly specialize `Pair`
    let pair_of_chars: Pair<char> = Pair { first: 'a', second: 'b' };

    // Implicitly specialize `Pair`
    let pair_of_ints = Pair { first: 1i32, second: 2 };

    // Explicitly specialize `Tuple2`
    let _tuple: Tuple2<char, i32> = Tuple2('R', 2);

    // Explicitly specialize `swap`
    let _swapped_pair_of_chars = swap::<char>(pair_of_chars);

    // Implicitly specialize `swap`
    let _swapped_pair_of_ints = swap(pair_of_ints);
}

