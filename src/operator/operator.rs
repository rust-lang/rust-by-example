struct Foo;
struct Bar;

#[deriving(Show)]
struct FooBar;

#[deriving(Show)]
struct BarFoo;

// the Add<T, U> trait needs two generic parameters:
// T is the type of the RHS summand, and
// U is the type of the sum
// This block implements the operation: Foo + Bar = FooBar
impl Add<Bar, FooBar> for Foo {
    fn add(&self, rhs: &Bar) -> FooBar {
        println!("> Foo.add(&Bar) was called");

        FooBar
    }
}

// Addition can be implemented in a non-commutative way
// This block implements the operation: Bar + Foo = BarFoo
impl Add<Foo, BarFoo> for Bar {
    fn add(&self, rhs: &Foo) -> BarFoo {
        println!("> Bar.add(&Foo) was called");

        BarFoo
    }
}

fn main() {
    println!("Foo + Bar = {}", Foo + Bar);
    println!("Bar + Foo = {}", Bar + Foo);
}
