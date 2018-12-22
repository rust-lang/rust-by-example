# structs

Similarly, a `struct` can be destructured as shown:

```rust,editable
fn main() {
    struct Foo { x: (u32, u32), y: u32 }

    // destructure members of the struct
    let foo = Foo { x: (1, 2), y: 3 };

    #[allow(unreachable_patterns)]
    match foo{
        Foo{x:first,y:second} => println!("The first field is {:?} and the second is {}",first,second),
        _ => println!("impossible")
    }
}
```

### See also:

[Structs](custom_types/structs.html), [The ref pattern](scope/borrow/ref.html)
