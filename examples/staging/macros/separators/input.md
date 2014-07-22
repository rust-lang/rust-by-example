So far we have been using a comma to separate the arguments fed into a macro,
but macros are more flexible than that! You are free to use symbols or words to
separate the input arguments of the macro.

Let's create a sugary macro to initialize hash maps:

{separators.play}

The expansion of the `map!` macro looks like this:

``` rust
let alphabet =
        {
            let mut _temp = std::collections::HashMap::new();
            _temp.insert('a', "apple");
            _temp.insert('b', "banana");
            _temp.insert('c', "carrot");
            _temp
        };
```
