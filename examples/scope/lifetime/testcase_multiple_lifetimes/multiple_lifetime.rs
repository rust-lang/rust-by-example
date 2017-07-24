#[derive(Debug)]
// Foo has now 2 lifetime parameters. Note that it does not change its computed
// lifetime: it is still the intersection of a and b lifetimes.
struct Foo<'a, 'b> {
    a: &'a i32,
    b: &'b i32,
}

// lifetimes are elided here. real signature is
// fn create_foo_and_get_b<'bparam>(b: &'bparam i32) -> &'bparam i32
fn create_foo_and_get_b(b: &i32) -> &i32 {
    let a = &1;
    // 'a is set to a's lifetime, 'b to the hidden 'bparam lifetime, which is
    // the lifetime of first_b in the main function.
    let foo = Foo {a:a, b:b};
    println!("Foo is {:?}", foo);
    get_b(&foo)
}

// the compiler will only check if b lives long enough, not the whole foo
// structure any more.
fn get_b<'a, 'b>(foo: &Foo<'a, 'b>) -> &'b i32 {
    println!("Extracting b from {:?}", foo);
    &foo.b
}

fn main() {
  let first_b = &2;
  let b = create_foo_and_get_b(first_b);
  println!("b is {}", b);
} // first_b and b end of lifetime
