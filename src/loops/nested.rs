fn main() {
    'outer: loop {
        println!("Entered the outer loop");

        'inner: loop {
            println!("Entered the inner loop");

            // this would break the inner loop
            //break;

            // this breaks the outer loop
            break 'outer;
        }

        println!("This point will never be reach");
    }

    println!("Exited the outer loop");
}
