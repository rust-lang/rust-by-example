When dealing with resources, the default behavior is to transfer them during
assignments or function calls. However, sometimes we need to make a 
copy of the resource as well.

The [`Clone`][clone] trait helps us do exactly this. Most commonly, we can 
use the `.clone()` method defined by the `Clone` trait.

{clone.play}

[clone]: https://doc.rust-lang.org/std/clone/trait.Clone.html