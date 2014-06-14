A `Timer` represents an underlying OS timer, and can generate *one-shot* and
*periodic* notifications via the `Receiver` endpoint of a channel.

{timers.play}

The playpen has a time limit, so you won't be able to see the (full) output in
the editor. Here's the output you should see, if you run this in a computer.

```
$ rustc timers.rs && time ./timers
Wait 1000 ms...
Done
Sleep for 1000 ms...
Done
Countdown
5
4
3
2
1
Ignition!
./timers  0.00s user 0.00s system 0% cpu 8.003 total
```
