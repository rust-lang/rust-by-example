#![feature(asm)]

fn add(a: i32, b: i32) -> i32 {
    let sum: i32;
    unsafe {
        asm!("add $2, $1; mov $1, $0" : "=r"(sum) : "r"(a), "r"(b));
    }
    sum
}
