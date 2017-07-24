Now that we know how to specify lifetime parameters, let's see what difference
they can make in practice.

Consider the following example and let's see what happens:

```rust
#[derive(Debug)]
// Rust compiler cannot use elision here, because there is 2 pointers. But we
// decide to use only one lifetime parameter here, 'foo will get coerced to
// the intersection of the lifetimes of a and b at runtime.
struct Foo<'foo> {
    a: &'foo i32,
    b: &'foo i32,
}
```

We can write a `get_b` function that returns a reference to b:

```rust
// Here we are saying that foo must outlive the return pointer. In other word,
// that the returning pointer is tied to foo.
// Indeed, if we discard foo, &foo.b won't be available any more.
fn get_b<'foo>(foo: &'foo Foo) -> &'foo i32 {
    println!("Extracting b from {:?}", foo);
    &foo.b
}

fn main() {
  let a = &1;
  let b = &2;
  // 'foo lifetime must be included in the intersection of a and b lifetime,
  // because it borrows a and b. It is the case here.
  // Note that we must declare foo after a and b, because variables go out of
  // scope in the reverse order they are declared.
  // In practice 'foo lifetime will be coerced to 'b lifetime.
  let foo = Foo {a: a, b: b};

  println!("&foo.b is {}", get_b(foo));
}
```

Here, things works just fine. However, we are coercing the lifetime of `get_b`
to the intersection of a and b lifetimes. This is overly restrictive, because
the return value is in fact only conditioned by b's lifetime. This prevents this
other program from compiling:

```rust
#[derive(Debug)]
struct Foo<'foo> {
    a: &'foo i32,
    b: &'foo i32,
}

// actual signature (the lifetime is elided):
// fn create_foo_and_get_b<'bparam>(b: &'bparam i32) -> &'bparam i32
fn create_foo_and_get_b(b: &i32) -> &i32 {
    let a = &1; // error: borrowed value does not live long enough
    let foo = Foo {a:a, b:b};
    println!("Foo is {:?}", foo);
    get_b(&foo) // error: `foo` does not live long enough
} // a lifetime ends.

fn get_b<'l>(foo: &'l Foo) -> &'l i32 {
    println!("Extracting b from {:?}", foo);
    &foo.b
}

fn main() {
  let first_b = &2;
  let b = create_foo_and_get_b(first_b);
  println!("b is {}", b);
}
```

Here, foo lifetime get coerced to the shortest lifetime, ie the lifetime of a.

The 'l lifetime parameter gets the lifetime of foo (ie the one of a) at runtime but
foo only lives in `create_foo_and_get_b` function. So the compiler complains foo
does not live long enough to return the value to the main function.

The compiler also shows us the root cause: a is the one that coerced foo's
lifetime, and thus `get_b` lifetime parameter.

The solution here is annotating lifetimes more precisely:

{multiple_lifetime.play}

Incidentally, we see here that lifetimes are not the same thing as scope: the
lifetime of `first_b` propagate to `get_b` function, even though `first_b` is
not in scope there. The lifetime of a value owner is indeed the union of its
scope and all its borrowers' scope.

Note that specifying lifetimes does not change the code behaviour, it is merely
a way to give information about the code to the compiler, to make it able to
correctly check the soundness of our program.

### See also:

[Borrowing (`&`)], [lifetime elision].

[Borrowing (`&`)]: /scope/borrow.html
[lifetime elision]: /scope/lifetime/elision.html
