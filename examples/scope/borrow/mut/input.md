Mutable data can be mutably borrowed using `&mut T`. This is called 
a *mutable reference* and gives read/write access to the borrower.
In contrast, `&T` borrows the data via an immutable reference, and 
the borrower can read the data but not modify it:

{mut.play}

### See also:
[`static`][static]

[static]: ../lifetime/static_lifetime.html
