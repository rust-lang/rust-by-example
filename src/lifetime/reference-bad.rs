struct Book {
    title: ~str,
    author: ~str,
    year: uint,
}

fn get_title<'a, 'b>(book: &'a Book) -> &'b str {
    // as_slice() will return a reference to the boxed string
    book.title.as_slice()
}

fn main() {

}
