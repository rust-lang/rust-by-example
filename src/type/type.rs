fn main() {
    // All the type annotations are superfluous
    let decimal: f32 = 65.4321;
    let integer: u8 = decimal as u8;
    let character: char = integer as char;

    println!("Casting: {} -> {} -> {}", decimal, integer, character);
}
