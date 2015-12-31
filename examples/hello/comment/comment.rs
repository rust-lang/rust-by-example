fn main() {
    // This is an example of a line comment
    // Notice how there are two slashes at the beginning of the line
    // And that nothing written inside these will be read by the compiler

    // println!("Hello, world!");

    // Run it. See? Now try deleting the two slashes, and run it again.

    /* 
     * This is another type of comment, the block comment. In general,
     * the line comment is the recommended comment style however the
     * block comment is extremely useful for debugging
     */

     /*
     Note, the previous column of `*` was entirely for style. There's
     no actual need for it.
     */

     // Observe how block comments allow easy expression manipulation
     // which line comments do not. Deleting the comment delimiters
     // will change the result:
     let x = 5 + /* 90 + */ 5;
     println!("Is `x` 10 or 100? x = {}", x);
}
