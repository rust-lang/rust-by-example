Some conditionals like `target_os` are implicitly provided by `rustc`, but
custom conditionals must be passed to `rustc` using the `--cfg` flag.

{custom.rs}

Without the custom `cfg` flag:

{custom.out}

With the custom `cfg` flag:

```
$ rustc --cfg some_condition custom.rs && ./custom
condition met!
```
