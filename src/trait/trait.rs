struct Dog {
    name: ~str,
}

struct Sheep {
    naked: bool,
    name: ~str,
}

// traits are collections of methods
trait Animal {
    // static method, Self refers to the implementor type
    fn new(name: ~str) -> Self;

    // instance methods, only signatures
    fn name<'a>(&'a self) -> &'a str;
    fn noise(&self) -> &'static str;

    // trait can provide method definitions
    fn talk(&self) {
        // trait method can access methods available in the same trait
        println!("{} says {}", self.name(), self.noise());
    }
}

// implement the Animal trait for the Dog struct
impl Animal for Dog {
    // replace Self with implementor type (i.e. Dog)
    fn new(name: ~str) -> Dog {
        Dog { name: name }
    }

    fn name<'a>(&'a self) -> &'a str {
        self.name.as_slice()
    }

    fn noise(&self) -> &'static str {
        "woof"
    }
}

impl Dog {
    fn wag_tail(&self) {
        // struct methods can access trait methods
        println!("{} wags tail", self.name());
    }
}

impl Animal for Sheep {
    fn new(name: ~str) -> Sheep {
        Sheep { name: name, naked: false }
    }

    fn name<'a>(&'a self) -> &'a str {
        self.name.as_slice()
    }

    fn noise(&self) -> &'static str {
        // implemented traits methods can access struct methods
        if self.is_naked() {
            "baaah!"
        } else {
            "baaaaaaaaaaaah!"
        }
    }
}

impl Sheep {
    fn is_naked(&self) -> bool {
        self.naked
    }

    fn shear(&mut self) {
        if self.is_naked() {
            println!("{} is already naked!", self.name());
        } else {
            println!("{} gets a haircut", self.name());
            self.talk();
            self.naked = true;
        }
    }
}

fn main() {
    let mut dolly: Sheep = Animal::new(~"Dolly");
    let spike: Dog = Animal::new(~"Spike");

    spike.wag_tail();
    dolly.shear();

    // dolly and spike can be borrowed as &Animal
    let animals: [&Animal, ..2] = [&spike as &Animal, &dolly as &Animal];

    // Ok: Animal methods can be called on the array members
    animals[0].talk();
    animals[1].talk();

    // Error: Dog and Sheep methods can't be accessed via the Animal trait
    //animals[0].wag_tail();
    //animals[1].shear();
}
