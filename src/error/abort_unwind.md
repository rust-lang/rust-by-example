# `abort` and `unwind`

The previous section illustrates the error handling mechanism `panic`.  The `cfg_panic` feature makes it possible to execute different code depending on the panic strategy. The current values available are `unwind` and `abort`. 


Building on the prior lemonade example, we explicitly use the panic strategy to execise different lines of code.  

```rust,editable,ignore,mdbook-runnable

fn drink(beverage: &str) {
   // You shouldn't drink too much sugary beverages.
    if beverage == "lemonade" {
        if cfg!(panic="abort"){ println!("This is not your party. Run!!!!");}
        else{ println!("Spit it out!!!!");}
    }
    else{ println!("Some refreshing {} is all I need.", beverage); }
}

fn main() {
    drink("water");
    drink("lemonade");
}
```

Here is another example focusing on rewriting `drink()` and explicitly use the `unwind` keyword.

```rust,editable,ignore

#[cfg(panic = "unwind")]
fn ah(){ println!("Spit it out!!!!");}

#[cfg(not(panic="unwind"))]
fn ah(){ println!("This is not your party. Run!!!!");}

fn drink(beverage: &str){
    if beverage == "lemonade"{ ah();}
    else{println!("Some refreshing {} is all I need.", beverage);}
}

fn main() {
    drink("water");
    drink("lemonade");
}
```

The panic strategy can be set from the command line by using `abort` or `unwind`.

```rust,editable,ignore
rustc  lemonade.rc -C panic=abort
```

