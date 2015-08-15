A *lifetime* is a construct the compiler (also called the borrow checker)
uses to ensure all borrows are valid. Specifically, a lifetime starts when
a variable is created and ends when it is destroyed. It can be visualized as
vertical distance but since size is only relevant for subsets and supersets,
scope is considered the more appropriate descriptive term.

Borrowing (via `&` for example) creates new lifetimes. A borrow is valid
as long as the borrow ends before (inside) the lender is destroyed.

{lifetime.play}

You may have noted that no names or types are assigned to label lifetimes.
This restricts how lifetimes will be able to be used as we will see.
