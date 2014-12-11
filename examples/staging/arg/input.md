The command line arguments can be accessed using `std::os::args`, which returns
a [vector](http://static.rust-lang.org/doc/master/std/vec/index.html) of strings:

{args.play}

```
$ ./args 1 2 3
My path is ./args.
I got 3 arguments: [1, 2, 3].
```
