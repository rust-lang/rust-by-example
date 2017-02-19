Rust has two different types of constants which can be declared in any scope
including global. Both require explicit type annotation:

* `const`: An unchangeable value (the common case).
* `static`: A possibly `mut`able variable with [`'static`][static] lifetime.

One special case is the `"string"` literal. It can be assigned directly to a
`static` variable without modification because its type signature:
`&'static str` has the required lifetime of `'static`. All other reference
types must be specifically annotated so that they fulfill the `'static`
lifetime. This may seem minor though because the required explicit annotation
hides the distinction.

{constants.play}

### See also:

[The `const`/`static` RFC](
https://github.com/rust-lang/rfcs/blob/master/text/0246-const-vs-static.md),
[`'static` lifetime][static]

[static]: /scope/lifetime/static_lifetime.html
