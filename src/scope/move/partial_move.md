# Partial moves

Within the [destructuring] of a single variable, both `by-move` and
`by-reference` pattern bindings can be used at the same time. Doing this will
result in a *partial move* of the variable, which means that parts of the
variable will be moved while other parts stay. In such a case, the parent
variable cannot be used afterwards as a whole, however the parts that are only
referenced (and not moved) can still be used.

```rust,editable
fn main() {
    #[derive(Debug)]
    struct Person {
        name: String,
        age: Box<u8>,
    }

    let person = Person {
        name: String::from("Alice"),
        age: Box::new(20),
    };

    // `name` is moved out of person, but `age` is referenced
    let Person { name, ref age } = person;

    println!("The person's age is {}", age);

    println!("The person's name is {}", name);

    // Error! borrow of partially moved value: `person` partial move occurs
    //println!("The person struct is {:?}", person);

    // `person` cannot be used but `person.age` can be used as it is not moved
    println!("The person's age from person struct is {}", person.age);
}
```

(In this example, we store the `age` variable on the heap to illustrate the
partial move: deleting `ref` in the above code would give an error as the
ownership of `person.age` would be moved to the variable `age`. If `Person.age`
were stored on the stack, `ref` would not be required as the definition of `age`
would copy the data from `person.age` without moving it.)

### See also:

[destructuring][destructuring]

[destructuring]: ../../flow_control/match/destructuring.md
