#[deriving(Show)]
struct Vec2<T> {
    x: T,
    y: T,
}

// The Add<T, U> trait overloads the + operator: (Self + T = U)
impl<
    // bound: T implements the Add trait
    T: Add<T, T>
> Add<Vec2<T>, Vec2<T>>
for Vec2<T> {
    // add() is a required method of the Add trait
    fn add(&self, rhs: &Vec2<T>) -> Vec2<T> {
        Vec2 {
            // x and y are of type T, and implement the add() method
            x: self.x.add(&rhs.x),
            y: self.x.add(&rhs.y),
        }
    }
}

// The Sub<T, U> trait overloads the - operator: (Self - T = U)
impl<
    // bound: T implements the Sub trait
    T: Sub<T, T>
> Sub<Vec2<T>, Vec2<T>>
for Vec2<T> {
    // sub() is a required method of the Sub trait
    fn sub(&self, rhs: &Vec2<T>) -> Vec2<T> {
        Vec2 {
            // x and y are of type T, and implement the sub() method
            x: self.x.sub(&rhs.x),
            y: self.x.sub(&rhs.y),
        }
    }
}

impl<
    // bound: T implements the Add trait and the Mul trait
    T: Add<T, T> + Mul<T, T>
> Vec2<T> {
    fn dot(&self, rhs: &Vec2<T>) -> T {
        self.x.mul(&rhs.x).add(&self.y.mul(&rhs.y))
    }
}

fn main() {
    let v1 = Vec2 { x: 1.0, y: 2.0 };
    let v2 = Vec2 { x: 2.0, y: 1.0 };

    println!("{} + {} = {}", v1, v2, v1 + v2);
    println!("{} - {} = {}", v1, v2, v1 - v2);
    println!("{} . {} = {}", v1, v2, v1.dot(&v2));
}
