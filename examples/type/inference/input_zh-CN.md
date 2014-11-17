Rust的类型推导系统非常智能。不仅能根据初始化语句里的[右值](https://en.wikipedia.org/wiki/Value_%28computer_science%29#lrvalue)推导类型，还能根据后续使用推导类型。下面是典型的例子：

{inference.play}

不强求明确声明变量的类型。编译器通常能自动推导出来。程序员也省事。
