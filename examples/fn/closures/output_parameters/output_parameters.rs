#![feature(fnbox)]

use std::boxed::FnBox;

// Return a closure taking no inputs and returning nothing
// which implements `FnBox` (capture by value).
fn create_fnbox() -> Box<FnBox()> {
    let text = "FnBox".to_owned();

    Box::new(move || println!("This is a: {}", text))
}

fn create_fn() -> Box<Fn()> {
    let text = "Fn".to_owned();

    Box::new(move || println!("This is a: {}", text))
}

fn create_fnmut() -> Box<FnMut()> {
    let text = "FnMut".to_owned();

    Box::new(move || println!("This is a: {}", text))
}

fn main() {
    let fn_plain = create_fn();
    let mut fn_mut = create_fnmut();
    let fn_box = create_fnbox();

    fn_plain();
    fn_mut();
    fn_box();
}
