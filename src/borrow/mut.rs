struct Book {
    // ~str is a heap allocated string
    author: ~str,
    title: ~str,
    year: uint,
}

// this function takes a reference to a book
fn borrow_book(book: &Book) {
    println!("I borrowed {} {} edition", book.title, book.year);
}

// this function, takes a reference to a mutable book
fn new_edition(book: &mut Book) {
    // the fields of the book can be modified
    book.year = 2014;
}

fn main() {
    // an immutable Book
    let geb = Book {
        // to_owned() will place the string in the heap
        author: "Douglas Hofstadter".to_owned(),
        title: "Gödel, Escher, Bach".to_owned(),
        year: 1979,
    };

    // borrow geb, geb can still be used afterwards
    borrow_book(&geb);

    // geb can be borrowed again, and again, and again ...
    borrow_book(&geb);

    // Error: can't borrow immutable object as mutable
    //new_edition(&mut geb);

    // pass ownership of geb to mutable_geb, changing mutability
    let mut mutable_geb = geb;

    // Ok: borrow a mutable object as mutable
    new_edition(&mut mutable_geb);

    // Ok: mutable objects can be immutably borrowed
    borrow_book(&mutable_geb);
}
