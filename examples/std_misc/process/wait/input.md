If you'd like to wait for a `process::Child` to finish, you must call
`Child::wait`, which will return a `process::ExitStatus`.

{wait.rs}

```
$ rustc wait.rs && ./wait
reached end of main
# `wait` keeps running for 5 seconds
# `sleep 5` command ends, and then our `wait` program finishes
```
