// function that returns a bool value
fn is_divisible_by(lhs: uint, rhs: uint) -> bool {
    // corner case, early return
    if rhs == 0 {
        return false
    }

    // this is an expression, the `return` keyword is not necessary here
    lhs % rhs == 0
}

// function that doesn't return a value
fn fizzbuzz(n: uint) {
    if is_divisible_by(n, 15) {
        println!("fizzbuzz");
    } else if is_divisible_by(n, 3) {
        println!("fizz");
    } else if is_divisible_by(n, 5) {
        println!("buzz");
    } else {
        println!("{}", n);
    }
}

fn main() {
    for n in range(1u, 101) {
        fizzbuzz(n);
    }
}
