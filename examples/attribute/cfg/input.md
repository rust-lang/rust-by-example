Условная компиляция возможна благодаря двум операторам:

* Атрибуту `cfg`: `#[cfg(...)]`, который указывается на месте атрибута
* Макросу `cfg!`: `cfg!(...)`, который можно использовать в условных выражениях

Обе инициализации имеют идентичный синтаксис для принятия аргументов.

{cfg.play}

### Смотрите также:

[the reference][ref], [`cfg!`][cfg], и [макросы][macros].

[cfg]: https://doc.rust-lang.org/std/macro.cfg!.html
[macros]: ../macros.html
[ref]: https://doc.rust-lang.org/reference.html#conditional-compilation
