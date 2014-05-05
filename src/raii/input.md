Variables in Rust do more that just hold data in the stack, they can also *own*
resources, e.g. `Box<T>` owns memory in the heap. Because Rust enforces the
RAII discipline, whenever an object goes out of scope, its destructor is called
and the resources *owned* by it are freed. This behavior shields against
*resource leak* bugs.

Don't take my word for it, let's check using `valgrind`

{raii.rs}

```
$ rustc raii.rs && valgrind ./raii
==7619== Memcheck, a memory error detector
==7619== Copyright (C) 2002-2013, and GNU GPL'd, by Julian Seward et al.
==7619== Using Valgrind-3.9.0 and LibVEX; rerun with -h for copyright info
==7619== Command: ./raii
==7619==
==7619==
==7619== HEAP SUMMARY:
==7619==     in use at exit: 0 bytes in 0 blocks
==7619==   total heap usage: 1,032 allocs, 1,032 frees, 8,888 bytes allocated
==7619==
==7619== All heap blocks were freed -- no leaks are possible
==7619==
==7619== For counts of detected and suppressed errors, rerun with: -v
==7619== ERROR SUMMARY: 0 errors from 0 contexts (suppressed: 2 from 2)
```

You'll never have to manually free memory again or worry about memory leaks!
