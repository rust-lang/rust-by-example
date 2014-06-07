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
    println!("0011 AND 0101 is {:04t}", 0b0011 & 0b0101);
    println!("0011 OR 0101 is {:04t}", 0b0011 | 0b0101);
    println!("0011 XOR 0101 is {:04t}", 0b0011 ^ 0b0101);
    println!("1 << 5 is {}", 1 << 5);
    println!("100 >> 2 is {}", 100 >> 2);

    // use underscores to improve readability!
    println!("One million is written as {}", 1_000_000);
}
