fn give_princess(gift: &str) {
    // Princesses hate snakes so definitely stop the computation while
    // she shouts her dislike.
    if gift == "snake" { panic!("AAAaaaaa!!!!"); }

    println!("I love {}s!!!!!", gift);
}

fn main() {
    give_princess("teddy bear");
    give_princess("snake");
}
