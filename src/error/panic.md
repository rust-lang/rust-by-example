# `panic`

The simplest error handling mechanism we will see is `panic`. It prints an error
message, starts unwinding the stack, and usually exits the program. Here, we
explicitly call `panic` on our error condition:

```rust,editable,ignore,mdbook-runnable
fn drink(beverage: &str) {
    // You shouldn't drink too much sugary beverages.
    if beverage == "lemonade" {
        panic!("AAAaaaaa!!!!");
    }

    println!("Some refreshing {} is all I need.", beverage);
}

fn main() {
    drink("water");
    drink("lemonade");
    drink("still water");
}
```

The first call to `drink` works. The second panics and thus the third is never
called.
