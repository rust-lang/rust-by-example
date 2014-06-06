In Rust, almost every statement is an expression, this means that the statement
returns a value. This may not always be desired, so the output can be
suppressed by ending the expression with a semicolon `;`.

A block is a collection of statements and expressions enclosed by braces `{}`.
Block are expressions too, so they can be used as rvalues in assignments. The
last expression in the block will be assigned to the lvalue. If the last
expression of the block ends with a semicolon, the return value will be `()`.

{expression.play}
