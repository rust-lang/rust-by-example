# Clone and Copy

When dealing with resources, the default behavior is to transfer them during
assignments or function calls. However, sometimes we need to make a
copy of the resource as well.

The [`Clone`][clone] trait helps us do exactly this. Most commonly, we can
use the `.clone()` method defined by the `Clone` trait.

## Copy: Implicit Cloning

The [`Copy`][copy] trait allows a type to be duplicated simply by copying bits,
with no additional logic required. When a type implements `Copy`, assignments
and function calls will implicitly copy the value instead of moving it.

**Important:** `Copy` requires `Clone` - any type that implements `Copy` must
also implement `Clone`. This is because `Copy` is defined as a subtrait:
`trait Copy: Clone {}`. The `Clone` implementation for `Copy` types simply
copies the bits.

Not all types can implement `Copy`. A type can only be `Copy` if:
- All of its components are `Copy`
- It doesn't manage external resources (like heap memory, file handles, etc.)

```rust,editable
// A unit struct without resources
// Note: Copy requires Clone, so we must derive both
#[derive(Debug, Clone, Copy)]
struct Unit;

// A tuple struct with resources that implements the `Clone` trait
// This CANNOT be Copy because Box<T> is not Copy
#[derive(Clone, Debug)]
struct Pair(Box<i32>, Box<i32>);

fn main() {
    // Instantiate `Unit`
    let unit = Unit;
    // Copy `Unit` - this is an implicit copy, not a move!
    // Because Unit implements Copy, the value is duplicated automatically
    let copied_unit = unit;

    // Both `Unit`s can be used independently
    println!("original: {:?}", unit);
    println!("copy: {:?}", copied_unit);

    // Instantiate `Pair`
    let pair = Pair(Box::new(1), Box::new(2));
    println!("original: {:?}", pair);

    // Move `pair` into `moved_pair`, moves resources
    // Pair does not implement Copy, so this is a move
    let moved_pair = pair;
    println!("moved: {:?}", moved_pair);

    // Error! `pair` has lost its resources
    //println!("original: {:?}", pair);
    // TODO ^ Try uncommenting this line

    // Clone `moved_pair` into `cloned_pair` (resources are included)
    // Unlike Copy, Clone is explicit - we must call .clone()
    let cloned_pair = moved_pair.clone();
    // Drop the moved original pair using std::mem::drop
    drop(moved_pair);

    // Error! `moved_pair` has been dropped
    //println!("moved and dropped: {:?}", moved_pair);
    // TODO ^ Try uncommenting this line

    // The result from .clone() can still be used!
    println!("clone: {:?}", cloned_pair);
}
```

[clone]: https://doc.rust-lang.org/std/clone/trait.Clone.html
[copy]: https://doc.rust-lang.org/std/marker/trait.Copy.html
