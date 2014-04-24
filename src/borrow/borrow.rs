struct Book {
    author: ~str,
    title: ~str,
    year: uint,
}

// & means a reference to a Book, no ownership is taken
fn borrow_book(book: &Book) {
    println!("I borrowed {} {} edition", book.title, book.year);
}

// &mut means a mutable reference, meaning the fields of the struct can be
// modified
fn new_edition(book: &mut Book) {
    book.year = 2014;
}

fn main() {
    // an immutable Book
    let geb = Book {
        author: ~"Douglas Hofstadter",
        title: ~"Gödel, Escher, Bach",
        year: 1979,
    };

    // borrow geb, geb can still be used afterwards
    borrow_book(&geb);

    // geb can be borrowed again, and again, and again ...
    borrow_book(&geb);

    // a mutable Book
    let mut mutable_geb = Book {
        author: ~"Douglas Hofstadter",
        title: ~"Gödel, Escher, Bach",
        year: 1979,
    };

    // Error: can't borrow immutable object as mutable
    //new_edition(&mut geb);

    // Ok: borrow a mutable object as mutable
    new_edition(&mut mutable_geb);

    // Ok: mutable objects can be immutably borrowed
    borrow_book(&mutable_geb);

    // ref can be used to take references when destructuring
    // Here geb_title and geb_author are references to ~str, i.e. &~str
    // geb_year is a copy of the year field
    let Book {
        author: ref geb_author,
        title: ref geb_title,
        year: geb_year
    } = geb;
}
