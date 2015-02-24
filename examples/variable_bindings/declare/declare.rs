fn main() {
    // Declare a variable
    let a_variable;

    {
        let x = 2;

        // Initialize the variable
        a_variable = x * x;
    }

    println!("a variable: {}", a_variable);

    let another_variable;

    // Error! Use of uninitialized variable
    println!("another variable: {}", another_variable);
    // FIXME ^ Comment out this line

    another_variable = 1;

    println!("another variable: {}", another_variable);
}
