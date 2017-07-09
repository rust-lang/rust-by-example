Так же, как и`if let`, `while let` может сделать неудобный `match`
более терпимым. Рассмотрим следующий пример, в котором мы увеличиваем значение `i`:

```rust
// Создадим переменную `optional` с типом `Option<i32>`
let mut optional = Some(0);

// Неоднократно повторим наш тест.
loop {
    match optional {
        // Если `optional` деструктурируется, выполним следующий блок.
        Some(i) => {
            if i > 9 {
                println!("Больше 9, уходим отсюда!");
                optional = None;
            } else {
                println!("`i` равен `{:?}`. Попробуем еще раз.", i);
                optional = Some(i + 1);
            }
            // ^ Требует 3 уровня вложенности!
        },
        // Выходим из цикла в случаи ошибки деструктуризации:
        _ => { break; }
        // ^ Зачем это нужно? Должен быть способ сделать это лучше!
    }
}
```

Использование `while let` делает этот пример немного приятнее:

{while_let.play}

### Смотрите также:

[`enum`][enum], [`Option`][option], and the [RFC][while_let_rfc]

[enum]: ../custom_types/enum.html
[option]: ../std/option.html
[while_let_rfc]: https://github.com/rust-lang/rfcs/pull/214
