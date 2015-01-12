struct Book {
    // `String` is a heap allocated string
    title: String,
    author: String,
    year: i32,
}

fn get_title<'a>(book: &'a Book) -> &'a str {
    book.title.as_slice()
}

fn main() {
    let geb = Book {
        // construct a `String` from a reference to a string (`&'static str`)
        // by copying of the data
        author: String::from_str("Douglas Hofstadter"),
        title: String::from_str("Godel, Escher, Bach"),
        year: 1979,
    };

    let title: &str = get_title(&geb);

    println!("I just read {}", title);
}
