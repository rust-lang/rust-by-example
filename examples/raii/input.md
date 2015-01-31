Variables in Rust do more than just hold data in the stack: they can also *own*
resources, e.g. `Box<T>` owns memory in the heap. Because Rust enforces the
[RAII][raii]
discipline, whenever an object goes out of scope, its destructor is called
and the resources *owned* by it are freed. This behavior shields against
*resource leak* bugs.

{raii.play}

Don't take my word for it, let's check using `valgrind`:

```
$ rustc raii.rs && valgrind ./raii
==26873== Memcheck, a memory error detector
==26873== Copyright (C) 2002-2013, and GNU GPL'd, by Julian Seward et al.
==26873== Using Valgrind-3.9.0 and LibVEX; rerun with -h for copyright info
==26873== Command: ./raii
==26873==
==26873==
==26873== HEAP SUMMARY:
==26873==     in use at exit: 0 bytes in 0 blocks
==26873==   total heap usage: 1,013 allocs, 1,013 frees, 8,696 bytes allocated
==26873==
==26873== All heap blocks were freed -- no leaks are possible
==26873==
==26873== For counts of detected and suppressed errors, rerun with: -v
==26873== ERROR SUMMARY: 0 errors from 0 contexts (suppressed: 2 from 2)
```

You'll never have to manually free memory or worry about memory leaks again!

[raii]: http://en.wikipedia.org/wiki/Resource_Acquisition_Is_Initialization
