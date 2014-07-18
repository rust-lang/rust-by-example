Rust provides a powerful macro system that allows metaprogramming. As you've
seen in previous chapters, macros look like functions, except that their name
ends with a bang `!`, but instead of generating a function call, macros are
expanded into source code that gets compiled with the rest of the program.

Macros are created using the `macro_rules!` macro.

{simple.rs}

{simple.out}

The arguments of a macro are prefixed by a dollar sign `$` and type annotated
with a *designator*.

{designators.rs}

{designators.out}

Macros can be overloaded to accept different combinations of arguments.

{overload.rs}

{overload.out}

Macros can also match and return repeated patterns. This is similar but
different from a regex. `$($x:expr),+` is a matching example. It uses the
`$()` grouping operator to mark `x` as repeatable. `+` or `*` denotes
repetition amount; one or more and zero or more respectively. `,`
is the only separator token available.

```{rust,ignore}
// The separator only checks separation; this is
// not a regex.
$($x:expr),+ // matches `1, 2, 3` but not `1, 2,`
```

When returned, it uses a similar notation: `$($x:expr),+` => `$($x),+`
however, a returned list will not be re-expanded whereas, matching
will find the whole thing.

```rust
#![feature(macro_rules)]

macro_rules! repeat_broken {
    // Attempt to match and return with one compact expression
    ($($x:expr),+) => { $($x),+ };
}

macro_rules! repeat_works {
    ($x:expr) => { $x }; // one element list
    // $x is the first element. `$($y:expr),+` is the tail
    ($x:expr, $($y:expr),+) => {
        // pass the tail to `repeat_works` so it can be expanded
        ($x, repeat_works!($($y),+))
    };
}

fn main() {
    println!("{}", repeat_broken!(1u)); // Works for single elements

    // Errors: `,` ignored. A list is never re-expanded so when
    // the `,` is reached, the rest of the line is ignored.
    // Rust does not allow incomplete expansion, so it errors. To
    // handle this, recursion is needed.
    //println!("{}", repeat_broken!(1u, 2u));

    println!("{}", repeat_works!(1u));
    // Would like it flat but it nests the results...
    println!("{}", repeat_works!(1u, 2u, 3u, 4u, 5u, 6u));
}
```

Another example making a min macro:
{repeat.rs}

{repeat.out}

Macros allow writing DRY code, by factoring out the common parts of functions
and/or test suites. Here is an example that implements and tests the `+=`, `*=`
and `-=` operators on `Vec<T>`.

{dry.rs}

{dry.out}

```
$ rustc --test dry.rs && ./dry
running 3 tests
test test::mul_assign ... ok
test test::add_assign ... ok
test test::sub_assign ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured
```
