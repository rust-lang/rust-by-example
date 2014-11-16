这是一个传统的 Hello World 程序的源代码：

{hello.play}

`println!` 是一个 *宏（Macro）* （后面会详细解释），用于输出文本到控制台（Console）。

使用 Rust 编译器 `rust` 编译该源代码生成可执行文件：

```
$ rustc hello.rs
```

上述命令将生成可执行文件 `hello`。

```
$ ./hello
Hello World!
```
