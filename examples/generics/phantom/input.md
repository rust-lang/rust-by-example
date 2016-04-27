A phantom type parameter is one that doesn't show up at runtime,
but is checked statically (and only) at compile time.

Data types can use extra generic type parameters to act as markers
or to perform type checking at compile time. These extra parameters 
hold no storage values, and have no runtime behavior.

In the following example, we combine [std::marker::PhantomData]
with the phantom type parameter concept to create tuples containing
different data types.

{phantom.play}

### See also:

[Derive], [struct], and [TupleStructs]

[Derive]: /trait/derive.html
[struct]: /custom_types/structs.html
[TupleStructs]: /custom_types/structs.html
[std::marker::PhantomData]: https://doc.rust-lang.org/std/marker/struct.PhantomData.html