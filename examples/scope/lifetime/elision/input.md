Some lifetime patterns are overwelmingly common and so the borrow checker
will implicitly add them to save typing and to improve readability.
This process of implicit addition is called elision. Elision exists in Rust
solely because these patterns are common.

The following code shows a few examples of elision. For a more comprehensive
description of elision, see [lifetime elision][elision] in the book.

{elision.play}

### Смотрите также:

[elision][elision]

[elision]: http://doc.rust-lang.org/book/lifetimes.html#lifetime-elision
