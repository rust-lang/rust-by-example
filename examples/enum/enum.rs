use std::owned::Box;

// linked list node, can take on any of these two values
enum Node {
    // data, [ ] -> next_node
    Cons(uint, Box<Node>),
    // terminal node
    Nil,
}

impl Node {
    // create an empty list
    fn new() -> Node {
        Nil
    }

    // consume list, and return the same list with a new element appended
    fn append(self, elem: uint) -> Node {
        Cons(elem, box self)
    }

    // return the head of the list
    fn head(&self) -> Option<uint> {
        match *self {
            // head gets copied, copy is returned wrapped in Some
            Cons(head, _) => Some(head),
            // if the list is empty, return None
            Nil => None,
        }
    }

    // return the length of the list
    fn len(&self) -> uint {
        match *self {
            // can't take ownership of the tail, because self is borrowed
            // instead take a reference to the tail
            Cons(_, ref tail) => 1 + tail.len(),
            // empty list has zero length
            Nil => 0
        }
    }

    // this method consumes the list and returns the tail
    fn tail(self) -> Option<Node> {
        match self {
            // if the list is empty, return None
            Nil => None,
            // unbox the tail, return it wrapped in Some
            Cons(_, box tail) => Some(tail),
        }
    }
}

fn main() {
    // linked list: 3, [ ] -> 2, [ ] -> 1, [ ] -> Nil
    let mut list = Node::new();
    list = list.append(1);
    list = list.append(2);
    list = list.append(3);

    println!("list size: {}", list.len());

    // continuously behead list until it's empty
    loop {
        // look at the list head
        let head = list.head();

        list = match list.tail() {
            // if list is empty, break this loop
            None => break,
            // unwrap tail
            Some(tail) => {
                // show the list head
                println!("list head: {}", head.unwrap());

                // tail is the new list
                tail
            },
        };

    }
}
