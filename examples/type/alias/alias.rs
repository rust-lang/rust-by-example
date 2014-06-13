// `NanoSecond` is a new name for `u64`
type NanoSecond = u64;
type Inch = u64;

// Use an attribute to silence warning
#[allow(non_camel_case_types)]
type uint64_t = u64;
// TODO ^ Try removing the attribute

fn main() {
    // `NanoSecond` = `Inch` = `uint64_t` = `u64`
    let nanoseconds: NanoSecond = 5 as uint64_t;
    let inches: Inch = 2 as uint64_t;

    // Note that type aliases *don't* provide any extra type safety, because
    // aliases are *not* new types
    println!("{} nanoseconds + {} inches = {} unit?",
             nanoseconds,
             inches,
             nanoseconds + inches);
}
