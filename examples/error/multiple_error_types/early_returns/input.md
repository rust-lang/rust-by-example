In the previous example, we explicitly handled the errors using combinators. 
Another way to deal with this case analysis is to use a combination of 
`match` statements and *early returns*. 

That is, we can simply stop executing the function and return the error if 
one occurs. For some, this form of code can be easier to both read and 
write. Consider this version of the previous example, rewritten using early returns:

{early_returns.play}

At this point, we've learned to explicitly handle errors using combinators 
and early returns. While we generally want to avoid panicking, always 
handling errors explicitly is cumbersome.

So what if we could `unwrap` it without possibly inducing `panic`? That is where we are headed next.
