// Вывод и реализация `fmt::Debug` для `Structure`. `Structure`
// `Structure` - это структура, которая содержит в себе один `i32`.
#[derive(Debug)]
struct Structure(i32);

// Добавим структуру `Structure` в структуру `Deep`.
// Релазиуем для `Deep` возможность вывода с помощью fmt::Debug`.
#[derive(Debug)]
struct Deep(Structure);

fn main() {
    // Printing with `{:?}` is similar to with `{}`.
    // Вывод с помощью `{:?}` аналогичен `{}`.
    println!("{:?} months in a year.", 12);
    println!("{1:?} {0:?} is the {actor:?} name.",
             "Slater",
             "Christian",
             actor="actor's");

    // `Structure` теперь можно напечатать!
    println!("Now {:?} will print!", Structure(3));

    // Проблема с `выводом (derive)`, в том, что у нас не будет контроля
    // над тем, как будет выглядить результат.
    // Что если мы хотим напечатать просто `7`?
    println!("Now {:?} will print!", Deep(Structure(7)));
}
