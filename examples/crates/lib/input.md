Let's create a library, and then see how to link it to another crate.

{erty.rs}

```
$ rustc --crate-type=lib erty.rs
$ ls lib*
liberty.rlib
```

Libraries get prefixed with "lib", and by default they get named after their
crate file, but this default name can be overridden using the
[`crate_name` attribute][crate-name].

[crate-name]: /attribute/crate.html
