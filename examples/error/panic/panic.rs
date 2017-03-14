fn give_princess(gift: &str) {
    // Принцесса ненавидит змей, поэтому нам нужно остановиться, если она не одобрит!
    if gift == "змея" { panic!("AAAaaaaa!!!!"); }

    println!("Я люблю тебя, {}!!!!!", gift);
}

fn main() {
    give_princess("плюшевый мишка");
    give_princess("змея");
}
