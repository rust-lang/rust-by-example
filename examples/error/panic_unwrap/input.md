The simplest error handling mechanism we will see is `panic`. It prints an 
error message, starts unwinding the task, and usually exits the program. 
Consider the following example:

{panic_unwrap.play}

This shows that we can induce program failure at will, but raises a 
question: what happens if the princess is *not* given a gift? We *could* 
explicitly test this with a check against the null string (`""`) as we do
with the snake, but this is not reliable. The problem is that programmers do 
not habitually test these checks unless required to by the compiler.

In order for this to be reliable, we'll want the compiler to point out 
cases where there may not be a gift. In this chapter, we will learn to use
`Option` to take care of this condition, as well as various functions to 
deal with the results of one or more uses of `Option`.
