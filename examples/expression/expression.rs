fn main() {
    // all the type annotations are superfluous
    let x: int = 5;

    let y: int = {
        // assignment statements are allowed inside blocks
        let x_squared = x * x;
        let x_cube = x_squared * x;

        // this expression will be assigned to y
        x_cube + x_squared + x
    };

    let z: () = {
        // the semicolon suppresses this expression,
        // and `()` is assigned to z
        2 * x;
    };

    println!("x is {}", x);
    println!("y is {}", y);
    println!("z is {}", z);
}
