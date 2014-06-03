Rust provides a powerful module system that can be used to hierarchically split
code in logical units (modules), and manage visibility (public/private)
between them.

A module is collection of items like: functions, structs, traits, impl blocks,
and even other modules.

{nested.rs}

{nested.play}

{nested.out}

By default, the items in a module have private visibility, but this can be
overridden using the `pub` modifier. Only the public items of a module can be
accessed from outside the module scope.

{visibility.rs}

{visibility.play}

{visibility.out}

The `use` declaration can be used to bind a full path to a new name, for easier
access.

{use.rs}

{use.play}

{use.out}

The `super` and `self` keywords can be used in the path, to remove ambiguity
when accessing items.

{super.rs}

{super.play}

{super.out}

Structs have an extra level of visibility, their fields can be public or
private. This allows encapsulation.

{structs.rs}

{structs.play}

{structs.out}

Modules can be mapped to a file/directory hierarchy. Let's break down the
second example in files:

```
$ tree .
.
|-- my
|   |-- inaccessible.rs
|   |-- mod.rs
|   `-- nested.rs
`-- split.rs
```

{split.rs}

{my/mod.rs}

{my/nested.rs}

{my/inaccessible.rs}

{split.out}
