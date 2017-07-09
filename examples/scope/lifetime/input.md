A *lifetime* is a construct the compiler (also called the borrow checker)
uses to ensure all borrows are valid. Specifically, a variable's lifetime
begins when it is created and ends when it is destroyed. While lifetimes
and scopes are often referred to together, they are not the same.

Take, for example, the case where we borrow a variable via `&`. The
borrow has a lifetime that is determined by where it is declared. As a result,
the borrow is valid as long as it ends before the lender is destroyed. However,
the scope of the borrow is determined by where the reference is used.

In the following example and in the rest of this section, we will see how
lifetimes relate to scopes, as well as how the two differ.

{lifetime.play}

Note that no names or types are assigned to label lifetimes.
This restricts how lifetimes will be able to be used as we will see.
