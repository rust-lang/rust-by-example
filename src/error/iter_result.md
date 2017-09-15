# Iterating over `Result`s

An `Iter::map` operation might fail, for example:

```rust,editable
fn main() {
    let strings = vec!["tofu", "93", "18"];
    let possible_numbers: Vec<Result<i32, _>> = strings
        .into_iter()
        .map(|s| s.parse())
        .collect();
    println!("Results: {:?}", possible_numbers);
}
```

Let's step through strategies for handling this.

## Ignore the failed items with `filter_map()`

`filter_map` calls a function and filters out the results that are `None`.

```rust,editable
fn main() {
    let strings = vec!["tofu", "93", "18"];
    let numbers: Vec<i32> = strings
        .into_iter()
        .map(|s| s.parse())
        .filter_map(Result::ok)
        .collect();
    println!("Results: {:?}", numbers);
}
```

## Fail the entire operation with `collect()`

`Result` implements `FromIter` so that a vector of results (`Vec<Result<T, E>>`)
can be turned into a result with a vector (`Result<Vec<T>, E>`). Once an
`Result::Err` is found, the iteration will terminate.

```rust,editable
fn main() {
    let strings = vec!["tofu", "93", "18"];
    let numbers: Result<Vec<i32>, _> = strings
        .into_iter()
        .map(|s| s.parse())
        .collect();
    println!("Results: {:?}", numbers);
}
```

This same technique can be used with `Option`.

## Collect all valid values and failures with `partition()`

```rust,editable
fn main() {
    let strings = vec!["tofu", "93", "18"];
    let (numbers, errors): (Vec<Result<i32, _>>, Vec<_>) = strings
        .into_iter()
        .map(|s| s.parse())
        .partition(Result::is_ok);
    println!("Numbers: {:?}", numbers);
    println!("Errors: {:?}", errors);
}
```

When you look at the results, you'll note that everything is still wrapped in
`Result`.  A little more boilerplate is needed for this.

```rust,editable
fn main() {
    let strings = vec!["tofu", "93", "18"];
    let (numbers, errors): (Vec<Result<i32, _>>, Vec<_>) = strings
        .into_iter()
        .map(|s| s.parse())
        .partition(Result::is_ok);
    let numbers: Vec<_> = numbers.into_iter().map(Result::unwrap).collect();
    let errors: Vec<_> = errors.into_iter().map(Result::unwrap_err).collect();
    println!("Numbers: {:?}", numbers);
    println!("Errors: {:?}", errors);
}
```
