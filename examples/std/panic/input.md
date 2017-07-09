Макрос `panic!` используется для генерации паники и раскрутки стека.
Во время раскрутки стека, среда выполнения возьмёт на себя всю ответственность по
освобождению ресурсов, которыми *владеет* текущий поток, вызывая деструкторы
всех объектов.

Так как в данном случае мы имеем дело с однопоточной программой, `panic!` заставит
программу вывести сообщение с ошибкой и завершится.

{panic.play}

Давайте убедимся, что `panic!` не приводит к утечки памяти.

```
$ rustc panic.rs && valgrind ./panic
==4401== Memcheck, a memory error detector
==4401== Copyright (C) 2002-2013, and GNU GPL'd, by Julian Seward et al.
==4401== Using Valgrind-3.10.0.SVN and LibVEX; rerun with -h for copyright info
==4401== Command: ./panic
==4401==
thread '<main>' panicked at 'division by zero', panic.rs:5
==4401==
==4401== HEAP SUMMARY:
==4401==     in use at exit: 0 bytes in 0 blocks
==4401==   total heap usage: 18 allocs, 18 frees, 1,648 bytes allocated
==4401==
==4401== All heap blocks were freed -- no leaks are possible
==4401==
==4401== For counts of detected and suppressed errors, rerun with: -v
==4401== ERROR SUMMARY: 0 errors from 0 contexts (suppressed: 0 from 0)
```
