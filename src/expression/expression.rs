fn is_divisible_by(this: uint, that: uint) -> bool {
    // if-else is an expression!
    if that == 0 {
        false
    } else {
        this % that == 0
    }
}

fn fizzbuzz(n: uint) {
    // expressions can be used as function/macro arguments
    println!("{}", if is_divisible_by(n, 15) {
        // ~ indicates this is a heap allocated string
        ~"fizzbuzz"
    } else if is_divisible_by(n, 3) {
        ~"fizz"
    } else if is_divisible_by(n, 5) {
        ~"buzz"
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
