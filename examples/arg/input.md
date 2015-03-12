The command line arguments can be accessed using `std::env::args`, which
returns an iterator that yields a String for each argument:

{args.play}

```
$ ./args 1 2 3
My path is ./args.
I got 3 arguments: ["1", "2", "3"].
```
