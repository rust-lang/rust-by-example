Just like generic types can be bounded, lifetimes as generics themselves
utilize bounds also. `:` has a slightly different meaning than in
[generics][bounds] but `+` hasn't changed. Both are described below:

1. `T: 'a`: *All* references in `T` must outlive lifetime `'a`.
2. `T: Trait + 'a`: Type `T` must implement trait `Trait` and *all* references
in `T` must outlive `'a`.

{bounds.play}

### See also:

[generics][generics] and [bounds in generics][bounds]

[generics]: /generics.html
[bounds]: /generics/bounds.html
