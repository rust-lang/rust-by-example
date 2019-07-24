# `panic`

The simplest error handling mechanism we will see is `panic`. It prints an 
error message, starts unwinding the stack, and usually exits the program. 
Here, we explicitly call `panic` on our error condition: 

```rust,editable,ignore,mdbook-runnable
fn give_princess(gift: &str) {
    // Princesses hate snakes, so we need to stop if she disapproves!
    if gift == "snake" { panic!("AAAaaaaa!!!!"); }

    println!("I love {}s!!!!!", gift);
}

fn main() {
    give_princess("teddy bear");
    give_princess("snake");
}
```
