The type inference engine is pretty smart. It does more than looking at the
type of the r-value during an initialization. It also looks how the variable is
used afterwards to infer its type. Here's an advanced example of type
inference:

{inference.play}

No type annotation of variables was needed, the compiler is happy and so is the
programmer!
