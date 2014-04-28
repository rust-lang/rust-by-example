struct Book {
    author: ~str,
    title: ~str,
    year: uint,
}

struct Pair(int, ~int);

fn main() {
    // Alice owns this book
    let alices_book = Book {
        author: ~"Douglas Hofstadter",
        title: ~"Gödel, Escher, Bach",
        year: 1979,
    };

    // Later, Alice gives her book to Bob (no data is copied)
    let bobs_book = alices_book;

    // Error, Alice no longer owns the book
    //println!("Alice owns {}", alices_book.title);

    // Valid, Bob owns the book
    println!("Bob owns {}", bobs_book.title);

    let pair = Pair(1, ~2);

    // The following two statements *cannot* be swapped because of the move
    // semantics
    // Copy the int value into x
    let Pair(x, _) = pair;

    // Move the boxed int into y, pair can't be used afterwards
    let Pair(_, y) = pair;
}
