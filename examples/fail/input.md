The `panic!` macro can be used to generate a *task* failure and start unwinding
its stack. While unwinding, the runtime will take care of freeing all the
resources *owned* by the task by calling the destructor of all its objects.

Since we are dealing with programs with only one task, `panic!` will cause the
program to report the failure message and exit.

{fail.play}

Let's check that `panic!` doesn't leak memory.

```
$ rustc fail.rs && valgrind ./fail
==2614== Memcheck, a memory error detector
==2614== Copyright (C) 2002-2013, and GNU GPL'd, by Julian Seward et al.
==2614== Using Valgrind-3.9.0 and LibVEX; rerun with -h for copyright info
==2614== Command: ./fail
==2614==
task '<main>' failed at 'division by zero', fail.rs:5
==2614==
==2614== HEAP SUMMARY:
==2614==     in use at exit: 0 bytes in 0 blocks
==2614==   total heap usage: 15 allocs, 15 frees, 928 bytes allocated
==2614==
==2614== All heap blocks were freed -- no leaks are possible
==2614==
==2614== For counts of detected and suppressed errors, rerun with: -v
==2614== ERROR SUMMARY: 0 errors from 0 contexts (suppressed: 2 from 2)
```
