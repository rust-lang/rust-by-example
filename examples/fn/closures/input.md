Closures are special functions that can capture the variables available in the
surrounding scope. Closures consist of three parts:

* A list of arguments enclosed by pipes `|`. These arguments can optionally be
  type annotated, but usually the compiler will infer their types
* Optionally the return type using an arrow `->`. Again, this usually gets
  inferred
* A block. The last expression is the return value

{closures.play}
