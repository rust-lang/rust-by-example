# Partial moves

Within the [destructuring] of a single variable, both `by-move` and 
`by-reference` pattern bindings can be used at the same time. Doing 
this will result in a _partial move_ of the variable, which means 
that parts of the variable will be moved while other parts stay. In 
such a case, the parent variable cannot be used afterwards as a 
whole, however the parts that are only referenced (and not moved) 
can still be used.

```rust,editable
fn main() {
    #[derive(Debug)]
    struct Person {
        name: String,
        age: u8,
    }

    let person = Person {
        name: String::from("Alice"),
        age: 20,
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
### See also:
[destructuring][destructuring]

[destructuring]: ../../flow_control/match/destructuring.md
