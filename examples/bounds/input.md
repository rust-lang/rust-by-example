When working with generics, the type parameters (e.g. `Ty`) may use traits
(e.g. `Tr`) as *bounds* (e.g. `Ty: Tr`, which reads as: `Ty` must implement the
`Tr` trait). Bounding has two effects:

* Generics instances (`let ty: Ty = (...)`) can now access the methods
  (`ty.tr()`) of the traits specified in the bounds.
* The generic can only be specialized for type parameters that conform to the
  bounds.

{bounds.play}
