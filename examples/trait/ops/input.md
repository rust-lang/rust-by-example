In Rust, many of the operators can be overloaded via traits. That is, some operators can
be used to accomplish different tasks based on their input arguments. This is possible
because operators are syntactic sugar for method calls. For example, the `+` operator in 
`a + b` calls the `add` method (as in `a.add(b)`). This `add` method is part of the `Add` 
trait. Hence, the `+` operator can be used by any implementor of the `Add` trait.

A list of the traits, such as `Add`, that overload operators are available [here][ops].

{operator.play}

###See Also

[Add][add], [Syntax Index][syntax]

[add]: http://doc.rust-lang.org/core/ops/trait.Add.html
[ops]: http://doc.rust-lang.org/core/ops/
[syntax]: https://doc.rust-lang.org/book/syntax-index.html
