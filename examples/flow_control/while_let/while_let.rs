fn main() {
    // Создадим переменную `optional` с типом `Option<i32>`
    let mut optional = Some(0);

    // Это можно прочитать так: "Пока `let` деструктурирует `optional` в
    // `Some(i)`, выполняем блок (`{}`). В противном случае `break`.
    while let Some(i) = optional {
        if i > 9 {
            println!("Больше 9, уходим отсюда!");
            optional = None;
        } else {
            println!("`i` равен `{:?}`. Попробуем ещё раз.", i);
            optional = Some(i + 1);
        }
        // ^ Меньше смещаемся вправо, к тому же
        // нет необходимости обрабатывать ошибки.
    }
    // ^ К `if let` можно добавить дополнительный блок `else`/`else if`
    // `while let` подобного нет.
}
