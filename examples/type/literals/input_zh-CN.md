数值字面常量（Numeric literals）后面加上类型名称作为后缀，可用作声明它的类型。有两个例外，`uint`类型使用`u`后缀、`int`类型使用`i`后缀。

未添加类型后缀的字面常量，其类型依赖于后续如何使用。如果编译器不能确定，将会报错。

{literals.play}

上述代码中出现一些新概念，简要介绍如下：

* `fun(&foo)` 向函数传递参数的 *引用* 而不是 *值* (`fun(foo)`). 详情请看 [借用](/borrow.html)
* `std::mem::size_of_val` 带有全路径（full path）的函数。代码可按逻辑分割为不同的模块（Modules），函数 `size_of_val` 被定义在模块 `mem` 内，而模块 `mem` 被定义在库（Libraries，Rust术语叫Crate） `std` 内。详情可见 [modules](/mod.html) 和 [crates](/crates.html) 。
