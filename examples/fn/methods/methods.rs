struct Point {
    x: f64,
    y: f64,
}

// Блок реализаций, все методы `Point` идут сюда
impl Point {
    // Это статический метод
    // Статические методы не нуждаются в вызове от экземпляра
    // Эти методы, как правило, используются как конструкторы
    fn origin() -> Point {
        Point { x: 0.0, y: 0.0 }
    }

    // Другой статический метод, берет два аргумента
    fn new(x: f64, y: f64) -> Point {
        Point { x: x, y: y }
    }
}

struct Rectangle {
    p1: Point,
    p2: Point,
}

impl Rectangle {
    // Это метод экземпляра
    // `&self` это сахар для `self: &Self`, где `Self` это тип
    // вызываемого объекта. В этом месте `Self` = `Rectangle`
    fn area(&self) -> f64 {
        // `self` даёт допуск к полям структуры через оператор точка
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;

        // `abs` это метод `f64` который возвращает абсолютную величину
        // вызываемого
        ((x1 - x2) * (y1 - y2)).abs()
    }

    fn perimeter(&self) -> f64 {
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;

        2.0 * ((x1 - x2).abs() + (y1 - y2).abs())
    }

    // Этот метод требует чтобы вызываемый объект был изменяемым
    // `&mut self` сахар для `self: &mut Self`
    fn translate(&mut self, x: f64, y: f64) {
        self.p1.x += x;
        self.p2.x += x;

        self.p1.y += y;
        self.p2.y += y;
    }
}

// `Pair` владеет ресурсами: два целых числа в куче
struct Pair(Box<i32>, Box<i32>);

impl Pair {
    // Этот метод "съедает" ресурсы вызываемого объекта
    // `self` сахар для `self: Self`
    fn destroy(self) {
        // деструктуризация `self`
        let Pair(first, second) = self;

        println!("Destroying Pair({}, {})", first, second);

        // `first` и `second` выходят из области видимости и освобождаются
    }
}

fn main() {
    let rectangle = Rectangle {
        // Статические методы вызываются двойными двоеточиями
        p1: Point::origin(),
        p2: Point::new(3.0, 4.0),
    };

    // Метод экземпляра вызывается с помощью оператора точка
    // Обратите внимание, что первый аргумент `&self` неявно пропускается т.е.
    // `rectangle.perimeter()` === `perimeter(&rectangle)`
    println!("Rectangle perimeter: {}", rectangle.perimeter());
    println!("Rectangle area: {}", rectangle.area());

    let mut square = Rectangle {
        p1: Point::origin(),
        p2: Point::new(1.0, 1.0),
    };

    // Ошибка! `rectangle` неизменяемый, но этот метод нуждается в изменяемом
    // объекте
    //rectangle.translate(1.0, 0.0);
    // ЗАДАНИЕ ^ Попробуйте удалить комментарий

    // Хорошо, изменяемый объект может вызывать изменяемые методы
    square.translate(1.0, 1.0);

    let pair = Pair(Box::new(1), Box::new(2));

    pair.destroy();

    // Ошибка! `destroy` вызывает "съеденный" `pair`
    //pair.destroy();
    // ЗАДАНИЕ ^ Попробуйте удалить комментарий
}
