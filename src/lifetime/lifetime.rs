struct Book {
    author: ~str,
    title: ~str,
    year: uint,
}

impl Book {
    // 'a is the lifetime of the caller object
    // this method will borrow the author field for as long as the caller
    // object is alive
    fn author<'a>(&'a self) -> &'a str {
        // as_slice() borrows an owned string (~str) as &str
        self.author.as_slice()
    }

    fn title<'a>(&'a self) -> &'a str {
        self.title.as_slice()
    }
}

fn feed_bookworms(book: Book) {
    // destroys book
}

fn main() {
    // this integer lifetime is the same as main(), let's name it 'main
    let stack_allocated_int = 0;

    // let's call geb lifetime: 'geb
    let geb = Book {
        author: ~"Douglas Hofstadter",
        title: ~"Gödel, Escher, Bach",
        year: 1979,
    };

    // Error: author takes ownership of geb, now geb owns nothing, and
    // title can't take the field title from geb
    //let author = geb.author;
    //let title = geb.title;

    {
        // Let's call this scope 'submain

        // these variables have type: &'geb str
        // but their lifetime is 'submain
        let author: &str = geb.author();
        let title = geb.title();

        // Error: can't destroy the book here, because is borrowed in the
        // 'submain scope
        //feed_bookworms(geb);

        // end of 'submain, author and title get destroyed
    }

    // here the lifetime 'geb is over, note that 'geb < 'main
    feed_bookworms(geb);

    // Error: 'geb is over, can't take any of its fields
    //let year = geb.year;

    // end of 'main
}
