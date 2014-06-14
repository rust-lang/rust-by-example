// A linked list node, which can take on any of these two variants
enum Node {
    // Cons: Tuple struct that wraps an element and a pointer to the next node
    // element, [ ] -> next_node
    Cons(uint, Box<Node>),
    // Nil: A node that signifies the end of the linked list
    Nil,
}

// Methods can be attached to an enum
impl Node {
    // Create an empty list
    fn new() -> Node {
        // `Nil` has type `Node`
        Nil
    }

    // Consume a list, and return the same list with a new element at its front
    fn append(self, elem: uint) -> Node {
        // `Cons` also has type Node
        Cons(elem, box self)
    }

    // Return the length of the list
    fn len(&self) -> uint {
        // `self` has to be matched, because the behavior of this method
        // depends on the variant of `self`
        // `self` has type `&Node`, and `*self` has type `Node`, matching on a
        // concrete type `T` is preferred over a match on a reference `&T`
        match *self {
            // Can't take ownership of the tail, because `self` is borrowed;
            // instead take a reference to the tail
            Cons(_, ref tail) => 1 + tail.len(),
            // An empty list has zero length
            Nil => 0
        }
    }

    // Return representation of the list as a (heap allocated) string
    fn stringify(&self) -> String {
        match *self {
            Cons(head, ref tail) => {
                // `format!` is similar to `print!`, but returns a heap
                // allocated string instead of printing to the console
                format!("{}, [ ] -> {}", head, tail.stringify())
            },
            Nil => {
                format!("Nil")
            },
        }
    }
}

fn main() {
    // Create an empty linked list
    let mut list = Node::new();

    // Append some elements
    list = list.append(1);
    list = list.append(2);
    list = list.append(3);

    // Show the final state of the list
    println!("linked list has length: {}", list.len());
    println!("{}", list.stringify());
}
