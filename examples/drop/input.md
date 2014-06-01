The `Drop` trait only has one method: `drop`, and this method is called when
an object goes out of scope. One use of the `Drop` trait is to free
resources that an object own.

`Box`, `File` and `Process` are some examples of types that implement the
`Drop` trait to free resources. The `Drop` trait can also be implemented by
custom data types.

{drop.rs}

```
$ rustc drop.rs && ./drop
compiling hello.rs...
successfully compiled hello.rs
output was:
Hello World!
end of the match block
deleted hello
end of the if block
hello no longer exists
```

Where:

{hello.rs}
