Error handling is the process of handling the possibility of failure. For 
example, failing to read a file and then continuing to use that *bad* input 
would clearly be problematic. Error handling allows us to notice and handle 
those errors in an explicit fashion, saving the rest of the program from 
potential issues.

The simplest error handling mechanism we will see is `panic`. It prints an 
error message, starts unwinding the task, and usually exits the program. 
Consider the following example:

{error.play}

This shows that we can induce program failure at will, but raises a 
question: what happens if the princess is *not* given a gift? We *could* 
explicitly test this with a check against the null string (`""`) as we do
with the snake, but this is not reliable. The problem is that programmers do 
not habitually test these checks unless required to by the compiler.

In order for this to be reliable, we'll want the compiler to point out 
cases where there may not be a gift. As you'll see in the next section, we 
can use `Option` to take care of this condition.
