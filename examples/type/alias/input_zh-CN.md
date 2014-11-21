使用 `type` 关键字定义某个类型的别名。类型名称必须采用骆驼命名法（即所有单词首字母大写），否则编译器将警告。只有基础类型（`uint` `f32`等）例外。

{alias.play}

类型别名的一个主要作用是得到更简短的名称。例如 [`IoResult<T>`](http://doc.rust-lang.org/std/io/type.IoResult.html) 是 `Result<T, IoError>` 的别名。
