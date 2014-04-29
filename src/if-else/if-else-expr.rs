fn main() {
    let n = 5;

    let big_n = if n < 10 {
        println!("small number, increase ten-fold");

        10 * n
    } else {
        println!("big number, reduce by two");

        n / 2
    };

    println!("{} -> {}", n, big_n);
}
