// generic struct
struct Pair<T> {
    first: T,
    second: T,
}

// generic function
fn swap<T>(pair: Pair<T>) -> Pair<T> {
    let Pair { first: first, second: second } = pair;

    Pair { first: second, second: first }
}

// reimplementation of a 2-element Tuple
struct Tuple2<T, U>(T, U);

fn main() {
    // specialize Pair (type annotation is superfluous)
    let pair_of_chars: Pair<char> = Pair { first: 'a', second: 'b' };
    let pair_of_ints = Pair { first: 1, second: 2 };

    // specialize Tuple2
    let tuple = Tuple2("one", 2.0);

    // call generic function
    let swapped_pair_of_chars = swap(pair_of_chars);
    let swapped_pair_of_ints = swap(pair_of_ints);
}

