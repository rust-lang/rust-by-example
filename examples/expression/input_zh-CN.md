Rust里面几乎所有语句（statement）都是表达式（expression），也就是具有值。在表达式结尾加上分号`;`可忽略掉该值。

代码块（Blocks）也是表达式，因而也能作为赋值语句中的[右值（r-values）](https://en.wikipedia.org/wiki/Value_%28computer_science%29#lrvalue)。
代码块的最后一条表达式的值，将作为该代码块的值，被赋值给[左值（l-value）](https://en.wikipedia.org/wiki/Value_%28computer_science%29#lrvalue)。
但是，如果代码块的最后一条语句以分号`;`结尾，其值将是`()`，也就是没有值。

{expression.play}
