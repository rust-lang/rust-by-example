There are three types of structures ("structs") that can be created using the
`struct` keyword:

* Tuple structs, which are, basically, named tuples.
* The classic [C structs][c_struct]
* Unit structs, which are field-less, are useful for generics.

{structs.play}

### Activity

1. Add a function `rect_area` which calculates the area of a rectangle (try 
   using nested destructuring). 
2. Add a function `square` which takes a `Point` and a `f32` as arguments, and returns a `Rectangle` with its lower left corner on the point, and a width and height corresponding to the `f32`.

### See also:

[`attributes`][attributes] and [destructuring][destructuring]

[attributes]: /attribute.html
[c_struct]: http://en.wikipedia.org/wiki/Struct_(C_programming_language)
[destructuring]: /flow_control/match/destructuring.html
