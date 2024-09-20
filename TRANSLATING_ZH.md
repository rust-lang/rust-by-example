# Rust by Example 中文版翻译指南

Rust by Example 中文版的翻译在 https://github.com/rust-lang-cn/rust-by-example 仓库进行审核和校对。

如果您希望参与翻译，请按照以下流程操作：

1. 复刻（fork）https://github.com/rust-lang/rust-by-example 仓库。

2. 在 `po/zh.po` 文件中添加或修改译文。

3. 向 https://github.com/rust-lang-cn/rust-by-example 仓库的 `zh` 分支提交 PR（Pull Request）。
   - 先在 rust-lang-cn 仓库进行审校，而非直接向原始仓库提交 PR，这样可以更方便进行译文讨论。
   - 中文社区仓库的默认分支为 `zh`，因此正常创建 PR 时会自动指向该分支。

rust-lang-cn 翻译组成员的维护流程：

1. 审核并校对提交的 PR，先合并到 `zh` 分支。

2. 向 https://github.com/rust-lang/rust-by-example 原始仓库提交 PR。

## 翻译规范

### 总体原则

* 文风应保持正式、清晰、简洁
* 标点符号原则上使用全角（如括号"（）"和冒号"："等）

### 术语翻译指南

* 对于已经广泛接受的技术术语，保留英文或使用通用的中文翻译
* 避免生硬的直译，优先考虑符合中文语境的自然表达
* 保持术语翻译的一致性，可参考 [Rust 语言术语中英文对照表](https://github.com/rust-lang-cn/english-chinese-glossary-of-rust/blob/master/rust-glossary.md)
