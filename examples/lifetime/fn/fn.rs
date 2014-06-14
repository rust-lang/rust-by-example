#[deriving(Show)]
struct Triplet {
    one: int,
    two: int,
    three: int,
}

impl Triplet {
    // First attempt: No explicit lifetimes
    // Error! The compiler needs information about the lifetimes
    //fn mut_one(&mut self) -> &mut int {
        //&mut self.one
    //}
    // TODO ^ Try uncommenting this method

    // Second attempt: We explicitly annotate the lifetimes on all the
    // references
    // Error! The compiler doesn't know what is the relationship between the
    // lifetime `structure` and the lifetime `field`
    //fn mut_two<'structure, 'field>(&'structure mut self) -> &'field mut int {
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
    fn mut_three<'s>(&'s mut self) -> &'s mut int {
        &mut self.three
    }
}

fn main() {
    let mut triplet = Triplet { one: 1, two: 2, three: 3 };

    println!("Before: {}", triplet);

    // Use mutable reference to modify the original struct
    *triplet.mut_three() = 0;

    println!("After: {}", triplet);
}
