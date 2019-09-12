# `impl Trait`

If your function returns a type that implements `MyTrait`, you can write its
return type as `-> impl MyTrait`. This can help simplify your type signatures quite a lot!

```rust,editable
use std::iter;
use std::vec::IntoIter;

// This function combines two `Vec<i32>` and returns an iterator over it.
// Look how complicated its return type is!
fn combine_vecs_explicit_return_type<'a>(
    v: Vec<i32>,
    u: Vec<i32>,
) -> iter::Cycle<iter::Chain<IntoIter<i32>, IntoIter<i32>>> {
    v.into_iter().chain(u.into_iter()).cycle()
}

// This is the exact same function, but its return type uses `impl Trait`.
// Look how much simpler it is!
fn combine_vecs<'a>(
    v: Vec<i32>,
    u: Vec<i32>,
) -> impl Iterator<Item=i32> {
    v.into_iter().chain(u.into_iter()).cycle()
}
```

More importantly, some Rust types can't be written out. For example, every
closure has its own unnamed concrete type. Before `impl Trait` syntax, you had
to allocate on the heap in order to return a closure. But now you can do it all
statically, like this:

```rust,editable
// Returns a function that adds `y` to its input
fn make_adder_function(y: i32) -> impl Fn(i32) -> i32 {
    let closure = move |x: i32| { x + y };
    closure
}

fn main() {
    let plus_one = make_adder_function(1);
    assert_eq!(plus_one(2), 3);
}
```

You can also use `impl Trait` to return an iterator that uses `map` or `filter`
closures! This makes using `map` and `filter` easier. Because closure types don't
have names, you can't write out an explicit return type if your function returns
iterators with closures. But with `impl Trait` you can do this easily:

```rust,editable
fn double_positives<'a>(numbers: &'a Vec<i32>) -> impl Iterator<Item = i32> + 'a {
    numbers
        .iter()
        .filter(|x| x > &&0)
        .map(|x| x * 2)
}
```
