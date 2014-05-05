fn is_divisible_by(lhs: uint, rhs: uint) -> bool {
    // if-else is an expression!
    if rhs == 0 {
        false
    } else {
        lhs % rhs == 0
    }
}

fn fizzbuzz(n: uint) {
    // expressions can be used as function/macro arguments
    println!("{}", if is_divisible_by(n, 15) {
        // ~ indicates this is a heap allocated string
        "fizzbuzz".to_owned()
    } else if is_divisible_by(n, 3) {
        "fizz".to_owned()
    } else if is_divisible_by(n, 5) {
        "buzz".to_owned()
    } else {
        // format! is like print!, but returns the formatted string
        format!("{}", n)
    })
}

fn main() {
    for n in range(1u, 101) {
        fizzbuzz(n);
    }
}
