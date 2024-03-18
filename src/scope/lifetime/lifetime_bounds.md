# Bounds

Just like generic types can be bounded, lifetimes (themselves generic) use
bounds as well. The `:` character has a slightly different meaning here, but `+`
is the same. Note how the following read:

1. `T: 'a`: *All* references in `T` must outlive lifetime `'a`.
2. `T: Trait + 'a`: Type `T` must implement trait `Trait` and *all* references
   in `T` must outlive `'a`.

The example below shows the above syntax in action used after keyword `where`:

```rust,editable
use std::fmt::Debug; // Trait to bound with.

#[derive(Debug)]
struct Ref<'a, T: 'a>(&'a T);
// `Ref` contains a reference to a generic type `T` that has
// an unknown lifetime `'a`. `T` is bounded such that any
// *references* in `T` must outlive `'a`. Additionally, the lifetime
// of `Ref` may not exceed `'a`.

// A generic function which prints using the `Debug` trait.
fn print<T>(t: T)
where
    T: Debug,
{
    println!("`print`: t is {:?}", t);
}

// Here a reference to `T` is taken where `T` implements
// `Debug` and all *references* in `T` outlive `'a`. In
// addition, `'a` must outlive the function.
fn print_ref<'a, T>(t: &'a T)
where
    T: Debug + 'a,
{
    println!("`print_ref`: t is {:?}", t);
}

fn main() {
    let x = 7;
    let ref_x = Ref(&x);

    print_ref(&ref_x);
    print(ref_x);
}
```

### See also:

[generics][generics], [bounds in generics][bounds], and
[multiple bounds in generics][multibounds]

[generics]: ../../generics.md
[bounds]: ../../generics/bounds.md
[multibounds]: ../../generics/multi_bounds.md
