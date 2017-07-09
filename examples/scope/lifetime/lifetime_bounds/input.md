Just like generic types can be bounded, lifetimes (themselves generic)
use bounds as well. The `:` character has a slightly different meaning here,
but `+` is the same. Note how the following read:

1. `T: 'a`: *All* references in `T` must outlive lifetime `'a`.
2. `T: Trait + 'a`: Type `T` must implement trait `Trait` and *all* references
in `T` must outlive `'a`.

The example below shows the above syntax in action:

{bounds.play}

### Смотрите также:

[generics][generics], [bounds in generics][bounds], and
[multiple bounds in generics][multibounds]

[generics]: ../../generics.html
[bounds]: ../../generics/bounds.html
[multibounds]: ../../generics/multi_bounds.html
