# Input functions

Since closures may be used as arguments, you might wonder if the same can be
said about functions. And indeed they can! If you declare a function that takes
a closure as parameter, then any function that satisfies the trait bound of that
closure can be passed as a parameter.

```rust,editable
// Define a function which takes a generic `F` argument
// bounded by `Fn`, and calls it
fn call_me<F: Fn()>(f: F) {
    f();
}

// Define a wrapper function satisfying the `Fn` bound
fn function() {
    println!("I'm a function!");
}

fn main() {
    // Define a closure satisfying the `Fn` bound
    let closure = || println!("I'm a closure!");

    call_me(closure);
    call_me(function);
}
```

As an additional note, the `Fn`, `FnMut`, and `FnOnce` `traits` dictate how a
closure captures variables from the enclosing scope.

### See also:

[`Fn`][fn], [`FnMut`][fn_mut], and [`FnOnce`][fn_once]

[fn]: https://doc.rust-lang.org/std/ops/trait.Fn.html
[fn_mut]: https://doc.rust-lang.org/std/ops/trait.FnMut.html
[fn_once]: https://doc.rust-lang.org/std/ops/trait.FnOnce.html
