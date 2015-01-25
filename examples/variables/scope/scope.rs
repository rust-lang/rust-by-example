fn main() {
    // This variable lives in the main function
    let long_lived_variable = 1;

    // This is a block, and has a smaller scope than the main function
    {
        // This variable only exists in this block
        let short_lived_variable = 2;

        println!("inner short: {}", short_lived_variable);

        // This variable *shadows* the outer one
        let long_lived_variable = 5_f32;

        println!("inner long: {}", long_lived_variable);
    }
    // End of the block

    // Error! `short_lived_variable` doesn't exist in this scope
    println!("outer short: {}", short_lived_variable);
    // FIXME ^ Comment out this line

    println!("outer long: {}", long_lived_variable);
}
