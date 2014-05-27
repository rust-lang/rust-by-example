struct Book {
    // `String` is a heap allocated string
    title: String,
    author: String,
    year: uint,
}

fn get_title<'a, 'b>(book: &'a Book) -> &'b str {
    // as_slice() will return a reference to the string (&str)
    book.title.as_slice()
}

fn main() {

}
