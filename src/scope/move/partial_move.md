# Partial moves

Pattern bindings can have `by-move` and `by-reference` bindings at
the same time which is used in [destructuring]. Using these pattern
will result in partial move for the variable, which means that part
of the variable is moved while other parts stayed. In this case, the
parent variable cannot be used afterwards as a whole. However, parts
of it that are referenced and not moved can be used.

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