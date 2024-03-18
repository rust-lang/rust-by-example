# HashMap

Where vectors store values by an integer index, `HashMap`s store values by key.
`HashMap` keys can be booleans, integers, strings, or any other type that
implements the `Eq` and `Hash` traits. More on this in the next section.

Like vectors, `HashMap`s are growable, but HashMaps can also shrink themselves
when they have excess space. You can create a HashMap with a certain starting
capacity using `HashMap::with_capacity(uint)`, or use `HashMap::new()` to get a
HashMap with a default initial capacity (recommended).

```rust,editable
use std::collections::HashMap;

fn call(number: &str) -> &str {
    match number {
        "798-1364" => {
            "We're sorry, the call cannot be completed as dialed. 
            Please hang up and try again."
        }
        "645-7689" => {
            "Hello, this is Mr. Awesome's Pizza. My name is Fred.
            What can I get for you today?"
        }
        _ => "Hi! Who is this again?",
    }
}

fn main() {
    let mut contacts = HashMap::new();

    contacts.insert("Daniel", "798-1364");
    contacts.insert("Ashley", "645-7689");
    contacts.insert("Katie", "435-8291");
    contacts.insert("Robert", "956-1745");

    // Takes a reference and returns Option<&V>
    match contacts.get(&"Daniel") {
        Some(&number) => println!("Calling Daniel: {}", call(number)),
        _ => println!("Don't have Daniel's number."),
    }

    // `HashMap::insert()` returns `None`
    // if the inserted value is new, `Some(value)` otherwise
    contacts.insert("Daniel", "164-6743");

    match contacts.get(&"Ashley") {
        Some(&number) => println!("Calling Ashley: {}", call(number)),
        _ => println!("Don't have Ashley's number."),
    }

    contacts.remove(&"Ashley");

    // `HashMap::iter()` returns an iterator that yields
    // (&'a key, &'a value) pairs in arbitrary order.
    for (contact, &number) in contacts.iter() {
        println!("Calling {}: {}", contact, call(number));
    }
}
```

For more information on how hashing and hash maps (sometimes called hash tables)
work, have a look at [Hash Table Wikipedia][wiki-hash]

[wiki-hash]: https://en.wikipedia.org/wiki/Hash_table
