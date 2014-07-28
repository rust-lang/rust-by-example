Let's create a library, and then see how to link it to another crate.

{erty.rs}

```
$ rustc --crate-type=lib erty.rs
$ ls lib*
liberty.rlib
```

Libraries get prefixed with "lib".
