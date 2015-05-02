#[allow(dead_code)]
#[derive(Clone, Copy)]
struct Book {
    // `&'static str` is a reference to a string allocated in read only memory
    author: &'static str,
    title: &'static str,
    year: u32,
}

// This function takes a reference to a book
fn borrow_book(book: &Book) {
    println!("I borrowed {} {} edition", book.title, book.year);
}

// This function takes a reference to a mutable book
fn new_edition(book: &mut Book) {
    // the fields of the book can be modified
    book.year = 2014;
}

fn main() {
    // An immutable Book
    let geb = Book {
        // string literals have type `&'static str`
        author: "Douglas Hofstadter",
        title: "Gödel, Escher, Bach",
        year: 1979,
    };

    // Immutably borrow `geb`
    borrow_book(&geb);

    // Error! Can't borrow an immutable object as mutable
    new_edition(&mut geb);
    // FIXME ^ Comment out this line

    // `mutable_geb` is a mutable copy of `geb`
    let mut mutable_geb = geb;

    // Borrow a mutable object as mutable
    new_edition(&mut mutable_geb);

    // Mutable objects can be immutably borrowed
    borrow_book(&mutable_geb);
}
