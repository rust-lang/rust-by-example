# Перевод

Перевод осуществляется согласно [правилам перевода][rules] сообщества.
Все термины, используемые в книге должны быть
переведены согласно [словарю терминов][dictionary].

# Создание новой задачи (issue)

## Отправление PR с исправлением мелкой ошибки

Если вы отправляете PR с исправлением мелкой ошибки в существующий пример, пожалуйста,
укажите id примера в заголовке PR, например "type/literals: исправление опечатки".

## Я нашёл опечатку в примере $X

Пожалуйста, напишите об этой опечатке в задаче [#9][typo-issue].

Если вы хотите сами исправить опечатку, но считаете, что у вас недостаточно опыта,
ознакомьтесь с [инструкцией][first-pr] на сайте сообщества.

Найти пример в исходном коде сайта довольно просто. Например,
если ссылка на пример выглядит так: `https://rurust.github.io/rust-by-example-ru/primitives.html`, то исходный код примера можно найти по пути `examples/primitives/`. Текст в файле `input.md`, а исходный код в `example_name.rs`.

Структуру книги можно изучить в файле `examples/structure.json`

## Я бы хотел увидеть пример по вопросу $TOPIC
## Я нашёл ошибку в примере $X
## Пример $X не полностью раскрывает тему
## У меня есть идеи по поводу примера $X
## Я бы хотел добавить новый пример $X

Данный проект является переводом. Задачи подобного рода лучше заводить в [репозитории][original-repo] оригинала.

# Code Style

## Markdown (.md)

* Строки должны содержать не более 99 символов.
* Ссылки должны быть выполнены в следующем стиле:

Вместо:

    [Goto my URL](http://www.myurl.com)

Используем:

    [Goto my URL][1]

    (Bottom of page)
    [1]: http://www.myurl.com

## Rust code (.rs)

* Строки должны содержать не более 99 символов.
* В комментариях код должен быть заключён в кавычки, например: ``` `println!` ```

[issues-all]: https://github.com/rust-lang/rust-by-example/issues/
[issues-open]: https://github.com/rust-lang/rust-by-example/issues?labels=&page=1&state=open
[readme]: README.md
[rules]: https://github.com/ruRust/rust_book_ru/wiki/%D0%9F%D1%80%D0%B0%D0%B2%D0%B8%D0%BB%D0%B0
[dictionary]: https://rustycrate.ru/dictionary.html
[first-pr]: https://rustycrate.ru/%D1%80%D1%83%D0%BA%D0%BE%D0%B2%D0%BE%D0%B4%D1%81%D1%82%D0%B2%D0%B0/2016/03/07/contributing.html
[original-repo]: https://github.com/rust-lang/rust-by-example
[typo-issue]: https://github.com/ruRust/rust-by-example-ru/issues/9
