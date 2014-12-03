#[deriving(Show)]
struct Vec2<T> {
    x: T,
    y: T,
}

impl<
    // Bound: `T` must implement the `Add` trait
    T: Add<T, T>
> Add<Vec2<T>, Vec2<T>>
for Vec2<T> {
    fn add(&self, rhs: &Vec2<T>) -> Vec2<T> {
        Vec2 {
            // `x` and `y` are of type `T`, and implement the `add` method
            x: self.x.add(&rhs.x),
            // The sugary `+` operator can also be used
            y: self.y + rhs.y,
        }
    }
}

impl<
    // Bound: `T` must implement the `Sub` trait
    T: Sub<T, T>
> Sub<Vec2<T>, Vec2<T>>
for Vec2<T> {
    fn sub(&self, rhs: &Vec2<T>) -> Vec2<T> {
        Vec2 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl<
    // Bound: `T` must implement *both* the `Add` trait and the `Mul` trait
    T: Add<T, T> + Mul<T, T>
> Mul<Vec2<T>, T>
for Vec2<T> {
    fn mul(&self, rhs: &Vec2<T>) -> T {
        (self.x * rhs.x) + (self.y * rhs.y)
    }
}

fn main() {
    // Floats implement the `Add`, `Mul` and `Sub` traits
    let v1 = Vec2 { x: 1.2_f32, y: 3.4 };
    let v2 = Vec2 { x: 5.6_f32, y: 7.8 };

    println!("{} + {} = {}", v1, v2, v1 + v2);
    println!("{} - {} = {}", v1, v2, v1 - v2);
    println!("{} . {} = {}", v1, v2, v1.dot(&v2));

    // Error! `char` doesn't implement the `Add` trait
    println!("{}", Vec2 { x: ' ', y: 'b' } + Vec2 { x: 'c', y: 'd' });
    // FIXME ^ Comment out this line
}
