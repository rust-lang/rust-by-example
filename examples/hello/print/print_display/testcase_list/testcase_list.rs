use std::fmt; // Импортируем модуль `fmt`.

// Определим структуру с именем `List`, которая хранит в себе `Vec`.
struct List(Vec<i32>);

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Получаем значение с помощью индекса кортежа
        // и создаём ссылку на `vec`.
        let vec = &self.0;

        write!(f, "[")?;

        // Пройдёмся по каждому `v` в `vec`.
        // Номер итерации хранится в `count`.
        for (count, v) in vec.iter().enumerate() {
            // Для каждого элемента, кроме первого, добавим запятую
            // до вызова `write!`. Используем оператор `?` или `try!`,
            // чтобы вернуться при наличие ошибок.
            if count != 0 { write!(f, ", ")?; }
            write!(f, "{}", v)?;
        }

        // Закроем открытую скобку и вернём значение `fmt::Result`
        write!(f, "]")
    }
}

fn main() {
    let v = List(vec![1, 2, 3]);
    println!("{}", v);
}
