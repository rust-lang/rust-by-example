use std::os;

fn main() {
    let args = os::args();
    println!("My path is {}.", args.get(0));
    println!("My arguments are {}.", args.tail());
}
