It's possible to declare variable bindings first, and initialize them later.
However, this form is seldom used, as it may lead to the use of uninitialized
variables.

{declare.play}

The compiler forbids use of uninitialized variables, as this would lead to
undefined behavior.
