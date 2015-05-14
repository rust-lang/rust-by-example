use std::fmt::Debug;

trait PrintInOption {
    fn print_in_option(self);
}

impl<T> PrintInOption for T where
    // Without a `where` clause we would have to express this as
    // `T: Debug` or use some other method to indirectly approach
    // this. The bound we want though is `Option<T>: Debug` because
    // that's what being printed. To do otherwise would be to use
    // the wrong bound. This requires a `where` clause.
    Option<T>: Debug {
    fn print_in_option(self) {
        println!("{:?}", Some(self));
    }
}

fn main() {
    let vec = vec![1, 2, 3];

    vec.print_in_option();
}
