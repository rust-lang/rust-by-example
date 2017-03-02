use std::fmt; // Импортируем `fmt`

// Структура, которая хранит в себе два числа.
//Вывод типажа `Debug` добавлен для сравнения с `Display`.
#[derive(Debug)]
struct MinMax(i64, i64);

// Реализуем `Display` для `MinMax`.
impl fmt::Display for MinMax {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Используем `self.number`, чтобы получить доступ к каждому полю структуры.
        write!(f, "({}, {})", self.0, self.1)
    }
}

// Определим структуру с именованными полями, для сравнения
#[derive(Debug)]
struct Point2D {
    x: f64,
    y: f64,
}

// По аналогии, реализуем `Display` для Point2
impl fmt::Display for Point2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Обращаться к полям структуры Point2 будет по имени
        write!(f, "x: {}, y: {}", self.x, self.y)
    }
}

fn main() {
    let minmax = MinMax(0, 14);

    println!("Compare structures:");
    println!("Display: {}", minmax);
    println!("Debug: {:?}", minmax);

    let big_range =   MinMax(-300, 300);
    let small_range = MinMax(-3, 3);

    println!("The big range is {big} and the small is {small}",
             small = small_range,
             big = big_range);

    let point = Point2D { x: 3.3, y: 7.2 };

    println!("Compare points:");
    println!("Display: {}", point);
    println!("Debug: {:?}", point);

    // Ошибка. Типажи `Debug` и `Display` были реализованны, но `{:b}`
    // необходима реализация `fmt::Binary`. Следующий код не сработает.
    // println!("What does Point2D look like in binary: {:b}?", point);
}
