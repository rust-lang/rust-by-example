Closures are special functions that can capture the variables available in the
surrounding scope. Closures consist of three parts:

* a list of arguments enclosed by pipes `|`, these arguments can optionally be
  type annotated, but usually the compiler will infer their types
* optionally the return type using an arrow `->`, again this usually gets
  inferred
* a block, the last expression is the return value

{closures.rs}

{closures.out}
