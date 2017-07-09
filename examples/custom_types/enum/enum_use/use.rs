// Атрибут, который убирает предупреждения компилятора
// о неиспользуемом коде
#![allow(dead_code)]

enum Status {
    Rich,
    Poor,
}

enum Work {
    Civilian,
    Soldier,
}

fn main() {
    // Используем `use` для каждого из вариантов, чтобы они были доступны
    // без указания области видимости.
    use Status::{Poor, Rich};
    // Автоматически используем `use` для каждого из вариантов в `Work`.
    use Work::*;

    // Эквивалентно `Status::Poor`.
    let status = Poor;
    // Эквивалентно to `Work::Civilian`.
    let work = Civilian;

    match status {
        // Обратите внимание, как используются варианты из перечисления `Status`
        // все благодаря `use`
        Rich => println!("У богатого куча денег!"),
        Poor => println!("У бедняка денег нет, но он держится..."),
    }

    match work {
        // И снова используем варианты напрямую.
        Civilian => println!("Гражданин работает!"),
        Soldier  => println!("Солдаты служат!"),
    }
}
