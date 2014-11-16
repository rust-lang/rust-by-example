在 Rust 语言中，许多运算符（Operators）都能通过接口（Traits）重载。这是因为运算符只是方法调用的语法糖。
例如，`a + b` 被当作 `a.add(&b)` 处理。这里的 `add` 方法是接口 `Add` 的一部分，所有 `Add` 接口的实现者都能使用 `+` 运算符。

{operator.play}

完整的可重载运算符接口列表可参见：[ops](http://doc.rust-lang.org/core/ops/)。
