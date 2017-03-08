The `std::io::fs` module contains several functions that deal with the
filesystem.

{fs.rs}

Here's the expected successful output:

```
$ rustc fs.rs && ./fs
`mkdir a`
`echo hello > a/b.txt`
`mkdir -p a/c/d`
`touch a/c/e.txt`
`ln -s ../b.txt a/c/b.txt`
`cat a/c/b.txt`
> hello
`ls a`
> "a/b.txt"
> "a/c"
`rm a/c/e.txt`
`rmdir a/c/d`
```

And the final state of the `a` directory is:

```
$ tree a
a
|-- b.txt
`-- c
    `-- b.txt -> ../b.txt

1 directory, 2 files
```

### Смотрите также:

[`cfg!`][cfg]

[cfg]: ../attribute/cfg.html
