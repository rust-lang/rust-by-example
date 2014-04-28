// linked list node
enum Node {
    // data (uint) -> next_node (~Node)
    Cons(uint, ~Node),
    // terminal node
    Nil,
}

impl Node {
    // return the head of the list
    fn head(&self) -> Option<uint> {
        match self {
            &Cons(head, _) => Some(head),
            &Nil => None,
        }
    }

    fn len(&self) -> uint {
        match self {
            // can't move tail out of the list, instead take a reference
            &Cons(_, ref tail) => 1 + tail.len(),
            &Nil => 0
        }
    }

    // this method consumes the list and returns the tail
    fn tail(self) -> Option<Node> {
        match self {
            Nil => None,
            // unbox the tail
            Cons(_, tail) => Some(*tail),
        }
    }
}

fn main() {
    // linked list: 1 -> 2 -> 3 -> Nil
    let mut list = Cons(1, ~Cons(2, ~Cons(3, ~Nil)));
    println!("list size: {}", list.len());

    // continuously behead list until it's empty
    loop {
        let head = list.head();

        list = match list.tail() {
            None => break,
            Some(tail) => tail,
        };

        println!("list head: {}", head.unwrap());
    }
}
