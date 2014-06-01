use std::io::process::Command;

fn main() {
    let process = Command::new("sleep").arg("5").spawn();

    println!("reached end of main");
}
