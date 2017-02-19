Variables in Rust do more than just hold data in the stack: they also *own*
resources, e.g. `Box<T>` owns memory in the heap. Rust enforces [RAII][raii]
(Resource Acquisition Is Initialization), so whenever an object goes out of 
scope, its destructor is called and its owned resources are freed. 

This behavior shields against *resource leak* bugs, so you'll never have to 
manually free memory or worry about memory leaks again! Here's a quick showcase:

{raii.play}

Of course, we can double check for memory errors using [`valgrind`][valgrind]:

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

No leaks here!

### See also:

[Box][box]

[raii]: http://en.wikipedia.org/wiki/Resource_Acquisition_Is_Initialization
[box]: /std/box.html
[valgrind]: http://valgrind.org/info/