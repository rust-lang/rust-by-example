Closures are inherently flexible and will do what the functionality requires
to make the closure work without annotation. This allows capturing to
flexibly adapt to the use case, sometimes moving and sometimes borrowing.
Closures can capture variables:

* by reference: `&T`
* by mutable reference: `&mut T`
* by value: `T`

They preferentially capture variables by reference and only go lower when
required.

{capture.play}

### See also:

[`Box`][box] and [`std::mem::drop`][drop]

[box]: /std/box.html
[drop]: http://doc.rust-lang.org/std/mem/fn.drop.html
