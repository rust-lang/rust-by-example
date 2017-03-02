Модули могут быть отображены на иерархию файлов и директорий.
Давайте разобьём [пример с видимостью модулей][visibility] на файлы:

```
$ tree .
.
|-- my
|   |-- inaccessible.rs
|   |-- mod.rs
|   `-- nested.rs
`-- split.rs
```

{split.rs}

{my/mod.rs}

{my/nested.rs}

{my/inaccessible.rs}

Убедимся, что всё работает так, как раньше:

```
$ rustc split.rs && ./split
вызвана `my::function()`
вызвана `function()`
вызвана `my::indirect_access()`, которая
> вызывает `my::private_function()`
вызвана `my::nested::function()`
```

[visibility]: ../mod/visibility.html
