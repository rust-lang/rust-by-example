Давайте создадим библиотеку и посмотрим, как связать её с другим контейнером.

{rary.rs}

```
$ rustc --crate-type=lib rary.rs
$ ls lib*
library.rlib
```

Библиотеки получают префикс «lib», и по умолчанию имеют то же имя,
что и их контейнеры, но это имя можно изменить
с помощью [атрибута `crate_name`][crate-name].

[crate-name]: ../attribute/crate.html
