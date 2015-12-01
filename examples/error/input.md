Error handling is the process of handling the possibility of failure. For example, failing to
read a file and then continuing to use that *bad* input regardless clearly would be problematic.
Error handling allows us to notice and handle those errors in some explicit fashion, saving the
rest of the program from pollution.

The simplest error handling mechanism we will see is the `panic`; it prints an error message,
starts unwinding the task, and usually exits the program. Consider the following example:

{error.play}

This easily shows that we can induce program failure at will but it has a problem: what happens
if the princess is *not* given a gift? Technically, we *could* explicitly test this with a check
against the null string (`""`) the same way as with the snake however this is not reliable. The
problem is that programmers do not habitually make these checks unless required by the compiler.
In order for this to always be reliable, we require the compiler to point out the cases where there
may not be a gift. `str` does not do that for us; we require something else called `Option`.
