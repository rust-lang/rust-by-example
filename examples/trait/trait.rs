trait Animal {
    // Static method signature; `Self` refers to the implementor type
    fn new(name: &'static str) -> Self;

    // Instance methods, only signatures
    fn name(&self) -> &'static str;
    fn noise(&self) -> &'static str;

    // A trait can provide default method definitions
    fn talk(&self) {
        // These definitions can access other methods declared in the same
        // trait
        println!("{} says {}", self.name(), self.noise());
    }
}

struct Dog { name: &'static str }

impl Dog {
    fn wag_tail(&self) {
        println!("{} wags tail", self.name);
    }
}

// Implement the `Animal` trait for `Dog`
impl Animal for Dog {
    // Replace `Self` with the implementor type: `Dog`
    fn new(name: &'static str) -> Dog {
        Dog { name: name }
    }

    fn name(&self) -> &'static str {
        self.name
    }

    fn noise(&self) -> &'static str {
        "woof!"
    }

    // Default trait methods can be overridden
    fn talk(&self) {
        // Traits methods can access the implementor methods
        self.wag_tail();

        println!("{} says {}", self.name, self.noise());
    }
}

struct Sheep { naked: bool, name: &'static str }

impl Sheep {
    fn is_naked(&self) -> bool {
        self.naked
    }

    fn shear(&mut self) {
        if self.is_naked() {
            // Implementor methods can use the implementor's trait methods
            println!("{} is already naked!", self.name());
        } else {
            println!("{} gets a haircut", self.name);

            self.talk();
            self.naked = true;
        }
    }
}

impl Animal for Sheep {
    fn new(name: &'static str) -> Sheep {
        Sheep { name: name, naked: false }
    }

    fn name(&self) -> &'static str {
        self.name
    }

    fn noise(&self) -> &'static str {
        if self.is_naked() {
            "baaah"
        } else {
            "baaaaaaaaaaaah"
        }
    }
}

fn main() {
    // Type annotation is necessary in this case
    let mut dolly: Sheep = Animal::new("Dolly");
    let spike: Dog = Animal::new("Spike");
    // TODO ^ Try removing the type annotations

    dolly.shear();

    spike.talk();
    dolly.talk();
}
