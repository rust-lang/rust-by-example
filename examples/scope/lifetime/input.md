A *lifetime* is a construct the compiler (also called the borrow checker)
uses to ensure all borrows are valid. Specifically, the lifetime refers to
the span which starts when the variable is created and ends when the variable
is destroyed. Borrowing (via `&` for example) also creates new lifetimes.

A borrow is valid as long as the borrow ends before (inside) the lender is
destroyed. As you can see below, the lifetime of a variable is directly related
to the scope in which it was created:

{lifetime.play}

You may have noted that no names or types are assigned to label lifetimes.
This restricts how lifetimes will be able to be used as we will see.
