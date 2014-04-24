fn main() {
    let mut count = 0;

    println!("Let's count until infinity!");

    // infinite loop
    loop {
        count += 1;

        if count == 3 {
            // skip this iteration
            continue
        }

        println!("{}", count);

        if count == 5 {
            println!("OK, that's enough");
            // exit this loop
            break
        }
    }
}
