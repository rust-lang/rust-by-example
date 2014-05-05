fn main() {
    // all type annotations are superfluous
    let n: int = 5;

    let big_n: int =
        if n < 10 {
            println!("small number, increase ten-fold");

            // this expression returns an int
            10 * n
        } else {
            println!("big number, reduce by two");

            // this expression must return an int as well
            n / 2
        };

    println!("{} -> {}", n, big_n);
}
