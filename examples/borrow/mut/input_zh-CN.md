`&T` borrows the data via an immutable reference, and the borrower can read the
data but not modify it. Mutable data can be mutably borrowed via a mutable
reference `&mut T`, giving read/write access to the borrower.

{mut.play}
