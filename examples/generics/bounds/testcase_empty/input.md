A consequence of how bounds work is that even if a `trait` doesn't
include any functionality, you can still use it as a bound. `Eq` and
`Ord` are examples of such `trait`s from the `std` library.

{empty.play}

### Смотрите также:

[`std::cmp::Eq`][eq], [`std::cmp::Ord`s][ord], and [`trait`s][traits]

[eq]: http://doc.rust-lang.org/std/cmp/trait.Eq.html
[ord]: http://doc.rust-lang.org/std/cmp/trait.Ord.html
[traits]: ../../trait.html
