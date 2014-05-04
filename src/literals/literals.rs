fn main() {
    // integer addition
    println!("1 + 2 = {}", 1u + 2u);

    // float division
    println!("1.0 / 2.0 = {}", 1.0f32 / 2.0f32);

    // short-circuiting boolean logic
    println!("true AND false is {}", true && false);
    println!("true OR false is {}", true || false);
    println!("NOT true is {}", !true);

    // bitwise operations
    println!("100 XOR 001 is {:t}", 0b100 ^ 0b001);

    // use underscores to improve readability!
    println!("One million is written as {}", 1_000_000)
}
