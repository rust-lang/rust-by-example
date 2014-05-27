struct Point {
    x: f64,
    y: f64,
}

// implementation block, all Point methods go in here
impl Point {
    // this is an static method
    // static methods don't need to be called by an instance
    // these methods are generally used for constructors
    fn origin() -> Point {
        Point { x: 0.0, y: 0.0 }
    }

    // another static method, that takes two arguments
    fn new(x: f64, y: f64) -> Point {
        Point { x: x, y: y }
    }
}

struct Rectangle {
    p1: Point,
    p2: Point,
}

impl Rectangle {
    // instance method, `&self` is sugar for `self: &Self`
    // where Self is the type of the caller object
    // in this case Self = Rectangle
    fn area(&self) -> f64 {
        // `self` gives access to the struct fields via the dot operator
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;

        // abs() is a method of f64 types
        ((x1 - x2) * (y1 - y2)).abs()
    }

    fn perimeter(&self) -> f64 {
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;

        2.0 * (x1 - x2).abs() + 2.0 * (y1 - y2).abs()
    }

    // this method requires the caller object to be mutable
    // `&mut self` desugars to `self: &mut Self`
    fn move(&mut self, x: f64, y: f64) {
        self.p1.x += x;
        self.p2.x += x;

        self.p1.y += y;
        self.p2.y += y;
    }
}

// Bomb owns resources: one heap allocated string
struct Bomb {
    name: String,
}

impl Bomb {
    // this method consumes the caller object
    // `self` desugars to `self: Self`
    fn boom(self) {
        println!("{} goes boom!", self.name);
        // self goes out of scope and it's destroyed
    }
}

fn main() {
    let rectangle = Rectangle {
        // static methods are called using double colons
        p1: Point::origin(),
        p2: Point::new(3.0, 4.0),
    };

    // instance method are called using the dot operator
    // note that the first argument `&self` is implicitly passed
    println!("Rectangle perimeter: {}", rectangle.perimeter());
    println!("Rectangle area: {}", rectangle.area());

    let mut square = Rectangle {
        p1: Point::origin(),
        p2: Point::new(1.0, 1.0),
    };

    // Error: object is immutable, but method requires a mutable object
    //rectangle.move(1.0, 0.0);

    // Ok: mutable object can call mutable methods
    square.move(1.0, 1.0);

    let bomb = Bomb { name: String::from_str("C4") };

    bomb.boom();

    // Error: previous boom() call destroyed the bomb
    //bomb.boom();
}
