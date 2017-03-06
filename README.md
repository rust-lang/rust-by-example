# Rust на примерах

[![Build Status][travis-image]][travis-link]

## От переводчика

Перевод `Rust by examples` находится в процессе выполнения. За ходом перевода можно наблюдать [тут](https://github.com/ruRust/rust-by-example-ru/issues/1).

Отдельное спасибо @suhr и @AKhranovskiy. Перевод основан на их работе.

Буду рад помощи.

Проверка правописания: [Яндекс.Спеллер][yaspeller]

## Что это за проект?

Это исходный код сайта [Rust by Example][website], переведённый на русский язык! Перевод можно найти по адресу https://rurust.github.io/rust-by-example-ru

## Как помочь проекту?

Смотри [CONTRIBUTING.md][how-to-contribute].

## Как сгенерировать статический сайт

### Предварительные настройки для Debian (Ubuntu)

Установить [Rust](http://www.rust-lang.org/install.html) и выполнить:

```
sudo apt-get install nodejs npm subversion
sudo ln -s /usr/bin/nodejs /usr/bin/node
```

### Предварительные настройки для не-Debian дистрибутивов

Установить Rust [nightly](http://www.rust-lang.org/install.html),
`node`, `npm`, and `subversion`.

### Инструкция по сборке проекта

Run:

```
make all
make html pdf epub
make test
```

Посмотреть результат с помощью `make serve`.

### Детали проекта

Мы используем следующие инструменты для генерации сайта:

* [Rust][rust-lang] \o/
* [GitBook][gitbook]

`gitbook` генерирует сайт из Markdown файлов (посмотреть, как это работает
можно [тут][gitbook-format]).

Перед запуском `gitbook`, мы делаем предварительную обработку, используя
[src/main.rs][main-rs].

Предварительная обработка состоит из двух шагов:

### Генерация `SUMMARY.md`

`SUMMARY.md` генерируется из
[examples/structure.json][structure] файла. Это JSON файл, который
хранит в себе древовидную структуру "примеров".

У каждого примера есть:

* id, например `hello`
* title, например `Hello World`
* необязательный объект - children, который будет являться дополнением к примерам, например `null`
* папку внутри `examples`, например [examples/hello][hello-folder]
* входную запись в examples/structure.json, например
  `{ "id": "hello", "title": "Hello World", "children": null }`
* файл(ы) исходного кода, например [examples/hello/hello.rs][hello-rs]
* входной markdown файл, например
  [examples/hello/input.md][hello-md]

При работе с дополнительными примерами, путь к нему будет содержать id оригинального примера, например `examples/variable/mut/input.md`, т.е пример `mut` является дополнительным примером к `variable`

### Обработка `input.md`

Вместо добавления кода на Rust в `input.md`, исходный код был сохранён в отдельном файле.
Данный шаг предварительной обработки добавит код в Markdown файл.

Например, чтобы добавить исходный код из файла `hello.rs`, в Markdown файле используется следующий синтаксис

* `{hello.play}` добавляется к исходному коду в онлайн редактор кода
* `{hello.rs}` добавляется к обычному исходному коду.
* `{hello.out}` добавляется к выводу, который отображается после исполнения исходного кода.

В Makefile доступны следующие сценарии:

* `make`: сборка `update.rs` и выполнения шагов предварительной обработки
* `make book`: запуск `gitbook` для генерации книги
* `make serve`: запуск `gitbook --serve` для генерации книги и публикации ее по адресу `localhost:4000`
* `make test`: проверка всего исходного кода на языке Rust на наличие ошибок компиляции

## Перевод на другие языки

* [Английский](https://github.com/rust-lang/rust-by-example)
* [Китайский](https://github.com/rust-lang-cn/rust-by-example-cn)
* [Японский](https://github.com/rust-lang-ja/rust-by-example-ja)

## Лицензия

`Rust на примерах` распространяется по двойной лицензии - лицензия Apache 2.0 и лицензия MIT.

Более подробную информацию можно найти в файлах LICENSE-APACHE и LICENSE-MIT соответственно.

[travis-image]: https://travis-ci.org/ruRust/rust-by-example-ru.svg?branch=master
[travis-link]: https://travis-ci.org/ruRust/rust-by-example-ru
[yaspeller]: https://tech.yandex.ru/speller/
[website]: http://rustbyexample.com
[how-to-contribute]: CONTRIBUTING.md
[rust-lang]: http://www.rust-lang.org/
[gitbook]: http://www.gitbook.io
[gitbook-format]: https://github.com/GitbookIO/gitbook#book-format
[main-rs]: src/main.rs
[structure]: examples/structure.json
[hello-folder]: examples/hello
[hello-rs]: examples/hello/hello.rs
[hello-md]: examples/hello/input.md
