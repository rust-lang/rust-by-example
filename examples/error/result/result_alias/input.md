How about when we want to reuse a specific `Result` type many times? 
It quickly becomes tedious to write out the full type name, but recall that Rust allows 
us to create [aliases][typealias]. A generic alias may conveniently be defined for the 
specific `Result` in question:

{alias.play}

At a module level, creating aliases can be particularly helpful. Errors 
found in a specific module often have the same `Err` type, so a single alias can succinctly 
define *all* associated `Results`. This is so useful that the `std` library even supplies one: `io::Result`!

### See also:

[`Result`][result] and [`io::Result`][io_result]

[typealias]: /cast/alias.html
[result]: http://doc.rust-lang.org/std/result/enum.Result.html
[io_result]: http://doc.rust-lang.org/std/io/type.Result.html
