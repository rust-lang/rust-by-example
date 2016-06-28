fn give_princess(gift: &str) {
    // Princesses hate snakes, so we need to stop if
    // she expresses her disapproval!
    if gift == "snake" { panic!("AAAaaaaa!!!!"); }

    println!("I love {}s!!!!!", gift);
}

fn main() {
    give_princess("teddy bear");
    give_princess("snake");
}
