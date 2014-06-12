The `Drop` trait only has one method: `drop`, and this method is called when
an object goes out of scope. The main use of the `Drop` trait is to free the
resources that the implementor instance owns.

`Box`, `Vec`, `String`, `File` and `Process` are some examples of types that
implement the `Drop` trait to free resources. The `Drop` trait can be
implemented for any custom data type.

{drop.play}
