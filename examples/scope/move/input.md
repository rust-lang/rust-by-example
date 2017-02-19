Because variables are in charge of freeing their own resources, 
**resources can only have one owner**. This also prevents resources 
from being freed more than once. Note that not all variables own 
resources (e.g. [references]).

When doing assignments (`let x = y`) or passing function arguments by value
(`foo(x)`), the *ownership* of the resources is transferred. In Rust-speak, 
this is known as a *move*.

After moving resources, the previous owner can no longer be used. This avoids
creating dangling pointers.

{move.play}

[references]: /flow_control/match/destructuring/destructure_pointers.html
