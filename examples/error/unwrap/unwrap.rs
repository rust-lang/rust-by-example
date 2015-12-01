// The commoner cannot bring down the task which precludes the option of `panic`.
// These must all be handled manually. `match` would be the correct approach.
fn give_commoner(gift: Option<&str>) {
    // Specify a specific course of action for each case.
    match gift {
        Some("snake") => println!("Yuck! Throws the snake in the fire."),
        Some(inner)   => println!("{}! How nice.", inner),
        None          => println!("No gift? Oh well."),
    }
}

// A princess is allowed to bring down the task at will so `panic` is an option.
fn give_princess(gift: Option<&str>) {
    // Using `unwrap` defers the case analysis to the std library which will
    // `panic` when it receives a `None`.
    let inside = gift.unwrap();
    if inside == "snake" { panic!("AAAaaaaa!!!!"); }

    println!("I love {}s!!!!!", inside);
}

fn main() {
    let food  = Some("chicken");
    let snake = Some("snake");
    let void  = None;

    give_commoner(food);
    give_commoner(snake);
    give_commoner(void);

    let bird = Some("robin");
    let nothing = None;

    give_princess(bird);
    give_princess(nothing);
}
