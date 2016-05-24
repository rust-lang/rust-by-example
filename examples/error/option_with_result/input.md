In the following sections, we will see how to combine separate operations returning 
`Option` and `Result` into a single operation that returns whichever one makes the 
most sense.

The previous examples have always been very convenient; a `Result` interacted 
with another `Result` and an `Option` interacted with another `Option`. Unfortunately, 
it's not always that easy. An `Option` may have to interact with a `Result`, and a 
`Result<T, Error1>` may have to interact with a `Result<T, Error2>`.

To start us off, the example below uses `Vec::first` and `parse::<i32>` with `unwrap` to 
generate errors. `Vec::first` returns an `Option`, while `parse::<i32>` 
returns a `Result<i32, ParseIntError>`.

Note that this code "works", but is meant to showcase **improper** error handling: 

{option_result.play}
