struct Book {
    title: ~str,
    author: ~str,
    year: uint,
}

fn get_title<'a>(book: &'a Book) -> &'a str {
    book.title.as_slice()
}

fn main() {
    let geb = Book {
        author: "Douglas Hofstadter".to_owned(),
        title: "Gödel, Escher, Bach".to_owned(),
        year: 1979,
    };

    let title: &str = get_title(&geb);

    println!("I just read {}", title);
}
