# Testcase: linked-list

A possible way to implement a linked list of `u32` elements is via enums:

```rust,editable
enum LinkedList {
    // Next: Tuple struct that wraps an element and a boxed link to the next node
    Next(u32, Box<LinkedList>),
    // End: A node that signifies the end of the linked list
    End,
}

// `use` brings the enum variants into scope, so they can be used without the `LinkedList::` prefix
use LinkedList::*;

// Methods can be attached to an enum
impl LinkedList {
    // Create an empty linked list
    fn new() -> LinkedList {
        // `End` has type `LinkedList`
        End
    }

    // Take ownership of a linked list, and return a new one with a new element at its front
    fn prepend(self, elem: u32) -> LinkedList {
        // `Next` also has type `LinkedList`, since it is a variant of the `LinkedList` enum
        Next(elem, Box::new(self))
    }

    // Return the length of the linked list
    fn len(&self) -> u32 {
        // `len` depends on which enum variant we have, so we pattern-match on `self`
        // Since this method only borrows `self`, the `tail` binding is also a borrow
        // Note: In Rust 2018+, match infers the needed references automatically
        match self {
            // Count this node and recursively count the rest of the linked list
            // Note: this is not tail-recursive and could overflow for very long linked lists
            Next(_, tail) => 1 + tail.len(),
            // Base case: the empty linked list has length 0.
            End => 0,
        }
    }
}

// Implement `Display` for `LinkedList` so it can be printed to the console using `println!`
impl std::fmt::Display for LinkedList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Next(head, tail) => write!(f, "({}, {})", head, tail),
            End => write!(f, "End"),
        }
    }
}

fn main() {
    // Create an empty linked list
    let mut list = LinkedList::new();

    // Prepend some elements
    list = list.prepend(1);
    list = list.prepend(2);
    list = list.prepend(3);

    // Show the final state of the list
    println!("linked list has length: {}", list.len());
    println!("{}", list);
}
```

### See also:

[`Box`][box] and [methods][methods]

[box]: ../../std/box.md
[methods]: ../../fn/methods.md
