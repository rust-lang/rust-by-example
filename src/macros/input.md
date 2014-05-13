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

Macros can use `+` in the argument list, to indicate that an argument may
repeat at least once, or `*`, to indicate that the argument may repeat zero or
more times.

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
