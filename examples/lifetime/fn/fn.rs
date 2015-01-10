#[derive(Show)]
struct Triplet {
    one: i32,
    two: i32,
    three: i32,
}

impl Triplet {
    // First attempt: No explicit lifetimes
    // The compiler infers that the field and the struct have the same lifetime
    fn mut_one(&mut self) -> &mut i32 {
        &mut self.one
    }

    // Second attempt: We explicitly annotate the lifetimes on all the
    // references
    // Error! The compiler doesn't know what is the relationship between the
    // lifetime `structure` and the lifetime `field`
    //fn mut_two<'structure, 'field>(&'structure mut self) -> &'field mut i32 {
        //&mut self.two
    //}
    // TODO ^ Try uncommenting this method

    // Third attempt: We think! What is the relationship between the lifetimes?
    // Clearly `'field` *can't* outlive `'structure`, because the field will be
    // destroyed when the struct gets destroyed
    // If the fields get destroyed along with the struct, then that means that
    // both the struct and its field have the same lifetime!
    // Ok, so we need to tell the compiler that `'structure` = `'field`
    // We can use a shorter name for the lifetime, it's common to use a single
    // letter lifetime, let's use `'s`, because it's the first letter of
    // structure
    fn mut_three<'s>(&'s mut self) -> &'s mut i32 {
        &mut self.three
    }
}

fn main() {
    let mut triplet = Triplet { one: 1, two: 2, three: 3 };

    println!("Before: {:?}", triplet);

    *triplet.mut_one() = 0;
    println!("After: {:?}", triplet);

    // Use mutable reference to modify the original struct
    *triplet.mut_three() = 0;

    println!("After: {:?}", triplet);
}
