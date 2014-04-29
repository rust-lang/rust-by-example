#[deriving(Show)]
struct Vec2<T> {
    x: T,
    y: T,
}

impl<
    // bound: T implements the Add trait
    T: Add<T, T>
> Add<Vec2<T>, Vec2<T>>
for Vec2<T> {
    fn add(&self, rhs: &Vec2<T>) -> Vec2<T> {
        Vec2 {
            // x and y are of type T, and implement the add() method
            x: self.x.add(&rhs.x),
            y: self.y.add(&rhs.y),
        }
    }
}

impl<
    // bound: T implements the Sub trait
    T: Sub<T, T>
> Sub<Vec2<T>, Vec2<T>>
for Vec2<T> {
    fn sub(&self, rhs: &Vec2<T>) -> Vec2<T> {
        Vec2 {
            // x and y are of type T, and implement the sub() method
            x: self.x.sub(&rhs.x),
            y: self.y.sub(&rhs.y),
        }
    }
}

impl<
    // bound: T implements *both* the Add trait and the Mul trait
    T: Add<T, T> + Mul<T, T>
> Vec2<T> {
    fn dot(&self, rhs: &Vec2<T>) -> T {
        // the sugary versions of mul() and add() can be used as well
        (self.x * rhs.x) + (self.y * rhs.y)
    }
}

fn main() {
    let v1 = Vec2 { x: 1.0, y: 2.0 };
    let v2 = Vec2 { x: 2.0, y: 1.0 };

    println!("{} + {} = {}", v1, v2, v1 + v2);
    println!("{} - {} = {}", v1, v2, v1 - v2);
    println!("{} . {} = {}", v1, v2, v1.dot(&v2));
}
