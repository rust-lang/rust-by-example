Some lifetime patterns are overwelmingly common and so they may be elided
(dropped) and the borrow checker will implicitly add them. Elision exists
solely because these patterns are common; saving typing and easing legibility.

This section is brief and not comprehensive. See [lifetime elision][elision]
in the book for a more comprehensive treatment.

{elision.play}

### See also:

[elision][elision]

[elision]: http://doc.rust-lang.org/book/lifetimes.html#lifetime-elision
