fn main() {
    let decimal = 65.4321_f32;

    // Error! No implicit conversion
    //let integer: u8 = decimal;
    // FIXME ^ Comment out this line

    // Explicit conversion
    let integer = decimal as u8;
    let character = integer as char;

    println!("Casting: {} -> {} -> {}", decimal, integer, character);

    // casting to a smaller unsigned type simply acts like a modulus, like in C
    println!("1000 as a u16 is: {}", 1000 as u16);
    println!("1000 mod 256 is: {}", 1000 % 256);
    println!("1000 as a u8 is: {}", 1000 as u8);

    println!(" 128 as i8: {}", 128 as i8);
    println!(" 129 as i8: {}", 129 as i8);

    println!(" -1 as u8: {}", (-1i8) as u8);
    println!(" -257 as u8: {}", (-257i16) as u8);

}
