Numeric literals can be type annotated by adding the type as a suffix, with the
exception of `uint` that uses the `u` suffix and `int` that uses the `i`
suffix. The type of unsuffixed literals will depend on how they are used, if no
constraint exists they will default to a signed pointer size type.

{literals.play}
