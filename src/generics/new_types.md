# New Type Idiom

The `newtype` idiom gives compile time guarantees that the right type of value is supplied
to a program.

For example, a function that measures distance in miles, *must* be given
a value of type `Miles`.

```rust, editable
struct Miles(f64);

struct Kilometers(f64);

impl Miles {
    pub fn to_kilometers(&self) -> Kilometers {
        Kilometers(self.0 * 1.609344)
    }
}

impl Kilometers {
    pub fn to_miles(&self) -> Miles {
        Miles(self.0 / 1.609344)
    }
}

fn is_a_marathon(distance: &Miles) -> bool {
    distance.0 >= 26.2
}

fn main() {
    let distance = Miles(30.0);
    let distance_km = distance.to_kilometers();
    println!("Is a marathon? {}", is_a_marathon(&distance));
    println!("Is a marathon? {}", is_a_marathon(&distance_km.to_miles()));
    // println!("Is a marathon? {}", is_a_marathon(&distance_km));
}
```

Uncomment the last print statement to observe that the type supplied must be `Miles`.

To obtain the `newtype`'s value as the base type, you may use the tuple or destructuring syntax like so:

```rust, editable
struct Miles(f64);

fn main() {
    let distance = Miles(42.0);
    let distance_as_primitive_1: f64 = distance.0; // Tuple
    let Miles(distance_as_primitive_2) = distance; // Destructuring
}
```

### See also:

[`structs`][struct]

[struct]: ../custom_types/structs.md
