// Единичная структура
struct Nil;

// Кортежная структура
struct Pair(i32, f64);

// Структура с двумя полями
struct Point {
    x: f64,
    y: f64,
}

// Структуры могут быть использованы как поля другой структуры
#[allow(dead_code)]
struct Rectangle {
    p1: Point,
    p2: Point,
}

fn main() {
    // Создаём связь со структурой `Point`
    let point: Point = Point { x: 0.3, y: 0.4 };

    // Получаем доступ к полям структуры `point`
    println!("координаты точки: ({}, {})", point.x, point.y);

    // Деструктурируем `point` создавая связь с помощью `let`
    let Point { x: my_x, y: my_y } = point;

    let _rectangle = Rectangle {
        // инициализация структуры так же является выражением
        p1: Point { x: my_y, y: my_x },
        p2: point,
    };

    // Создаём связь с единичной структурой
    let _nil = Nil;

    // Создаём связь с кортежной структурой
    let pair = Pair(1, 0.1);

    // Деструктурируем кортежную структуру
    let Pair(integer, decimal) = pair;

    println!("Pair хранит в себе {:?} и {:?}", integer, decimal);
}
